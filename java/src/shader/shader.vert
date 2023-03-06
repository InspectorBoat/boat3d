#version 460

const mat4[] normalTransforms = {
// south
mat4(
0, 0, 0, 0,
0, 0, 0, 0,
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
0, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
1, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),

//north
mat4(
1, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 01,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 01,
0, 0, 0, 1
),
mat4(
1, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 01,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 01,
0, 0, 0, 1
),

// west
mat4(
0, 0, 0, 0,
0, 0, 0, 0,
1, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 1, 0, 0,
1, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),

// east
mat4(
0, 0, 0, 01,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 01,
0, 0, 0, 0,
1, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 01,
0, 1, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 01,
0, 1, 0, 0,
1, 0, 0, 0,
0, 0, 0, 1
),

// down
mat4(
0, 0, 0, 0,
0, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 1
),
mat4(
1, 0, 0, 0,
0, 0, 0, 0,
0, 1, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
1, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 0,
0, 0, 0, 1
),

// up
mat4(
0, 0, 0, 0,
0, 0, 0, 01,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
1, 0, 0, 0,
0, 0, 0, 01,
0, 0, 0, 0,
0, 0, 0, 1
),
mat4(
0, 0, 0, 0,
0, 0, 0, 01,
0, 1, 0, 0,
0, 0, 0, 1
),
mat4(
1, 0, 0, 0,
0, 0, 0, 01,
0, 1, 0, 0,
0, 0, 0, 1
),

};

/*
 * 0 - south
 * 1 - north
 * 2 - west
 * 3 - east
 * 4 - down
 * 5 - up
 */

struct Face {
        /*
        x - 8
        y - 8
        z - 8
        normal - 3
        */
        uint xyzn;
        /*
        width - 8
        height - 8
        texture - 16
        */
        uint wht;
};

layout(location = 0) uniform mat4 u_transform;
layout(std430, binding = 0) readonly buffer Faces {
        ivec4 chunkPos;
        Face faces[];
};

out vec2 v_texCoord;
out float v_texId;

vec4 getPosInfo(int faceIndex) {
        return vec4((faces[faceIndex].xyzn >> 24) & 0xff, (faces[faceIndex].xyzn >> 16) & 0xff, (faces[faceIndex].xyzn >> 8) & 0xff, 0);
}

uint getNormal(int faceIndex) {
        return (faces[faceIndex].xyzn >> 0) & 0xff;
}

vec4 getSize(int faceIndex) {
        return vec4(((faces[faceIndex].wht >> 24) & 0xff) + 1, ((faces[faceIndex].wht >> 16) & 0xff) + 1, 0, 1);
}

float getTexture(int faceIndex) {
        return float(faces[faceIndex].wht & 0xff);
}

void main() {
        int cornerIndex = gl_VertexID % 4;
        int faceIndex = gl_VertexID / 4;
        vec4 vertexPos = getPosInfo(faceIndex);
        vec4 offset = getSize(faceIndex) * normalTransforms[getNormal(faceIndex) * 4 + (cornerIndex)];

        v_texCoord = (((vertexPos * normalTransforms[getNormal(faceIndex) * 4 + 3]).xy) + (getSize(faceIndex) * normalTransforms[cornerIndex]).xy) / vec2(16, -16);
        vertexPos += offset;
        v_texId = getTexture(faceIndex);
        gl_Position = u_transform * (vertexPos + chunkPos * 256);
}
