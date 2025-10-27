
use std::collections::HashSet;

use my_web_app::countfile_struct::CountFileMetaColumnDesc;
use my_web_app::DatasetDescResponse;
use yew::{html, Callback, Component, Context, Html, MouseEvent, NodeRef};
use yew::Properties;

use crate::appstate::AsyncData;
use crate::component_umap_main::get_palette_for_cats;


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
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {            


            MsgMetadata::SetColorBy(metadata_name) => {
                self.last_colorby=metadata_name.clone();
                ctx.props().on_colorbymeta.emit(metadata_name);
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

        /*
        html! {

            <div style="margin: 0px; padding: 0px; user-select: none; width: 245px; display: flex; justify-content: space-between;">
                <div style="display: flex; align-items: baseline;">
                    <span class="ignore-capture" style="margin: 0px; height: 18px;">
                        <label class="bp5-control bp5-checkbox">
                            <input id="value-toggle-checkbox-assay-10x 3' v3" data-testid="categorical-value-select-assay-10x 3' v3" type="checkbox" checked=""/>
                            <span class="bp5-control-indicator">
                            </span>
                        </label>
                    </span>
                    <span class="bp5-popover-target" style="width: 188px; color: black; font-style: normal; display: inline-block; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px;">
                        <label for="value-toggle-checkbox-assay-10x 3' v3" data-testid="categorical-value" tabindex="-1" aria-label="10x 3' v3" aria-expanded="false" class="" style="width: 188px; color: black; font-style: normal; display: inline-block; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px;">
                            <span style="width: 100%; color: black; font-style: normal; display: flex; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px; justify-content: flex-start; padding: 0px;">
                                <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-shrink: 1; min-width: 5px;">
                                    {"10x&nbsp;"}
                                </span>
                                <span style="position: relative; overflow: hidden; white-space: nowrap;">
                                    <span style="color: transparent;">
                                        {"3' v3"}
                                    </span>
                                    <span style="position: absolute; right: 0px; color: black;">
                                        {"3' v3"}
                                    </span>
                                </span>
                            </span>
                        </label>
                    </span>
                    <div style="display: none;">
                    </div>
                </div>

                <span style="flex-shrink: 0;">
                </span>
            </div>
        };
         */
        /* 

        let _stuff = html! {

            <div style="padding: 4px 10px 4px 7px; display: flex; align-items: baseline; justify-content: space-between; margin-bottom: 2px; border-radius: 2px;">
                <div style="margin: 0px; padding: 0px; user-select: none; width: 245px; display: flex; justify-content: space-between;">
                    <div style="display: flex; align-items: baseline;">
                        <span class="ignore-capture" style="margin: 0px; height: 18px;">
                            <label class="bp5-control bp5-checkbox">
                                <input type="checkbox" checked=false/>
                                <span class="bp5-control-indicator">
                                </span>
                            </label>
                        </span>
                        <span style="width: 188px; color: black; font-style: normal; display: inline-block; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px;">
                            <label style="width: 188px; color: black; font-style: normal; display: inline-block; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px;">
                                <span style="width: 100%; color: black; font-style: normal; display: flex; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px; justify-content: flex-start; padding: 0px;">
                                    <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-shrink: 1; min-width: 5px;">
                                            {"10x&nbsp;"}
                                    </span>
                                    <span style="position: relative; overflow: hidden; white-space: nowrap;">
                                        <span style="color: transparent;">
                                            {"3' v3"} //why?
                                        </span>
                                        <span style="position: absolute; right: 0px; color: black;">
                                            {"3' v3"}
                                        </span>
                                    </span>
                                </span>
                            </label>
                        </span>
                        <div style="display: none;">
                        </div>
                    </div>
                <span style="flex-shrink: 0;">
                </span>
                </div>
                <div style="white-space: nowrap;">
                    <span style="display: inline-block; vertical-align: baseline;">
                        <span style="color: black; top: 10px;">
                            {"12345"}
                        </span>
                       <span style="vertical-align: baseline;">
                           <svg display="auto" style="top: 3px; width: 15px; height: 15px; margin-left: 5px; position: relative; background-color: rgb(110, 64, 170);"></svg>
                        </span>
                    </span>
                </div>
            </div>

        };
*/

        
        let current_datadesc = ctx.props().current_datadesc.clone();

        if let AsyncData::Loaded(current_datadesc) = &current_datadesc {

            for (meta_name,meta_data) in current_datadesc.meta.iter() {

                //////////// Discrete categories
                if let CountFileMetaColumnDesc::Categorical(categories ) = meta_data {

                    let palette = get_palette_for_cats(categories.len());

                    //// List of all levels
                    let mut list_levels = Vec::new();

                    if self.expanded_meta.contains(meta_name) {
                        for (level_i, level_name) in categories.iter().enumerate() {

                            //Show a palette if this category is selected
                            //TODO extract color
                            //let r=100;
                            let col = palette.get(level_i % palette.len()).unwrap();

                            let num_cells = "";
                            
                            list_levels.push(                                
                                html! {
                                    <div style="padding: 4px 10px 4px 7px; display: flex; align-items: baseline; justify-content: space-between; margin-bottom: 2px; border-radius: 2px;">
                                        <div style="margin: 0px; padding: 0px; user-select: none; width: 245px; display: flex; justify-content: space-between;">
                                            <div style="display: flex; align-items: baseline;">
                                                <span class="ignore-capture" style="margin: 0px; height: 18px;">
                                                    <label class="bp5-control bp5-checkbox">
                                                        <input type="checkbox" checked=true/> /////////// hightlight even if not hovering
                                                        //<span class="bp5-control-indicator">
                                                        //</span>
                                                    </label>
                                                </span>
                                                <span style="width: 188px; color: black; font-style: normal; display: inline-block; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px;">
                                                    <label style="width: 188px; color: black; font-style: normal; display: inline-block; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px;">
                                                        <span style="width: 100%; color: black; font-style: normal; display: flex; overflow: hidden; line-height: 1.1em; height: 1.1em; vertical-align: middle; margin-right: 16px; justify-content: flex-start; padding: 0px;">
                                                            <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-shrink: 1; min-width: 5px;" class="bisci-label-left">
                                                                {level_name}
                                                            </span>
                                                        </span>
                                                    </label>
                                                </span>
                                                //<div style="display: none;">
                                                //</div>
                                            </div>
                                        <span style="flex-shrink: 0;">
                                        </span>
                                        </div>
                                        <div style="white-space: nowrap;">
                                            <span style="display: inline-block; vertical-align: baseline;">
                                                <span style="color: black; top: 10px;" class="bisci-label-left">
                                                    {format!("{}",num_cells)} 
                                                </span>
                                            <span style="vertical-align: baseline;">
                                                <svg display="auto" style={format!{"top: 3px; width: 15px; height: 15px; margin-left: 5px; position: relative; background-color: rgb({}, {}, {});", col.0*255.0, col.1*255.0, col.2*255.0}}></svg>
                                                </span>
                                            </span>
                                        </div>
                                    </div>

                                }
                            );                                

                        }
                    }

                    
                    let meta_name_copy = meta_name.clone();
                    let toggle_expand = ctx.link().callback(move |_e: MouseEvent | { 
                        MsgMetadata::ToggleExpand(meta_name_copy.clone())
                    });

                    //Callback to color by this column
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

                    let style_colorbutton = if self.last_colorby == *meta_name {
                        "background-color:  #FF0000; "
                    } else {
                        ""
                    };

                    //Callback to color by this column
                    let meta_name_copy = meta_name.clone();
                    let cb_color_by = ctx.link().callback(move |_e: MouseEvent | { 
                        MsgMetadata::SetColorBy(meta_name_copy.clone())
                    });

                    //// Option to color by continuous metadata
                    list_meta_cont.push(
                        html! { 
                            <div>
                                <div style="width:100%; display:table;">
                                    <div style="display:table-cell;">
                                        <input type="checkbox" checked=true />
                                        { meta_name.clone() }
                                        //<span onclick={toggle_expand}>
                                        //    { arrow_down_svg.clone()}
                                        //</span>
                                    </div>
                                    <div style="text-align: right;">
                                        <button type="button" style={style_colorbutton} onclick={cb_color_by}>
                                            {colorby_svg.clone()}
                                        </button>
                                    </div>
                                </div> 
                            </div>
                        }
/*

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
                         */

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




