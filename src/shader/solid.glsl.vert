#version 460

#define PI    3.1415926535897932384626433832795
#define SQRT2 1.4142135623730950488016887242096
const mat4[] pos_transforms = {
        // north
        mat4(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
        ),
        // east
        mat4(
        0, 0, 1, -15,
        0, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
        // down
        mat4(
        0, 1, 0, 0,
        0, 0, 1, 0,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
        // south
        mat4(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 1,
        0, 0, 0, 1
        ),
        // west
        mat4(
        0, 0, 1, 0,
        0, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
        // up
        mat4(
        0, 1, 0, 0,
        0, 0, 1, -15,
        1, 0, 0, 0,
        0, 0, 0, 1
        ),
        // diagonal
        mat4(
        sin(PI / 4), 0, cos(PI / 4), (1 - 1 / SQRT2) / 2 * 16,
        0, 1, 0, 0,
        cos(PI / 4), 0, sin(PI / 4), (1 - 1 / SQRT2) / 2 * 16,
        0, 0, 0, 1
        ),
        // other diagonal
        mat4(
        sin(-PI / 4), 0, cos(-PI / 4), 16 - (1 - 1 / SQRT2) / 2 * 16,
        0, 1, 0, 0,
        cos(-PI / 4), 0, sin(-PI / 4), (1 - 1 / SQRT2) / 2 * 16,
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
}

uint get_normal(int face_index) {
        return faces[face_index].ndvu >> 24;
}

vec4 get_size(int face_index) {
        return (uvec4(faces[face_index].tthw) >> uvec4(0, 8, 16, 24) & uvec4(0xff, 0xff, 0x00, 0x00)) + uvec4(1, 1, 0, 1);
}

vec4 get_chunk_pos() {
        return vec4(int(faces[gl_BaseVertex / 4].ndvu), int(faces[gl_BaseVertex / 4].tthw), int(faces[gl_BaseVertex / 4 + 1].ndvu), 0);
}

uint get_light_page_index_offset() {
        return faces[gl_BaseVertex / 4 + 1].tthw;
}

uint get_texture(int face_index) {
        return faces[face_index].tthw >> 16;
}

layout(location = 0) uniform mat4 u_transform;

out uint light_index;
out uint quad_width;
out uint texture_id;
out vec2 texture_pos;

void main() {
        int corner_index = gl_VertexID % 4;
        int face_index = gl_VertexID / 4 + 4 / 2;
        uint base_face_index = gl_VertexID / 4 - gl_BaseVertex / 4;
        uint normal = get_normal(face_index);

        vec4 vertex_pos = get_relative_pos(face_index);
        vec4 corner_offset = get_size(face_index) * corner_transforms[corner_index] * pos_transforms[normal];
        vertex_pos += corner_offset;
        
        gl_Position = u_transform * vec4((vertex_pos * 0.0625 + get_chunk_pos() * 16).xyz, 1);
        
        texture_pos = (get_size(face_index) * corner_transforms[corner_index]).xy / 16;
        texture_id = get_texture(face_index);
        light_index = light[get_light_page_index_offset() + base_face_index] + get_light_page_index_offset();
        quad_width = uint(ceil(get_size(face_index).x / 16));
}