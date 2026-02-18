//!
//! ## `rushx_term` <ins>module</ins>: Terminal Emulator (GTK4)
//!
//! GTK4-based terminal emulator GUI. Manages application window lifecycle,
//! rendering pipeline, and drawing surface. Currently paints a solid background;
//! future iterations will integrate PTY I/O, ANSI parsing, and text rendering.
//!
//! ## Metadata
//!
//! - **File**: src/rushx_term/mod.rs
//! - **Module**: rushx_term
//! - **Last Update**: 02/17/2026
//! - **Last Updated By**: sch0penheimer
//! - **Version**: 0.1.0
//!  - **Copyright**: © 2026 The HaiKaw Pr0tocol
//!

/*=============================================================================*/

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk4 as gtk;

const BG_COLOR: (f64, f64, f64) = (0.117, 0.117, 0.180);

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     build_ui(app: &Application) -> ()
/// ```
/// Constructs the GTK terminal window UI.
///
/// ### Arguments
/// - `app`: GTK Application handle
///
/// ### Behavior
/// - Creates a 800x500 window
/// - Attaches a DrawingArea with solid background color
/// - Presents the window on screen
/// 
fn build_ui(app: &Application) -> () {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("RushX Terminal")
        .default_width(800)
        .default_height(500)
        .build();

    let drawing_area = DrawingArea::new();
    drawing_area.set_draw_func(|_, ctx, width, height| {
        ctx.set_source_rgb(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2);
        ctx.rectangle(0.0, 0.0, width as f64, height as f64);
        let _ = ctx.fill();
    });

    window.set_child(Some(&drawing_area));
    window.present();
}

///
/// #### **<ins>Function</ins>** 
/// ```Rust
///     run_rushx_terminal() -> ()
/// ```
/// Launches the RushX terminal emulator window.
///
/// ### Behavior
/// - Initializes GTK application with ID `org.rushx.terminal`
/// - Wires `build_ui` to activation signal
/// - Enters GTK main event loop (blocks until window closes)
pub fn run_rushx_terminal() -> () {
    let app = Application::builder()
        .application_id("org.rushx.terminal")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
