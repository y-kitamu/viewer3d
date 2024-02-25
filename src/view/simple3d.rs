use cgmath::prelude::*;
use glium;
use glium::glutin::surface::WindowSurface;
use glium::Surface;
use glium::{implement_vertex, uniform};
use tracing::info;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::ModifiersState;

use crate::shader;
use crate::shader::ShaderSrc;

const DEFAULT_MASK_WINDOW_WIDTH: f32 = 1.0;
const DEFAULT_MASK_WINDOW_LEVEL: f32 = 0.5;
const DEFAULT_IMAGE_WINDOW_WIDTH: f32 = 600.0;
const DEFAULT_IMAGE_WINDOW_LEVEL: f32 = 200.0;

#[derive(Copy, Clone)]
struct Simple3DVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

#[derive(Debug)]
struct Texture {
    pub image: Option<crate::io::Image3D>,
    pub texture: glium::texture::Texture3d,
    pub model_matrix: cgmath::Matrix4<f32>,
    pub window_width: f32,
    pub window_level: f32,
}

impl Texture {
    pub fn empty(display: &glium::Display<WindowSurface>) -> Self {
        Texture {
            image: None,
            texture: glium::texture::Texture3d::empty(display, 0, 0, 0).unwrap(),
            model_matrix: cgmath::Matrix4::identity(),
            window_width: 1.0,
            window_level: 0.0,
        }
    }

    pub fn set_model_matrix(&mut self, current_axis: &u32) {
        let (spacing_x, spacing_y) = match &self.image {
            Some(image) => match current_axis {
                0 => (image.spacing.2, image.spacing.1),
                1 => (image.spacing.2, image.spacing.0),
                2 => (image.spacing.1, image.spacing.0),
                _ => panic!("Invalid axis : {}", current_axis),
            },
            _ => (1.0, 1.0),
        };

        self.model_matrix = cgmath::Matrix4::from([
            [spacing_x, 0.0, 0.0, 0.0],
            [0.0, spacing_y, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn set_image(
        &mut self,
        display: &glium::Display<WindowSurface>,
        image: crate::io::Image3D,
        current_axis: u32,
    ) {
        let image3d = glium::texture::RawImage3d {
            data: std::borrow::Cow::Borrowed(&image.data),
            width: image.shape.0,
            height: image.shape.1,
            depth: image.shape.2,
            format: glium::texture::ClientFormat::F32,
        };
        if self.image.is_none() {
            if image.is_mask {
                self.window_width = DEFAULT_MASK_WINDOW_WIDTH;
                self.window_level = DEFAULT_MASK_WINDOW_LEVEL;
            } else {
                self.window_width = DEFAULT_IMAGE_WINDOW_WIDTH;
                self.window_level = DEFAULT_IMAGE_WINDOW_LEVEL;
            }
        }
        self.texture = match image.format {
            Some(format) => glium::texture::Texture3d::with_format(
                display,
                image3d,
                format,
                image.mipmaps.unwrap(),
            )
            .unwrap(),
            None => {
                glium::texture::Texture3d::with_mipmaps(display, image3d, image.mipmaps.unwrap())
                    .unwrap()
            }
        };
        self.image = Some(image);
        self.set_model_matrix(&current_axis);
    }
}

pub struct Simple3DView {
    indices: glium::index::NoIndices,
    vertex_buffer: glium::VertexBuffer<Simple3DVertex>,
    program: glium::Program,
    image: Texture,
    mask: Texture,
    view_matrix: cgmath::Matrix4<f32>,        // カメラの位置, zoom
    perspective_matrix: cgmath::Matrix4<f32>, // aspect比
    axis: u32,
    current_pos: [u32; 3],
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
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Simple3DVertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Simple3DVertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Simple3DVertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Simple3DVertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Simple3DVertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let shader = shader!("simple3d");
        let program = shader.compile(display);

        let (width, height) = display.get_framebuffer_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let f = if 1.0.lt(&aspect_ratio) {
            1.0
        } else {
            1.0 / aspect_ratio
        };

        Simple3DView {
            indices,
            vertex_buffer,
            program,
            image: Texture::empty(display),
            mask: Texture::empty(display),
            view_matrix: cgmath::Matrix4::identity(),
            perspective_matrix: cgmath::Matrix4::from([
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            axis: 2,
            current_pos: [0, 0, 0],
            is_left_button_pressed: false,
            is_right_button_pressed: false,
            is_shift_button_pressed: false,
            prev_mouse_pos: None,
        }
    }
}

impl super::View for Simple3DView {
    fn set_image(&mut self, display: &glium::Display<WindowSurface>, data_path: &std::path::Path) {
        let image3d = crate::io::load_image3d(data_path);
        info!(
            "Image shape : {:?}, data len : {}, spacing : {:?}",
            image3d.shape,
            image3d.data.len(),
            image3d.spacing
        );

        if image3d.is_mask {
            self.mask.set_image(display, image3d, self.axis);
        } else {
            if self.image.image.is_none() && self.mask.image.is_none() {
                self.axis = 2;
                self.current_pos = [
                    image3d.shape.0 / 2,
                    image3d.shape.1 / 2,
                    image3d.shape.2 / 2,
                ];
            }
            self.image.set_image(display, image3d, self.axis);
        }
        info!("Image loaded");
    }

    fn draw(&self, display: &glium::Display<WindowSurface>) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        let image_model: [[f32; 4]; 4] = self.image.model_matrix.into();
        let mask_transform: [[f32; 4]; 4] =
            (self.image.model_matrix * self.mask.model_matrix.inverse_transform().unwrap()).into();
        let view: [[f32; 4]; 4] = self.view_matrix.into();
        let perspective: [[f32; 4]; 4] = self.perspective_matrix.into();
        let current_pos = [
            self.current_pos[0] as f32 / self.image.texture.get_width() as f32,
            self.current_pos[1] as f32 / self.image.texture.get_height().unwrap() as f32,
            self.current_pos[2] as f32 / self.image.texture.get_depth().unwrap() as f32,
        ];
        // draw image
        let uniforms = uniform! {
            axis: self.axis as i32,
            current_pos: current_pos,
            tex: glium::uniforms::Sampler(&self.image.texture, behavior),
            mask: glium::uniforms::Sampler(&self.mask.texture, behavior),
            perspective: perspective,
            view: view,
            model: image_model,
            mask_texture_transform: mask_transform,
            window_width: self.image.window_width,
            window_level: self.image.window_level,
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
        if event.state == ElementState::Released {
            match event.physical_key {
                winit::keyboard::PhysicalKey::Code(x) => match x {
                    winit::keyboard::KeyCode::KeyX => {
                        self.axis = (self.axis + 1) % 3;
                        self.image.set_model_matrix(&self.axis);
                        self.mask.set_model_matrix(&self.axis);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
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
                self.view_matrix[3][0] += dx as f32;
                self.view_matrix[3][1] += dy as f32;
            }
        }
        self.prev_mouse_pos = Some(*position);
    }

    fn handle_mouse_wheel(
        &mut self,
        display: &glium::Display<WindowSurface>,
        delta: &MouseScrollDelta,
        _phase: &TouchPhase,
    ) {
        if self.is_shift_button_pressed {
            let scale = match delta {
                MouseScrollDelta::LineDelta(_, y) => 1.0 + y / 10.0,
                MouseScrollDelta::PixelDelta(_) => 1.0,
            };
            match self.prev_mouse_pos {
                Some(pos) => {
                    let x = (pos.x as f32 / display.get_framebuffer_dimensions().0 as f32 * 2.0
                        - 1.0)
                        / self.perspective_matrix[0][0];
                    let y = (1.0
                        - pos.y as f32 / display.get_framebuffer_dimensions().1 as f32 * 2.0)
                        / self.perspective_matrix[1][1];
                    let pre_trans = cgmath::Matrix4::from([
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [-x, -y, 0.0, 1.0],
                    ]);
                    let scale = cgmath::Matrix4::from([
                        [scale, 0.0, 0.0, 0.0],
                        [0.0, scale, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ]);
                    let post_trans = cgmath::Matrix4::from([
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [x, y, 0.0, 1.0],
                    ]);
                    self.view_matrix = post_trans * scale * pre_trans * self.view_matrix;
                }
                None => {
                    self.view_matrix[3][0] += (1.0 - scale) * self.view_matrix[3][0];
                    self.view_matrix[3][1] += (1.0 - scale) * self.view_matrix[3][1];
                    self.view_matrix[0][0] *= scale as f32;
                    self.view_matrix[1][1] *= scale as f32;
                }
            }
        } else {
            let index = self.current_pos[self.axis as usize] as f32;
            let index = match delta {
                MouseScrollDelta::LineDelta(_, y) => index + y.abs().ceil() * y.signum(),
                MouseScrollDelta::PixelDelta(_) => index,
            };
            let max = match self.axis {
                0 => self.image.texture.get_width(),
                1 => self.image.texture.get_height().unwrap(),
                2 => self.image.texture.get_depth().unwrap(),
                _ => 0,
            };
            self.current_pos[self.axis as usize] = (index as i32).max(0).min(max as i32) as u32;
        }
    }

    fn handle_window_resized(
        &mut self,
        _display: &glium::Display<WindowSurface>,
        window_size: winit::dpi::PhysicalSize<u32>,
    ) {
        let aspect_ratio = window_size.height as f32 / window_size.width as f32;
        let f = if 1.0.lt(&aspect_ratio) {
            0.5
        } else {
            0.5 / aspect_ratio
        };
        self.perspective_matrix = cgmath::Matrix4::from([
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }
}
