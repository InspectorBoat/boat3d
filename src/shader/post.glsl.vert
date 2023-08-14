#version 450

out vec2 TexCoords;

void main() {
    switch (gl_VertexID) {
        case 0:
            gl_Position = vec4(-1, 1, 0, 1);
            TexCoords = vec2(0.0, 1.0);
            break;
        case 1:
            gl_Position = vec4(-1, -1, 0, 1);
            TexCoords = vec2(0.0, 0.0);
            break;
        case 2:
            gl_Position = vec4(1, -1, 0, 1);
            TexCoords = vec2(1.0, 0.0);
            break;
        case 3:
            gl_Position = vec4(-1, 1, 0, 1);
            TexCoords = vec2(0.0, 1.0);
            break;
        case 4:
            gl_Position = vec4(1, -1, 0, 1);
            TexCoords = vec2(1.0, 0.0);
            break;
        case 5:
            gl_Position = vec4(1, 1, 0, 1);
            TexCoords = vec2(1.0, 1.0);
            break;
    }
}