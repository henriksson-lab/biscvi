
use std::collections::HashSet;

use my_web_app::countfile_struct::CountFileMetaColumnDesc;
use my_web_app::DatasetDescResponse;
use yew::{html, Callback, Component, Context, Html, MouseEvent, NodeRef};
use yew::Properties;

use crate::appstate::AsyncData;


// see https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs




////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgMetadata {
    SetColorBy(String),
    ToggleExpand(String)
}


////////////////////////////////////////////////////////////
/// x
#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_datadesc: AsyncData<DatasetDescResponse>,
    pub on_colorbymeta: Callback<String>,
}


////////////////////////////////////////////////////////////
/// 
/// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct MetadataView {
    pub node_ref: NodeRef,

    pub expanded_meta: HashSet<String>,
    pub selected_meta: HashSet<String>,

    pub last_colorby: String,
}



////////////////////////////////////////////////////////////
/// x
impl Component for MetadataView {
    type Message = MsgMetadata;
    type Properties = Props;

    ////////////////////////////////////////////////////////////
    /// x
    fn create(_ctx: &Context<Self>) -> Self {    
        Self {
            node_ref: NodeRef::default(),
            expanded_meta: HashSet::new(),
            selected_meta: HashSet::new(),
            last_colorby: "".into(),
        }
    }





    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {            


            MsgMetadata::SetColorBy(metadata_name) => {
                self.last_colorby=metadata_name.clone();
                true
            },


            MsgMetadata::ToggleExpand(metadata_name) => {
                if self.expanded_meta.contains(&metadata_name) {
                    self.expanded_meta.remove(&metadata_name);
                } else {
                    self.expanded_meta.insert(metadata_name);
                }
                true
            },

        }
    }




    ////////////////////////////////////////////////////////////
    /// x
    fn view(&self, ctx: &Context<Self>) -> Html {

        /*
        log::debug!("====================== render umap ");
        let umap = &ctx.props().umap;
        log::debug!("{:?}", umap);
        log::debug!("############################");
 */


        let mut list_meta_cat:Vec<Html> = Vec::new();
        let mut list_meta_cont:Vec<Html> = Vec::new();

        //TODO read from file
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

        let colorby_svg = html! {
            <svg data-icon="tint" height="16" role="img" viewBox="0 0 16 16" width="16">
                <path d="M7.88 1s-4.9 6.28-4.9 8.9c.01 2.82 2.34 5.1 4.99 5.1 2.65-.01 5.03-2.3 5.03-5.13C12.99 7.17 7.88 1 7.88 1z" fill-rule="evenodd"></path>
            </svg>
        };


        let current_datadesc = ctx.props().current_datadesc.clone();

        if let AsyncData::Loaded(current_datadesc) = &current_datadesc {

            for (meta_name,meta_data) in current_datadesc.meta.iter() {

                //////////// Discrete categories
                if let CountFileMetaColumnDesc::Categorical(categories ) = meta_data {

                    //// List of all levels
                    let mut list_levels = Vec::new();

                    if self.expanded_meta.contains(meta_name) {
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
                    }

                    
                    let meta_name_copy = meta_name.clone();
                    let toggle_expand = ctx.link().callback(move |_e: MouseEvent | { 
                        MsgMetadata::ToggleExpand(meta_name_copy.clone())
                    });

                    let meta_name_copy = meta_name.clone();
                    let cb_color_by = ctx.link().callback(move |_e: MouseEvent | { 
                        MsgMetadata::SetColorBy(meta_name_copy.clone())
                    });


                    let style_colorbutton = if self.last_colorby == *meta_name {
                        "background-color:  #FF0000; "
                    } else {
                        ""
                    };

                    //// Option to color by discrete metadata
                    list_meta_cat.push(
                        html! { 
                            <div>
                                <div style="width:100%; display:table;">
                                    <div style="display:table-cell;">
                                        <input type="checkbox" checked=true />
                                        { meta_name.clone() }
                                        <span onclick={toggle_expand}>
                                            { arrow_down_svg.clone()}
                                        </span>
                                    </div>
                                    <div style="text-align: right;">
                                        <button type="button" style={style_colorbutton} onclick={cb_color_by}>
                                            {colorby_svg.clone()}
                                        </button>
                                    </div>
                                </div> 
                                { list_levels }
                            </div>
                        }
                    );

                }

                //////////// Continuous categories
                if let CountFileMetaColumnDesc::Numeric() = meta_data {

                    //// Option to color by continuous metadata
                    list_meta_cont.push(
                        html! { 
                            <div>
                                <input type="checkbox" checked=true />
                                { meta_name.clone() }
                                <button type="button">
                                    {colorby_svg.clone()}
                                </button>
                                //// histogram or something here
                            </div> 
                        }
                    );

                }

                

            }

        }

        html! {
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
        }

    }



    ////////////////////////////////////////////////////////////
    /// x
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {



    }
}




