use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use gtk4 as gtk;

const BG_COLOR: (f64, f64, f64) = (0.117, 0.117, 0.180);

fn build_ui(app: &Application) {
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

/*-- Launch the RushX terminal window --*/
pub fn run() {
    let app = Application::builder()
        .application_id("org.rushx.terminal")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
