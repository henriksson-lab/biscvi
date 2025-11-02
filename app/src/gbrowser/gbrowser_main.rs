
use std::sync::Mutex;

use my_web_app::DatasetDescResponse;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{Callback, Component, Context, Event, Html, KeyboardEvent, MouseEvent, NodeRef, html};
use yew::Properties;

use crate::appstate::{AsyncData};
use crate::core_model::MsgCore;
use crate::gbrowser::{ClientGBrowseData, GBrowserCamera};
use crate::resize::ComponentSize;


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgGBrowse {
    //SetColorBy(String),
    //ToggleExpand(String)
    Zoom(f32),
    Propagate(MsgCore),

    SetRangeFrom(String, bool),
    SetRangeTo(String, bool),

}


////////////////////////////////////////////////////////////
/// Properties for MetadataView
#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_datadesc: AsyncData<DatasetDescResponse>,
    pub current_gff: AsyncData<Mutex<ClientGBrowseData>>,

    pub last_component_size: ComponentSize,
    
    pub on_propagate: Callback<MsgCore>,
}


////////////////////////////////////////////////////////////
/// This component shows a list of all metadata columns
pub struct GBrowseView {
    pub node_ref: NodeRef,

    pub camera: GBrowserCamera,

}

impl Component for GBrowseView {
    type Message = MsgGBrowse;
    type Properties = Props;

    ////////////////////////////////////////////////////////////
    /// Create this component
    fn create(_ctx: &Context<Self>) -> Self {    

        let camera = GBrowserCamera {
            from: 0,
            to: 1000000,
            chr: "1".into()
        };

        Self {
            node_ref: NodeRef::default(),
            camera,
        }
    }

    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {            

            ////////////////////////////////////////////////////////////
            // Message: Propagate message to component above
            MsgGBrowse::Propagate(msg) => {
                ctx.props().on_propagate.emit(msg);
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Zoom image around middle point
            MsgGBrowse::Zoom(scale) => {
                self.camera.zoom(scale);
                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Set "to" position
            MsgGBrowse::SetRangeTo(value, is_enter) => {
                if is_enter {
                    let value = value.parse::<i64>();
                    if let Ok(value) = value {
                        if value > self.camera.from {
                            self.camera.to = value;
                        }
                    }
                    true
                } else {
                    false
                }
            },

            ////////////////////////////////////////////////////////////
            // Message: Set "from" position
            MsgGBrowse::SetRangeFrom(value, is_enter) => {
                if is_enter {
                    let value = value.parse::<i64>();
                    if let Ok(value) = value {
                        if value < self.camera.to {
                            self.camera.from = value;
                        }
                    }
                    true
                } else {
                    false
                }
            },

        }
    }




    ////////////////////////////////////////////////////////////
    /// Render the left pane of metadata entries
    fn view(&self, ctx: &Context<Self>) -> Html {


        //Callbacks for zooming
        let cb_zoom_in_10x = ctx.link().callback(move |_e: MouseEvent | { MsgGBrowse::Zoom(10.0) });
        let cb_zoom_in_3x = ctx.link().callback(move |_e: MouseEvent | { MsgGBrowse::Zoom(3.0) });

        let cb_zoom_out_10x = ctx.link().callback(move |_e: MouseEvent | { MsgGBrowse::Zoom(1.0/10.0) });
        let cb_zoom_out_3x = ctx.link().callback(move |_e: MouseEvent | { MsgGBrowse::Zoom(1.0/3.0) });

        //Callback for keypresses in "from" input
        let input_from_onkeyup = ctx.link().callback(move |e: KeyboardEvent | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlInputElement = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).expect("wrong type");
            let cur_value = input.value();

            e.prevent_default();
            let is_enter = e.key() == "Enter" || e.key_code() == 13;
            MsgGBrowse::SetRangeFrom(cur_value, is_enter)
        });

        //Callback for keypresses in "to" input
        let input_to_onkeyup = ctx.link().callback(move |e: KeyboardEvent | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlInputElement = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).expect("wrong type");
            let cur_value = input.value();

            e.prevent_default();
            let is_enter = e.key() == "Enter" || e.key_code() == 13;
            MsgGBrowse::SetRangeTo(cur_value, is_enter)
        });


        let mut list_chr_html = Vec::new();

        let main_area = if let AsyncData::Loaded(current_gff) = &ctx.props().current_gff {
            let current_gff = current_gff.lock().unwrap();

            //Get list of chromosomes
            for chr in current_gff.desc.chrom_sizes.keys() {

                list_chr_html.push(html! {
                    <option>{chr.to_string()}</option>
                });

            }                

            //Figure out size of working area
            let gbrowse_width = ctx.props().last_component_size.width as f32;
            let gbrowse_height = 800.0 as f32; //ctx.props().last_component_size.height * 0.8;

            //log::debug!("gbrowse size {} {}", gbrowse_width, gbrowse_height);

            //Create vertical guide lines
            let mut list_vertlines = Vec::new();
            for i in 1..10 {
                let cur_x = i*50;
                list_vertlines.push(
                    html! {
                        <line x1={cur_x.to_string()} y1="0" x2={cur_x.to_string()} y2="1000" stroke="gray"></line>
                    }
                );
            }                

            //Figure out what data we need
            

            let mut list_comp = Vec::new(); // separate transcripts vs others

            //log::debug!("---features---");

            for (_chunk_id, chunk_data) in &current_gff.chunks {

                if let AsyncData::Loaded(chunk_data) = chunk_data {

                    for rec in &chunk_data.records {

                        // five_prime_utr
                        // CDS
                        /*
                        
                        CDS
exon
five_prime_utr
gene
start_codon
stop_codon
three_prime_utr
transcript

                         */

                        if rec.ty=="transcript" {
                            let pos_start = self.camera.world2cam(rec.start as i64, gbrowse_width);
                            let pos_end = self.camera.world2cam(rec.end as i64, gbrowse_width);
                            let width = pos_end - pos_start;

                            //log::debug!("feature {} {}", pos_start, pos_end);

                            list_comp.push(html!{
                                <rect x={pos_start.to_string()} y="50" width={width.to_string()} height="70"/>
                            });
                        }
                    }
                } else {
                    //Draw a gray area here
                }
            }


            //Draw main area
            html! {
                <div style="border-color: #92a8d1; width: 100%; height: 70%; ">
                    <svg viewBox={format!("0 0 {} {}", gbrowse_width, gbrowse_height)}>  // seems ignored  "0 0 1000 1000"    height="100%" width={screen_width.to_string()} 

                        {list_vertlines}

                        <line x1="0" y1="100" x2="1000" y2="100" stroke="black"/>

                        {list_comp}
                    </svg>
                </div>
            }
        } else {
            html! {
                <div></div>
            }
        };




        //Render main view
        html! {
            <div class="biscvi-gbrowse-maindiv">

                <div style="display: flex;  justify-content: center;  align-items: center;">  // width:100%;
                    {"Chr:"}
                    <select>
                        {list_chr_html}
                    </select>

                    <div style="width: 10px;"/>

                    {"From:"}
                    <input type="text" value={self.camera.from.to_string()} onkeyup={input_from_onkeyup}/>

                    {"To:"}
                    <input type="text" value={self.camera.to.to_string()} onkeyup={input_to_onkeyup}/>

                    <div style="width: 10px;"/>

                    {"Move:"}
                    <button>{"<<"}</button>
                    <button>{">>"}</button>

                    <div style="width: 10px;"/>

                    {"Zoom in:"}
                    <button onclick={cb_zoom_in_3x}>{"3x"}</button>
                    <button onclick={cb_zoom_in_10x}>{"10x"}</button>

                    <div style="width: 10px;"/>

                    {"Zoom out:"}
                    <button onclick={cb_zoom_out_3x}>{"3x"}</button>
                    <button onclick={cb_zoom_out_10x}>{"10x"}</button>

                    <div style="width: 10px;"/>

                    {"Search:"}
                    <input type="text" />
                </div>
                {main_area}
            </div>       
         }
    }



    ////////////////////////////////////////////////////////////
    /// Called after DOM has been generated
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    }
}




