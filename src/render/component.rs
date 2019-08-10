use std::collections::HashMap;
use std::cell::RefCell;

use js_sys;
use js_sys::WebAssembly;
use nalgebra::{Isometry3, Vector3};
use wasm_bindgen;
use wasm_bindgen::JsCast;

use chal_engine::render::{Render, Vao, VaoExtension};
use chal_engine::shader::{Shader, ShaderKind, ShaderSystem};
use chal_engine::state::State;
use js_sys::Reflect;
use specs::Entities;
use specs::{Component, VecStorage};
use specs::{Read, ReadStorage, System};
use web_sys::WebGlRenderingContext as GL;

use crate::engine::{GLC, GameState};
use crate::shader::{WebShader, WebShaderSystem};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Mesh {
    name: &'static str,
    pub vertices: Vec<f32>,
    shader_kind: ShaderKind,
    // shader: WebShader
}

impl Mesh {
    pub fn new(name: &'static str, vertices: Vec<f32>, shader_kind: ShaderKind) -> Mesh {
        Mesh {
            name, vertices, shader_kind
        }
    }
}

pub struct RenderSystem {
    vao_ext: VaoExtension<js_sys::Object>,
    shader_sys: WebShaderSystem,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Read<'a, GameState>,
        Entities<'a>,
        ReadStorage<'a, Mesh>
    );

    fn run(&mut self, (state, entity, mesh): Self::SystemData) {
        use specs::Join;
        let gl = &GLC.contexts.borrow()[0];
        let state = &state.0;
        state.camera().view();

        gl.clear_color(0.53, 0.8, 0.98, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        gl.viewport(0, 0, 500, 500);

        for (pos, mesh) in (&entity, &mesh).join() {
            let shader = self.shader_sys.get_shader(&mesh.shader_kind()).unwrap();
            self.shader_sys.use_program(gl, mesh.shader_kind());
            self.prepare_for_render(mesh, gl, shader);
            mesh.render(gl);
        }
    }
}

impl RenderSystem {
    pub fn new() -> RenderSystem {
        let gl = &GLC.contexts.borrow()[0];
        let shader_sys = WebShaderSystem::new(&gl);

        let oes_vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("EOS ext")
            .expect("EOS ext");

        let vao_ext = VaoExtension {
            oes_vao_ext,
            vaos: RefCell::new(HashMap::new()),
        };

        RenderSystem {
            shader_sys,
            vao_ext,
        }
    }

    fn prepare_for_render(&self, mesh: &Mesh, gl: &GL, shader: &WebShader) {
        // TODO: instead of key use entityID?
        let mut vaos = self.vao_ext.vaos.borrow_mut();
        let vao = vaos.get(mesh.name);

        match vao {
            Some(vao) => {
                self.bind_vao(vao);
            }
            None => {
                let vao = self.create_vao();
                self.bind_vao(&vao);
                // TODO: buffer attributes
                mesh.buffer_attributes(shader);
                vaos.insert(mesh.name.to_string(), vao);
            }
        }
    }

    fn create_vao(&self) -> Vao<js_sys::Object> {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let create_vao_ext = Reflect::get(oes_vao_ext, &"createVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        Vao(
            Reflect::apply(&create_vao_ext, oes_vao_ext, &js_sys::Array::new())
                .expect("Created vao")
                .into(),
        )
    }

    fn bind_vao(&self, vao: &Vao<js_sys::Object>) {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let bind_vao_ext = Reflect::get(&oes_vao_ext, &"bindVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        let args = js_sys::Array::new();
        args.push(&vao.0);

        Reflect::apply(&bind_vao_ext, oes_vao_ext, &args).expect("Bound VAO");
    }
}

impl<'a> Render<'a, GL, WebShader> for Mesh {
    fn shader_kind(&self) -> ShaderKind {
        self.shader_kind
    }

    fn buffer_attributes(&self, shader: &WebShader) {
        let gl = &GLC.contexts.borrow()[0];
        // let mesh = self.mesh;

        let vertex_data_attrib = gl.get_attrib_location(&shader.program, "a_position");
        // let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        // let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

        gl.enable_vertex_attrib_array(vertex_data_attrib as u32);
        // gl.enable_vertex_attrib_array(normal_attrib as u32);
        // gl.enable_vertex_attrib_array(uv_attrib as u32);

        log!("verts {:?}", self.vertices);
        Mesh::buffer_f32_data(&gl, &self.vertices[..], vertex_data_attrib as u32, 2);
        // Mesh::buffer_f32_data(&gl, &mesh.vertex_normals[..], normal_attrib as u32, 3);
        // Mesh::buffer_f32_data(
        //     &gl,
        //     &mesh.vertex_uvs.as_ref().expect("uvs")[..],
        //     uv_attrib as u32,
        //     2,
        // );
        // Mesh::buffer_u16_indices(&gl, &mesh.vertex_position_indices[..]);
    }

    fn render(&self, gl: &GL) { //, shader: &WebShader, state: &State) {
        // TODO: state in render
        // gl.uniform1i(
        //     shader.get_uniform_location(gl, "texture").as_ref(),
        //     self.texture_unit as i32,
        // );

        gl.draw_arrays(GL::TRIANGLES, 0, 3);
    }

    fn buffer_f32_data(gl: &GL, data: &[f32], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32 / 4;

        let data_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, GL::FLOAT, false, 0, 0);
    }

    fn buffer_u8_data(gl: &GL, data: &[u8], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32;

        let data_array = js_sys::Uint8Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, GL::UNSIGNED_BYTE, false, 0, 0);
    }

    fn buffer_u16_indices(gl: &GL, indices: &[u16]) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let indices_location = indices.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + indices.len() as u32);

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
        );
    }
}
