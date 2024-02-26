#version 430
out vec4 fragColor;

uniform float brightness;
uniform vec4 color;

in vec3 normal_interpolated;
in vec3 vertex_position;

vec3 bounce_vector(vec3 incoming, vec3 normal) {
    return incoming - 2.0 * dot(incoming, normal) * normal;
}

vec3 light_position = vec3(2.0, 3.0, 0.0);
vec3 material_color = vec3(1.0, 0.85, 0.45);

void main() {
    float direction = -dot(normalize(normal_interpolated), normalize(vertex_position));
    if (direction < 0.0) {
        discard;
    }

    vec3 bounce = bounce_vector(normalize(vertex_position), normalize(normal_interpolated));
    vec3 light_direction = normalize(light_position - vertex_position);

    float diffuse = pow(max(dot(normalize(normal_interpolated), light_direction), 0.0), 1.5);
    float specular = pow(max(dot(bounce, light_direction), 0.0), 8.0);

    fragColor = vec4(color.rgb * (diffuse + specular), 1.0);
}