

use my_web_app::ReductionRequest;
use web_sys::window;
use yew::prelude::*;

use my_web_app::ClusterRequestResponse;
use my_web_app::ReductionRequestResponse;
use bytes::Buf;

////////////////////////////////////////////////////////////
/// Which page is currently being shown?
#[derive(Debug)]
#[derive(PartialEq)]
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

    GetReduction(String),
    SetReduction(ReductionRequestResponse)

}



////////////////////////////////////////////////////////////
/// State of the page
pub struct Model {
    pub current_page: CurrentPage,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();


    ////////////////////////////////////////////////////////////
    /// Create a new component
    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(Msg::GetReduction("kraken_umap".into()));

//        ctx.link().send_message(MsgUMAP::GetReduction());






        Self {
            current_page: CurrentPage::Home,
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

            /*
            
            
            Msg::GetReduction(reduction_name) => { ///////////////////////////// all sorts of data

                //TODO need to get a list of reductions, metadata etc

                
                async fn get_data() -> Msg {
                    let client = reqwest::Client::new();
                    //log::debug!("get coloring");
                    let res = client.get(format!("{}/get_reduction",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body("") // no body
                        .send()
                        .await
                        .expect("Failed to send request").bytes().await.expect("Could not get binary data");
                    //log::debug!("got bytes");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    //log::debug!("got deserialized");


                    Msg::SetReduction(res)
                }
                ctx.link().send_future(get_data());
                false
            },

             */
                //TODO need to get a list of reductions, metadata etc






            ////////////////////////////////////////////////////////////
            // Message: Get a given reduction
            Msg::GetReduction(reduction_name) => {

                let query = ReductionRequest {
                    reduction_name: reduction_name
                };
                let query_json = serde_json::to_vec(&query).expect("Could not convert to json");
                
                let get_data = async move {
                    let client = reqwest::Client::new();
                    //log::debug!("get coloring");
                    let res = client.post(format!("{}/get_reduction",get_host_url()))
                        .header("Content-Type", "application/json")
                        .body(query_json) 
                        .send()
                        .await
                        .expect("Failed to send request")
                        .bytes()
                        .await
                        .expect("Could not get binary data");
                    //log::debug!("got bytes");
                    let res = serde_cbor::from_reader(res.reader()).expect("Failed to deserialize");
                    //log::debug!("got deserialized");


                    Msg::SetReduction(res)
                };

                ctx.link().send_future(get_data);
                false
            },

            ////////////////////////////////////////////////////////////
            // Message: Set reduction data, sent from server
            Msg::SetReduction(res) => {
                println!("got reduction {:?}",res);
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