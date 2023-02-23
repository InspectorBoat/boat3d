#version 460
layout(location = 0) in ivec4 a_posInfo;

layout(location = 0) uniform mat4 u_transform;
layout(location = 1) uniform ivec3 u_chunkPos;

out vec2 v_texCoord;
out float v_texId;
//out vec4 pos;

void main() {
        gl_Position = u_transform * vec4(a_posInfo.xyz + u_chunkPos, 1);
        int texInfo = a_posInfo.w;
        v_texCoord = vec2((texInfo & 0x02) >> 1, texInfo & 0x01);
        v_texId = float(texInfo >> 2);
//        pos = vec4(vec3(a_posInfo.xyz) / 255, 1);
}