use serde::{Deserialize, Serialize};


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountFileMat {
    pub list_feature_names: Vec<String>,  // Compact, but would a hashmap be better? or treemap ideally?
    pub list_indptr: Vec<u32>,
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

