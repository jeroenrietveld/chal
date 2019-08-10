use std::cell::RefCell;
use std::rc::Rc;

use chal_engine::assets::Assets;
use chal_engine::components::Position;
use chal_engine::shader::ShaderKind;
use chal_engine::state::State;
use specs::RunNow;
use specs::{Builder, World};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

use crate::canvas::create_webgl_context;
use crate::render::component::{Mesh, RenderSystem};
use crate::render::Renderer;
use crate::utils;

pub struct WebGlContext {
    pub contexts: RefCell<Vec<WebGlRenderingContext>>,
}

impl WebGlContext {
    // pub fn context(&self) -> &WebGlRenderingContext {
    //     &self.contexts.borrow()[0]
    // }

    // pub fn context_mut(&mut self) -> &WebGlRenderingContext {
    //     &self.contexts.borrow_mut()[0]
    // }
}

unsafe impl Send for WebGlContext {}
unsafe impl Sync for WebGlContext {}

lazy_static! {
    pub static ref GLC: WebGlContext = WebGlContext {
        contexts: RefCell::new(Vec::new()),
    };
}

#[wasm_bindgen]
pub struct Engine {
    renderer: Renderer,
    assets: Assets,
    world: World,
    render_system: RenderSystem,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        utils::set_panic_hook();
        log!("new");

        // let state = Rc::new(RefCell::new(State::new()));
        let state = State::new();
        // let gl_context = create_webgl_context(Rc::clone(&state)).unwrap();
        let gl_context = create_webgl_context().unwrap();
        GLC.contexts.borrow_mut().push(gl_context);

        let renderer = Renderer::new();
        let mut assets = Assets::new();
        let world = setup_world(state);

        let render_system = RenderSystem::new();

        assets.download_meshes(include_bytes!("./meshes/meshes.bytes"));

        Engine {
            renderer,
            assets,
            world,
            render_system,
        }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        log!("start");
        self.world.create_entity()
            .with(Position { x: 0., y: 0. })
            .with(Mesh::new(
            "Test",
            vec![
                // 0., 1., 1., 0., 0., 0., // first
                // 0., 1., 1., 1., 1., 0.,
                0., 0., 0., 0.5, 0.7, 0.
            ],
            ShaderKind::NonSkinnedMesh,
        )).build();
        Ok(())
    }

    pub fn update(&self, dt: f32) {
        // self.state.borrow_mut().advance_clock(dt);
        let mut world_dt = self.world.write_resource::<DeltaTime>();
        *world_dt = DeltaTime(dt);
    }

    pub fn render(&mut self) {
        // self.renderer.render(&self.state.borrow(), &self.assets)
        self.render_system.run_now(&self.world.res);
        self.world.maintain();
    }
}

use std::default::Default;

struct DeltaTime(f32);
pub struct GameState(pub State);

impl Default for GameState {
    fn default() -> GameState {
        GameState(State::default())
    }
}

fn setup_world(state: State) -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Mesh>();

    world.add_resource(DeltaTime(0.0));
    world.add_resource(GameState(state));

    // world
    //     .create_entity()
    //     // .with(Mesh{}) TODO
    //     .build();

    world
}
