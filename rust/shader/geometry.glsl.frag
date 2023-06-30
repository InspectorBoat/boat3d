#version 460
layout(location = 0) out vec4 relative_pos_chunk_id;
layout(location = 1) out vec4 texture_pos_id;

// relative pos, chunk id f32 * 3 + i32
// texture pos, texture id f32 * 2 + i32

flat in int chunk_id_out;
in vec4 relative_pos;
in vec2 texture_pos;
in float texture_id;

void main() {
        relative_pos_chunk_id = vec4(relative_pos.xyz, intBitsToFloat(chunk_id_out));
        texture_pos_id = vec4(1, 0, 0, texture_id);
}