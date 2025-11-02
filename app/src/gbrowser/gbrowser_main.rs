
use std::sync::Mutex;

use my_web_app::DatasetDescResponse;
use my_web_app::gbrowser_struct::{GBrowserGFFchunkID, GBrowserGFFchunkRequest};
use wasm_bindgen::JsCast;
use web_sys::{DomRect, EventTarget, HtmlInputElement, HtmlSelectElement, SvgElement};
use yew::{Callback, Component, Context, Event, Html, KeyboardEvent, MouseEvent, NodeRef, WheelEvent, html};
use yew::Properties;

use bstr::BString;

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

    MouseMove(f32,f32, bool),
    MouseWheel(f32),

    SetChromosome(BString),
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

    pub last_pos: (f32,f32),
    pub enable_verlines: bool,
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
            last_pos: (0.0,0.0),
            enable_verlines: true,
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
            // Message: Set chromosome to show
            MsgGBrowse::SetChromosome(chr)  => {
                self.camera.chr=chr;
                true
            }

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

            ////////////////////////////////////////////////////////////
            // Message: Mouse has moved
            MsgGBrowse::MouseMove(x,y, press_left) => {

                let gbrowse_width = get_canvas_width(ctx);

                let last_pos = self.last_pos;
                self.last_pos = (x,y);
//                log::debug!("now at screen pos {:?}", last_pos);
                    let wx_now = self.camera.cam2world(x, gbrowse_width);
                    //log::debug!("now at world pos {:?}", wx_now);

                //Handle panning
                if press_left {

                    //Handle pointer in world coordinates
                    let wx_last = self.camera.cam2world(last_pos.0, gbrowse_width);

                    //let dx = x - last_pos.0;
                    let wdx = wx_last - wx_now;
                    //log::debug!("dd {:?}", (dx,dy));

                    self.camera.to += wdx as i64;
                    self.camera.from += wdx as i64;
                    return true;
                }
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Mouse wheel rotated
            MsgGBrowse::MouseWheel(dy) => {
                let gbrowse_width = get_canvas_width(ctx);

                let (cx,_cy) = self.last_pos;
                let wx = self.camera.cam2world(cx, gbrowse_width);
                  //  log::debug!("zoom now at world pos {:?}", wx);
                let scale = (10.0f32).powf(dy / 1000.0);
                //log::debug!("zoom scale {}",scale);
                self.camera.zoom_around(scale, wx as i64);
                true
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


        let cb_mousemoved = ctx.link().callback(move |e: MouseEvent | { 
            e.prevent_default();
            let (x_cam, y_cam) = mouseevent_get_pos(&e);
            let press_left = e.buttons() & 1 > 0;
            MsgGBrowse::MouseMove(x_cam,y_cam, press_left)
        });
        
        let cb_mousewheel = ctx.link().callback(move |e: WheelEvent | { 
            e.prevent_default();
            MsgGBrowse::MouseWheel(e.delta_y() as f32)
        });
        /*

        let cb_mouseclicked = ctx.link().callback(move |_e: MouseEvent | { 
            MsgReduction::MouseClick
        });
 */
        let cb_set_chr = ctx.link().callback(move |e: Event | { 
            let target: Option<EventTarget> = e.target();
            let input: HtmlSelectElement = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()).expect("wrong type");
            let val = input.value();
            MsgGBrowse::SetChromosome(val.into())
        });
        
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
            let gbrowse_width = get_canvas_width(ctx); 
            let gbrowse_height = 800.0 as f32; //ctx.props().last_component_size.height * 0.8;

            //log::debug!("gbrowse size {} {}", gbrowse_width, gbrowse_height);

            //Create vertical guide lines
            let mut list_vertlines = Vec::new();
            if self.enable_verlines {
                let camera_span = self.camera.to - self.camera.from;

                let closest_power10 = (10.0f32).powf((camera_span as f32).log10().floor()) as i64;
                let grid_size = closest_power10/10;
                //log::debug!("make grid at {}", grid_size);

                let first_x = (self.camera.from / grid_size) * grid_size;
                let last_x = (self.camera.to / grid_size + 1) * grid_size;

                //let steps:Vec<i64> = (first_x..=last_x).step_by(grid_size as usize).collect();
                //log::debug!("make grid steps {:?}", steps);

                for wx in (first_x..=last_x).step_by(grid_size as usize) {
                    let cx = self.camera.world2cam(wx, gbrowse_width);                    
                    list_vertlines.push(
                        html! {
                            <line x1={cx.to_string()} y1="0" x2={cx.to_string()} y2={gbrowse_height.to_string()} stroke="#DDDDDD"></line>
                        }
                    );
                    list_vertlines.push(
                        html! {
                            <text text-anchor="middle" x={cx.to_string()} y="15" fill="red">{wx.to_string()}</text>
                        }
                    );
                }   
            }
             

            //For each track, figure out what chunks we need
            let mut list_get_chunks = Vec::new();
            let clamped_from = clamp0_i64(self.camera.from);
            let clamped_to = clamp0_i64(self.camera.to);
            for (track_id, chunk_size) in current_gff.desc.chunk_sizes.iter().enumerate() {

                let first_chunk = clamped_from / chunk_size;
                let last_chunk = (clamped_to / chunk_size) + 1;

                //let num_chunks = last_chunk-first_chunk;
                //log::debug!("Get chunks: {}",num_chunks);

                for bin in first_chunk..last_chunk {
                    list_get_chunks.push(GBrowserGFFchunkID {
                        chr: self.camera.chr.clone(),
                        track: track_id as u64,
                        bin
                    });
                }
            }
            
            //log::debug!("Get chunks: {:?}",list_get_chunks);


            //Render all chunks
            //log::debug!("---features---");
            let mut list_transcripts = Vec::new(); // separate transcripts vs others
            let mut list_comp = Vec::new(); // separate transcripts vs others
            let mut list_unknown = Vec::new(); // separate transcripts vs others
            let mut list_request = Vec::new();
            for chunk_id in list_get_chunks {

                let chunk_data = current_gff.chunks.get(&chunk_id);
                if let Some(AsyncData::Loaded(chunk_data)) = chunk_data {

                    for rec in &chunk_data.records {
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

                        let pos_start = self.camera.world2cam(rec.start as i64, gbrowse_width);
                        let pos_end = self.camera.world2cam(rec.end as i64, gbrowse_width);
                        let width = pos_end - pos_start;
                            //log::debug!("feature {} {}", pos_start, pos_end);

                        let mid_y = 50;

                        //Perform additional clipping to reduce content to render, if possible
                        if pos_end > 0.0 || pos_start < gbrowse_width {
                            if rec.ty=="transcript" {
                                list_transcripts.push(html!{
                                    <line x1={pos_start.to_string()} y1={mid_y.to_string()} x2={pos_end.to_string()} y2={mid_y.to_string()} stroke="black"/>
                                });
                            }

                            if rec.ty=="exon" {
                                let height = 20;
                                let y_upper = mid_y - height/2;
                                list_comp.push(html!{
                                    <rect x={pos_start.to_string()} y={y_upper.to_string()} width={width.to_string()} height={height.to_string()}/>
                                });
                            }
                        }

                    }
                } else {
                    //Draw a gray area here, indicating data is loading
                    let chunk_size = current_gff.desc.chunk_sizes.get(chunk_id.track as usize).expect("Inconsistent chunk id");
                    let pos_start = chunk_size * chunk_id.bin;
                    let pos_end = chunk_size * (chunk_id.bin+1);
                    let pos_mid = (pos_start + pos_end)/2;
                    let width = pos_end - pos_start;

                    list_unknown.push(html!{
                        <rect x={pos_start.to_string()} y="0" width={width.to_string()} height="300" fill="#EEEEEE" />
                    });
                    list_unknown.push(html!{
                        <text text-anchor="middle" x={pos_mid.to_string()} y="100" fill="red">{"Loading..."}</text>  // class="small"
                    });

                    //log::debug!("Missing {:?}", chunk_id);
                    if let Some(AsyncData::Loading) = chunk_data {
                        //Do nothing if loading
                    } else {
                        list_request.push(chunk_id);
                    }
                }
            }

            //Make a request for missing data
            if !list_request.is_empty() {
                //Grab some data at a time for faster update and to not kill the browser  ----- how to do this best?
                if list_request.len()>100 {
                    let query = GBrowserGFFchunkRequest {
                        to_get: vec![list_request.last().unwrap().clone()]  //list_request[0..1].to_vec()  // start from end to get highest level features first
                    };
                    ctx.props().on_propagate.emit(MsgCore::RequestGFFchunks(query));
                    //NOTE: odd behavior with showing "loading" boxes when using this feature
                } else {
                    let query = GBrowserGFFchunkRequest {
                        to_get: list_request
                    };
                    ctx.props().on_propagate.emit(MsgCore::RequestGFFchunks(query));
                }
            }

            //Draw main area
            html! {
                <div style="border-color: #92a8d1; width: 100%; height: 70%; ">
                    <svg 
                        viewBox={format!("0 0 {} {}", gbrowse_width, gbrowse_height)}

                        onmousemove={cb_mousemoved} 
                        onwheel={cb_mousewheel} 
                        /*
                        onclick={cb_mouseclicked} 
                         */
                        >
                        {list_unknown}
                        {list_vertlines}
                        {list_transcripts}
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
                    <select onchange={cb_set_chr}>
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





////////////////////////////////////////////////////////////
/// Get current camera position from a mouse event
fn mouseevent_get_pos(e: &MouseEvent) -> (f32,f32) {

    // ctx: &Context<GBrowseView>, 

    let target: Option<EventTarget> = e.target();
    let canvas: SvgElement = target.and_then(|t| t.dyn_into::<SvgElement>().ok()).expect("wrong type");

    let rect = canvas.get_bounding_client_rect();
    let x = e.client_x() - (rect.left() as i32);
    let y = e.client_y() - (rect.top() as i32);

//    log::debug!("pos of component {} {}", e.left(), e.top());
//    log::debug!("actual size of component {} {}", rect.width(), rect.height());
/*
    let actual_w = rect.width() as f32;
    let actual_h = rect.height() as f32;

    let think_w = get_canvas_width(ctx);

    let fix_scale_x = think_w/actual_w;

    log::debug!("correct {}",fix_scale_x);
 */

//    let x = x as f32;
//    let y = y as f32;
//(


//)
/*


    let x_cam = (x as f32 - w/2.0)/(w/2.0);
    let y_cam = (y as f32 - h/2.0)/(h/2.0);

//    log::debug!("getcx  {} {}", x_cam, y_cam);

    (x_cam, y_cam)
     */
    (x as f32,y as f32)
}



fn get_canvas_width(ctx: &Context<GBrowseView>) -> f32 {
    let gbrowse_width = ctx.props().last_component_size.width as f32;
    gbrowse_width
}


fn clamp0_i64(pos: i64) -> u64 {
    if pos < 0 {
        0
    } else {
        pos as u64
    }
}