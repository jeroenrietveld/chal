#[macro_use]
extern crate lazy_static;
extern crate nalgebra;
extern crate blender_mesh;
extern crate blender_armature;
extern crate bincode;
extern crate chal_engine;
extern crate specs;
#[macro_use]
extern crate specs_derive;

#[macro_use]
mod utils;
mod engine;
mod shader;
mod render;
mod canvas;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
