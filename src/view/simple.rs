use glium;
use glium::glutin::surface::WindowSurface;
use glium::texture::{MipmapsOption, UncompressedFloatFormat};
use glium::Surface;
use glium::{implement_vertex, uniform};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::ModifiersState;

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
    matrix: [[f32; 4]; 4],
    is_left_button_pressed: bool,
    is_right_button_pressed: bool,
    is_shift_button_pressed: bool,
    prev_mouse_pos: Option<PhysicalPosition<f64>>,
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

impl super::View for Simple2DView {
    fn set_image(&mut self, display: &glium::Display<WindowSurface>, data_path: &std::path::Path) {
        let image = std::fs::read(data_path).unwrap();
        let image = image::load(std::io::Cursor::new(image), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();
        // let data_path = std::path::Path::new(concat!(
        //     env!("CARGO_MANIFEST_DIR"),
        //     "/data/cas/1-200/1.img.nii.gz"
        // ));
        // let image = crate::io::load_image_slice(data_path);
        println!(
            "Image shape : {:?}, data len : {}",
            image.dimensions(),
            image.len()
        );

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        println!("image client format : {:?}", image.format);
        self.texture = glium::Texture2d::with_format(
            display,
            image,
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
        )
        .unwrap();
        println!("texture : {:?}", self.texture);
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
