use bstr::BString;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::read::GzEncoder;
use my_web_app::gbrowser_struct::GBrowserGFF;
use my_web_app::gbrowser_struct::GBrowserGFFchunkID;
use my_web_app::gbrowser_struct::GBrowserGFFchunkpos;
use my_web_app::gbrowser_struct::GBrowserGFFdescription;
use my_web_app::gbrowser_struct::GBrowserRecordBuf;
use my_web_app::gbrowser_struct::{GBrowserGFFchunkRequest, GBrowserGFFchunkResponse};
use noodles::gff::feature::RecordBuf;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::{io::{SeekFrom}};
use std::path::PathBuf;
use noodles::gtf;
use noodles::gff;

use tokio::{io::{AsyncReadExt, AsyncSeekExt}};

use crate::gbrowser_noodles::convert_record;


////////////////////////////////////////////////////////////
/// 
pub struct GFFparseSettings {
}





////////////////////////////////////////////////////////////
/// Index into binary file
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GBrowserGFFindex {
    pub chunk_sizes: Vec<u64>,
    pub remainder: Vec<GBrowserRecordBuf>,
    pub chrom_sizes: HashMap<BString, u64>,

    #[serde_as(as = "Vec<(_, _)>")]
    pub chunk_coordinates: HashMap<GBrowserGFFchunkID,GBrowserGFFchunkpos>,
}

impl GBrowserGFFindex {

    pub fn get_description(&self) -> GBrowserGFFdescription {
        GBrowserGFFdescription {
            chunk_sizes: self.chunk_sizes.clone(),
            remainder: self.remainder.clone(),
            chrom_sizes: self.chrom_sizes.clone(),
        }
    }
}



////////////////////////////////////////////////////////////
/// 
pub struct FeatureCollection {
}
impl FeatureCollection {


    ////////////////////////////////////////////////////////////
    /// Get path of GFF chunks file
    fn get_path_chunks(path: &PathBuf) -> PathBuf {
        let mut path = path.clone();
       path.set_extension("chunks");
       path
    }

    ////////////////////////////////////////////////////////////
    /// Get path of GFF index file
    fn get_path_index(path: &PathBuf) -> PathBuf {
        let mut path = path.clone();
       path.set_extension("index");
       path
    }

    ////////////////////////////////////////////////////////////
    /// Check GFF index exists
    pub fn index_exists(path: &PathBuf) -> bool {
        let path_chunks = FeatureCollection::get_path_chunks(path);
        let path_index = FeatureCollection::get_path_index(path);
        println!("index files: {:?} {:?}", path_chunks, path_index);
        path_chunks.exists() && path_index.exists() 
    }

    ////////////////////////////////////////////////////////////
    /// Write GFF index and chunks files
    pub fn write_gff_index(gff: &GBrowserGFF, path: &PathBuf) -> anyhow::Result<()> {

        let path_chunks = FeatureCollection::get_path_chunks(path);
        let path_index = FeatureCollection::get_path_index(path);

        let gff_desc = gff.get_description();
        let mut index = GBrowserGFFindex {
            chunk_sizes: gff_desc.chunk_sizes,
            chunk_coordinates: HashMap::new(),
            remainder: gff.remainder.clone(),
            chrom_sizes: gff.chrom_sizes.clone(),
        };

        println!("Writing GFF index, with {} items in remainder", gff.remainder.len());

        //Prepare chunks file
        let mut file_chunks = File::create(path_chunks)?;
        let mut current_chunks_file_pos = 0;

        //Compress each chunk in each track
        for (track_id, track) in gff.tracks.iter().enumerate() {
            for (chunk_id, v) in &track.records {

                //Serialize chunk
                let ser_out = serde_cbor::to_vec(&v)?;
                let reader = Cursor::new(ser_out);
                let mut gz = GzEncoder::new(reader, Compression::fast());
                let mut ret_vec = Vec::new();
                gz.read_to_end(&mut ret_vec)?;

                //Store chunk in file, figure out coordinates
                let pos_start = current_chunks_file_pos as u64;
                file_chunks.write_all(ret_vec.as_slice())?;
                current_chunks_file_pos += ret_vec.len();
                let pos_end = current_chunks_file_pos as u64;

                //Add coordinates to index
                let chunk_id = GBrowserGFFchunkID {
                    chr: chunk_id.0.clone(),
                    track: track_id as u64,
                    bin: chunk_id.1 as u64,
                };
                let chunk_pos = GBrowserGFFchunkpos {
                    start: pos_start,
                    end: pos_end,
                };
                index.chunk_coordinates.insert(chunk_id, chunk_pos);
            }
        }

        //Store index
        let file_index = File::create(path_index)?;
        let writer_index = BufWriter::new(file_index);
        serde_cbor::to_writer(writer_index, &index)?;


        anyhow::Ok(())
    }


    ////////////////////////////////////////////////////////////
    /// Create an empty GFF index with suitable chunk sizes
    pub fn make_default_gff() -> GBrowserGFF {
        let mut gff = GBrowserGFF::new();
        // right now,  -- (same as 1mb + 10mb. 45mb file => 167mb file)
        gff.add_track(1000000);  //1 mb
        gff.add_track(5000000);  //5 mb
        gff.add_track(50000000); //50 mb
        gff
    }    

    ////////////////////////////////////////////////////////////
    /// For GFF/GTF reading, process one record
    fn add_gene_record(gff: &mut GBrowserGFF, _params: &GFFparseSettings, record: &RecordBuf) {
        let newrec: GBrowserRecordBuf = convert_record(record);
        gff.add_record(newrec);
    }

    ////////////////////////////////////////////////////////////
    /// Read records from a GFF file
    fn read_gff_from_reader<R>(
        gff: &mut GBrowserGFF,
        reader: &mut gff::io::Reader<R>,
        params: &GFFparseSettings,
    ) -> anyhow::Result<()>
    where R: std::io::BufRead {
        let mut num_record = 0;
        for result in reader.record_bufs() {
            let record = result.expect("Could not read a GFF record; is it actually a GTF?");
            Self::add_gene_record(gff, params, &record);
            num_record += 1;
            if num_record % 100000 == 0 {
                println!("Processed {} GFF records", num_record);
            }
        }
        anyhow::Ok(())
    }

    ////////////////////////////////////////////////////////////
    /// Read records from a GTF file 
    fn read_gtf_from_reader<R>(
        gff: &mut GBrowserGFF,
        reader: &mut gtf::io::Reader<R>,
        params: &GFFparseSettings,
    ) -> anyhow::Result<()>
    where R: std::io::BufRead {
        let mut num_record = 0;
        for result in reader.record_bufs() {
            let record = result.expect("Could not read a GFF record; is it actually a GTF?");
            Self::add_gene_record(gff, params, &record);
            num_record += 1;
            if num_record % 100000 == 0 {
                println!("Processed {} GTF records", num_record);
            }
        }
        anyhow::Ok(())
    }

    ////////////////////////////////////////////////////////////
    /// Read records from a GFF-like file
    /// https://gmod.org/wiki/GFF3
    pub fn read_file(
        gff: &mut GBrowserGFF,
        path_gff: &PathBuf,
        params: &GFFparseSettings,
    ) -> anyhow::Result<()> {
        let spath = path_gff.to_string_lossy();

        if spath.ends_with("gff.gz") {
            println!("Reading gzipped GFF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(GzDecoder::new)
                .map(BufReader::new)
                .map(gff::io::Reader::new)?;
            Self::read_gff_from_reader(gff, &mut reader, params)
        } else if spath.ends_with("gff") {
            println!("Reading flat GFF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(BufReader::new)
                .map(gff::io::Reader::new)?;
            Self::read_gff_from_reader(gff, &mut reader, params)
        } else if spath.ends_with("gtf.gz") {
            println!("Reading gzipped GTF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(GzDecoder::new)
                .map(BufReader::new)
                .map(gtf::io::Reader::new)?;
            Self::read_gtf_from_reader(gff, &mut reader, params)
        } else if spath.ends_with("gtf") {
            println!("Reading gzipped GTF: {:?}", path_gff);
            let mut reader = File::open(&path_gff)
                .map(BufReader::new)
                .map(gtf::io::Reader::new)?;
            Self::read_gtf_from_reader(gff, &mut reader, params)
        } else {
            anyhow::bail!("Could not tell file format for GFF/GTF file {:?}", path_gff);
        }?;


        //See how well it worked
        let track_lens = gff.tracks.iter().map(|t| t.records.len()).collect::<Vec<_>>();
        println!("Done reading GFF; number of chunks in each track: {:?};  remainder bin features: {}", track_lens, gff.remainder.len());
        
        anyhow::Ok(())
    }


    ////////////////////////////////////////////////////////////
    /// Get GFF chunks given a request
    pub async fn get_gff_response(req: &GBrowserGFFchunkRequest, index: &GBrowserGFFindex,  path: &PathBuf) -> anyhow::Result<GBrowserGFFchunkResponse> {
        let path_chunks = FeatureCollection::get_path_chunks(path);
        let mut f = tokio::fs::File::open(path_chunks).await?;

        //Read all requested chunks
        let mut list_chunks: Vec<(GBrowserGFFchunkID, Vec<u8>)> = Vec::new();
        for id in &req.to_get {
            //Try to get the block. it might not exist if there are no features there
            let coord = index.chunk_coordinates.get(id);
            if let Some(coord) = coord {

                println!("Getting data {:?} => {:?}", id, coord);


                //Read the chunk if present
                let len = coord.end - coord.start;
                f.seek(SeekFrom::Start(coord.start)).await?;
                let mut buf = vec![0; len as usize];
                f.read_exact(&mut buf).await?;


/*
                //Test reading part
                let cursor = Cursor::new(&buf);
                let chunk_data: GBrowserChunk = serde_cbor::from_reader(cursor.reader()).expect("Failed to deserialize chunk");
                println!("{:?}",chunk_data);
 */



                list_chunks.push((id.clone(),buf));
            } else {
                //TODO this, or just don't send anything back. sender can check if anything was missing
                //let buf = vec![0; 0];
                //list_chunks.push((id.clone(),buf));
            }
        }

        anyhow::Ok(GBrowserGFFchunkResponse { 
            data: list_chunks 
        })
    }


    ////////////////////////////////////////////////////////////
    /// Read GFF index file
    pub fn read_gff_index(path: &PathBuf) -> anyhow::Result<GBrowserGFFindex> {
        //let path_index = path.join(".index");
        let path_index=FeatureCollection::get_path_index(path);

        let f = std::fs::File::open(path_index)?;
        let reader = std::io::BufReader::new(f);
        let res = serde_cbor::from_reader(reader).expect("Failed to deserialize");
        anyhow::Ok(res)
    }

}



//use async_stream::stream::

