use bstr::BString;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;


////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GBrowserPhase {
    Zero,
    One,
    Two,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GBrowserStrand {
    None,
    Forward,
    Reverse,
    Unknown,
}


////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GBrowserAttributeValue {
    String(BString),
//    Array(Array<'a>),
}

////////////////////////////////////////////////////////////
/// A GFF record that can be serialized
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserRecordBuf {
    pub reference_sequence_name: BString,
    //source: BString,
    pub ty: BString,
    pub start: u64, //1-based
    pub end: u64, //1-based
    //score: Option<f32>,
    pub strand: GBrowserStrand,
    pub phase: Option<GBrowserPhase>,
    pub attributes: HashMap<BString, GBrowserAttributeValue>,
}



////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserChunk {
//    pub records: HashMap<u64, GBrowserRecordBuf>,
    pub records: Vec<GBrowserRecordBuf>,
}



////////////////////////////////////////////////////////////
/// 
pub struct GBrowserChunkTrack {
    pub chunk_size: u64,
    pub records: HashMap<(BString, usize), GBrowserChunk>,      //TODO split into a sub class
}

impl GBrowserChunkTrack {

    ////////////////////////////////////////////////////////////
    /// Which bin does a position refer to?
    pub fn pos_to_bin(&self, pos: u64) -> usize {
        (pos / self.chunk_size) as usize
    }
}


////////////////////////////////////////////////////////////
/// 
pub struct GBrowserGFF {
    pub tracks: Vec<GBrowserChunkTrack>,
    pub remainder: Vec<GBrowserRecordBuf>,
    pub chrom_sizes: HashMap<BString, u64>,
}

impl GBrowserGFF {

    ////////////////////////////////////////////////////////////
    /// Constructor
    pub fn new() -> GBrowserGFF {
        GBrowserGFF {
            tracks: Vec::new(),
            remainder: Vec::new(),
            chrom_sizes: HashMap::new(),
        }
    }

    ////////////////////////////////////////////////////////////
    /// add track. call from smallest to largest
    pub fn add_track(&mut self, chunk_size: u64) {
        self.tracks.push(GBrowserChunkTrack {
            chunk_size,
            records: HashMap::new()
        });        
    }


    ////////////////////////////////////////////////////////////
    /// add record
    pub fn add_record(&mut self, rec: GBrowserRecordBuf) {

        //Update chromosome size
        let cur_max = self.chrom_sizes
            .entry(rec.reference_sequence_name.clone())  //Plenty cloning here TODO; only update if new record different sequence?
            .or_insert(0);
        if rec.end > *cur_max {
            *cur_max = rec.end
        }

        //Try to place record in a track
        for t in &mut self.tracks {
            let bin_start = t.pos_to_bin(rec.start);
            let bin_end = t.pos_to_bin(rec.end);

            if bin_start==bin_end {
                //Insert record here if it fits
                let bin_id = (rec.reference_sequence_name.clone(),bin_start);
                let chunk = t.records.get_mut(&bin_id);
                if let Some(chunk) = chunk {
                    chunk.records.push(rec);
                } else {
                    let mut chunk = GBrowserChunk {
                        records: Vec::new()
                    };
                    chunk.records.push(rec);
                    t.records.insert(bin_id, chunk);
                }
                return;
            }
        }
        //Give up and store in remainder bin
        self.remainder.push(rec);
    }


    ////////////////////////////////////////////////////////////
    /// Get a description of the GFF store
    pub fn get_description(&self) -> GBrowserGFFdescription {
        let mut chunk_sizes: Vec<u64> = Vec::new();
        for t in &self.tracks {
            chunk_sizes.push(t.chunk_size);
        }
        GBrowserGFFdescription {
            chunk_sizes,
            remainder: self.remainder.clone(), 
            chrom_sizes: self.chrom_sizes.clone()
        }
    }

}





////////////////////////////////////////////////////////////
/// Description to be sent over the network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserGFFdescription {
    pub chunk_sizes: Vec<u64>,
    pub remainder: Vec<GBrowserRecordBuf>,
    pub chrom_sizes: HashMap<BString, u64>,
}

impl GBrowserGFFdescription {

    pub fn new() -> GBrowserGFFdescription {
        GBrowserGFFdescription {
            chunk_sizes: Vec::new(),
            remainder: Vec::new(),
            chrom_sizes: HashMap::new(),
        }
    }
}



////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct GBrowserGFFchunkpos {
    // These are coordinates in bytes  (or store len?)
    pub start: u64,
    pub end: u64
}



////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct GBrowserGFFchunkID {
    pub chr: BString,
    pub track: u64,
    pub bin: u64
}
impl GBrowserGFFchunkID {

    pub fn new(chr: BString, track: u64, bin: u64) -> GBrowserGFFchunkID {
        GBrowserGFFchunkID {
            chr, track, bin
        }
    }    

}




////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserGFFchunkRequest {
    pub to_get: Vec<GBrowserGFFchunkID>
}



////////////////////////////////////////////////////////////
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserGFFchunkResponse {
    pub data: Vec<(GBrowserGFFchunkID, Vec<u8>)>,
}




////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct GBrowserGFFdescriptionRequest {
}

////////////////////////////////////////////////////////////
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct GBrowserGFFdescriptionResponse {
    //pub matrices: HashMap<String, CountFileMat>,
    //p//ub reductions: HashMap<String, CountFileRed>,    
    //pub meta: HashMap<String, CountFileMetaColumnDesc>,
}