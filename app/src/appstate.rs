use my_web_app::DatasetDescResponse;




pub struct BiscviData {

    pub current_datadesc: Option<DatasetDescResponse>,



}
impl BiscviData {


    pub fn new() -> BiscviData {
        BiscviData {
            current_datadesc: None
        }
    }

}