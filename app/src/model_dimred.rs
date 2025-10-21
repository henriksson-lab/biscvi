use crate::{appstate::AsyncData, component_umap::UmapView, core_model::*};

use my_web_app::{countfile_struct::CountFileMetaColumnDesc};
use yew::{prelude::*};



impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_dimred_page(&self, _ctx: &Context<Self>) -> Html {


        let mut list_meta_cat:Vec<Html> = Vec::new();
        let mut list_meta_cont:Vec<Html> = Vec::new();

        let _arrow_right_svg = html! {
            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 320 512" height="1em" width="1em" style="font-size: 10px; margin-left: 5px;">
                <path d="M285.476 272.971L91.132 467.314c-9.373 9.373-24.569 9.373-33.941 0l-22.667-22.667c-9.357-9.357-9.375-24.522-.04-33.901L188.505 256 34.484 101.255c-9.335-9.379-9.317-24.544.04-33.901l22.667-22.667c9.373-9.373 24.569-9.373 33.941 0L285.475 239.03c9.373 9.372 9.373 24.568.001 33.941z"></path>
            </svg>
        };

        let arrow_down_svg = html! {
            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" height="1em" width="1em" style="font-size: 10px; margin-left: 5px;">
                <path d="M207.029 381.476L12.686 187.132c-9.373-9.373-9.373-24.569 0-33.941l22.667-22.667c9.357-9.357 24.522-9.375 33.901-.04L224 284.505l154.745-154.021c9.379-9.335 24.544-9.317 33.901.04l22.667 22.667c9.373 9.373 9.373 24.569 0 33.941L240.971 381.476c-9.373 9.372-24.569 9.372-33.942 0z"></path>
            </svg>
        };

        let color_button = html! {
            <button type="button" aria-expanded="false">
                //<span aria-hidden="true" class="bp5-icon bp5-icon-tint">
                    <svg data-icon="tint" height="16" role="img" viewBox="0 0 16 16" width="16">
                        <path d="M7.88 1s-4.9 6.28-4.9 8.9c.01 2.82 2.34 5.1 4.99 5.1 2.65-.01 5.03-2.3 5.03-5.13C12.99 7.17 7.88 1 7.88 1z" fill-rule="evenodd"></path>
                    </svg>
                //</span>
            </button>
        };

        if let Some(current_datadesc) = &self.current_datadesc {

            for (meta_name,meta_data) in current_datadesc.meta.iter() {

                //////////// Discrete categories
                if let CountFileMetaColumnDesc::Categorical(categories ) = meta_data {

                    let mut list_levels = Vec::new();
                    for lvl in categories {
                        list_levels.push(
                            html! { 
                                <div style="margin-left: 15px">
                                    <input type="checkbox" checked=true />
                                    { lvl.clone() }
                                    //Could show number of cells here, as in cellxgene
                                    //hovering should make points bold, as in cellxgene
                                </div>
                            }
                        );
                    }

                    list_meta_cat.push(
                        html! { 
                            <div>
                                <input type="checkbox" checked=true />
                                { meta_name.clone() }
                                { arrow_down_svg.clone() }
                                { color_button.clone() }
                                { list_levels }
                            </div> 
                        }
                    );

                }

                //////////// Continuous categories
                if let CountFileMetaColumnDesc::Numeric() = meta_data {

                    list_meta_cont.push(
                        html! { 
                            <div>
                                <input type="checkbox" checked=true />
                                { meta_name.clone() }
                                { color_button.clone() }
                                //// histogram or something here
                            </div> 
                        }
                    );

                }

                

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
            current_umap_data = self.current_data.lock().unwrap().get_reduction(current_reduction)
        }

        log::debug!("view_dimred_page");


        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    <UmapView on_cell_hovered={on_cell_hovered} on_cell_clicked={on_cell_clicked} umap={current_umap_data} />
                </div>
                <div class="biscvi-dimred-leftdiv">
                    <div>
                        <span style="color:blue;font-weight:bold;">
                            {"Discrete categories:"}
                        </span>
                        { list_meta_cat }
                        <span style="color:blue;font-weight:bold;">
                            {"Continuous categories:"}
                        </span>
                        { list_meta_cont }
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

