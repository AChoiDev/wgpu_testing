#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 color_out;
layout(set = 0, binding = 0, r11f_g11f_b10f) readonly uniform image2D source;

in vec4 gl_FragCoord;

void main() {
    ivec2 coord = ivec2(gl_FragCoord - vec4(0.5));
    vec4 color_input = imageLoad(source, coord);
    color_out = color_input;
}