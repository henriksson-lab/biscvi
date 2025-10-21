use crate::{appstate::AsyncData, component_umap::UmapView, core_model::*};

use yew::{html::{ImplicitClone, IntoPropValue}, prelude::*};



impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_dimred_page(&self, _ctx: &Context<Self>) -> Html {


        let mut list_meta:Vec<Html> = Vec::new();

        if let Some(current_datadesc) = &self.current_datadesc {

            for s in current_datadesc.meta.keys() {
                list_meta.push(
                    html! { 
                        <li>
                            { s.clone() }
                        </li> 
                    }
                );
            }

        }

        let on_cell_hovered = Callback::from(move |_name: Option<usize>| {
        });

        let on_cell_clicked = Callback::from(move |_name: Vec<usize>| {
        });

       // let on_cell_clicked= ctx.link().callback(move |name: Vec<usize>| {
            //Msg::ClickSequence(name)
        //});

        //self.current_reduction
        let mut current_umap_data = AsyncData::NotLoaded;
        if let Some(current_reduction) = &self.current_reduction {
            current_umap_data = self.current_data.lock().unwrap().get_reduction(current_reduction)  //reductions.get(current_reduction);
        }

        log::debug!("view_dimred_page");


        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    {"UMAP here"}            

                    <UmapView on_cell_hovered={on_cell_hovered} on_cell_clicked={on_cell_clicked} umap={current_umap_data} />

                </div>
                <div class="biscvi-dimred-leftdiv">
                    <div>
                        <ul>
                            {
                                list_meta
                            }
                        </ul>
                        {"Color by category here"}
                    </div>
                    <div>
                        {"Histogram of category?"}                        
                    </div>
                </div>
                <div class="biscvi-dimred-rightdiv">
                    {"Genes:"}
                    <input type="text"/>
                </div>
            </div>
        }
    }



}

