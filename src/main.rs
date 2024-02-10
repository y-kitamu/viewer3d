use glium;
use glium::{implement_vertex, uniform, Surface};

#[path = "../assets/tuto-07-teapot.rs"]
mod teapot;
use teapot::{Normal, Vertex};

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("Event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #5")
        .build(&event_loop);

    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../assets/opengl.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);
    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
    ];
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;

    uniform mat4 perspective;
    uniform mat4 model;

    void main() {
        v_tex_coords = tex_coords;
        gl_Position = perspective * model * vec4(position, 0.0, 1.0);
    }
    "#;
    let fragment_shader_src = r#"
    #version 140

    in vec2 v_tex_coords;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, v_tex_coords);
        // color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    //     let vertex_shader_src = r#"
    // #version 150

    // in vec3 position;
    // in vec3 normal;

    // out vec3 v_normal;

    // uniform mat4 perspective;
    // uniform mat4 view;
    // uniform mat4 model;

    // void main() {
    //     mat4 modelview = view * model;
    //     v_normal = transpose(inverse(mat3(modelview))) * normal;
    //     gl_Position = perspective * modelview * vec4(position, 1.0);
    // }
    // "#;

    //     let fragment_shader_src = r#"
    // #version 150

    // in vec3 v_normal;
    // out vec4 color;
    // uniform vec3 u_light;

    // void main() {
    //     float birghtness = dot(normalize(v_normal), normalize(u_light));
    //     vec3 dark_color = vec3(0.6, 0.0, 0.0);
    //     vec3 regular_color = vec3(1.0, 0.0, 0.0);
    //     color = vec4(mix(dark_color, regular_color, birghtness), 1.0);
    // }
    // "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // implement_vertex!(Vertex, position);
    // implement_vertex!(Normal, normal);
    // let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    // let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    // let indices = glium::IndexBuffer::new(
    //     &display,
    //     glium::index::PrimitiveType::TrianglesList,
    //     &teapot::INDICES,
    // )
    // .unwrap();

    event_loop
        .run(move |event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    // target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    let params = glium::DrawParameters {
                        // depth: glium::Depth {
                        //     test: glium::draw_parameters::DepthTest::IfLess,
                        //     write: true,
                        //     ..Default::default()
                        // },
                        ..Default::default()
                    };
                    let perspective = {
                        let (width, height) = target.get_dimensions();
                        let aspect_ratio = height as f32 / width as f32;

                        let fov: f32 = 3.141592 / 3.0;
                        let zfar = 1024.0;
                        let znear = 0.0;

                        let f = 1.0 / (fov / 2.0).tan();

                        // [
                        //     [f * aspect_ratio, 0.0, 0.0, 0.0],
                        //     [0.0, f, 0.0, 0.0],
                        //     [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                        //     [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                        // ]
                        [
                            [f * aspect_ratio, 0.0, 0.0, 0.0],
                            [0.0, f, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0],
                        ]
                    };
                    // let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);
                    let uniforms = uniform! {
                        tex: &texture,
                        perspective: perspective,
                        // view: view,
                        model: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ],
                    };
                    target
                        .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
                        .unwrap();
                    target.finish().unwrap();
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