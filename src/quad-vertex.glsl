// attribute vec2 vertexData; // <vec2 position>

// varying vec2 texCoords;

// void main() {
    // gl_Position = vec4(vertexData.xy, 0.0, 1.0);

    // texCoords = vertexData.zw;
// }

attribute vec4 a_position;

void main() {
  gl_Position = a_position;
}