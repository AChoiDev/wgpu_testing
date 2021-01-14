#version 460 core

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 colorOut;

layout(set = 0, binding = 0) uniform texture2D source;
layout(set = 0, binding = 1) uniform sampler sourceSampler;

in vec4 gl_FragCoord;

void main() {
    ivec2 coord = ivec2(gl_FragCoord - vec4(0.5));
    vec4 colorInput = texture(sampler2D(source, sourceSampler), tex_coords);
    //vec4 colorInput = vec4(0.5);
    //vec4 colorInput = vec4(tex_coords, 0.0, 1.0);
    colorOut = colorInput;
}