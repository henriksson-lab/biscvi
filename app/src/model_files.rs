use crate::core_model::*;

use yew::prelude::*;

impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_files_page(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    {"UMAP here"}
                </div>
                <div class="biscvi-dimred-leftdiv">
                    <div>
                        {"Color by category here"}
                    </div>
                    <div>
                        {"Histogram of category?"}                        
                    </div>
                </div>
                <div class="biscvi-dimred-rightdiv">
                    {"Genes:"}
                    <input type="text"/>
                </div>
            </div>
        }
    }



}
