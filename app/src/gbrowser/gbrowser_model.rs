use crate::{core_model::*, gbrowser::GBrowseView};

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// Render genome browser pane
    pub fn view_gbrowser_page(&self, ctx: &Context<Self>) -> Html {


        //Callback: send message to component above
        let on_propagate= ctx.link().callback(move |sig: MsgCore| {
            log::debug!("propagate {:?}", sig);
            sig
        });



        html! {
            <div>
                <GBrowseView 
    //                on_cell_hovered={on_cell_hovered} 
  //                  on_cell_clicked={on_cell_clicked} 
                    on_propagate={on_propagate}
                    last_component_size={self.last_component_size.clone()}
//                    current_colorby={self.current_colorby.clone()}
  //                  reductions={self.reductions.clone()}
    //                metadatas={self.metadatas.clone()}
                    current_datadesc={self.current_datadesc.clone()}
                    current_gff={self.current_gff.clone()}
      //              current_reduction_name={self.current_reduction.clone()}
                />


            </div>
        }
    }



}







