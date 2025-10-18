

// Go through dir, index all files

use std::collections::BTreeMap;
use std::path::Path;
use std::fs::read_dir;


pub struct ListFiles {
    pub files: BTreeMap<String, ShardSet>,

}


pub struct ShardSet {

    pub lst: Vec<String>,
    pub list_cells: Vec<String>, //which file is which? sqlite database?
    
}


pub struct BascetZipFile {


}





pub fn index_bascet_dir(bascet_dir: &Path) {



    

    let paths = std::fs::read_dir(bascet_dir).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }



}