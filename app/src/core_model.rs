

use web_sys::window;
use yew::prelude::*;

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
    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            current_page: CurrentPage::Home,
        }
    }




    ////////////////////////////////////////////////////////////
    /// Handle an update message
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::OpenPage(page) => {
                self.current_page = page;
                true
            }

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