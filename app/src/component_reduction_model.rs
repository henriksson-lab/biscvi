use crate::{appstate::{PerCellDataSource}, component_reduction_main::{ReductionView}, core_model::*};

use yew::{prelude::*};

use crate::component_reduction_left::MetadataView;
use crate::component_reduction_right::FeatureView;

impl Model {


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
            MsgCore::RequestSetColorByMeta(name)  // UmapColoring instead?
        });

        let on_propagate= ctx.link().callback(move |sig: MsgCore| {
            log::debug!("propagate {:?}", sig);
            sig
        });
         
        html! {
            <div>
                <div class="biscvi-dimred-maindiv"> ////////// if behind everything, could take full screen!! but buttons need space adjustment
                    <ReductionView 
                        on_cell_hovered={on_cell_hovered} 
                        on_cell_clicked={on_cell_clicked} 
                        on_propagate={on_propagate}
                        last_component_size={self.last_component_size.clone()}
                        current_colorby={self.current_colorby.clone()}
                        reductions={self.reductions.clone()}
                        metadatas={self.metadatas.clone()}
                        current_datadesc={self.current_datadesc.clone()}
                        current_reduction_name={self.current_reduction.clone()}
                    />
                </div>
                <MetadataView 
                    current_datadesc={self.current_datadesc.clone()} 
                    on_colorbymeta={on_colorbymeta.clone()}
                    current_colorby={self.current_colorby.clone()}
                />
                <FeatureView
                    metadatas={self.metadatas.clone()}
                    current_datadesc={self.current_datadesc.clone()}
                    on_colorbyfeature={on_colorbymeta}  //expand, not just meta?
                    current_colorby={self.current_colorby.clone()}
                    //current_data={self.current_data.clone()}
                />
            </div>
        }
    }


}