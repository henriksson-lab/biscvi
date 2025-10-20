use serde::{Deserialize, Serialize};




/*

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct StrainRequest {
    pub list: Vec<String>
}
 */





////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ReductionRequest {
    pub reduction_name: String,
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReductionRequestResponse {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}




////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterRequest {
    pub counts_name: String,
    pub row: u32,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterRequestResponse {
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
    Categorical(Vec<u32>, Vec<String>), //u32 is a lot
}
impl CountFileMetaColumnData {
}


////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataColumnRequestResponse {
    pub data: CountFileMetaColumnData
}


