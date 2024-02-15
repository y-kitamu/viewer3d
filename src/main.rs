mod io;
mod shader;
mod view;
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
    fn get_view(&self) -> &Box<dyn View> {
        &self.views[self.current_view]
    }

    fn get_view_mut(&mut self) -> &mut Box<dyn View> {
        &mut self.views[self.current_view]
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("image viewer")
        .build(&event_loop);

    let mut view2d = Simple2DView::new(&display);
    view2d.set_image(
        &display,
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/opengl.png")),
    );
    let mut view3d = Simple3DView::new(&display);
    view3d.set_image(
        &display,
        std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/cas/1-200/1.img.nii.gz"
        )),
    );

    let mut view_mode = ViewMode {
        current_view: 0,
        views: vec![Box::new(view3d), Box::new(view2d)],
    };

    event_loop
        .run(move |event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
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
