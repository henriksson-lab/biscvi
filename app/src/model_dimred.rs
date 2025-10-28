use crate::{appstate::{AsyncData, PerCellDataSource}, component_umap_main::{UmapColoring, UmapColoringWithData, UmapView}, core_model::*};

use yew::{prelude::*};

use crate::component_umap_left::MetadataView;
use crate::component_umap_right::FeatureView;

impl Model {
    
    ////////////////////////////////////////////////////////////
    /// Get the current coloring data
    pub fn get_umap_coloring(&self) -> UmapColoringWithData {
        match &self.color_umap_by {
            UmapColoring::None => UmapColoringWithData::None,
            UmapColoring::ByMeta(name) => {
                let dat=self.current_data.lock().unwrap().get_metadata(&name);
                UmapColoringWithData::ByMeta(name.clone(), dat)
            },
        }
    }


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_dimred_page(&self, ctx: &Context<Self>) -> Html {

        //Callback: Hovering a certain cell
        let on_cell_hovered = Callback::from(move |_name: Option<usize>| {
        });

        //Callback: Clicked on a cell
        let on_cell_clicked = Callback::from(move |_name: Vec<usize>| {
        });

        //Callback: coloring by something
        let on_colorbymeta= ctx.link().callback(move |name: PerCellDataSource| {
            Msg::RequestSetColorByMeta(name)  // UmapColoring instead?
        });

        //Get reduction
        let mut current_umap_data = AsyncData::NotLoaded;
        if let Some(current_reduction) = &self.current_reduction {
            current_umap_data = self.current_data.lock().unwrap().get_reduction(current_reduction)
        }

        //Get current coloring data
        let coloring_data = self.get_umap_coloring();

        html! {
            <div>
                <div class="biscvi-dimred-maindiv"> ////////// if behind everything, could take full screen!! but buttons need space adjustment
                    <UmapView 
                        on_cell_hovered={on_cell_hovered} 
                        on_cell_clicked={on_cell_clicked} 
                        umap={current_umap_data} 
                        color_umap_by={coloring_data.clone()} 
                        last_component_size={self.last_component_size.clone()}
                    />
                </div>
                <MetadataView 
                    current_datadesc={self.current_datadesc.clone()} 
                    on_colorbymeta={on_colorbymeta.clone()}
                    current_colorby={self.current_colorby.clone()}
                />
                <FeatureView
                    current_datadesc={self.current_datadesc.clone()}
                    on_colorbyfeature={on_colorbymeta}  //expand, not just meta?
                    current_colorby={self.current_colorby.clone()}
                />
            </div>
        }
    }


}