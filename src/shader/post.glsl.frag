#version 450

out vec4 FragColor;

in vec2 TexCoords;

layout(location = 0) uniform isampler2D framebuffer;
layout(location = 1) uniform sampler2DArray block_texture;

// texture pos, texture id, quad width, light index f32 * 2 + u16 + u16 + u32

layout(std430, binding = 1) readonly restrict buffer Light {
    uint light[];
};

void unpack_framebuffer(out vec2 texture_pos, out uint texture_id, out uint quad_width, out uint light_index) {
    ivec4 data = texture(framebuffer, TexCoords);
    texture_pos = uintBitsToFloat(data.xy);
    texture_id = data.z >> 16 & 0xffff;
    quad_width = data.z & 0xffff;
    light_index = data.w;
}

void main() {
    vec2 texture_pos;
    uint texture_id;
    uint quad_width;
    uint light_index;
    unpack_framebuffer(texture_pos, texture_id, quad_width, light_index);
    uint index_offset = uint(texture_pos.x) + uint(texture_pos.y) * quad_width;
    FragColor = vec4(texture(block_texture, vec3(texture_pos, texture_id)).xyz * (float(light[light_index + index_offset]) * 0.05 + 0.25), 1.0);
    // FragColor = vec4(texture(block_texture, vec3(texture_pos, texture_id)).xyz, 1.0);
}