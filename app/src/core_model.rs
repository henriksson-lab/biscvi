use std::collections::HashMap;
use std::sync::Mutex;

use my_web_app::FeatureCountsRequest;
use my_web_app::DatasetDescRequest;
use my_web_app::DatasetDescResponse;
use my_web_app::MetadataColumnRequest;
use my_web_app::MetadataColumnResponse;
use my_web_app::ReductionRequest;
use my_web_app::ReductionResponse;

use my_web_app::gbrowser_struct::GBrowserGFFchunkID;
use my_web_app::gbrowser_struct::GBrowserGFFchunkRequest;
use my_web_app::gbrowser_struct::GBrowserGFFchunkResponse;
use my_web_app::gbrowser_struct::GBrowserGFFdescription;
use my_web_app::gbrowser_struct::GBrowserGFFdescriptionRequest;
use web_sys::window;
use yew::prelude::*;

use bytes::Buf;

use crate::appstate::AsyncData;
use crate::appstate::BiscviCache;
use crate::appstate::MetadataData;
use crate::appstate::PerCellDataSource;
use crate::appstate::ReductionData;
use crate::gbrowser::ClientGBrowseData;
use crate::redview::redview_main::convert_from_response_to_reduction_data;
use crate::resize::ComponentSize;
use crate::resize::ComponentSizeObserver;


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
pub enum MsgCore {

    OpenPage(CurrentPage),

    GetDatasetDesc(),
    SetDatasetDesc(DatasetDescResponse),

    GetGffDesc(),
    SetGffDesc(GBrowserGFFdescription),

    GetReduction(String),
    SetReduction(String, ReductionResponse),

    RequestSetColorByMeta(PerCellDataSource),
    SetColorByMeta(PerCellDataSource, Option<MetadataColumnResponse>),

    DataChanged, //Just update using "true"

    WindowResize(ComponentSize),

    RequestGFFchunks(GBrowserGFFchunkRequest),
    SetGFFchunks(GBrowserGFFchunkResponse),

}




////////////////////////////////////////////////////////////
/// State of the page
pub struct Model {
    pub current_page: CurrentPage,
    pub current_reduction: Option<String>,              //should be state of a page; move later
    pub current_datadesc: AsyncData<DatasetDescResponse>,  //For now, makes sense to keep this here, as it is static. but risks becoming really large

    pub current_gff: AsyncData<Mutex<ClientGBrowseData>>,

    // For count tables
    pub reductions: BiscviCache<ReductionData>,        
    pub metadatas: BiscviCache<MetadataData>,          // call something else? countdatas?

    pub current_colorby: PerCellDataSource,
    pub last_component_size: ComponentSize,

}
impl Component for Model {

    type Message = MsgCore;
    type Properties = ();

    ////////////////////////////////////////////////////////////
    /// Create a new component
    fn create(ctx: &Context<Self>) -> Self {

        //Get initial data to show
        ctx.link().send_message(MsgCore::GetDatasetDesc());  //reduction desc?
        ctx.link().send_message(MsgCore::GetGffDesc());

        let query = GBrowserGFFchunkRequest {
            to_get: vec![GBrowserGFFchunkID {
                chr: "1".into(),
                bin: 0,
                track: 0
            }]
        };
        ctx.link().send_message(MsgCore::RequestGFFchunks(query));
 

        Self {
            current_page: CurrentPage::Home,
            current_reduction: None,
            current_datadesc: AsyncData::NotLoaded,
            current_gff: AsyncData::NotLoaded,

            reductions: BiscviCache::new(ReductionData::new()),
            metadatas: BiscviCache::new(MetadataData::new()),
            last_component_size: ComponentSize { width: 100.0, height: 100.0 },
            current_colorby: PerCellDataSource::Metadata("".into()),
        }
    }




    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            ////////////////////////////////////////////////////////////
            // Message: Data changed, redraw
            MsgCore::DataChanged => {
                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Open a given page
            MsgCore::OpenPage(page) => {
                self.current_page = page;
                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Get general dataset description
            MsgCore::GetDatasetDesc() => {
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
                    MsgCore::SetDatasetDesc(res)
                };
                ctx.link().send_future(get_data);
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            MsgCore::SetDatasetDesc(res) => {
                //log::debug!("got desc {:?}",res);
                self.current_datadesc = AsyncData::new(res);
                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Get x description
            MsgCore::GetGffDesc() => {
                let query = GBrowserGFFdescriptionRequest {
                };
                let query_json = serde_json::to_vec(&query).expect("Could not convert to json");
                
                let get_data = async move {
                    let client = reqwest::Client::new();
                    //log::debug!("get coloring");
                    let res = client.post(format!("{}/get_gff_desc",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(query_json) 
                        .send()
                        .await
                        .expect("Failed to send request")
                        .bytes()
                        .await
                        .expect("Could not get binary data");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    MsgCore::SetGffDesc(res)
                };
                ctx.link().send_future(get_data);
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Set xx, sent from server
            MsgCore::SetGffDesc(res) => {
                //log::debug!("got desc {:?}",res);



                self.current_gff = AsyncData::new(Mutex::new(ClientGBrowseData { 
                    desc: res,
                    chunks: HashMap::new()
                }));
                true
            },

            ////////////////////////////////////////////////////////////      ////////////////// call only when data needed?
            // Message: Get a given reduction
            MsgCore::GetReduction(reduction_name) => {

                //Show new reduction
                //log::debug!("GetReduction ask for reduction {:?}",reduction_name);
                self.current_reduction = Some(reduction_name.clone());

                //Insert a loading place holder until data received
                self.reductions = BiscviCache::new(self.reductions.data.insert(&reduction_name, AsyncData::Loading));
                //log::debug!("for now added Loading reduction {:?}",reduction_name);

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
                    MsgCore::SetReduction(reduction_name, res)
                };
                ctx.link().send_future(get_data);

                true //can already show loading status, so true
            },

            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            MsgCore::SetReduction(reduction_name, res) => {
                //log::debug!("set reduction from server {} :: {:?}; this should trigger a refresh??",reduction_name, res);
                //log::debug!("set reduction from server {} ",reduction_name);

                let new_reduction = convert_from_response_to_reduction_data(res);
                self.reductions = BiscviCache::new(self.reductions.data.insert(&reduction_name, AsyncData::new(new_reduction)));

                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            MsgCore::RequestSetColorByMeta(name) => {   //name??

                //log::debug!("RequestSetColorByMeta {} ",name);

                let has_data = self.metadatas.data.metadatas.contains_key(&name);

                //For now, point to show new data. But we might not yet have it
                self.current_colorby = name.clone();
                ctx.link().send_message(MsgCore::SetColorByMeta(name.clone(), None));

                //If needed, request data
                if !has_data {

                    match &name {
                        PerCellDataSource::Metadata(column_name) => {

                            let query: MetadataColumnRequest = MetadataColumnRequest {
                                column_name: column_name.clone(),
                            };
                            let query_json = serde_json::to_vec(&query).expect("Could not convert to json");

                            let name=name.clone();
                            let get_data = async move {
                                let client = reqwest::Client::new();
                                let res = client.post(format!("{}/get_metacolumn",get_host_url())) 
                                    .header("Content-Type", "application/json")
                                    .body(query_json) 
                                    .send()
                                    .await
                                    .expect("Failed to send request")
                                    .bytes()
                                    .await
                                    .expect("Could not get binary data");
                                let res: MetadataColumnResponse  = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");

                                //log::debug!("got MetadataColumnRequest response {:?}",res);

                                MsgCore::SetColorByMeta(name, Some(res))
                            };
                            ctx.link().send_future(get_data);                            

                        },
                        PerCellDataSource::Counts(counts_name, feature_name) => {

                            let query = FeatureCountsRequest {
                                counts_name: counts_name.clone(),
                                feature_name: feature_name.clone(), // 0, // column_name.clone(),   feature_name
                            };
                            let query_json = serde_json::to_vec(&query).expect("Could not convert to json");

                            let name=name.clone();
                            let get_data = async move {
                                let client = reqwest::Client::new();
                                let res = client.post(format!("{}/get_featurecounts",get_host_url()))  /////////////////////////////////
                                    .header("Content-Type", "application/json")
                                    .body(query_json) 
                                    .send()
                                    .await
                                    .expect("Failed to send request")
                                    .bytes()
                                    .await
                                    .expect("Could not get binary data");
                                let res: MetadataColumnResponse  = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");

                                //log::debug!("got FeatureCountsRequest response {:?}",res);

                                MsgCore::SetColorByMeta(name, Some(res))
                            };
                            ctx.link().send_future(get_data);

                        },
                    }

                }                
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            MsgCore::SetColorByMeta(name, res) => {  
                //log::debug!("SetColorByMeta {} {:?}",name, res);
                //Update data if needed
                if let Some(res) = res {
                    self.metadatas = BiscviCache::new(self.metadatas.data.insert(&name, AsyncData::new(res.data)));
                }
                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Window is resized
            MsgCore::WindowResize(size) => {  
                self.last_component_size = size;
                true
            },

            ////////////////////////////////////////////////////////////
            // Message: Get GFF data for genome browser
            MsgCore::RequestGFFchunks(query) => {

                //Insert loading place holders until data received
                if let AsyncData::Loaded(current_gff) = &self.current_gff {
                    let mut current_gff = current_gff.lock().unwrap();
                    current_gff.set_loading(&query);
                }

                //Request data
                let query_json = serde_json::to_vec(&query).expect("Could not convert to json");

                let get_data = async move {
                    let client = reqwest::Client::new();
                    let res = client.post(format!("{}/get_gff_chunks",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(query_json) 
                        .send()
                        .await
                        .expect("Failed to send request")
                        .bytes()
                        .await
                        .expect("Could not get binary data");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    MsgCore::SetGFFchunks(res)
                };
                ctx.link().send_future(get_data);
                true //can already show loading status, so true
            },

            ////////////////////////////////////////////////////////////
            // Message: Set GFF data, sent from server
            MsgCore::SetGFFchunks(res) => {  //may need to know what is empty. or store in message back
                log::debug!("SetGFFchunks");
                if let AsyncData::Loaded(current_gff) = &self.current_gff {
                    let mut current_gff = current_gff.lock().unwrap();
                    current_gff.set_chunks(res);
                }
                true
            },


        }
    }


    ////////////////////////////////////////////////////////////
    /// Top renderer of the page
    fn view(&self, ctx: &Context<Self>) -> Html {


        let current_page = match self.current_page { 
            CurrentPage::Home => self.view_dimred_page(&ctx),
            CurrentPage::GenomeBrowser => self.view_gbrowser_page(&ctx),
            CurrentPage::Files => self.view_files_page(&ctx),
            CurrentPage::About => self.view_about_page(&ctx),
        };

        fn active_if(cond: bool) -> String {
            if cond {
                "btn_top_active".to_string()
            } else {
                "btn_top_inactive".to_string()
            }
        }

        //let window = window().expect("no window");
        // window.addEventListener('resize', resizeCanvas, false);
        // https://yew.rs/docs/next/concepts/html/events

        let onsize = ctx.link().callback(|size: ComponentSize| {
            MsgCore::WindowResize(size)
        });

        html! {
            <div style="position: relative;"> // added style
                <ComponentSizeObserver onsize={onsize} />
                <div class="biscvi-topdiv">
                    <div style="float: left; padding: 10px; font-size: 30px; font-family: 'Roboto', sans-serif; font-weight: 900;">
                        {"Biscvi"}
                    </div>

                    <a class={active_if(self.current_page==CurrentPage::About)}          onclick={ctx.link().callback(|_| MsgCore::OpenPage(CurrentPage::About))}>{"About"}</a> 
                    <a class={active_if(self.current_page==CurrentPage::GenomeBrowser)}  onclick={ctx.link().callback(|_| MsgCore::OpenPage(CurrentPage::GenomeBrowser))}>{"Genome Browser"}</a> 
                    <a class={active_if(self.current_page==CurrentPage::Files)}          onclick={ctx.link().callback(|_| MsgCore::OpenPage(CurrentPage::Files))}>{"Files"}</a> 
                    <a class={active_if(self.current_page==CurrentPage::Home)}           onclick={ctx.link().callback(|_| MsgCore::OpenPage(CurrentPage::Home))}>{"Dimensional Reduction"}</a> 

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


////////////////////////////////////////////////////////////
/// Construct a URL to this website
pub fn get_host_url() -> String {
    let document = window().expect("no window").document().expect("no document on window");
    let location = document.location().expect("no location");
    let protocol = location.protocol().expect("no protocol");
    let host = location.host().expect("no host");

    let url = format!("{}//{}", protocol, host);
    //log::debug!("{}",url);
    url
}