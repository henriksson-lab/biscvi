use std::{collections::HashMap, path::PathBuf};
use hdf5::{types::FixedAscii, Dataset, File, H5Type, Result};
//use ndarray::{arr2, s};

use my_web_app::countfile_struct::CountFileMat;
use my_web_app::countfile_struct::CountFileMetaColumnDesc;
use my_web_app::countfile_struct::CountFileRed;
use my_web_app::ClusterResponse;
use my_web_app::CountFileMetaColumnData;
use my_web_app::DatasetDescResponse;
use my_web_app::MetadataColumnResponse;
use my_web_app::ReductionResponse;

use ndarray::Axis;
use serde::Deserialize;
use serde::Serialize;


use anyhow::Context;






////////////////////////////////////////////////////////////
/// 
pub struct CountFile {
    pub file: File,
    pub matrices: HashMap<String, CountFileMat>,
    pub reductions: HashMap<String, CountFileRed>,    
    pub meta: HashMap<String, CountFileMetaColumnDesc>,
}
impl CountFile {


    ////////////////////////////////////////////////////////////
    /// 
    pub fn get_counts_for_cell(&self, count_name: &String, row: u32) -> anyhow::Result<ClusterResponse> {
        //H5T_IEEE_F64LE

        let group_counts = self.file.group("/counts")?; 
        let group_cnt = group_counts.group(&count_name)?;

        let cnt = self.matrices.get(count_name.into()).context("0")?;//.ok_or("Could not get matrix")?; // .expect("could not get matrix");  // D

        let row_start = *cnt.list_indptr.get(row as usize).context("1")? as usize;
        let row_end = *cnt.list_indptr.get(1 + row as usize).context("2")? as usize;

        let df_data = group_cnt.dataset("data")?;
        let df_indices = group_cnt.dataset("indices")?;


        
        //Get column indices
        let ret_indices = df_indices.read_slice_1d::<u32, _>(
            row_start..row_end
        )?.iter().map(|x| *x).collect::<Vec<_>>();        

        //Get values at given columns
        let ret_data = df_data.read_slice_1d::<f32, _>(
            row_start..row_end
        )?.iter().map(|x| *x).collect::<Vec<_>>();

        let v = ClusterResponse {
            indices: ret_indices,
            data: ret_data
        };

        Ok(v)

    }



    ////////////////////////////////////////////////////////////
    /// 
    pub fn get_reduction(&self, reduction_name: &String) -> anyhow::Result<ReductionResponse> {

        let group_counts = self.file.group("/reductions")?; 
        let df_thisred = group_counts.dataset(&reduction_name)?;

        //Check that it is there. Need no further info right now
        let _red = self.reductions.get(reduction_name.into()).context("0")?;

        let my_array = df_thisred.read_2d::<f32>()?;

        //println!("got {:?}",my_array);

        let subarray_x = my_array.select(Axis(0), &[0]);
        let subarray_y = my_array.select(Axis(0), &[1]);

        let x=subarray_x.iter().map(|x| *x).collect::<Vec<_>>();
        let y=subarray_y.iter().map(|x| *x).collect::<Vec<_>>();

        //println!("got {:?}",x);

        let out = ReductionResponse {
            x,y
        };
        Ok(out)

    }



    ////////////////////////////////////////////////////////////
    /// 
    pub fn get_metacolumn(&self, column_name: &String) -> anyhow::Result<MetadataColumnResponse> {

        let group_meta = self.file.group("/obs")?; 

        //Check that it is there. Need no further info right now
        let red = self.meta.get(column_name.into()).context("0")?;

        match red {
            CountFileMetaColumnDesc::Numeric() => {

                let df_thiscol = group_meta.dataset(&column_name)?;
                let data = read_hdf5_f32vec(&df_thiscol)?;                

                let out = MetadataColumnResponse {
                    data: CountFileMetaColumnData::Numeric(data)
                };
                Ok(out)                
            },
            CountFileMetaColumnDesc::Categorical(cats) => {

                let group_thiscol = group_meta.group(&column_name)?;

                let df_thiscol = group_thiscol.dataset("codes")?;
                let data = read_hdf5_intvec(&df_thiscol)?;                

                let out = MetadataColumnResponse {
                    data: CountFileMetaColumnData::Categorical(data, cats.clone())
                };
                Ok(out)                
            }
        }

    }    


    ////////////////////////////////////////////////////////////
    /// 
    pub fn get_desc(&self) -> anyhow::Result<DatasetDescResponse> {
        Ok(DatasetDescResponse {
            matrices: self.matrices.clone(),
            reductions: self.reductions.clone(),
            meta: self.meta.clone(),
        })
    }    


}






////////////////////////////////////////////////////////////
/// 
pub fn prepare_countfile(p: &PathBuf) -> anyhow::Result<CountFile> {

    println!("======== parsing count file ========");
    
    let file = hdf5::File::open(p)?; 

    /////// Gather all count matrices
    let group_counts = file.group("/counts")?; 
    let mut map_matrices: HashMap<String, CountFileMat> = HashMap::new();
    let count_names = group_counts.member_names()?;
    for count_name in count_names {
        let cnt = group_counts.group(&count_name)?;

        println!("Indixing count matrix: {}", count_name);

        // Data in this datset:
        // ./data -- read on demand
        // ./indices -- read on demand
        // ./indptr -- read right away, as it speeds up lookups later
        // ./feature_names -- read right away, to enable searching for user

        let list_indptr = read_hdf5_intvec(&cnt.dataset("indptr")?)?;
        let list_feature_names = read_hdf5_stringvec(&cnt.dataset("feature_names")?)?;

        let c = CountFileMat {
            list_feature_names,
            list_indptr
        };
        map_matrices.insert(count_name.clone(), c);
    }

    /////// Gather all reductions
    let group_reds = file.group("/reductions")?; 
    let mut map_reductions: HashMap<String, CountFileRed> = HashMap::new();
    let red_names = group_reds.member_names()?;

    for red_name in red_names {
        let ds_thisred = group_reds.dataset(&red_name)?;


        let shape = ds_thisred.shape();
        println!("indexing reduction {} with dim {:?}",red_name, shape);

        let num_sample = *shape.get(0).expect("Failed to get num samples for red");
        let num_dim = *shape.get(1).expect("Failed to get num dimensions for red");

        let c = CountFileRed {
            num_sample: num_sample,
            num_dim: num_dim,
        };
        map_reductions.insert(red_name.clone(), c);

    }


    /////// Gather all metadata
    let group_meta = file.group("/obs")?; 
    let mut map_meta: HashMap<String, CountFileMetaColumnDesc> = HashMap::new();
    let meta_names = group_meta.member_names()?;
    println!("Indexing Metadata columns {:?}", meta_names);
    for meta_name in meta_names {
//        let group_thismeta = group_meta.group(&meta_name)?;
        let ds_thismeta = group_meta.dataset(&meta_name);

        let desc = if let Ok(_ds_thismeta) = ds_thismeta {
            CountFileMetaColumnDesc::Numeric()
        } else {
            println!("{}",meta_name);
            let group_thismeta = group_meta.group(&meta_name)?;
            let ds_categories = group_thismeta.dataset("categories")?;
            let categories = read_hdf5_stringvec(&ds_categories)?;  //  FixedAscii(19)
            CountFileMetaColumnDesc::Categorical(categories)
        };

        println!("Meta column {} --- {:?}", meta_name, desc);
        map_meta.insert(meta_name.clone(), desc);

    }


    println!("======== parsing count file DONE ========");

    Ok(CountFile {
        file: file,
        matrices: map_matrices,
        reductions: map_reductions,
        meta: map_meta,
    })
}



////////////////////////////////////////////////////////////
/// 
pub fn read_hdf5_stringvec(ds: &hdf5::Dataset) -> anyhow::Result<Vec<String>>{

    //let t = ds.dtype()?;
    //println!("{:?}", t.to_descriptor()?);
    //println!("{:?}", ds.shape());

    let v = ds.
        read_1d::<hdf5::types::VarLenAscii>()?;
    let v = v.
        iter().collect::<Vec<_>>();
    let out = v.iter().map(|s| s.to_string().clone()).collect::<Vec<_>>();
    Ok(out)
}



////////////////////////////////////////////////////////////
/// 
pub fn read_hdf5_intvec(ds: &hdf5::Dataset) -> anyhow::Result<Vec<u32>>{  // H5T_STD_I32LE

    let v = ds.read_1d::<u32>()?;
    let out = v.iter().map(|x| *x).collect::<Vec<_>>();

    //let out = v.iter().map(|s| s.to_string().clone()).collect::<Vec<_>>();

    Ok(out)
}



////////////////////////////////////////////////////////////
/// 
pub fn read_hdf5_f32vec(ds: &hdf5::Dataset) -> anyhow::Result<Vec<f32>>{  

    let v = ds.read_1d::<f32>()?;
    let out = v.iter().map(|x| *x).collect::<Vec<_>>();
    Ok(out)
}