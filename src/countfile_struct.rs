use std::collections::HashMap;
use serde::{Deserialize, Serialize};


////////////////////////////////////////////////////////////
/// Pointers into a sparse matrix stored on disk. 
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
/// Size of a reduction
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountFileRed {
    pub num_sample: usize,
    pub num_dim: usize,
}

////////////////////////////////////////////////////////////
/// A description of an array  TODO generalize
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CountFileMetaColumnDesc {
    Numeric(),
    Categorical(Vec<String>),
}

////////////////////////////////////////////////////////////
/// Description of set of metadata, for each cell
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CountFileMeta {
    pub names: Vec<String>,
    pub columns: Vec<CountFileMetaColumnDesc>
}

