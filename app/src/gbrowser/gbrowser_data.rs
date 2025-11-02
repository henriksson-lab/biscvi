use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::sync::Arc;

use bytes::Buf;
use my_web_app::gbrowser_struct::{GBrowserGFFchunkID, GBrowserGFFchunkRequest, GBrowserGFFchunkResponse, GBrowserGFFdescription};
use my_web_app::gbrowser_struct::GBrowserChunk;

use crate::appstate::AsyncData;

use flate2::read::GzDecoder;


////////////////////////////////////////////////////////////
/// Current GFF file data
/// !!! need to be wrapped in a mutex to avoid expensive copying
pub struct ClientGBrowseData {
    pub desc: GBrowserGFFdescription,
    pub chunks: HashMap<GBrowserGFFchunkID,AsyncData<GBrowserChunk>>,
}

impl ClientGBrowseData {

    ////////////////////////////////////////////////////////////
    /// Set loading status for chunks being requested
    pub fn set_loading(&mut self, query: &GBrowserGFFchunkRequest) {
        for id in &query.to_get {
            self.chunks.insert(id.clone(), AsyncData::Loading);
        }
    }


    ////////////////////////////////////////////////////////////
    /// Set loaded chunks from response
    pub fn set_chunks(&mut self, res: GBrowserGFFchunkResponse) {
        for (id, chunk_bytes) in res.data {

            let cursor = Cursor::new(chunk_bytes);

            //Uncompress data
            let reader = cursor.reader();
            let reader = GzDecoder::new(reader);
            let reader = BufReader::new(reader);

            //Deserialize
            let chunk_data = serde_cbor::from_reader(reader).expect("Failed to deserialize chunk");
            self.chunks.insert(id, AsyncData::Loaded(Arc::new(chunk_data)));
        }

        // should above be async to speed up interface?

        //TODO: fill in missing chunks too
    }
}


