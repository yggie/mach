#version 330

in vec3 position;
in vec3 normal;

out vec3 vertex_normal;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

void main() {
    mat4 model_view_matrix = view_matrix * model_matrix;
    vertex_normal = transpose(inverse(mat3(model_view_matrix))) * normal;
    gl_Position = projection_matrix * model_view_matrix * vec4(position, 1.0);
}
