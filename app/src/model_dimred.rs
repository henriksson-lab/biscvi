use crate::core_model::*;

use yew::prelude::*;




impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_dimred_page(&self, _ctx: &Context<Self>) -> Html {

        let mut list_meta:Vec<Html> = Vec::new();

        if let Some(current_datadesc) = &self.current_datadesc {

            for s in current_datadesc.meta.keys() {
                list_meta.push(
                    html! { 
                        <li>
                            { s.clone() }
                        </li> 
                    }
                );
            }

        }


        html! {
            <div>
                <div class="biscvi-dimred-maindiv">
                    {"UMAP here"}            

                    

                </div>
                <div class="biscvi-dimred-leftdiv">
                    <div>
                        <ul>
                            {
                                list_meta
                            }
                        </ul>
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
