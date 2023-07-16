#version 460

layout(location = 0) out vec4 framebuffer;
layout(location = 1) out vec4 unused;

// texture pos, texture id, quad width, light index f32 * 2 + u16 + u16 + u32

flat in uint light_index;
flat in uint quad_width;
flat in uint texture_id;
in vec2 texture_pos;

void main() {
        framebuffer = vec4(texture_pos, uintBitsToFloat(texture_id << 16 | quad_width), uintBitsToFloat(light_index));
}