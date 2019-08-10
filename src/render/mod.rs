mod mesh;
pub mod component;

use std::cell::RefCell;
use std::collections::HashMap;

use chal_engine::assets::Assets;
use chal_engine::render::{Render, VaoExtension, Vao};
use chal_engine::shader::{ShaderSystem, ShaderKind};
use chal_engine::state::State;
use js_sys;
use js_sys::Reflect;
use web_sys::WebGlRenderingContext as GL;

use crate::shader::{WebShaderSystem, WebShader};
use crate::render::mesh::{MeshRenderOpts, NonSkinnedMesh};
use crate::engine::GLC;

pub struct Renderer {
    shader_sys: WebShaderSystem,
    vao_ext: VaoExtension<js_sys::Object>,
}

impl Renderer {
    pub fn new() -> Renderer {
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

        Renderer {
            shader_sys,
            vao_ext,
        }
    }

    pub fn render(&mut self, state: &State, assets: &Assets) {
        let gl = &GLC.contexts.borrow()[0];
        gl.clear_color(0.53, 0.8, 0.98, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let above = 1000000.0;
        let clip_plane = [0., 1., 0., above];

        gl.viewport(0, 0, 800, 800);

        self.render_meshes(gl, state, assets, clip_plane);
    }

    fn render_meshes(&self, gl: &GL, state: &State, assets: &Assets, clip_plane: [f32; 4]) {
        let non_skinned_shader = self
            .shader_sys
            .get_shader(&ShaderKind::NonSkinnedMesh)
            .unwrap();
        self.shader_sys.use_program(gl, ShaderKind::NonSkinnedMesh);

        let mesh_opts = MeshRenderOpts {
            pos: (0., 0., 0.),
            clip_plane,
        };

        let mesh_name = "Terrain";
        let terrain = NonSkinnedMesh {
            mesh: assets.get_mesh(mesh_name).expect("Terrain mesh"),
            shader: non_skinned_shader,
            opts: &mesh_opts,
        };

        // self.prepare_for_render(gl, &terrain, mesh_name);
        // terrain.render(gl, state);
    }

    fn prepare_for_render<'a>(
        &self,
        gl: &GL,
        renderable: &impl Render<'a, GL, WebShader>,
        key: &str,
    ) {
        if self.vao_ext.vaos.borrow().get(key).is_none() {
            let vao = self.create_vao();
            self.bind_vao(&vao);
            // renderable.buffer_attributes(gl);
            self.vao_ext.vaos.borrow_mut().insert(key.to_string(), vao);
            return;
        }

        let vaos = self.vao_ext.vaos.borrow();
        let vao = vaos.get(key).unwrap();
        self.bind_vao(vao);
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
