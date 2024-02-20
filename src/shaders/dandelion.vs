#version 430

// min_x: -3.959266, min_y: -0.194916, max_x: 4.011385, max_y: 13.035965

in vec3 position;
in vec3 normal;

uniform vec2 screen_size;
uniform mat4 translation;
uniform mat4 rotation;
uniform mat4 scale;

out vec3 normal_interpolated;
out vec3 vertex_position;

void main() {
    float r = screen_size.x / screen_size.y;
    float t = 1.0;
    float n = 0.1;
    float f = 100.0;
    float fov = 3.1415 / 3;
    float tan_fov = tan(fov / 2.0);
    // perspective transformation
    mat4 perspective_matrix = mat4(
        1.0 / (r * tan_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_fov, 0.0, 0.0,
        0.0, 0.0, -(f + n) / (f - n), -1.0,
        0.0, 0.0, -2.0 * f * n / (f - n), 0.0
    );
    mat4 view_matrix = mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    mat4 M = translation * rotation * scale;
    vec4 world_position = M * vec4(position.x, position.y - 8.0, position.z, 1.0);
    gl_Position = perspective_matrix * view_matrix * world_position;
    
    normal_interpolated = mat3(transpose(inverse(M))) * normal;
    vertex_position = world_position.xyz;
}