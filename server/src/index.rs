use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::ConfigFile;
use crate::countfile::{index_countfile, CountFile};
use crate::gbrowser_gff::{FeatureCollection, GBrowserGFFindex, GFFparseSettings};


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
    pub counts: CountFile,
    pub gff_data: Option<(GBrowserGFFindex,PathBuf)>,
}


////////////////////////////////////////////////////////////
/// Go through dir, index all files
pub fn index_bascet_dir(bascet_dir: &Path, config: &ConfigFile) -> anyhow::Result<BascetDir> {

    let path_cf = bascet_dir.join("counts.biscvi5");

    //Prepare count files
    let cf = index_countfile(&path_cf)?;

    let paths = std::fs::read_dir(bascet_dir).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    //Optional: Parse GFF file
    let gff_data = if let Some(gff_path) = &config.gff {
        println!("GFF provided");

        //Index file if needed
        if !FeatureCollection::index_exists(gff_path) {
            println!("No GFF index exists; reading");
            let params = GFFparseSettings {};

            let mut gff = FeatureCollection::make_default_gff();
            FeatureCollection::read_file(&mut gff, gff_path, &params)?;
            println!("Writing gff index...");
            FeatureCollection::write_gff_index(&gff, gff_path)?;
            println!("Done writing gff index");
        }

        //Read index
        let index = FeatureCollection::read_gff_index(gff_path)?;
        //println!("index: {:?}", index);
        Some((index, gff_path.clone()))
    } else {
        None
    };

    Ok(BascetDir {
        counts: cf,
        gff_data,
    })
}