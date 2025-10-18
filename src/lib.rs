use serde::{Deserialize, Serialize};






#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct StrainRequest {
    pub list: Vec<String>
}





