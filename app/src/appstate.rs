use std::{collections::{BTreeMap, HashMap}, sync::Arc};
use my_web_app::CountFileMetaColumnData;

use crate::component_umap_main::UmapData;



pub struct BiscviData {

    pub reductions: BTreeMap<String, AsyncData<UmapData>>,  //converted from ReductionResponse
    pub metadatas: HashMap<String, AsyncData<CountFileMetaColumnData>>,  //type ???

}
impl BiscviData {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn new() -> BiscviData {
        BiscviData {
            reductions: BTreeMap::new(),
            metadatas: HashMap::new(),
        }
    }

    ////////////////////////////////////////////////////////////
    /// x
    pub fn get_reduction(&self, k: &String) -> AsyncData<UmapData> {
        let v = self.reductions.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }

    ////////////////////////////////////////////////////////////
    /// x
    pub fn get_metadata(&self, k: &String) -> AsyncData<CountFileMetaColumnData> {
        let v = self.metadatas.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }


}


////////////////////////////////////////////////////////////
/// x
#[derive(Debug)]
pub enum AsyncData<T> {
    NotLoaded,
    Loading,
    Loaded(Arc<T>)
}
impl<T> AsyncData<T> {

    pub fn new(data: T) -> AsyncData<T> {
        AsyncData::Loaded(Arc::new(data))
    }
    
}

/* 
impl<T> ImplicitClone for AsyncData<T> {
    //All methods provided. Leave empty. This just means that .clone() exists, and is cheap.
    //Not clear that Arc is that cheap to clone though. future work!
}
*/


//Ensure cloning just clones the Arc;
//derive(Clone) adds overly restrictive requirements on T
impl<T> Clone for AsyncData<T> {
    fn clone(&self) -> Self {
        match self {
            AsyncData::Loaded(this) => {
                AsyncData::Loaded(this.clone())
            },
            AsyncData::NotLoaded => {
                AsyncData::NotLoaded
            },
            AsyncData::Loading => {
                AsyncData::Loading
            },
        }        
    }
}



//For yew - 
impl<T> PartialEq for AsyncData<T> {
    fn eq(&self, other: &Self) -> bool {

        match self {
            AsyncData::Loaded(this) => {

                match other {
                    AsyncData::Loaded(other) => Arc::ptr_eq(this,other),
                    _ => false
                }

            },
            AsyncData::NotLoaded => {

                match other {
                    AsyncData::NotLoaded => true,
                    _ => false
                }

            },
            AsyncData::Loading => {

                match other {
                    AsyncData::Loading => true,
                    _ => false
                }

            },
        }


        /* 

        if let AsyncData::Loaded(this) = self {
            if let AsyncData::Loaded(other) = other {
                let ptr_this: *const T = this;
                let ptr_other: *const T = other;
                ptr_this == ptr_other
            } else {
                false
            }
        } else {
            if let AsyncData::Loaded(_other) = other {
                false
            } else {
                true
            }
        }
        */
    }
}
