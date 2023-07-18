#version 450

out vec4 FragColor;

in vec2 TexCoords;

layout(location = 0) uniform sampler2D framebuffer;
layout(location = 1) uniform sampler2DArray block_texture;

// texture pos, texture id, quad width, light index f32 * 2 + u16 + u16 + u32

layout(std430, binding = 1) readonly restrict buffer Light {
    uint light[];
};

void unpack_framebuffer(out vec2 texture_pos, out uint texture_id, out uint quad_width, out uint light_index) {
    vec4 data = texture(framebuffer, TexCoords);
    texture_pos = data.xy;
    texture_id = floatBitsToUint(data.z) >> 16 & 0xffff;
    quad_width = floatBitsToUint(data.z) & 0xffff;
    light_index = floatBitsToUint(data.w);
}

void main() {
    vec2 texture_pos;
    uint texture_id;
    uint quad_width;
    uint light_index;
    unpack_framebuffer(texture_pos, texture_id, quad_width, light_index);
    uint index_offset = uint(texture_pos.x) + uint(texture_pos.y) * quad_width;
    FragColor = vec4(light[light_index + index_offset] * 0.0625, 0.0, 0.0, 1.0);
    // FragColor = texture(block_texture, vec3(texture_pos, texture_id)) * light[light_index + index_offset] * 0.0625;
    // FragColor = texture(block_texture, vec3(TexCoords, 1.0));
}