use glium;
use glium::glutin::surface::WindowSurface;
use glium::Surface;
use glium::{implement_vertex, uniform};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase};

use crate::shader;
use crate::shader::ShaderSrc;

#[derive(Copy, Clone)]
struct SimpleVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct Simple2DView {
    indices: glium::index::NoIndices,
    vertex_buffer: glium::VertexBuffer<SimpleVertex>,
    program: glium::Program,
    texture: glium::texture::Texture2d,
}

impl Simple2DView {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        implement_vertex!(SimpleVertex, position, tex_coords);
        let shape = vec![
            SimpleVertex {
                position: [-0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            SimpleVertex {
                position: [0.5, -0.5],
                tex_coords: [1.0, 0.0],
            },
            SimpleVertex {
                position: [0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            SimpleVertex {
                position: [0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            SimpleVertex {
                position: [-0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            SimpleVertex {
                position: [-0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let shader = shader!("simple");
        let program = shader.compile(display);

        Simple2DView {
            indices,
            vertex_buffer,
            program,
            texture: glium::texture::Texture2d::empty(display, 0, 0).unwrap(),
        }
    }

    pub fn set_image(&mut self, display: &glium::Display<WindowSurface>, image: &[u8]) {
        let image = image::load(std::io::Cursor::new(image), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        self.texture = glium::Texture2d::new(display, image).unwrap();
    }

    pub fn draw(&self, display: &glium::Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
            let fov: f32 = 3.141592 / 3.0;
            let f = 1.0 / (fov / 2.0).tan();
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
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
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

    pub fn handle_keyboard_input(
        &mut self,
        event: &winit::event::KeyEvent,
        display: &glium::Display<WindowSurface>,
        window_target: &winit::event_loop::EventLoopWindowTarget<()>,
    ) {
        println!("{:?}", event);
    }

    pub fn handle_mouse_input(&mut self, state: &ElementState, button: &MouseButton) {
        println!("{:?} {:?}", state, button);
    }

    pub fn handle_cursor_moved(&mut self, position: &PhysicalPosition<f64>) {
        println!("{:?}", position);
    }

    pub fn handle_mouse_wheel(&mut self, delta: &MouseScrollDelta, phase: &TouchPhase) {
        println!("{:?} {:?}", delta, phase);
    }
}
