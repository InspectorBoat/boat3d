#version 460
layout(location = 0) in vec4 a_pos;
layout(location = 1) in float a_texId;

layout(location = 0) uniform mat4 u_transform;
layout(location = 1) uniform vec4 u_chunkPos;

out vec2 v_texCoord;
out float v_texId;

void main() {
        gl_Position = u_transform * (a_pos + u_chunkPos);
        v_texCoord = a_pos.xy;
        v_texId = a_texId;
}