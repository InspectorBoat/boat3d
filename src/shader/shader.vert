#version 460
layout(location = 0) in vec4 a_pos;

layout(location = 0) uniform mat4 u_transform;
layout(location = 1) uniform vec4 u_chunkPos;
out vec4 color;

void main() {
        gl_Position = u_transform * (a_pos + u_chunkPos);
        color = a_pos / 16;
}