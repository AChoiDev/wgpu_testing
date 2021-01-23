#version 460 core

layout(location = 0) in vec2 texCoords;
layout(location = 0) out vec4 colorOut;

layout(set = 0, binding = 0, r32f) uniform readonly image2D depthInput;
layout(set = 0, binding = 1, r11f_g11f_b10f) uniform readonly image2D source;

in vec4 gl_FragCoord;

void main() {
    ivec2 sourceSize = imageSize(source);
    ivec2 sourceCoords = ivec2(vec2(sourceSize) * texCoords);

    vec4 colorInput = vec4(imageLoad(source, sourceCoords));
    colorOut = colorInput;
}