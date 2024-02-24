use glium::glutin::surface::WindowSurface;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
};

pub trait View {
    fn draw(&self, display: &glium::Display<WindowSurface>);
    fn set_image(&mut self, display: &glium::Display<WindowSurface>, data_path: &std::path::Path);
    fn handle_keyboard_input(
        &mut self,
        display: &glium::Display<WindowSurface>,
        event: &winit::event::KeyEvent,
    );
    fn handle_modifiers_changed(
        &mut self,
        display: &glium::Display<WindowSurface>,
        modifiers: &winit::event::Modifiers,
    );
    fn handle_mouse_input(
        &mut self,
        display: &glium::Display<WindowSurface>,
        state: &ElementState,
        button: &MouseButton,
    );
    fn handle_cursor_moved(
        &mut self,
        display: &glium::Display<WindowSurface>,
        position: &PhysicalPosition<f64>,
    );
    fn handle_mouse_wheel(
        &mut self,
        display: &glium::Display<WindowSurface>,
        delta: &MouseScrollDelta,
        phase: &TouchPhase,
    );
    fn handle_window_resized(
        &mut self,
        display: &glium::Display<WindowSurface>,
        window_size: winit::dpi::PhysicalSize<u32>,
    );
}

pub mod simple;
pub mod simple3d;
