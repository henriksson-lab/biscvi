pub mod core_model;
pub mod model_dimred;
pub mod model_files;
pub mod camera;
pub mod component_umap_main;
pub mod component_umap_left;
pub mod component_umap_right;
pub mod umap_index;
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
