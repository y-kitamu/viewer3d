mod io;
mod shader;
mod view;
use tracing::info;
use tracing_subscriber;
use view::simple::Simple2DView;
use view::simple3d::Simple3DView;
use view::View;
use winit::{
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

struct ViewMode {
    current_view: usize,
    views: Vec<Box<dyn View>>,
}

impl ViewMode {
    fn set_2d_view(&mut self) {
        info!("Set current view to 2D");
        self.current_view = 1;
    }

    fn set_3d_view(&mut self) {
        info!("Set current view to 3D");
        self.current_view = 0;
    }

    fn get_view_mut(&mut self) -> &mut Box<dyn View> {
        &mut self.views[self.current_view]
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting image viewer");

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop building");
    // event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("image viewer")
        .build(&event_loop);

    let view2d = Simple2DView::new(&display);
    let view3d = Simple3DView::new(&display);

    let mut view_mode = ViewMode {
        current_view: 0,
        views: vec![Box::new(view3d), Box::new(view2d)],
    };

    event_loop
        .run(move |event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::DroppedFile(path) => {
                    info!("dropped file: {:?}", path);
                    match path.extension() {
                        Some(ext) => match ext.to_ascii_lowercase().to_str().unwrap() {
                            "png" | "jpg" | "jpeg" => {
                                view_mode.set_2d_view();
                            }
                            _ => view_mode.set_3d_view(),
                        },
                        None => (),
                    }
                    view_mode.get_view_mut().set_image(&display, &path);
                }
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(NamedKey::Escape) => window_target.exit(),
                        _ => view_mode
                            .get_view_mut()
                            .handle_keyboard_input(&display, &event),
                    }
                }
                winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                    view_mode
                        .get_view_mut()
                        .handle_modifiers_changed(&display, &modifiers);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    view_mode
                        .get_view_mut()
                        .handle_mouse_input(&display, &state, &button);
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    view_mode
                        .get_view_mut()
                        .handle_cursor_moved(&display, &position);
                }
                winit::event::WindowEvent::MouseWheel { delta, phase, .. } => {
                    view_mode
                        .get_view_mut()
                        .handle_mouse_wheel(&display, &delta, &phase);
                }
                winit::event::WindowEvent::RedrawRequested => {
                    view_mode.get_view_mut().draw(&display);
                }
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                    view_mode
                        .get_view_mut()
                        .handle_window_resized(&display, window_size);
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
