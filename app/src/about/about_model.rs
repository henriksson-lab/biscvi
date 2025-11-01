use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// Render about pane
    pub fn view_about_page(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    {"About"}
                </div>
            </div>
        }
    }



}
