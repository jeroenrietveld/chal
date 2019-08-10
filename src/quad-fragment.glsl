// precision mediump float;

// varying vec2 texCoords;

// uniform sampler2D texture;

// void main() {
    // gl_FragColor = texture2D( texture, vec2(texCoords.s, texCoords.t) );
    // gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
// }

precision mediump float;

void main() {
  gl_FragColor = vec4(1, 0, 0.5, 1); // return redish-purple
}