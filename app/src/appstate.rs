use std::{collections::{BTreeMap, HashMap}, sync::Arc};
use my_web_app::CountFileMetaColumnData;

use std::fmt;

use crate::redview::ReductionViewData;


//TODO: Possibility of a struct, mapping int <-> cell. can share this


////////////////////////////////////////////////////////////
/// Data loaded into Biscvi. This is effectively a cache of 
/// previously loaded data.
pub struct BiscviCache<T> {
    pub data: Arc<T>
}
impl<T> BiscviCache<T> {

    ////////////////////////////////////////////////////////////
    /// Contructor
    pub fn new(d: T) -> BiscviCache<T> {
        BiscviCache {
            data: Arc::new(d)
        }
    }

}

////////////////////////////////////////////////////////////
/// For yew - AsyncData is "equal" if pointers are the same. Otherwise assume the data changed.
/// This speeds up comparison
impl<T> PartialEq for BiscviCache<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

////////////////////////////////////////////////////////////
/// Ensure cloning just clones the Arc;
/// derive(Clone) adds overly restrictive requirements on T
impl<T> Clone for BiscviCache<T> {
    fn clone(&self) -> Self {
        BiscviCache {
            data: Arc::clone(&self.data)
        }        
    }
}














////////////////////////////////////////////////////////////
/// Data loaded into Biscvi. This is effectively a cache of 
/// previously loaded data.
pub struct ReductionData {
    pub reductions: BTreeMap<String, AsyncData<ReductionViewData>>,  //converted from ReductionResponse
}
impl ReductionData {

    ////////////////////////////////////////////////////////////
    /// Contructor
    pub fn new() -> ReductionData {
        ReductionData {
            reductions: BTreeMap::new(),
        }
    }

    ////////////////////////////////////////////////////////////
    /// Get a reduction, or empty if nothing if data is missing
    pub fn get(&self, k: &String) -> AsyncData<ReductionViewData> {
        let v = self.reductions.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }    

    ////////////////////////////////////////////////////////////
    /// Insert new entry, return new datas structure
    pub fn insert(&self, k: &String, value: AsyncData<ReductionViewData>) -> ReductionData {
        let mut newself =  ReductionData {
            reductions: self.reductions.clone()
        };
        newself.reductions.insert(k.clone(), value);
        newself
    }

}








////////////////////////////////////////////////////////////
/// Data loaded into Biscvi. This is effectively a cache of 
/// previously loaded data.
pub struct MetadataData {
    pub metadatas: HashMap<PerCellDataSource, AsyncData<CountFileMetaColumnData>>,
}
impl MetadataData {
    ////////////////////////////////////////////////////////////
    /// Contructor of initial Biscvi state
    pub fn new() -> MetadataData {
        MetadataData {
            metadatas: HashMap::new(),
        }
    }

    ////////////////////////////////////////////////////////////
    /// Get metadata or feature counts for a given cell
    pub fn get(&self, k: &PerCellDataSource) -> AsyncData<CountFileMetaColumnData> {
        let v = self.metadatas.get(k);
        if let Some(v) = v {
            v.clone()
        } else {
            AsyncData::NotLoaded
        }
    }

    ////////////////////////////////////////////////////////////
    /// Insert new entry, return new datas structure
    pub fn insert(&self, k: &PerCellDataSource, value: AsyncData<CountFileMetaColumnData>) -> MetadataData {
        let mut newself =  MetadataData {
            metadatas: self.metadatas.clone()
        };
        newself.metadatas.insert(k.clone(), value);
        newself
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
