pub mod core_model;

pub mod fileview;
pub mod redview;
pub mod gbrowser;
pub mod about;

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
