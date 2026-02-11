use std::num::NonZeroU32;
use std::sync::Arc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowAttributes, WindowId};

const BG_COLOR: u32 = 0x1E_1E_2E;

/*-- Terminal application state : Window + Softbuffer --*/
struct TerminalApp {
    context: Context<OwnedDisplayHandle>,
    window: Option<Arc<Window>>,
    surface: Option<Surface<OwnedDisplayHandle, Arc<Window>>>,
}

/**
 * ApplicationHandler implementation for TerminalApp.
 * Handles window events and drawing the terminal surface.
 */
impl ApplicationHandler for TerminalApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title("RushX Terminal")
            .with_inner_size(LogicalSize::new(800u32, 500u32));

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        );

        let surface = Surface::new(&self.context, window.clone())
            .expect("failed to create softbuffer surface");

        self.surface = Some(surface);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            WindowEvent::Resized(_) => {
                if let Some(win) = self.window.as_ref() {
                    win.request_redraw();
                }
            }
            _ => {}
        }
    }
}

impl TerminalApp {
    fn draw(&mut self) {
        let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut()) else {
            return;
        };

        let size = window.inner_size();
        let (Some(width), Some(height)) = (
            NonZeroU32::new(size.width),
            NonZeroU32::new(size.height),
        ) else {
            return;
        };

        surface
            .resize(width, height)
            .expect("failed to resize surface");

        let mut buffer = surface.buffer_mut().expect("failed to get buffer");

        for pixel in buffer.iter_mut() {
            *pixel = BG_COLOR;
        }

        buffer.present().expect("failed to present buffer");
    }
}

/*-- Launch the RushX terminal window --*/
pub fn run() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let context = Context::new(event_loop.owned_display_handle())
        .expect("failed to create softbuffer context");

    let mut app = TerminalApp {
        context,
        window: None,
        surface: None,
    };

    event_loop
        .run_app(&mut app)
        .expect("event loop terminated with error");
}
