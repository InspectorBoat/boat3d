#version 450

out vec4 framebuffer;
layout(location = 1) uniform sampler2DArray block_texture;

// texture pos, texture id, quad width, light index f32 * 2 + u16 + u16 + u32

flat in uint light_index;
flat in uint quad_width;
flat in uint texture_id;
in vec2 texture_pos;

layout(std430, binding = 1) readonly restrict buffer Light {
    uint light[];
};

void main() {
        if (light[light_index] == 0xACAB1312) {
                framebuffer = vec4(0, 0, 0, 0.5);
        } else {
                framebuffer = vec4(1, 1, 1, 0.5);
        }
        // framebuffer = vec4(texture(block_texture, vec3(texture_pos, texture_id)).xyz * (float(light[light_index]) * 0.0625), 0.5);
}