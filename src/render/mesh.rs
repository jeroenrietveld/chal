use blender_mesh::BlenderMesh;
use chal_engine::render::Render;
use chal_engine::shader::{Shader, ShaderKind};
use chal_engine::state::State;
use js_sys;
use js_sys::WebAssembly;
use nalgebra::{Isometry3, Vector3};
use wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;

use crate::shader::WebShader;

pub struct NonSkinnedMesh<'a> {
    pub mesh: &'a BlenderMesh,
    pub shader: &'a WebShader,
    pub opts: &'a MeshRenderOpts,
}

pub struct MeshRenderOpts {
    pub pos: (f32, f32, f32),
    pub clip_plane: [f32; 4],
}

// impl<'a> Render<'a, GL, WebShader> for NonSkinnedMesh<'a> {
//     fn shader_kind() -> ShaderKind {
//         ShaderKind::NonSkinnedMesh
//     }

//     fn shader(&'a self) -> &'a WebShader {
//         &self.shader
//     }

//     fn buffer_attributes(&self, gl: &GL) {
//         let shader = self.shader();
//         let mesh = self.mesh;

//         let pos_attrib = gl.get_attrib_location(&shader.program, "position");
//         let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
//         let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

//         gl.enable_vertex_attrib_array(pos_attrib as u32);
//         gl.enable_vertex_attrib_array(normal_attrib as u32);
//         gl.enable_vertex_attrib_array(uv_attrib as u32);

//         NonSkinnedMesh::buffer_f32_data(&gl, &mesh.vertex_positions[..], pos_attrib as u32, 3);
//         NonSkinnedMesh::buffer_f32_data(&gl, &mesh.vertex_normals[..], normal_attrib as u32, 3);
//         NonSkinnedMesh::buffer_f32_data(
//             &gl,
//             &mesh.vertex_uvs.as_ref().expect("uvs")[..],
//             uv_attrib as u32,
//             2,
//         );
//         NonSkinnedMesh::buffer_u16_indices(&gl, &mesh.vertex_position_indices[..]);
//     }

//     fn render(&self, gl: &GL, state: &State) {
//         let shader = self.shader();

//         let mesh = self.mesh;
//         let opts = self.opts;
//         let pos = opts.pos;

//         let model_uni = shader.get_uniform_location(gl, "model");
//         let view_uni = shader.get_uniform_location(gl, "view");
//         let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
//         let perspective_uni = shader.get_uniform_location(gl, "perspective");
//         let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");
//         // let mesh_texture_uni = shader.get_uniform_location(gl, "meshTexture");

//         gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &mut opts.clip_plane.clone()[..]);

//         let mut view = state.camera().view();
//         gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

//         let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
//         let mut model_array = [0.0; 16];
//         model_array.copy_from_slice(model.to_homogeneous().as_slice());
//         gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

//         let camera_pos = state.camera().get_eye_pos();
//         let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
//         gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);

//         // gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stone.texture_unit());

//         let mut perspective = state.camera().projection();
//         gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

//         let num_indices = mesh.vertex_position_indices.len();
//         gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
//     }

//     fn buffer_f32_data(gl: &GL, data: &[f32], attrib: u32, size: i32) {
//         let memory_buffer = wasm_bindgen::memory()
//             .dyn_into::<WebAssembly::Memory>()
//             .unwrap()
//             .buffer();

//         let data_location = data.as_ptr() as u32 / 4;
//         let data_array = js_sys::Float32Array::new(&memory_buffer)
//             .subarray(data_location, data_location + data.len() as u32);
//         let buffer = gl.create_buffer().unwrap();

//         gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
//         gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
//         gl.vertex_attrib_pointer_with_i32(attrib, size, GL::FLOAT, false, 0, 0);
//     }

//     fn buffer_u8_data(gl: &GL, data: &[u8], attrib: u32, size: i32) {
//         let memory_buffer = wasm_bindgen::memory()
//             .dyn_into::<WebAssembly::Memory>()
//             .unwrap()
//             .buffer();

//         let data_location = data.as_ptr() as u32;

//         let data_array = js_sys::Uint8Array::new(&memory_buffer)
//             .subarray(data_location, data_location + data.len() as u32);

//         let buffer = gl.create_buffer().unwrap();

//         gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
//         gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
//         gl.vertex_attrib_pointer_with_i32(attrib, size, GL::UNSIGNED_BYTE, false, 0, 0);
//     }

//     fn buffer_u16_indices(gl: &GL, indices: &[u16]) {
//         let memory_buffer = wasm_bindgen::memory()
//             .dyn_into::<WebAssembly::Memory>()
//             .unwrap()
//             .buffer();

//         let indices_location = indices.as_ptr() as u32 / 2;
//         let indices_array = js_sys::Uint16Array::new(&memory_buffer)
//             .subarray(indices_location, indices_location + indices.len() as u32);

//         let index_buffer = gl.create_buffer().unwrap();
//         gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
//         gl.buffer_data_with_array_buffer_view(
//             GL::ELEMENT_ARRAY_BUFFER,
//             &indices_array,
//             GL::STATIC_DRAW,
//         );
//     }
// }
