use std::{collections::{BTreeMap, HashMap}, sync::Arc};
use my_web_app::CountFileMetaColumnData;

use std::fmt;

use crate::component_umap_main::UmapData;

//TODO: Possibility of a struct, mapping int <-> cell. can share this

////////////////////////////////////////////////////////////
/// Data loaded into Biscvi. This is effectively a cache of 
/// previously loaded data.
pub struct BiscviData {

    pub reductions: BTreeMap<String, AsyncData<UmapData>>,  //converted from ReductionResponse
    pub metadatas: HashMap<PerCellDataSource, AsyncData<CountFileMetaColumnData>>,

}
impl BiscviData {


    ////////////////////////////////////////////////////////////
    /// Contructor of initial Biscvi state
    pub fn new() -> BiscviData {
        BiscviData {
            reductions: BTreeMap::new(),
            metadatas: HashMap::new(),
        }
    }

    ////////////////////////////////////////////////////////////
    /// Get a reduction, or empty if nothing if data is missing
    pub fn get_reduction(&self, k: &String) -> AsyncData<UmapData> {
        let v = self.reductions.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }

    ////////////////////////////////////////////////////////////
    /// Get metadata or feature counts for a given cell
    pub fn get_metadata(&self, k: &PerCellDataSource) -> AsyncData<CountFileMetaColumnData> {
        let v = self.metadatas.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }

}



////////////////////////////////////////////////////////////
/// List of data for each cell, e.g. metadata or feature counts
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PerCellDataSource {
    Metadata(String),       // metadata column
    Counts(String, String), // count table name, feature name
}

impl std::fmt::Display for PerCellDataSource {

    ////////////////////////////////////////////////////////////
    /// Pretty print PerCellDataSource
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PerCellDataSource::Metadata(x) => {
                write!(f, "Metadata({})", x)
            },
            PerCellDataSource::Counts(x,y) => {
                write!(f, "Counts({},{})", x,y)
            },
        }
    }

}



////////////////////////////////////////////////////////////
/// Data that is not loaded, loading, or loaded. Designed for yew;
/// this means that data is considered equal iff it is stored
/// in the same position in memory
#[derive(Debug)]
pub enum AsyncData<T> {
    NotLoaded,
    Loading,
    Loaded(Arc<T>)
}
impl<T> AsyncData<T> {

    ////////////////////////////////////////////////////////////
    /// Wrap data as loaded AsyncData
    pub fn new(data: T) -> AsyncData<T> {
        AsyncData::Loaded(Arc::new(data))
    }
    
}



////////////////////////////////////////////////////////////
/// Ensure cloning just clones the Arc;
/// derive(Clone) adds overly restrictive requirements on T
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



////////////////////////////////////////////////////////////
/// For yew - AsyncData is "equal" if pointers are the same. Otherwise assume the data changed.
/// This speeds up comparison
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
    }

}
