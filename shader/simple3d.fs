#version 140

in vec2 v_tex_coords;
in vec2 v_mask_tex_coords;
out vec4 color;

uniform int axis;
uniform vec3 current_pos;
uniform sampler3D tex;
uniform sampler3D mask;
uniform float window_width;
uniform float window_level;


vec4 get_color(sampler3D image, vec2 tex_coords, vec3 cur_pos, int ax) {
    if (tex_coords.x < 0.0 || tex_coords.x > 1.0 || tex_coords.y < 0.0 || tex_coords.y > 1.0) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
    if (axis == 2) {
        return texture(image, vec3(tex_coords, cur_pos.z));
    } else if (axis == 1) {
        return texture(image, vec3(tex_coords.x, cur_pos.y, tex_coords.y));
    } else {
        return texture(image, vec3(cur_pos.x, tex_coords));
    }
}

void main() {
    color = get_color(tex, v_tex_coords, current_pos, axis);
    vec4 mask_color = get_color(mask, v_mask_tex_coords, current_pos, axis);
    float min_val = window_level - window_width / 2;
    float val = (color.r - min_val) / (window_width);
    color.r = max(val, mask_color.r);
    color.gb = vec2(val);
    color.a = 1.0;
}
