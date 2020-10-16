#version 450

// const vec2 positions[3] = vec2[3](
//   vec2(0.0, 0.5),
//   vec2(-0.5, -0.5),
//   vec2(0.5, -0.5)
// );

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;

layout(location=0) out vec3 v_color;

void main() {
  // gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
  v_color = a_color;
  gl_Position = vec4(a_position, 1.0);
}
