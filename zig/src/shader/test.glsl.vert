#version 460

layout(location = 0) in vec4 pos;

uniform mat4 projection;

void main() {
    gl_Position = projection * pos;
}