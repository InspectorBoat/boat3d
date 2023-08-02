#version 450

layout(location = 0) out vec4 framebuffer;
layout(location = 1) uniform sampler2DArray block_texture;

// texture pos, texture id, quad width, light index f32 * 2 + u16 + u16 + u32

flat in uint light_index;
flat in uint quad_width;
flat in uint texture_id;
in vec2 texture_pos;

void main() {
        // if (texture(block_texture, vec3(texture_pos, texture_id)).x > 0.9) discard;
        framebuffer = vec4(texture_pos, uintBitsToFloat(texture_id << 16 | quad_width), uintBitsToFloat(light_index));
}