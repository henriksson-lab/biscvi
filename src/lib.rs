use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod countfile_struct;

use countfile_struct::CountFileMat;
use countfile_struct::CountFileMetaColumnDesc;
use countfile_struct::CountFileRed;




////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ReductionRequest {
    pub reduction_name: String,
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReductionResponse {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}




////////////////////////////////////////////////////////////      is this a bad name???
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureCountsRequest {
    pub counts_name: String,
    pub feature_name: String,
}

////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterResponse {
    pub indices: Vec<u32>,
    pub data: Vec<f32>     ////////// nooo! not really! several options?
}







////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataColumnRequest {
    pub column_name: String,
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CountFileMetaColumnData {
    Numeric(Vec<f32>),
    SparseNumeric(Vec<u32>, Vec<f32>), // indices, data
    Categorical(Vec<u32>, Vec<String>), //u32 is a lot
}
impl CountFileMetaColumnData {
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataColumnResponse {
    pub data: CountFileMetaColumnData
}







////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct DatasetDescRequest {
}




////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct DatasetDescResponse {
    pub matrices: HashMap<String, CountFileMat>,
    pub reductions: HashMap<String, CountFileRed>,    
    pub meta: HashMap<String, CountFileMetaColumnDesc>,
}
impl DatasetDescResponse {

    pub fn new() -> DatasetDescResponse {
        DatasetDescResponse {
            matrices: HashMap::new(),
            reductions: HashMap::new(),
            meta: HashMap::new(),
        }
    }

}

