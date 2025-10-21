use std::{collections::HashMap, pin::Pin, sync::Arc};

use my_web_app::ReductionResponse;
use yew::html::{ImplicitClone, IntoPropValue};

use crate::component_umap::UmapData;


pub struct BiscviData {

    //pub current_datadesc: Option<DatasetDescResponse>,
//   SetDatasetDesc(DatasetDescResponse),
//    SetReduction(ReductionResponse)

    pub reductions: HashMap<String, AsyncData<UmapData>>  //converted from ReductionResponse

}
impl BiscviData {


    pub fn new() -> BiscviData {
        BiscviData {
            //current_datadesc: None
            reductions: HashMap::new()
        }
    }

    pub fn get_reduction(&self, k: &String) -> AsyncData<UmapData> {
        let v = self.reductions.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }

}


//#[derive(Clone)]
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


//Ensure cloning just clones the Arc; not sure why derive(Clone) does not automatically understand this
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

/*

pub struct EqualPointer<T> {
    data: T
}
impl<T> PartialEq for EqualPointer<T> {
    fn eq(&self, other: &Self) -> bool {
        let ptr_this: *const T = &self.data;
        let ptr_other: *const T = &other.data;
        ptr_this == ptr_other
    }
}

 */




/*
// not possible to construct below

fn get_async_hashmap<K,V>(map: HashMap<K,AsyncData<V>>, key: &K) -> &AsyncData<V> where K: std::cmp::Eq + std::hash::Hash {
    let out = map.get(key);
    if let Some(out) = out {
        out
    } else {
        &AsyncData::Missing
    }
}
    */

