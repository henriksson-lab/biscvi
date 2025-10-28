use std::collections::BTreeMap;
use std::path::{Path};

use crate::countfile::{index_countfile, CountFile};


////////////////////////////////////////////////////////////
/// 
pub struct ListFiles { //TODO
    pub files: BTreeMap<String, ShardSet>,

}

////////////////////////////////////////////////////////////
/// 
pub struct ShardSet {  //TODO

    pub lst: Vec<String>,
    pub list_cells: Vec<String>, //which file is which? sqlite database?
    
}

////////////////////////////////////////////////////////////
/// 
pub struct BascetZipFile { //TODO


}

////////////////////////////////////////////////////////////
/// 
pub struct BascetDir { //TODO
    pub counts: CountFile
}


////////////////////////////////////////////////////////////
/// Go through dir, index all files
pub fn index_bascet_dir(bascet_dir: &Path) -> anyhow::Result<BascetDir> {

    let cf = index_countfile(&"/home/mahogny/github/rbiscvi/counts.biscvi5".into())?;

    let paths = std::fs::read_dir(bascet_dir).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    Ok(BascetDir {
        counts: cf
    })
}