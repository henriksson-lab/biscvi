use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// Render about pane
    pub fn view_about_page(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="biscvi-about-maindiv">
                    <h1>
                        {"About"}
                    </h1>
                    <p>
                        {"Biscvi (Bacterial Integrated Single-Cell VIewer) is maintained by HenLab and Carroll lab"}
                    </p>
                </div>
            </div>
        }
    }



}
