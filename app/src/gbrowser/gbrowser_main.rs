
use std::collections::HashSet;

use my_web_app::DatasetDescResponse;
use yew::{Callback, Component, Context, Event, Html, MouseEvent, NodeRef, html};
use yew::Properties;

use crate::appstate::{AsyncData};
use crate::gbrowser::GBrowserCamera;
use crate::resize::ComponentSize;


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum MsgGBrowse {
    //SetColorBy(String),
    //ToggleExpand(String)
    Zoom(f32),
}


////////////////////////////////////////////////////////////
/// Properties for MetadataView
#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_datadesc: AsyncData<DatasetDescResponse>,
    pub last_component_size: ComponentSize,
    
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
            to: 10000,
            chr: "1".into()
        };

        Self {
            node_ref: NodeRef::default(),
            camera,
        }
    }

    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {            
            MsgGBrowse::Zoom(scale) => {  // and current cursor pos??
                self.camera.zoom(scale);
                true
            }
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

        //Figure out size of working area
        let gbrowse_width = ctx.props().last_component_size.width;
        let gbrowse_height = 800.0; //ctx.props().last_component_size.height * 0.8;

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


        
        html! {
            <div class="biscvi-gbrowse-maindiv">

                <div style="display: flex;  justify-content: center;  align-items: center;">  // width:100%;
                    {"Chr:"}
                    <select>
                        <option>{"Chr1"}</option>
                    </select>

                    <div style="width: 10px;"/>

                    {"From:"}
                    <input type="text" value={self.camera.from.to_string()}/>

                    {"To:"}
                    <input type="text" value={self.camera.to.to_string()}/>

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



                <div style="border-color: #92a8d1; width: 100%; height: 70%; ">
                    <svg viewBox={format!("0 0 {} {}", gbrowse_width, gbrowse_height)}>  // seems ignored  "0 0 1000 1000"    height="100%" width={screen_width.to_string()} 

                        {list_vertlines}

                        <rect x="120" y="50" width="100" height="100"/>

                        <line x1="0" y1="100" x2="1000" y2="100" stroke="black"/>

                    </svg>
                </div>
            </div>       
         }
    }



    ////////////////////////////////////////////////////////////
    /// Called after DOM has been generated
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    }
}




