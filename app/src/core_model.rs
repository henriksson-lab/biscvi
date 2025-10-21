

use std::sync::Arc;
use std::sync::Mutex;

use my_web_app::DatasetDescRequest;
use my_web_app::DatasetDescResponse;
use my_web_app::ReductionRequest;
use my_web_app::ReductionResponse;

use web_sys::window;
use yew::prelude::*;

use bytes::Buf;

use crate::appstate::AsyncData;
use crate::appstate::BiscviData;
use crate::component_umap::from_response_to_umap_data;

////////////////////////////////////////////////////////////
/// Which page is currently being shown?
#[derive(Debug,PartialEq)]
pub enum CurrentPage {
    Home,
    Files,
    GenomeBrowser,
    About,
}


////////////////////////////////////////////////////////////
/// Message sent to the event system for updating the page
#[derive(Debug)]
pub enum Msg {

    OpenPage(CurrentPage),

    GetDatasetDesc(),
    SetDatasetDesc(DatasetDescResponse),

    GetReduction(String),
    SetReduction(String, ReductionResponse)

}



////////////////////////////////////////////////////////////
/// State of the page
pub struct Model {
    pub current_page: CurrentPage,
    pub current_reduction: Option<String>,              //should be state of a page; move later
    pub current_datadesc: Option<DatasetDescResponse>,  //For now, makes sense to keep this here, as it is static. but risks becoming really large
    pub current_data: Arc<Mutex<BiscviData>>,           //Has interior mutability. Yew will not be able to sense updates! Need to signal in other ways
}
impl Component for Model {

    type Message = Msg;
    type Properties = ();

    ////////////////////////////////////////////////////////////
    /// Create a new component
    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(Msg::GetDatasetDesc());
        ctx.link().send_message(Msg::GetReduction("kraken_umap".into()));
//        ctx.link().send_message(MsgUMAP::GetReduction());

//        log::debug!("fooo");

        let current_data = Arc::new(Mutex::new(BiscviData::new()));

        Self {
            current_page: CurrentPage::Home,
            current_reduction: None,
            current_datadesc: None,
            current_data: current_data,
        }
    }




    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            ////////////////////////////////////////////////////////////
            // Message: Open a given page
            Msg::OpenPage(page) => {
                self.current_page = page;
                true
            },


            ////////////////////////////////////////////////////////////
            // Message: Get general dataset description
            Msg::GetDatasetDesc() => {
                let query = DatasetDescRequest {
                };
                let query_json = serde_json::to_vec(&query).expect("Could not convert to json");
                
                let get_data = async move {
                    let client = reqwest::Client::new();
                    //log::debug!("get coloring");
                    let res = client.post(format!("{}/get_dataset_desc",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(query_json) 
                        .send()
                        .await
                        .expect("Failed to send request")
                        .bytes()
                        .await
                        .expect("Could not get binary data");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    Msg::SetDatasetDesc(res)
                };
                ctx.link().send_future(get_data);
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            Msg::SetDatasetDesc(res) => {
                //log::debug!("got desc {:?}",res);
                self.current_datadesc = Some(res);
                true
            },


            ////////////////////////////////////////////////////////////      ////////////////// call only when data needed?
            // Message: Get a given reduction
            Msg::GetReduction(reduction_name) => {

                //Show new reduction
                log::debug!("ask for reduction {:?}",reduction_name);
                self.current_reduction = Some(reduction_name.clone());

                //Insert a loading place holder until data received
                let mut current_data = self.current_data.lock().unwrap();
                current_data.reductions.insert(reduction_name.clone(), AsyncData::Loading);
                log::debug!("for now added Loading reduction {:?}",reduction_name);

                //Request data
                let query = ReductionRequest {
                    reduction_name: reduction_name.clone()
                };
                let query_json = serde_json::to_vec(&query).expect("Could not convert to json");

                let get_data = async move {
                    let client = reqwest::Client::new();
                    let res = client.post(format!("{}/get_reduction",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(query_json) 
                        .send()
                        .await
                        .expect("Failed to send request")
                        .bytes()
                        .await
                        .expect("Could not get binary data");
                    //log::debug!("sent reduction request {:?}",res);
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    Msg::SetReduction(reduction_name, res)
                };
                ctx.link().send_future(get_data);

                true //can already show loading status, so true
            },



            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            Msg::SetReduction(reduction_name, res) => {
                //log::debug!("set reduction from server {} :: {:?}; this should trigger a refresh??",reduction_name, res);
                log::debug!("set reduction from server {} ",reduction_name);

                let mut current_data = self.current_data.lock().unwrap();
                let umap_data = from_response_to_umap_data(res);
                
                current_data.reductions.insert(reduction_name, AsyncData::new(umap_data));

                true
            },

        }
    }


    ////////////////////////////////////////////////////////////
    /// Top renderer of the page
    fn view(&self, ctx: &Context<Self>) -> Html {


        let current_page = match self.current_page { 
            CurrentPage::Home => self.view_dimred_page(&ctx),
            CurrentPage::GenomeBrowser => self.view_dimred_page(&ctx),
            CurrentPage::Files => self.view_files_page(&ctx),
            CurrentPage::About => self.view_dimred_page(&ctx),
        };

        fn active_if(cond: bool) -> String {
            if cond {
                "btn_top_active".to_string()
            } else {
                "btn_top_inactive".to_string()
            }
        }


        html! {
            <div>
                <div class="biscvi-topdiv">
                    <div style="float: left; padding: 10px; font-size: 30px; font-family: 'Roboto', sans-serif; font-weight: 900;">
                        {"Biscvi"}
                    </div>

                    <a class={active_if(self.current_page==CurrentPage::About)}          onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::About))}>{"About"}</a> 
                    <a class={active_if(self.current_page==CurrentPage::GenomeBrowser)}  onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::GenomeBrowser))}>{"Genome Browser"}</a> 
                    <a class={active_if(self.current_page==CurrentPage::Files)}          onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::Files))}>{"Files"}</a> 
                    <a class={active_if(self.current_page==CurrentPage::Home)}           onclick={ctx.link().callback(|_| Msg::OpenPage(CurrentPage::Home))}>{"Dimensional Reduction"}</a> 

                </div>
                { current_page }


            </div>
        }
    }







}








////////////////////////////////////////////////////////////
/// Show an alert message
pub fn alert(s: &str) {
    let window = window().expect("no window");
    window.alert_with_message(s).unwrap();
}


pub fn get_host_url() -> String {
    let document = window().expect("no window").document().expect("no document on window");
    let location = document.location().expect("no location");
    let protocol = location.protocol().expect("no protocol");
    let host = location.host().expect("no host");

    let url = format!("{}//{}", protocol, host);
    //log::debug!("{}",url);
    url
}

// https://yew.rs/docs/next/advanced-topics/struct-components/hoc