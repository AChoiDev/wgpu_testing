#version 460 core

layout(location = 0) in vec2 texCoords;
layout(location = 0) out vec4 colorOut;

layout(set = 0, binding = 0, r32f) uniform readonly image2D depthInput;
layout(set = 0, binding = 1, r11f_g11f_b10f) uniform readonly image2D source;
//layout(set = 0, binding = 2) uniform sampler sourceSampler;

in vec4 gl_FragCoord;

void main() {
    //ivec2 pixelCoord = ivec2(gl_FragCoord - vec4(0.5));
    ivec2 sourceSize = imageSize(source);
    ivec2 sourceCoords = ivec2(vec2(sourceSize) * texCoords);

    vec4 colorInput = vec4(imageLoad(source, sourceCoords));
    //float rayLength = imageLoad(depthInput, sourceCoords).r;
    //vec4 colorInput = vec4(exp(rayLength * -0.13));
    //vec4 colorInput = texture(sampler2D(source, sourceSampler), tex_coords);
    //vec4 colorInput = vec4(0.5);
    //vec4 colorInput = vec4(tex_coords, 0.0, 1.0);
    colorOut = colorInput;
}