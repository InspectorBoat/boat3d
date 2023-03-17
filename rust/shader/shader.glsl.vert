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
        uint uvdn;
        uint whtt;
};

layout(location = 0) uniform mat4 u_transform;
layout(std430, binding = 0) readonly buffer Faces {
        ivec4 chunk_pos;
        Face faces[];
};

out vec2 tex_coord;
out float tex_id;

vec4 get_relative_pos(int face_index) {
        return (uvec4(faces[face_index].uvdn) >> uvec4(0, 8, 16, 24)) & uvec4(0xff, 0xff, 0xff, 0x00);
        // return vec4(
                // (faces[face_index].uvdn >>  0) & 0xff,
                // (faces[face_index].uvdn >>  8) & 0xff,
                // (faces[face_index].uvdn >> 16) & 0xff,
                // 0
        // );
}

uint get_normal(int face_index) {
        return (faces[face_index].uvdn >> 24) & 0xff;
}

vec4 get_size(int face_index) {
        return (uvec4(faces[face_index].whtt) >> uvec4(0, 8, 16, 24) & uvec4(0xff, 0xff, 0x00, 0x00)) + uvec4(1, 1, 0, 1);
        // return vec4(
                // ((faces[face_index].whtt >> 0) & 0xff) + 1,
                // ((faces[face_index].whtt >> 8) & 0xff) + 1,
                // 0,
                // 1
        // );
}

float get_texture(int face_index) {
        return float(faces[face_index].whtt & 0xff);
}

void main() {
        int corner_index = gl_VertexID % 4;
        int face_index = gl_VertexID / 4;
        uint normal = get_normal(face_index);

        int z = 4;
        bool x = bool(z);
        int y = int(x);

        vec4 vertex_pos = get_relative_pos(face_index);
        vec4 offset = get_size(face_index) * corner_transforms[corner_index];
        vertex_pos += offset;
        
        gl_Position = u_transform * (vertex_pos * pos_transforms[normal] + chunk_pos * 256);

        tex_coord = vec2(0, 0);
        tex_id = normal;
}