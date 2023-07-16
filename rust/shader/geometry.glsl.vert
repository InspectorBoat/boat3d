#version 460

const mat4[] pos_transforms = {
        mat4(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
        ),
        mat4(
        0, 0, 1, 0,
        0, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
        mat4(
        0, 1, 0, 0,
        0, 0, 1, 0,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),

        mat4(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, -15,
        0, 0, 0, 1
        ),
        mat4(
        0, 0, 1, -15,
        0, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
        mat4(
        0, 1, 0, 0,
        0, 0, 1, -15,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
};

const mat4[] corner_transforms = {
        mat4(
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 1
        ),
        mat4(
        0, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 1
        ),
        mat4(
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 1
        ),
        mat4(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 1
        ),
};

struct Face {
        // uint uvdn;
        // uint whtt;
        uint ndvu;
        uint tthw;
};

layout(std430, binding = 0) readonly restrict buffer Faces {
        Face faces[];
};

layout(std430, binding = 1) readonly restrict buffer Light {
        uint light[];
};

vec4 get_relative_pos(int face_index) {
        return (uvec4(faces[face_index].ndvu) >> uvec4(0, 8, 16, 24)) & uvec4(0xff, 0xff, 0xff, 0x00);
        // return vec4(
        //         (faces[face_index].ndvu >>  0) & 0xff,
        //         (faces[face_index].ndvu >>  8) & 0xff,
        //         (faces[face_index].ndvu >> 16) & 0xff,
        //         0
        // );
}

uint get_normal(int face_index) {
        return (faces[face_index].ndvu >> 24) & 0xff;
}

vec4 get_size(int face_index) {
        return (uvec4(faces[face_index].tthw) >> uvec4(0, 8, 16, 24) & uvec4(0xff, 0xff, 0x00, 0x00)) + uvec4(1, 1, 0, 1);
        // return vec4(
        //         ((faces[face_index].tthw >> 0) & 0xff) + 1,
        //         ((faces[face_index].tthw >> 8) & 0xff) + 1,
        //         0,
        //         1
        // );
}

uint get_texture(int face_index) {
        return faces[face_index].tthw >> 24 & 0xff;
}

layout(location = 0) uniform mat4 u_transform;
layout(location = 1) uniform ivec3 chunk_pos;
layout(location = 2) uniform uint light_page_index_offset;
layout(location = 3) uniform uint face_index_offset;

out uint light_index;
out uint quad_width;
out uint texture_id;
out vec2 texture_pos;

void main() {
        int corner_index = gl_VertexID % 4;
        int face_index = gl_VertexID / 4;
        uint normal = get_normal(face_index);

        vec4 vertex_pos = get_relative_pos(face_index);
        vec4 offset = get_size(face_index) * corner_transforms[corner_index];
        vertex_pos += offset;
        
        gl_Position = u_transform * (vertex_pos * pos_transforms[normal] + vec4(chunk_pos, 0) * 256);
        
        texture_pos = offset.xy / 16;
        texture_id = get_texture(face_index);
        light_index = light[light_page_index_offset + face_index - face_index_offset] + light_page_index_offset;
        quad_width = uint(ceil(get_size(face_index).x / 16));
}