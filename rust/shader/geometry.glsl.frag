#version 460
layout(location = 0) out vec4 chunk_pos;
layout(location = 1) out vec4 relative_pos_out;
// layout(location = 2) out vec4 texture;
// chunk pos i32 * (3 + 1)
// relative pos f32 * (3 + 1)
// texture pos f32 * 2
// texture id f32

in ivec4 chunk_pos_out;
in vec4 relative_pos;
in vec2 texture_pos;
in float texture_id;


void main() {
        chunk_pos = relative_pos / 256;
        relative_pos_out = vec4(0);
        // texture = vec4(0, 1.0, 0, 0);
}