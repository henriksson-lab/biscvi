use std::collections::HashSet;

use my_web_app::DatasetDescResponse;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::virtual_dom::VNode;
use yew::{Callback, Component, Context, Html, KeyboardEvent, MouseEvent, NodeRef, html};
use yew::Properties;

use crate::appstate::{AsyncData, PerCellDataSource};


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgFeature {
    SetColorBy(PerCellDataSource),
//    ToggleExpand(String)
    FeatureSearchChange(String, bool),
    SetLastCountName(String),
}


////////////////////////////////////////////////////////////
/// Properties for FeatureView
#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_datadesc: AsyncData<DatasetDescResponse>,
    pub on_colorbyfeature: Callback<PerCellDataSource>,

    pub current_colorby: PerCellDataSource,
}


////////////////////////////////////////////////////////////
/// This component shows a list of features that the main plot can be colored by
pub struct FeatureView {
    pub node_ref: NodeRef,

    pub expanded_meta: HashSet<String>,
    pub selected_meta: HashSet<String>,

    pub open_features: Vec<PerCellDataSource>,

    pub last_counttype_select: String,
}

impl Component for FeatureView {
    type Message = MsgFeature;
    type Properties = Props;

    ////////////////////////////////////////////////////////////
    /// Create this component
    fn create(_ctx: &Context<Self>) -> Self {    
        log::debug!("creating right controls");
        Self {
            node_ref: NodeRef::default(),
            expanded_meta: HashSet::new(),
            selected_meta: HashSet::new(),
            open_features: Vec::new(),
            last_counttype_select: String::new(), /////// TODO: need to grab the value!
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
/* 
            MsgFeature::AddSearchFeature(counts_name, feature_name) => {
                //let counts_name = "RNA".to_string();
                let feature_name: PerCellDataSource = PerCellDataSource::Counts(counts_name, feature_name);
                //self.last_colorby=feature_name.clone();
                //ctx.props().on_colorbyfeature.emit(feature_name);
                log::debug!("adding features");
                self.open_features.push(feature_name);
                log::debug!("{:?}", self.open_features);
                true
            },*/

            //////// Key pressed in search feature input
            MsgFeature::FeatureSearchChange(value, is_enter) => {
                let feature_name = PerCellDataSource::Counts(self.last_counttype_select.clone(), value.clone());
                if is_enter {
                    //Check that the feature is not already there, or empty
                    if !self.open_features.contains(&feature_name) && value != "" {
                        self.open_features.push(feature_name.clone());
                        ctx.props().on_colorbyfeature.emit(feature_name); // Color by this feature right away
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },

            //////// Component updated, and a list of count tables is now present. UI has already been updated
            MsgFeature::SetLastCountName(countname) => {
                self.last_counttype_select = countname;
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


        log::debug!("open features");
        log::debug!("{:?}", self.open_features);

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
        list_feature_types.push("RNA".to_string());

        let mut list_feature_types_html = Vec::new();
        for t in &list_feature_types {
            list_feature_types_html.push(html! {
                <option value={t.clone()}>
                    {t}
                </option>

            });
        }

        //Keep track of currently selected count table. It is not populated at first, so need to grab a value once the data becomes available
        if self.last_counttype_select=="" && !list_feature_types.is_empty() {
            let first_entry = list_feature_types.get(0).unwrap();
            ctx.link().send_message(MsgFeature::SetLastCountName(first_entry.clone()));         
        }

        //Callback for keypresses on the feature search input
        let input_onkeyup = ctx.link().callback(move |e: KeyboardEvent | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlInputElement = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).expect("wrong type");
            let cur_value = input.value();

            e.prevent_default();
            let is_enter = e.key() == "Enter" || e.key_code() == 13;
            if is_enter {
                log::debug!("got input");
                input.set_value(""); //empty it
            }
            MsgFeature::FeatureSearchChange(cur_value, is_enter)
        });

        //Compose the view
        html! {
            <div class="biscvi-dimred-rightdiv">
                <div>
                    <div> //  class="bp5-input-group bp5-fill bp5-popover-target bp5-popover-open"
                        <select>
                            {list_feature_types_html}
                        </select>
                        <span> //  aria-hidden="true" tabindex="-1" class="bp5-icon bp5-icon-search"
                            <input type="text" autocomplete="off" placeholder="Search feature" aria-autocomplete="list" value="" onkeyup={input_onkeyup}/> // aria-controls="listbox-7"  class="bp5-input" aria-haspopup="listbox" role="combobox"   ref={input_node_ref} 
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

        let histo_svg = self.make_histogram();


        //Callback to color by this column
        //let meta_name_copy = meta_name.clone();
        let combo_feature_copy= combo_feature.clone();// = PerCellDataSource::Counts(count_name.clone(), feature_name.clone());
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
    fn make_histogram(&self) -> VNode {
        html! {
            <svg width="100%" height="15" style="display: block;">  // id="histogram_XBP1_svg" 
                <g class="histogram-container" transform="translate(0,0)">
                    <g>
                        <rect x="1" y="0" width="4.25" height="15" style="fill: rgb(175, 240, 91);"></rect>
                        <rect x="5.25" y="14.99482731179373" width="4.25" height="0.0051726882062705926" style="fill: rgb(163, 242, 88);"></rect>
                        <rect x="9.5" y="14.970157568040745" width="4.250000000000002" height="0.029842431959254512" style="fill: rgb(151, 243, 87);"></rect>
                        <rect x="13.750000000000002" y="14.94906891612287" width="4.249999999999998" height="0.050931083877129524" style="fill: rgb(139, 244, 87);"></rect>
                        <rect x="18" y="14.859143721152316" width="4.25" height="0.14085627884768392" style="fill: rgb(127, 246, 88);"></rect>
                        <rect x="22.25" y="14.726643323253223" width="4.2500000000000036" height="0.27335667674677744" style="fill: rgb(115, 246, 90);"></rect>
                        <rect x="26.500000000000004" y="14.53445806143562" width="4.25" height="0.46554193856438" style="fill: rgb(103, 247, 94);"></rect>
                        <rect x="30.750000000000004" y="14.448909756485754" width="4.2499999999999964" height="0.5510902435142455" style="fill: rgb(93, 246, 98);"></rect>
                        <rect x="35" y="14.385643800732135" width="4.250000000000007" height="0.6143561992678652" style="fill: rgb(82, 246, 103);"></rect>
                        <rect x="39.25000000000001" y="14.434585389145314" width="4.249999999999993" height="0.5654146108546865" style="fill: rgb(73, 245, 109);"></rect>
                        <rect x="43.5" y="14.47357950023874" width="4.250000000000007" height="0.5264204997612598" style="fill: rgb(64, 243, 115);"></rect>
                        <rect x="47.75000000000001" y="14.586582842591119" width="4.25" height="0.4134171574088814" style="fill: rgb(56, 241, 123);"></rect>
                        <rect x="52.00000000000001" y="14.632739137354767" width="4.249999999999993" height="0.3672608626452334" style="fill: rgb(48, 239, 130);"></rect>
                        <rect x="56.25" y="14.673722743912144" width="4.250000000000007" height="0.32627725608785596" style="fill: rgb(42, 235, 138);"></rect>
                        <rect x="60.50000000000001" y="14.759271048862008" width="4.249999999999993" height="0.2407289511379922" style="fill: rgb(37, 232, 146);"></rect>
                        <rect x="64.75" y="14.77041222346013" width="4.25" height="0.22958777653986928" style="fill: rgb(33, 227, 155);"></rect>
                        <rect x="69" y="14.783940792614993" width="4.250000000000014" height="0.2160592073850065" style="fill: rgb(29, 223, 163);"></rect>
                        <rect x="73.25000000000001" y="14.82651599554353" width="4.25" height="0.17348400445646917" style="fill: rgb(27, 217, 171);"></rect>
                        <rect x="77.50000000000001" y="14.814181123667039" width="4.249999999999986" height="0.18581887633296112" style="fill: rgb(26, 212, 179);"></rect>
                        <rect x="81.75" y="14.845615151997453" width="4.25" height="0.1543848480025467" style="fill: rgb(25, 206, 187);"></rect>
                        <rect x="86" y="14.83208658284259" width="4.25" height="0.16791341715740948" style="fill: rgb(26, 199, 194);"></rect>
                        <rect x="90.25" y="14.843227757440713" width="4.250000000000014" height="0.15677224255928657" style="fill: rgb(27, 193, 201);"></rect>
                        <rect x="94.50000000000001" y="14.850787840203724" width="4.25" height="0.1492121597962761" style="fill: rgb(29, 186, 206);"></rect>
                        <rect x="98.75000000000001" y="14.853573133853255" width="4.25" height="0.14642686614674538" style="fill: rgb(32, 178, 212);"></rect>
                        <rect x="103.00000000000001" y="14.856358427502785" width="4.249999999999986" height="0.14364157249721465" style="fill: rgb(35, 171, 216);"></rect>
                        <rect x="107.25" y="14.8798344739774" width="4.25" height="0.12016552602259978" style="fill: rgb(39, 163, 220);"></rect>
                        <rect x="111.5" y="14.893363043132261" width="4.250000000000014" height="0.10663695686773877" style="fill: rgb(44, 156, 223);"></rect>
                        <rect x="115.75000000000001" y="14.912860098678975" width="4.25" height="0.08713990132102545" style="fill: rgb(49, 148, 224);"></rect>
                        <rect x="120.00000000000001" y="14.927184466019417" width="4.25" height="0.07281553398058271" style="fill: rgb(54, 140, 225);"></rect>
                        <rect x="124.25000000000001" y="14.94071303517428" width="4.249999999999986" height="0.059286964825719934" style="fill: rgb(60, 132, 225);"></rect>
                        <rect x="128.5" y="14.961005888906573" width="4.25" height="0.03899411109342665" style="fill: rgb(65, 125, 224);"></rect>
                        <rect x="132.75" y="14.965780678020055" width="4.25" height="0.03421932197994515" style="fill: rgb(71, 118, 222);"></rect>
                        <rect x="137" y="14.983288238102817" width="4.25" height="0.016711761897182598" style="fill: rgb(76, 110, 219);"></rect>
                        <rect x="141.25" y="14.989256724494668" width="4.250000000000028" height="0.01074327550533205" style="fill: rgb(82, 104, 216);"></rect>
                        <rect x="145.50000000000003" y="14.99403151360815" width="4.249999999999972" height="0.005968486391850547" style="fill: rgb(87, 97, 211);"></rect>
                        <rect x="149.75" y="14.993633614515359" width="4.250000000000028" height="0.0063663854846414125" style="fill: rgb(92, 90, 206);"></rect>
                        <rect x="154.00000000000003" y="14.99761260544326" width="4.249999999999972" height="0.0023873945567398636" style="fill: rgb(96, 84, 200);"></rect>
                        <rect x="158.25" y="14.99840840362884" width="4.25" height="0.001591596371159909" style="fill: rgb(100, 79, 193);"></rect>
                        <rect x="162.5" y="14.99960210090721" width="4.250000000000028" height="0.00039789909279086544" style="fill: rgb(104, 73, 186);"></rect>
                        <rect x="166.75000000000003" y="14.99920420181442" width="4.249999999999972" height="0.0007957981855799545" style="fill: rgb(107, 68, 178);"></rect>
                    </g>
                </g>
            </svg>
        }
    }
}
