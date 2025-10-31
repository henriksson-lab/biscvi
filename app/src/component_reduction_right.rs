use std::collections::HashSet;

use my_web_app::DatasetDescResponse;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use web_sys::{EventTarget, HtmlInputElement};
use yew::Event;
use yew::virtual_dom::VNode;
use yew::{Callback, Component, Context, Html, KeyboardEvent, MouseEvent, NodeRef, html};
use yew::Properties;

use crate::appstate::{AsyncData, BiscviCache, MetadataData, PerCellDataSource};
use crate::histogram::FeatureHistogram;

////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgFeature {
    SetColorBy(PerCellDataSource),
//    ToggleExpand(String)
    FeatureSearchChange(String, bool),
    SetLastCountName(String),
    //FeatureSearchMatChange(String),
}


////////////////////////////////////////////////////////////
/// Properties for FeatureView
#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_datadesc: AsyncData<DatasetDescResponse>,
    pub on_colorbyfeature: Callback<PerCellDataSource>,

    pub current_colorby: PerCellDataSource,
    //pub current_data: Arc<Mutex<BiscviData>>,

    pub metadatas: BiscviCache<MetadataData>,          // call something else? countdatas?
}


////////////////////////////////////////////////////////////
/// This component shows a list of features that the main plot can be colored by
pub struct FeatureView {
    pub node_ref: NodeRef,

    pub expanded_meta: HashSet<String>,
    pub selected_meta: HashSet<String>,

    pub open_features: Vec<PerCellDataSource>,

    //Search feature state
    pub last_search_feature_input: String,
    pub last_search_feature_mat: String,
}

impl Component for FeatureView {
    type Message = MsgFeature;
    type Properties = Props;

    ////////////////////////////////////////////////////////////
    /// Create this component
    fn create(_ctx: &Context<Self>) -> Self {    
        Self {
            node_ref: NodeRef::default(),
            expanded_meta: HashSet::new(),
            selected_meta: HashSet::new(),
            open_features: Vec::new(),
            last_search_feature_input: String::new(),
            last_search_feature_mat: String::new(),
        }
    }


    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {            

            //////// Color by a given feature
            MsgFeature::SetColorBy(feature_name) => {
                //self.last_colorby=feature_name.clone();
                ctx.props().on_colorbyfeature.emit(feature_name);
                true
            },

            //////// Key pressed in search feature input
            MsgFeature::FeatureSearchChange(value, is_enter) => {
                self.last_search_feature_input = value.clone();
                let feature_name = PerCellDataSource::Counts(self.last_search_feature_mat.clone(), value.clone());
                if is_enter {
                    self.last_search_feature_input = String::new();
                    //Check that the feature is not already there, or empty
                    if !self.open_features.contains(&feature_name) && value != "" {
                        self.open_features.push(feature_name.clone());
                        ctx.props().on_colorbyfeature.emit(feature_name); // Color by this feature right away
                    }
                }
                true
            },

            //////// Component updated, and a list of count tables is now present. UI has already been updated
            MsgFeature::SetLastCountName(countname) => {
                self.last_search_feature_mat = countname;
                false
            }

/*
            MsgFeature::ToggleExpand(Feature_name) => {
                if self.expanded_meta.contains(&Feature_name) {
                    self.expanded_meta.remove(&Feature_name);
                } else {
                    self.expanded_meta.insert(Feature_name);
                }
                true
            },
 */

        }
    }



    ////////////////////////////////////////////////////////////
    /// Main rendering function for panel of features
    fn view(&self, ctx: &Context<Self>) -> Html {

        let _svg_arrowdown = html! {
            <svg height="16" role="img" viewBox="0 0 16 16" width="16">
                <path d="M12 5c-.28 0-.53.11-.71.29L8 8.59l-3.29-3.3a1.003 1.003 0 00-1.42 1.42l4 4c.18.18.43.29.71.29s.53-.11.71-.29l4-4A1.003 1.003 0 0012 5z" fill-rule="evenodd"></path>
            </svg>
        };


        //log::debug!("open features");
        //log::debug!("{:?}", self.open_features);

        //Create controls for all open features
        let mut list_features:Vec<Html> = Vec::new();
        for f in &self.open_features {
            match f {
                PerCellDataSource::Counts(count_name, feature_name) => {
                    let one_gene = self.make_one_feature(ctx, count_name, feature_name);
                    list_features.push(one_gene);
                },
                _ => {
                    log::error!("PerCellDataSource in feature table {}", f);
                }
            }
        }

        //SVG for search icon
        let svg_search = html! {
            <svg data-icon="search" height="16" role="img" viewBox="0 0 16 16" width="16">
                <path d="M15.55 13.43l-2.67-2.68a6.94 6.94 0 001.11-3.76c0-3.87-3.13-7-7-7s-7 3.13-7 7 3.13 7 7 7c1.39 0 2.68-.42 3.76-1.11l2.68 2.67a1.498 1.498 0 102.12-2.12zm-8.56-1.44c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z" fill-rule="evenodd"></path>
            </svg>
        };

        //Generate SELECT for all count tables
        let mut list_feature_types = Vec::new();
        if let AsyncData::Loaded(current_datadesc) = &ctx.props().current_datadesc {
            for (mat_name, _mat) in &current_datadesc.matrices {
                list_feature_types.push(mat_name.clone());
            }
        }
        let mut list_feature_types_html = Vec::new();
        for t in &list_feature_types {
            list_feature_types_html.push(html! {
                <option value={t.clone()} selected={&self.last_search_feature_mat==t}>
                    {t}
                </option>
            });
        }

        //Keep track of currently selected count table. It is not populated at first, so need to grab a value once the data becomes available
        if self.last_search_feature_mat=="" && !list_feature_types.is_empty() {
            let first_entry = list_feature_types.get(0).unwrap();
            ctx.link().send_message(MsgFeature::SetLastCountName(first_entry.clone()));         
        }

        //Callback: change of count matrix
        let cb_change_search_mat = ctx.link().callback(move |e: Event | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlSelectElement = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()).expect("wrong type");
            let t=input.value();
            e.prevent_default();
            MsgFeature::SetLastCountName(t.clone())
        });

        //Callback for keypresses on the feature search input
        let input_onkeyup = ctx.link().callback(move |e: KeyboardEvent | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlInputElement = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).expect("wrong type");
            let cur_value = input.value();

            e.prevent_default();
            let is_enter = e.key() == "Enter" || e.key_code() == 13;
            if is_enter {
                //empty it
                input.set_value(""); 
            } 
            MsgFeature::FeatureSearchChange(cur_value, is_enter)
        });

        //Create autocomplete list for search -- get relevant features
        let mut list_autocomplete_red = Vec::new();
        //log::debug!("self.last_search_feature_input {}", self.last_search_feature_input);
        if !self.last_search_feature_input.is_empty() {
            if let AsyncData::Loaded(current_datadesc) = &ctx.props().current_datadesc {
                let mat = current_datadesc.matrices.get(&self.last_search_feature_mat);
                if let Some(mat) = mat {
                    let last_search_feature_input = self.last_search_feature_input.to_lowercase();  //// TODO: need to store feature name too
                    for item in &mat.list_feature_names {
                        let item_lower = item.to_lowercase();
                        if item_lower.starts_with(&last_search_feature_input) {  // TODO: for speed, it would make sense to have features sorted in alphabetic order and do some type of binary search for the start position in list
                            list_autocomplete_red.push(item.clone());
                        }
                    }
                } else {
                    log::debug!("No count table found for {}", self.last_search_feature_mat);
                }
            }
        }
        //Create autocomplete list for search -- make html
        let mut list_autocomplete_html = Vec::new();
        for t in &list_autocomplete_red {

            let t_copy = t.clone();
            let cb_onclick = ctx.link().callback(move |e: MouseEvent | { 
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();

                //Empty the search input
                let target = document.get_element_by_id("search-feature-input");
                let input: HtmlInputElement = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).expect("wrong type");
                input.set_value("");

                e.prevent_default();
                MsgFeature::FeatureSearchChange(t_copy.clone(), true)
            });

            list_autocomplete_html.push(html! {
                <div onclick={cb_onclick}>
                    {t}
                </div>
            });
        }

        //Compose the view
        html! {
            <div class="biscvi-dimred-rightdiv">
                <div>
                    <div> //  class="bp5-input-group bp5-fill bp5-popover-target bp5-popover-open"
                        <select onchange={cb_change_search_mat}>
                            {list_feature_types_html}
                        </select>
                        <span> //  aria-hidden="true" tabindex="-1" class="bp5-icon bp5-icon-search"
                            <div class="autocomplete">
                                <input type="text" autocomplete="off" placeholder="Search feature" aria-autocomplete="list" value={self.last_search_feature_input.clone()} onkeyup={input_onkeyup} id="search-feature-input"/> // aria-controls="listbox-7"  class="bp5-input" aria-haspopup="listbox" role="combobox"   ref={input_node_ref} 
                                <div class="autocomplete-items"> 
                                    {list_autocomplete_html}
                                </div>
                            </div>
                            {svg_search}
                        </span>
                    </div>

                </div>
                <div>
                    {list_features}                
                </div>
            </div>
        }
    }



    ////////////////////////////////////////////////////////////
    /// Called after component has been rendered
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    }


}


impl FeatureView {

    ////////////////////////////////////////////////////////////
    /// Render controls for one open feature
    fn make_one_feature(&self, ctx: &Context<Self>, count_name: &String, feature_name: &String) -> VNode {
        let combo_feature = PerCellDataSource::Counts(count_name.clone(), feature_name.clone());

        let _infocircle_svg = html! {
            <svg width="16" height="16" focusable="false" viewBox="0 0 16 16" fillcontrast="white"> // class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium css-qtic4f" 
                <path fill-rule="evenodd" d="M14.4 8A6.4 6.4 0 1 1 1.6 8a6.4 6.4 0 0 1 12.8 0M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0M6.971 4.857a1.029 1.029 0 1 1 2.058 0 1.029 1.029 0 0 1-2.058 0m1.943 6.4a.914.914 0 1 1-1.828 0V7.943a.914.914 0 1 1 1.828 0z"></path>
            </svg>
        };

        let svg_maximize = html! {
            <svg data-icon="maximize" height="10" role="img" viewBox="0 0 16 16" width="10">
                <path d="M5.99 8.99c-.28 0-.53.11-.71.29l-3.29 3.29v-1.59c0-.55-.45-1-1-1s-1 .45-1 1v4c0 .55.45 1 1 1h4c.55 0 1-.45 1-1s-.45-1-1-1H3.41L6.7 10.7a1.003 1.003 0 00-.71-1.71zm9-9h-4c-.55 0-1 .45-1 1s.45 1 1 1h1.59l-3.3 3.3a.99.99 0 00-.29.7 1.003 1.003 0 001.71.71l3.29-3.29V5c0 .55.45 1 1 1s1-.45 1-1V1c0-.56-.45-1.01-1-1.01z" fill-rule="evenodd"></path>
            </svg>
        };

        //SVG: icon to represent "showing more info"
        let svg_more = html! {
            <svg data-icon="more" height="10" role="img" viewBox="0 0 16 16" width="10">
                <path d="M2 6.03a2 2 0 100 4 2 2 0 100-4zM14 6.03a2 2 0 100 4 2 2 0 100-4zM8 6.03a2 2 0 100 4 2 2 0 100-4z" fill-rule="evenodd"></path>
            </svg>
        };

        //SVG: icon to color by this meta
        let svg_colorby = html! {
            <svg data-icon="tint" height="12" role="img" viewBox="0 0 16 16" width="12">
                <path d="M7.88 1s-4.9 6.28-4.9 8.9c.01 2.82 2.34 5.1 4.99 5.1 2.65-.01 5.03-2.3 5.03-5.13C12.99 7.17 7.88 1 7.88 1z" fill-rule="evenodd"></path>
            </svg>
        };

        let histo_svg = self.make_histogram(ctx, count_name, feature_name);


        //Callback to color by this column
        let combo_feature_copy= combo_feature.clone();
        let cb_color_by = ctx.link().callback(move |_e: MouseEvent | { 
            MsgFeature::SetColorBy(combo_feature_copy.clone())
        });

        let style_colorby_button = if ctx.props().current_colorby == combo_feature {
            "background-color: #FF0000;"
        } else {
            ""
        };



        html! {
            <div>
                <div style="margin-left: 5px; margin-right: 0px; margin-top: 2px; display: flex; justify-content: space-between; align-items: center;">
                    <div style="display: flex; justify-content: space-between; width: 100%;">  // role="menuitem" tabindex="0" data-testid="XBP1:gene-expand"    cursor: pointer; 
                        <div>
                            <span style="width: 60px; display: inline-block;">  // class="bp5-popover-target" 
                                <span style="width: 60px; display: inline-block;"> // data-testid="XBP1:gene-label" aria-label="XBP1" aria-expanded="false" class="" tabindex="0" 
                                    <span style="width: 100%; display: flex; overflow: hidden; justify-content: flex-start; padding: 0px;">
                                        <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex-shrink: 1; min-width: 5px;">
                                            {feature_name}
                                        </span>
                                    </span>
                                </span>
                            </span>
                        </div>
                        /* 
                        <div> //  class="css-567ik4"
                            <button type="button"> //class="MuiButtonBase-root MuiIconButton-root MuiIconButton-sizeMedium css-1m6iddg" tabindex="0"  data-chromatic="ignore"
                                <div> // class="css-1ft6wgl"
                                    {infocircle_svg}
                                </div>
                            </button>
                        </div>
                        */
                        <div style="display: flex; flex-grow: 1;"> //width: 5%;
                            <div style="padding: 0px; background-color: white;">
                                {histo_svg}
                            </div>
                        </div>
                    </div>
                    <div style="flex-shrink: 0; margin-left: 2px; display: flex;">
                        <button type="button" style="margin-right: 2px;"> // class="bp5-button bp5-minimal bp5-small" 
                            <span aria-hidden="true"> // class="bp5-icon bp5-icon-maximize"
                                {svg_maximize}
                            </span>
                        </button>
                        <div aria-controls="listbox-11"  aria-expanded="false" aria-haspopup="listbox" role="combobox"> // class="bp5-popover-target"
                            <button type="button"  style="margin-right: 2px;"> // class="bp5-button bp5-minimal bp5-small"
                                <span aria-hidden="true"> // class="bp5-icon bp5-icon-more"
                                    {svg_more}
                                </span>
                            </button>
                        </div>
                        <button type="button" onclick={cb_color_by} style={style_colorby_button}> // class="bp5-button bp5-active bp5-minimal bp5-small bp5-intent-primary"
                            <span aria-hidden="true"> //  class="bp5-icon bp5-icon-tint"
                                {svg_colorby}
                            </span>
                        </button>
                    </div>
                </div>
            </div>
        }

    }




    ////////////////////////////////////////////////////////////
    /// Render the histogram for one feature
    fn make_histogram(&self, ctx: &Context<Self>, count_name: &String, feature_name: &String) -> VNode {

        let mut list_bins_html = Vec::new();

        let hist_height=15.0;
        let hist_width=150.0; //fit resizing component to get this?

        let id = PerCellDataSource::Counts(count_name.clone(), feature_name.clone());
        if let AsyncData::Loaded(x) = ctx.props().metadatas.data.get(&id) {

            let h = FeatureHistogram::build(x.as_ref());

            //log::debug!("made hist {:?}",h);
            if let FeatureHistogram::ContinuousFeatureHistogram(h) = h {

                //let max_count = h.max_count as f64;
                let scale_y = (hist_height)/(h.max_count as f32);
                let scale_x = hist_width/(h.max as f32);
                let bin_width = hist_width/(h.bin.len() as f32);

                for (bin,cnt) in h.bin.iter().zip(h.count.iter()) {
                    let h = (*cnt as f32) * scale_y;
                    let x = (*bin as f32)*scale_x;
                    list_bins_html.push(
                        html! {
                            <rect 
                                x={x.to_string()} 
                                y={(hist_height-h).to_string()}
                                width={bin_width.to_string()}
                                height={h.to_string()} 
                                style="fill: rgb(0, 0, 0);"  
                            />
                        }
                    );
                }
            }
        }
        // "fill: rgb(175, 240, 91);"

                   // log::debug!("hist html {:?}",list_bins_html);


        html! {
            <svg width={hist_width.to_string()} height={hist_height.to_string()} style="display: block;">  
                {list_bins_html}
            </svg>
        }
    }
}

