#version 460
out vec4 f_color;

in vec2 v_texCoord;
in float v_texId;

uniform sampler2DArray t_textures;

void main() {
//        f_color = vec4(0, 0, 0, 1);
        f_color = texture(t_textures, vec3(v_texCoord, v_texId));
}