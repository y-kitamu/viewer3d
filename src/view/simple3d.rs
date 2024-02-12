use std::path::Path;

use glium;
use glium::glutin::surface::WindowSurface;
use glium::Surface;
use glium::{implement_vertex, uniform};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::ModifiersState;

use crate::shader;
use crate::shader::ShaderSrc;

#[derive(Copy, Clone)]
struct Simple3DVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct Simple3DView {
    indices: glium::index::NoIndices,
    vertex_buffer: glium::VertexBuffer<Simple3DVertex>,
    program: glium::Program,
    texture: glium::texture::Texture3d,
    matrix: [[f32; 4]; 4],
    is_left_button_pressed: bool,
    is_right_button_pressed: bool,
    is_shift_button_pressed: bool,
    prev_mouse_pos: Option<PhysicalPosition<f64>>,
}

impl Simple3DView {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        implement_vertex!(Simple3DVertex, position, tex_coords);
        let shape = vec![
            Simple3DVertex {
                position: [-0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            Simple3DVertex {
                position: [0.5, -0.5],
                tex_coords: [1.0, 0.0],
            },
            Simple3DVertex {
                position: [0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Simple3DVertex {
                position: [0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Simple3DVertex {
                position: [-0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Simple3DVertex {
                position: [-0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let shader = shader!("simple3d");
        let program = shader.compile(display);

        Simple3DView {
            indices,
            vertex_buffer,
            program,
            texture: glium::texture::Texture3d::empty(display, 0, 0, 0).unwrap(),
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            is_left_button_pressed: false,
            is_right_button_pressed: false,
            is_shift_button_pressed: false,
            prev_mouse_pos: None,
        }
    }
}

impl super::View for Simple3DView {
    fn set_image(&mut self, display: &glium::Display<WindowSurface>, data_path: &std::path::Path) {
        let image = crate::io::load_image3d(data_path);
        let image = glium::texture::RawImage3d {
            data: std::borrow::Cow::Borrowed(&image.data),
            width: image.shape.0,
            height: image.shape.1,
            depth: image.shape.2,
            format: glium::texture::ClientFormat::I16,
        };
        self.texture = glium::texture::Texture3d::new(display, image).unwrap();
    }

    fn draw(&self, display: &glium::Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            let f = 1.0;
            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };
        let uniforms = uniform! {
            tex: &self.texture,
            perspective: perspective,
            model: self.matrix,
        };
        target
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms,
                &glium::DrawParameters::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }

    fn handle_keyboard_input(
        &mut self,
        _display: &glium::Display<WindowSurface>,
        event: &winit::event::KeyEvent,
    ) {
        println!("{:?}", event);
    }

    fn handle_modifiers_changed(
        &mut self,
        _display: &glium::Display<WindowSurface>,
        modifiers: &winit::event::Modifiers,
    ) {
        self.is_shift_button_pressed =
            modifiers.state() & ModifiersState::SHIFT == ModifiersState::SHIFT;
    }

    fn handle_mouse_input(
        &mut self,
        _display: &glium::Display<WindowSurface>,
        state: &ElementState,
        button: &MouseButton,
    ) {
        println!("{:?} {:?}", state, button);
        match button {
            MouseButton::Left => self.is_left_button_pressed = state == &ElementState::Pressed,
            MouseButton::Right => self.is_right_button_pressed = state == &ElementState::Pressed,
            _ => (),
        }
    }

    fn handle_cursor_moved(
        &mut self,
        display: &glium::Display<WindowSurface>,
        position: &PhysicalPosition<f64>,
    ) {
        let (_, height) = display.get_framebuffer_dimensions();
        if self.is_left_button_pressed {
            if let Some(prev) = self.prev_mouse_pos {
                let dx = (position.x - prev.x) / height as f64 * 2.0;
                let dy = -(position.y - prev.y) / height as f64 * 2.0;
                self.matrix[3][0] += dx as f32;
                self.matrix[3][1] += dy as f32;
            }
            self.prev_mouse_pos = Some(*position);
        } else {
            self.prev_mouse_pos = None;
        }
    }

    fn handle_mouse_wheel(
        &mut self,
        _display: &glium::Display<WindowSurface>,
        delta: &MouseScrollDelta,
        _phase: &TouchPhase,
    ) {
        if self.is_shift_button_pressed {
            let scale = match delta {
                MouseScrollDelta::LineDelta(_, y) => 1.0 + y / 10.0,
                MouseScrollDelta::PixelDelta(_) => 1.0,
            };
            self.matrix[0][0] *= scale as f32;
            self.matrix[1][1] *= scale as f32;
        }
    }
}