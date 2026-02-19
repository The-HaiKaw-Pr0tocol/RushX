//!
//! ## `rushx_term` <ins>module</ins>: Terminal Emulator (GTK4)
//!
//! GTK4-based terminal emulator GUI with integrated PTY backend. Spawns a
//! shell child process over a pseudoterminal, reads its output via the PTY
//! master fd, and renders text in a `DrawingArea`. Keyboard input is
//! captured and forwarded to the shell through the PTY master.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_term/mod.rs
//! - **Module**: rushx_term
//! - **Last Update**: 02/18/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

pub mod config;
pub mod pty;

use std::cell::RefCell;
use std::os::fd::RawFd;
use std::rc::Rc;

use gtk::gdk;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, EventControllerKey};
use gtk4 as gtk;

use nix::errno::Errno;
use nix::unistd;

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     run_rushx_terminal() -> ()
/// ```
/// Launches the RushX terminal emulator with an attached shell.
///
/// ### Behavior
/// 1. Allocates a PTY master/slave pair via `pty::open_pty_pair()`
/// 2. Spawns the shell child process attached to the slave fd via `pty::spawn_shell()`
/// 3. Initializes the GTK application and enters the main event loop
///
/// ### Panics
/// - If PTY pair allocation fails
/// - If shell spawning (fork) fails
///
pub fn run_rushx_terminal() -> () {
    let pty_pair = pty::open_pty_pair().expect("Failed to allocate PTY pair");
    let shell = pty::spawn_shell(pty_pair).expect("Failed to spawn shell process");

    let master_raw: RawFd = shell.master_fd;

    //-- GTK initialization -//
    let app = Application::builder()
        .application_id(config::APP_ID)
        .build();

    app.connect_activate(move |app| {
        build_ui(app, master_raw);
    });

    app.run();
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     build_ui(app: &Application, master_fd: RawFd) -> ()
/// ```
/// Constructs the GTK terminal window and wires PTY I/O.
///
/// ### Arguments
/// - `app`: GTK Application handle
/// - `master_fd`: Raw PTY master file descriptor for bidirectional shell I/O
///
/// ### Behavior
/// 1. Allocates shared state for the terminal text buffer and cursor visibility
/// 2. Spawns a thread to read from `master_fd` and send output to the main thread via an mpsc channel
/// 3. Sets up a `DrawingArea` with a custom draw function to render the terminal text and cursor
/// 4. Sets up a keyboard event controller to capture key presses and write them to `master_fd` as input to the shell
/// 5. Starts timers for redrawing on new output and blinking the cursor
/// 6. Presents the window and enters the GTK main loop
///
fn build_ui(app: &Application, master_fd: RawFd) -> () {
    let text_buffer: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
    let cursor_visible: Rc<RefCell<bool>> = Rc::new(RefCell::new(true));

    //-- PTY reader thread → mpsc channel → GTK main loop --//
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();

    let reader_fd = master_fd;
    std::thread::spawn(move || {
        let mut buf = [0u8; config::PTY_READ_BUF_SIZE];
        loop {
            match unistd::read(reader_fd, &mut buf) {
                //-- EOF: shell closed --//
                Ok(0) => break,
                Ok(n) => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        //-- Receiver dropped --//
                        break;
                    }
                }
                //-- Slave side closed --//
                Err(Errno::EIO) => break,
                //-- Other read error --//
                Err(_) => break,
            }
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title(config::WINDOW_TITLE)
        .default_width(config::WINDOW_WIDTH)
        .default_height(config::WINDOW_HEIGHT)
        .build();

    let drawing_area = DrawingArea::new();
    drawing_area.set_focusable(true);

    let buf_for_draw = text_buffer.clone();
    let cv_for_draw = cursor_visible.clone();
    drawing_area.set_draw_func(move |_widget, ctx, width, height| {
        //-- Background fill --//
        ctx.set_source_rgb(config::BG_COLOR.0, config::BG_COLOR.1, config::BG_COLOR.2);
        ctx.rectangle(0.0, 0.0, width as f64, height as f64);
        let _ = ctx.fill();

        //-- Font setup --//
        ctx.select_font_face(
            config::FONT_FAMILY,
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Normal,
        );
        ctx.set_font_size(config::FONT_SIZE);

        //-- Draw text lines --//
        ctx.set_source_rgb(config::FG_COLOR.0, config::FG_COLOR.1, config::FG_COLOR.2);

        let text = buf_for_draw.borrow();
        let max_lines = (height as f64 / config::LINE_HEIGHT) as usize;
        let lines: Vec<&str> = text.split('\n').collect();
        let start = lines.len().saturating_sub(max_lines);
        let visible = &lines[start..];

        let mut y = config::LINE_HEIGHT;
        for line in visible {
            ctx.move_to(config::TEXT_PADDING, y);
            let _ = ctx.show_text(line);
            y += config::LINE_HEIGHT;
        }

        //-- Draw cursor --//
        if *cv_for_draw.borrow() {
            let last_line = visible.last().copied().unwrap_or("");
            let num_visible = visible.len().max(1);
            let cursor_baseline = config::LINE_HEIGHT * num_visible as f64;

            let cursor_x = if last_line.is_empty() {
                config::TEXT_PADDING
            } else {
                let ext = ctx.text_extents(last_line).unwrap();
                config::TEXT_PADDING + ext.x_advance()
            };

            let font_ext = ctx.font_extents().unwrap();
            let char_ext = ctx.text_extents("M").unwrap();

            ctx.set_source_rgb(config::FG_COLOR.0, config::FG_COLOR.1, config::FG_COLOR.2);
            ctx.rectangle(
                cursor_x,
                cursor_baseline - font_ext.ascent(),
                char_ext.x_advance(),
                font_ext.ascent() + font_ext.descent(),
            );
            let _ = ctx.fill();
        }
    });

    let buf_for_rx = text_buffer.clone();
    let cv_for_rx = cursor_visible.clone();
    let da_for_rx = drawing_area.clone();

    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        let mut needs_redraw = false;

        while let Ok(data) = rx.try_recv() {
            process_pty_output(&mut buf_for_rx.borrow_mut(), &data);
            needs_redraw = true;
        }

        if needs_redraw {
            //-- Reset cursor to visible on new output --//
            *cv_for_rx.borrow_mut() = true;
            da_for_rx.queue_draw();
        }

        glib::ControlFlow::Continue
    });

    //-- Cursor blink timer (~500 ms) --//
    let cv_for_blink = cursor_visible.clone();
    let da_for_blink = drawing_area.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let mut vis = cv_for_blink.borrow_mut();
        *vis = !*vis;
        da_for_blink.queue_draw();
        glib::ControlFlow::Continue
    });

    //-- Keyboard input → PTY master fd --//
    let key_controller = EventControllerKey::new();
    let writer_fd = master_fd;

    key_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
        //-- Ctrl+<key> → control character (0x01–0x1A) --//
        if state.contains(gdk::ModifierType::CONTROL_MASK) {
            if let Some(ch) = keyval.to_unicode() {
                if ch.is_ascii_alphabetic() {
                    let ctrl_byte = (ch.to_ascii_lowercase() as u8) - b'a' + 1;
                    let _ = unistd::write(writer_fd, &[ctrl_byte]);
                    return glib::Propagation::Stop;
                }
            }
        }

        //-- Special keys --//
        match keyval {
            gdk::Key::Return | gdk::Key::KP_Enter => {
                let _ = unistd::write(writer_fd, b"\r");
            }
            gdk::Key::BackSpace => {
                let _ = unistd::write(writer_fd, b"\x7f");
            }
            gdk::Key::Tab => {
                let _ = unistd::write(writer_fd, b"\t");
            }
            gdk::Key::Escape => {
                let _ = unistd::write(writer_fd, b"\x1b");
            }
            //-- Arrow keys → ANSI escape sequences --//
            gdk::Key::Up => {
                let _ = unistd::write(writer_fd, b"\x1b[A");
            }
            gdk::Key::Down => {
                let _ = unistd::write(writer_fd, b"\x1b[B");
            }
            gdk::Key::Right => {
                let _ = unistd::write(writer_fd, b"\x1b[C");
            }
            gdk::Key::Left => {
                let _ = unistd::write(writer_fd, b"\x1b[D");
            }
            gdk::Key::Delete => {
                let _ = unistd::write(writer_fd, b"\x1b[3~");
            }
            gdk::Key::Home => {
                let _ = unistd::write(writer_fd, b"\x1b[H");
            }
            gdk::Key::End => {
                let _ = unistd::write(writer_fd, b"\x1b[F");
            }
            //-- Printable characters → UTF-8 bytes --//
            _ => {
                if let Some(ch) = keyval.to_unicode() {
                    let mut utf8_buf = [0u8; 4];
                    let encoded = ch.encode_utf8(&mut utf8_buf);
                    let _ = unistd::write(writer_fd, encoded.as_bytes());
                }
            }
        }

        glib::Propagation::Stop
    });

    //-- Assemble and present --//
    window.add_controller(key_controller);
    window.set_child(Some(&drawing_area));
    window.present();
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     process_pty_output(buffer: &mut String, data: &[u8]) -> ()
/// ```
/// Interprets raw PTY output bytes and updates the text buffer in place.
///
/// ### Arguments
/// - `buffer`: Mutable reference to the accumulated terminal text
/// - `data`: Raw bytes read from the PTY master fd
///
/// ### Behavior
/// Processes each character sequentially, handling:
/// - `\x08` (BS): Erases the last character on the current line
/// - `\r\n`: Treated as a single newline
/// - `\r` alone: Truncates back to the start of the current line (carriage return)
/// - `\x1b[…`: Strips CSI escape sequences (cursor movement, colors, etc.)
/// - `\x1b]…`: Strips OSC escape sequences (e.g. terminal title changes)
/// - `\x07` (BEL), `\x00` (NUL): Silently ignored
/// - All other characters: Appended to the buffer verbatim
///
fn process_pty_output(buffer: &mut String, data: &[u8]) {
    let text = String::from_utf8_lossy(data);
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            //-- Backspace: erase last char on current line --//
            '\x08' => {
                if !buffer.is_empty() && !buffer.ends_with('\n') {
                    buffer.pop();
                }
            }
            //-- Carriage return --//
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    //-- \r\n → single newline --//
                    chars.next();
                    buffer.push('\n');
                } else {
                    //-- Standalone \r → rewind to start of current line --//
                    if let Some(pos) = buffer.rfind('\n') {
                        buffer.truncate(pos + 1);
                    } else {
                        buffer.clear();
                    }
                }
            }
            //-- ANSI escape sequences --//
            '\x1b' => {
                match chars.peek() {
                    //-- CSI sequence: \x1b[ … <final byte> --//
                    Some(&'[') => {
                        chars.next();
                        loop {
                            match chars.next() {
                                Some(c) if ('@'..='~').contains(&c) => break,
                                Some(_) => continue,
                                None => break,
                            }
                        }
                    }
                    //-- OSC sequence: \x1b] … (BEL | ST) --//
                    Some(&']') => {
                        chars.next();
                        loop {
                            match chars.next() {
                                Some('\x07') => break,
                                Some('\x1b') => {
                                    if chars.peek() == Some(&'\\') {
                                        chars.next();
                                        break;
                                    }
                                }
                                Some(_) => continue,
                                None => break,
                            }
                        }
                    }
                    //-- Other single-char escapes: skip --//
                    _ => {}
                }
            }
            //-- Bell, null: ignore --//
            '\x07' | '\x00' => {}
            //-- Everything else: append --//
            _ => {
                buffer.push(ch);
            }
        }
    }
}
