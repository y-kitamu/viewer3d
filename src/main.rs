mod shader;
mod simple;
use simple::Simple2DView;
use winit::{
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #5")
        .build(&event_loop);

    let mut view = Simple2DView::new(&display);
    view.set_image(&display, include_bytes!("../assets/opengl.png"));

    event_loop
        .run(move |event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(NamedKey::Escape) => window_target.exit(),
                        _ => view.handle_keyboard_input(&event, &display, window_target),
                    }
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    view.handle_mouse_input(&state, &button);
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    view.handle_cursor_moved(&position);
                }
                winit::event::WindowEvent::MouseWheel { delta, phase, .. } => {
                    view.handle_mouse_wheel(&delta, &phase);
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
