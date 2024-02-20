#version 430
out vec4 fragColor;

in vec3 normal_interpolated;
in vec3 vertex_position;

void main() {
    float direction = -dot(normalize(normal_interpolated), normalize(vertex_position));
    if (direction < 0.0) {
        discard;
    }
    direction = exp(-5.0 * direction * direction);
    fragColor = vec4(vec3(1.0) * direction, 1.0);
}