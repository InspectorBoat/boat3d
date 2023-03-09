#version 460
out vec4 color;

in vec2 tex_coord;
in float tex_id;

uniform sampler2DArray textures;

void main() {
        color = texture(textures, vec3(tex_coord, tex_id));
        color = vec4(tex_id / 12, 0, 0, 1);
}