use crate::core_model::*;

use yew::prelude::*;

impl Model {

    ////////////////////////////////////////////////////////////
    /// Render files pane
    pub fn view_files_page(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    {"File viewer here"}
                </div>
                <div class="biscvi-dimred-leftdiv">
                    <div>
                        {"File list"}
                    </div>
                </div>
            </div>
        }
    }



}
