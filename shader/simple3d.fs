#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform float z_index;
uniform sampler3D tex;

void main() {
    color = texture(tex, vec3(v_tex_coords, z_index));
    color.gb = vec2(color.r, color.r);
    color = (color + 1024) / (1024 + 3071);
    color.a = 1.0;
}
