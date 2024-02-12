mod io;
mod shader;
mod view;
use view::simple::Simple2DView;
use view::View;
use winit::{
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("image viewer")
        .build(&event_loop);

    let image = io::load_image3d(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/"
    )));

    let mut view = Simple2DView::new(&display);
    view.set_image(
        &display,
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/opengl.png")),
    );

    event_loop
        .run(move |event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(NamedKey::Escape) => window_target.exit(),
                        _ => view.handle_keyboard_input(&display, &event),
                    }
                }
                winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                    view.handle_modifiers_changed(&display, &modifiers);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    view.handle_mouse_input(&display, &state, &button);
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    view.handle_cursor_moved(&display, &position);
                }
                winit::event::WindowEvent::MouseWheel { delta, phase, .. } => {
                    view.handle_mouse_wheel(&display, &delta, &phase);
                }
                winit::event::WindowEvent::RedrawRequested => {
                    view.draw(&display);
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
