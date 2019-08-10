use std::cell::RefCell;
use std::collections::HashMap;

use chal_engine::shader::{
    Shader, ShaderKind, ShaderSystem,
};
use wasm_bindgen::prelude::*;
use web_sys::*;

// static MESH_NON_SKINNED_VS: &'static str = include_str!("./mesh-non-skinned-vertex.glsl");
// static MESH_NON_SKINNED_FS: &'static str = include_str!("./mesh-non-skinned-fragment.glsl");

static MESH_NON_SKINNED_VS: &'static str = include_str!("./quad-vertex.glsl");
static MESH_NON_SKINNED_FS: &'static str = include_str!("./quad-fragment.glsl");

pub struct WebShader {
    pub program: WebGlProgram,
    uniforms: RefCell<HashMap<String, WebGlUniformLocation>>
}

pub struct WebShaderSystem {
    programs: HashMap<ShaderKind, WebShader>,
    active_program: RefCell<ShaderKind>
}

impl ShaderSystem<WebGlRenderingContext, WebShader> for WebShaderSystem {
    fn new(gl: &WebGlRenderingContext) -> WebShaderSystem {
        let mut programs = HashMap::new();

        let non_skinned_shader =
            WebShader::new(&gl, MESH_NON_SKINNED_VS, MESH_NON_SKINNED_FS).unwrap();

        let active_program = RefCell::new(ShaderKind::NonSkinnedMesh);
        gl.use_program(Some(&non_skinned_shader.program));

        programs.insert(ShaderKind::NonSkinnedMesh, non_skinned_shader);

        WebShaderSystem {
            programs,
            active_program,
        }
    }

    fn get_shader(&self, shader_kind: &ShaderKind) -> Option<&WebShader> {
        self.programs.get(shader_kind)
    }

    fn use_program(&self, gl: &WebGlRenderingContext, shader_kind: ShaderKind) {
        if *self.active_program.borrow() == shader_kind {
            return;
        }

        gl.use_program(Some(&self.programs.get(&shader_kind).unwrap().program));
        *self.active_program.borrow_mut() = shader_kind;
    }
}

impl Shader<WebGlRenderingContext, WebGlUniformLocation> for WebShader {
    type Error = JsValue;
    type Output = WebShader;

    fn new(
        gl: &WebGlRenderingContext,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<WebShader, Self::Error> {
        let vert_shader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vert_shader)?;
        let frag_shader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, frag_shader)?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        let uniforms = RefCell::new(HashMap::new());

        Ok(WebShader { program, uniforms })
    }

    fn get_uniform_location(
        &self,
        gl: &WebGlRenderingContext,
        name: &str,
    ) -> Option<WebGlUniformLocation> {
        // TODO: why option?
        let mut uniforms = self.uniforms.borrow_mut();

        if uniforms.get(name).is_none() {
            let uniform_location = gl
                .get_uniform_location(&self.program, name)
                .expect(&format!(r#"Uniform '{}' not found"#, name));

            uniforms.insert(name.to_string(), uniform_location);
        }

        Some(
            uniforms
                .get(name)
                .expect("Could not find uniform location")
                .clone(),
        )
    }
}

/// Create a shader program using the WebGL APIs
fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

/// Link a shader program using the WebGL APIs
fn link_program(
    gl: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);

    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
