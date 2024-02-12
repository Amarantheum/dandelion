#version 430

// min_x: -3.959266, min_y: -0.194916, max_x: 4.011385, max_y: 13.035965

in vec3 position;

uniform float time;

void main() {
    // rotation matrix on the z axis:
    mat3 rotation = mat3(
        cos(time), -sin(time), 0,
        sin(time), cos(time), 0,
        0, 0, 1
    );
    // rotation matrix on the x axis:
    mat3 rotation_x = mat3(
        1, 0, 0,
        0, cos(time * 0.9), -sin(time * 0.9),
        0, sin(time * 0.9), cos(time * 0.9)
    );
    // rotation matrix on the y axis:
    mat3 rotation_y = mat3(
        cos(time * 2.1), 0, sin(time * 2.1),
        0, 1, 0,
        -sin(time * 2.1), 0, cos(time * 2.1)
    );

    // perspective matrix:
    mat4 perspective = mat4(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, -1,
        0, 0, 0, 1
    );
    vec3 position = position / 20.0;
    vec3 rotated = rotation_x * rotation * rotation_y * position;
    rotated.z -= 1.0;
    gl_Position = perspective * vec4(rotated, 1.0);
}