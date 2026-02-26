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
//! - **Last Update**: 02/26/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//! - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

pub mod ansi;
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
/// Orchestrates the five subsystems that make up the terminal:
/// 1. `spawn_reader_thread()`, background thread reading PTY master → mpsc channel
/// 2. `setup_draw_func()`, Cairo rendering of text buffer and cursor
/// 3. `setup_poll_timer()`, 16ms timer draining channel → buffer → redraw
/// 4. `setup_blink_timer()`, 500ms cursor blink toggle
/// 5. `setup_keyboard()`, keyboard event controller writing to PTY master
///
fn build_ui(app: &Application, master_fd: RawFd) -> () {
    let text_buffer: Rc<RefCell<Vec<ansi::StyledSpan>>> = Rc::new(RefCell::new(Vec::new()));
    let cursor_visible: Rc<RefCell<bool>> = Rc::new(RefCell::new(true));
    let ansi_parser: ansi::AnsiParser = ansi::AnsiParser::new();

    let rx = spawn_reader_thread(master_fd);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(config::WINDOW_TITLE)
        .default_width(config::WINDOW_WIDTH)
        .default_height(config::WINDOW_HEIGHT)
        .build();

    let drawing_area = DrawingArea::new();
    drawing_area.set_focusable(true);

    setup_draw_func(&drawing_area, &text_buffer, &cursor_visible);
    setup_poll_timer(
        rx,
        &drawing_area,
        &text_buffer,
        &cursor_visible,
        &window,
        ansi_parser,
    );
    setup_blink_timer(&drawing_area, &cursor_visible);
    setup_keyboard(&drawing_area, master_fd);

    window.set_child(Some(&drawing_area));
    window.present();
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     spawn_reader_thread(master_fd: RawFd) -> std::sync::mpsc::Receiver<Vec<u8>>
/// ```
/// Spawns a background thread that reads raw bytes from the PTY master fd
/// and sends them to the GTK main thread via an mpsc channel.
///
/// ### Arguments
/// - `master_fd`: PTY master "RAW" file descriptor to read from
///
/// ### Returns
/// The receiving end of the channel. Each message is a `Vec<u8>` chunk
/// of raw PTY output (up to `config::PTY_READ_BUF_SIZE` bytes).
///
/// ### Thread Lifecycle
/// The thread terminates when:
/// - `read()` returns 0 (EOF, shell closed its stdout)
/// - `read()` returns `Errno::EIO` (slave side of PTY closed)
/// - The channel receiver is dropped (GTK side shut down)
///
fn spawn_reader_thread(master_fd: RawFd) -> std::sync::mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();

    let reader_fd: RawFd = master_fd;
    std::thread::spawn(move || {
        let mut buf: [u8; config::PTY_READ_BUF_SIZE] = [0u8; config::PTY_READ_BUF_SIZE];
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

    return rx;
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     setup_draw_func(
///         drawing_area: &DrawingArea,
///         text_buffer: &Rc<RefCell<String>>,
///         cursor_visible: &Rc<RefCell<bool>>,
///     ) -> ()
/// ```
/// Installs the Cairo draw callback on the `DrawingArea`.
///
/// ### Arguments
/// - `drawing_area`: GTK drawing surface to attach the callback to
/// - `text_buffer`: Shared terminal text (read during each frame)
/// - `cursor_visible`: Shared blink state (read during each frame)
///
/// ### Rendering
/// - Fills the background with `config::BG_COLOR`
/// - Renders the last N lines (that fit the window height) in monospace
/// - Draws a block cursor at the end of the last visible line when `cursor_visible` is true
///
fn setup_draw_func(
    drawing_area: &DrawingArea,
    text_buffer: &Rc<RefCell<Vec<ansi::StyledSpan>>>,
    cursor_visible: &Rc<RefCell<bool>>,
) -> () {
    let buf_for_draw = text_buffer.clone();
    let cv_for_draw = cursor_visible.clone();

    drawing_area.set_draw_func(move |_widget, ctx, width, height| {
        //-- Background fill --//
        ctx.set_source_rgb(config::BG_COLOR.0, config::BG_COLOR.1, config::BG_COLOR.2);
        ctx.rectangle(0.0, 0.0, width as f64, height as f64);
        let _ = ctx.fill();

        ctx.select_font_face(
            config::FONT_FAMILY,
            gtk::cairo::FontSlant::Normal,
            gtk::cairo::FontWeight::Normal,
        );
        ctx.set_font_size(config::FONT_SIZE);

        let font_ext = ctx.font_extents().unwrap();
        let cell_w = ctx.text_extents("M").map(|e| e.x_advance()).unwrap_or(8.0);
        let cell_h = font_ext.ascent() + font_ext.descent();

        let spans = buf_for_draw.borrow();
        let all_lines = ansi::build_render_lines(&spans);
        let max_lines = (height as f64 / config::LINE_HEIGHT) as usize;
        let start = all_lines.len().saturating_sub(max_lines);
        let visible = &all_lines[start..];

        let mut y = config::LINE_HEIGHT;
        for line_segs in visible {
            let mut x = config::TEXT_PADDING;

            for (text, style) in line_segs {
                let text_advance = ctx
                    .text_extents(text)
                    .map(|e| e.x_advance())
                    .unwrap_or(cell_w * text.chars().count() as f64);

                //-- Draw background cell when it differs from the terminal default --//
                if style.bg != ansi::Color::Default {
                    let (br, bg, bb) = style.bg.to_rgb_f64(config::BG_COLOR);
                    ctx.set_source_rgb(br, bg, bb);
                    ctx.rectangle(x, y - font_ext.ascent(), text_advance, cell_h);
                    let _ = ctx.fill();
                }

                let (fr, fg, fb) = if style.reverse {
                    style.bg.to_rgb_f64(config::BG_COLOR)
                } else {
                    style.fg.to_rgb_f64(config::FG_COLOR)
                };

                ctx.set_source_rgb(fr, fg, fb);
                ctx.move_to(x, y);
                let _ = ctx.show_text(text);

                x += text_advance;
            }

            y += config::LINE_HEIGHT;
        }

        //-- Draw block cursor --//
        if *cv_for_draw.borrow() {
            let last_segs = visible.last().map(|v| v.as_slice()).unwrap_or(&[]);
            let num_visible = visible.len().max(1);
            let cursor_baseline = config::LINE_HEIGHT * num_visible as f64;

            let cursor_x = {
                let mut cx = config::TEXT_PADDING;
                for (text, _) in last_segs {
                    cx += ctx
                        .text_extents(text)
                        .map(|e| e.x_advance())
                        .unwrap_or(cell_w * text.chars().count() as f64);
                }
                cx
            };

            ctx.set_source_rgb(config::FG_COLOR.0, config::FG_COLOR.1, config::FG_COLOR.2);
            ctx.rectangle(
                cursor_x,
                cursor_baseline - font_ext.ascent(),
                cell_w,
                cell_h,
            );
            let _ = ctx.fill();
        }
    });
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     setup_poll_timer(
///         rx: std::sync::mpsc::Receiver<Vec<u8>>,
///         drawing_area: &DrawingArea,
///         text_buffer: &Rc<RefCell<String>>,
///         cursor_visible: &Rc<RefCell<bool>>,
///     ) -> ()
/// ```
/// Starts a 16ms (~60 fps) timer on the GTK main loop that drains the
/// reader thread's channel and updates the terminal display.
///
/// ### Arguments
/// - `rx`: Receiving end of the PTY reader channel (consumed, moved into closure)
/// - `drawing_area`: Widget to `queue_draw()` when new output arrives
/// - `text_buffer`: Shared buffer to append processed output into
/// - `cursor_visible`: Reset to `true` on new output (interrupts blink)
/// - `window`: Application window, closed when the shell exits (triggers app quit)
///
/// ### Behavior
/// Each tick drains all pending messages from `rx`, feeds each chunk
/// through `process_pty_output()`, and triggers a redraw if anything changed.
/// When the channel disconnects (shell exited, reader thread ended),
/// closes the window which causes the GTK application to quit.
///
fn setup_poll_timer(
    rx: std::sync::mpsc::Receiver<Vec<u8>>,
    drawing_area: &DrawingArea,
    text_buffer: &Rc<RefCell<Vec<ansi::StyledSpan>>>,
    cursor_visible: &Rc<RefCell<bool>>,
    window: &ApplicationWindow,
    parser: ansi::AnsiParser,
) -> () {
    let buf_for_rx = text_buffer.clone();
    let cv_for_rx = cursor_visible.clone();
    let da_for_rx = drawing_area.clone();
    let win_for_rx = window.clone();
    let mut parser = parser;

    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        let mut needs_redraw = false;

        loop {
            match rx.try_recv() {
                Ok(data) => {
                    let bell = process_pty_output(&mut buf_for_rx.borrow_mut(), &mut parser, &data);
                    if bell {
                        if let Some(display) = gdk::Display::default() {
                            display.beep();
                        }
                    }
                    needs_redraw = true;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    //-- Shell exited: close window → quit app --//
                    win_for_rx.close();
                    return glib::ControlFlow::Break;
                }
            }
        }

        if needs_redraw {
            *cv_for_rx.borrow_mut() = true;
            da_for_rx.queue_draw();
        }

        glib::ControlFlow::Continue
    });
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     setup_blink_timer(
///         drawing_area: &DrawingArea,
///         cursor_visible: &Rc<RefCell<bool>>,
///     ) -> ()
/// ```
/// Starts a 500ms timer that toggles cursor visibility and redraws.
///
/// ### Arguments
/// - `drawing_area`: Widget to `queue_draw()` on each blink toggle
/// - `cursor_visible`: Shared flag toggled every 500ms
///
fn setup_blink_timer(drawing_area: &DrawingArea, cursor_visible: &Rc<RefCell<bool>>) -> () {
    let cv_for_blink = cursor_visible.clone();
    let da_for_blink = drawing_area.clone();

    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let mut vis = cv_for_blink.borrow_mut();
        *vis = !*vis;
        da_for_blink.queue_draw();
        glib::ControlFlow::Continue
    });
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     setup_keyboard(drawing_area: &DrawingArea, master_fd: RawFd) -> ()
/// ```
/// Installs a keyboard event controller on the `DrawingArea` that translates
/// GTK key events into byte sequences and writes them to the PTY master fd.
///
/// ### Arguments
/// - `drawing_area`: Widget to attach the `EventControllerKey` to
/// - `master_fd`: PTY master fd to write translated key bytes into
///
/// ### Key Mapping
/// | Input              | Bytes written to PTY        |
/// |--------------------|-----------------------------|
/// | Ctrl+\<letter\>    | Control character 0x01-0x1A |
/// | Enter              | `\r`                        |
/// | Backspace          | `\x7f` (DEL)               |
/// | Tab                | `\t`                        |
/// | Escape             | `\x1b`                      |
/// | Arrow keys         | `\x1b[A`-`\x1b[D`          |
/// | Delete             | `\x1b[3~`                   |
/// | Home / End         | `\x1b[H` / `\x1b[F`        |
/// | Printable chars    | UTF-8 encoded bytes         |
///
fn setup_keyboard(drawing_area: &DrawingArea, master_fd: RawFd) -> () {
    let key_controller = EventControllerKey::new();
    let writer_fd = master_fd;

    key_controller.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
        //-- Ctrl+<key> → control character (0x01-0x1A) --//
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

    drawing_area.add_controller(key_controller);
}

///
/// #### **<ins>Function</ins>**
/// ```Rust
///     process_pty_output(
///         buffer: &mut Vec<ansi::StyledSpan>,
///         parser: &mut ansi::AnsiParser,
///         data: &[u8],
///     ) -> bool
/// ```
/// Feeds a raw PTY byte chunk through the ANSI parser and applies the
/// resulting actions to the styled span buffer.
///
/// ### Arguments
/// - `buffer`: Mutable reference to the terminal's accumulated styled span buffer
/// - `parser`: The stateful [`ansi::AnsiParser`] that owns current SGR state
/// - `data`: Raw bytes read from the PTY master fd
///
/// ### Returns
/// `true` if a `BEL` (`\x07`) character was encountered in `data`; the
/// caller is responsible for ringing the system bell.
///
/// ### Behavior
/// Delegates all byte interpretation to [`ansi::AnsiParser::feed`] and
/// applies each emitted [`ansi::Action`] to `buffer` via [`ansi::apply_action`].
/// This includes styled text appending, SGR color/attribute tracking,
/// backspace, carriage return, and display-erase handling.
///
fn process_pty_output(
    buffer: &mut Vec<ansi::StyledSpan>,
    parser: &mut ansi::AnsiParser,
    data: &[u8],
) -> bool {
    let mut bell = false;

    for action in parser.feed(data) {
        //-- Surface BEL to the caller before applying --//
        if let ansi::Action::Bell = action {
            bell = true;
        }
        ansi::apply_action(buffer, action);
    }

    bell
}
