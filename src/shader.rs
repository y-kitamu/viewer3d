use glium::glutin::surface::WindowSurface;

pub struct ShaderSrc {
    pub stem: &'static str,
    pub vertex: &'static str,
    pub fragment: &'static str,
    pub geometry: Option<&'static str>,
}

impl ShaderSrc {
    pub fn compile(self, display: &glium::Display<WindowSurface>) -> glium::Program {
        glium::Program::from_source(display, self.vertex, self.fragment, self.geometry).unwrap()
    }
}

#[macro_export]
macro_rules! shader {
    ($stem:literal) => {
        ShaderSrc {
            stem: $stem,
            vertex: include_str!(concat!("../shader/", $stem, ".vs")),
            fragment: include_str!(concat!("../shader/", $stem, ".fs")),
            geometry: None,
        }
    };
    ($stem:literal, geometry) => {
        ShaderSrc {
            stem: $stem,
            vertex: include_str!(concat!("../shader/", $stem, ".vs")),
            fragment: include_str!(concat!("../shader/", $stem, ".fs")),
            geometry: Some(include_str!(concat!("../shader/", $stem, ".gs"))),
        }
    };
}
