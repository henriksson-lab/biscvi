pub mod core_model;
pub mod model_dimred;
pub mod model_files;
pub mod camera;
pub mod component_umap;
pub mod umap_index;

use crate::core_model::*;

////////////////////////////////////////////////////////////
/// x
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}
