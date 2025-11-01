use std::collections::HashMap;
use serde::{Deserialize, Serialize};


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountFileMat {
    pub list_feature_names: Vec<String>,
    pub list_indptr: Vec<u32>,

    #[serde(skip)]
    pub map_feature_names_pos: HashMap<String, usize>,      
}
impl CountFileMat {

    pub fn build_map(&mut self){
        for (i,v) in self.list_feature_names.iter().enumerate() {
            self.map_feature_names_pos.insert(v.clone(), i);
        }
    }

}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountFileRed {
    pub num_sample: usize,
    pub num_dim: usize,
}





////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CountFileMetaColumnDesc {
    Numeric(),
    Categorical(Vec<String>),
}





////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountFileMeta {
    pub names: Vec<String>,
    pub columns: Vec<CountFileMetaColumnDesc>
}

