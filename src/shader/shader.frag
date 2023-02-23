#version 460
out vec4 f_color;

in vec2 v_texCoord;
in float v_texId;
//in vec4 pos;

uniform sampler2DArray t_textures;

void main() {
//        f_color = vec4(pos.xyz, 1);
        f_color = texture(t_textures, vec3(v_texCoord, v_texId));
//        f_color = texture(t_textures, vec3(v_texCoord, v_texId));
//        f_color = vec4(v_texId, 0, 0, 1);
}