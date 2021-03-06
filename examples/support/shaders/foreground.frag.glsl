#version 330

in vec3 vertex_normal;

out vec4 color;

uniform vec3 light_direction;
uniform vec4 surface_color;
uniform bool is_contact;

void main() {
    if (is_contact) {
        color = surface_color;
    } else {
        float brightness = max(dot(normalize(vertex_normal), -normalize(light_direction)), 0.0);
        color = surface_color * (0.1 + 0.9 * brightness);
    }
}
