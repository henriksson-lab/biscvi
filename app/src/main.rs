pub mod core_model;
pub mod component_reduction_model;
pub mod model_files;
pub mod camera;
pub mod component_reduction_main;
pub mod component_reduction_left;
pub mod component_reduction_right;
pub mod closestpoint;
pub mod appstate;
pub mod resize;
pub mod histogram;

use crate::core_model::*;

////////////////////////////////////////////////////////////
/// Entry point for wasm application
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}
