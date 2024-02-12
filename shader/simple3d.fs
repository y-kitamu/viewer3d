#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform int z_index;
uniform sampler3D tex;

void main() {
    color = texture(tex, vec3(v_tex_coords, z_index));
}
