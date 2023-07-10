#version 460

out vec4 FragColor;

in vec2 TexCoords;

layout(location = 0) uniform sampler2D rel_pos_chunk_id;
layout(location = 1) uniform sampler2D texture_pos_id;

layout(std430, binding = 1) readonly restrict buffer Light {
    uint light[];
};

void main() {
    // uint chunk_id = floatBitsToInt(texture(rel_pos_chunk_id, TexCoords).w);
    
    FragColor = vec4(texture(texture_pos_id, TexCoords).xyz, 32);
    // FragColor = vec4(texture(texture_pos_id, TexCoords).xyz, 1.0);
}