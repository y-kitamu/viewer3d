#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
out vec2 v_mask_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform mat4 mask_texture_transform;

void main() {
    v_tex_coords = tex_coords;
    v_mask_tex_coords = (mask_texture_transform * vec4(tex_coords, 0.0, 1.0)).xy;
    gl_Position = perspective * view * model * vec4(position, 0.0, 1.0);
}
