use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// Render genome browser pane
    pub fn view_gbrowser_page(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    {"Genome browser here"}
                </div>
            </div>
        }
    }



}
