use bstr::BString;
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GBrowserPhase {
    Zero,
    One,
    Two,
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GBrowserStrand {
    None,
    Forward,
    Reverse,
    Unknown,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GBrowserAttributeValue {
    String(BString),
//    Array(Array<'a>),
}

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



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserChunk {
    pub records: Vec<GBrowserRecordBuf>,
}



////////////////////////////////////////////////////////////
/// 
pub struct GBrowserChunkTrack {
    pub chunk_size: u64,
    pub records: HashMap<usize, GBrowserChunk>,    
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
}

impl GBrowserGFF {

    ////////////////////////////////////////////////////////////
    /// Constructor
    pub fn new() -> GBrowserGFF {
        GBrowserGFF {
            tracks: Vec::new(),
            remainder: Vec::new(),
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

        //Try to place record in a track
        for t in &mut self.tracks {
            let bin_start = t.pos_to_bin(rec.start);
            let bin_end = t.pos_to_bin(rec.end);

            if bin_start==bin_end {
                //Insert record here if it fits
                let chunk = t.records.get_mut(&bin_start);
                if let Some(chunk) = chunk {
                    chunk.records.insert(bin_start, rec);
                } else {
                    let mut chunk = GBrowserChunk {
                        records: Vec::new()
                    };
                    chunk.records.insert(bin_start, rec);
                    t.records.insert(bin_start, chunk);
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
            remainder: Vec::new() ///////////////////// ???
        }
    }

}





////////////////////////////////////////////////////////////
/// Description to be sent over the network
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserGFFdescription {
    pub chunk_sizes: Vec<u64>,
    pub remainder: Vec<GBrowserRecordBuf>,
}




////////////////////////////////////////////////////////////
/// Index into binary file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserGFFindex {
    pub chunk_sizes: Vec<u64>,
    pub remainder: Vec<GBrowserRecordBuf>,

    pub chunk_coordinates: HashMap<(u32, u64),GBrowserGFFchunkpos>,  // (trackID, chunkID) => (start,end) in bytes

}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GBrowserGFFchunkpos {
    pub start: u64,
    pub end: u64
}
