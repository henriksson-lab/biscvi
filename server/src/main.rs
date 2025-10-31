pub mod index;
pub mod countfile;
pub mod err;
pub mod gbrowser_gff;
pub mod gbrowser_noodles;

use std::fs::File;
use std::path::{Path};
use std::sync::Mutex;
use std::io::BufReader;

use actix_files::Files;
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{web, web::Data, App, HttpResponse, HttpServer, post};
use my_web_app::{FeatureCountsRequest, DatasetDescRequest, MetadataColumnRequest, ReductionRequest};
use serde::Deserialize;
use serde::Serialize;

use crate::err::MyError;
use crate::index::{index_bascet_dir, BascetDir};

////////////////////////////////////////////////////////////
/// Backend state
pub struct ServerData {
    bdir: BascetDir
}

////////////////////////////////////////////////////////////
/// Config file for the backend
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    bind: String,
    datadir: String,
}


////////////////////////////////////////////////////////////
/// REST entry point: Get feature counts for a given cell
#[post("/get_featurecounts")]
async fn get_featurecounts(server_data: Data<Mutex<ServerData>>, req_body: web::Json<FeatureCountsRequest>) -> Result<HttpResponse, MyError> { 

    println!("get_featurecounts {:?}",req_body);
    let Json(req) = req_body;

    let server_data =server_data.lock().unwrap();

    let feature_index = server_data.bdir.counts.get_feature_index(&req.counts_name, &req.feature_name)?;

    let mat = server_data.bdir.counts.get_counts_for_cell(&req.counts_name.into(), feature_index as u32)?;
    let ser_out = serde_cbor::to_vec(&mat)?;

    println!("get_featurecounts response {:?}",mat);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .body(ser_out))
}

////////////////////////////////////////////////////////////
/// REST entry point: Get coordinates for a reduction
#[post("/get_reduction")]
async fn get_reduction(server_data: Data<Mutex<ServerData>>, req_body: web::Json<ReductionRequest>) -> Result<HttpResponse, MyError> { 

    println!("get_reduction {:?}",req_body);
    let Json(req) = req_body;

    let server_data =server_data.lock().unwrap();
    let mat = server_data.bdir.counts.get_reduction(&req.reduction_name.into())?;
    let ser_out = serde_cbor::to_vec(&mat)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .body(ser_out))
}

////////////////////////////////////////////////////////////
/// REST entry point: Get a metadata column
#[post("/get_metacolumn")]
async fn get_metacolumn(server_data: Data<Mutex<ServerData>>, req_body: web::Json<MetadataColumnRequest>) -> Result<HttpResponse, MyError> { 

    println!("get_metacolumn {:?}",req_body);
    let Json(req) = req_body;

    let server_data =server_data.lock().unwrap();
    let mat = server_data.bdir.counts.get_metacolumn(&req.column_name.into())?;
    let ser_out = serde_cbor::to_vec(&mat)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .body(ser_out))
}

////////////////////////////////////////////////////////////
/// REST entry point
#[post("/get_dataset_desc")]
async fn get_dataset_desc(server_data: Data<Mutex<ServerData>>, req_body: web::Json<DatasetDescRequest>) -> Result<HttpResponse, MyError> { 

    println!("get_dataset_desc {:?}",req_body);

    let server_data =server_data.lock().unwrap();
    let mat = server_data.bdir.counts.get_desc()?; 
    let ser_out = serde_cbor::to_vec(&mat)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .body(ser_out))
}


////////////////////////////////////////////////////////////
/// Backend entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Read the config file
    let f_meta = File::open("config.json").expect("Could not open config.json");
    let config_reader = BufReader::new(f_meta);
    let config_file:ConfigFile = serde_json::from_reader(config_reader).expect("Could not open config file");

    let bascet_dir = Path::new(&config_file.datadir);
    let bdir = index_bascet_dir(&bascet_dir).expect("Failed to index data");
    
    let data = Data::new(Mutex::new(
        ServerData {
            bdir: bdir
        }
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())  //for debugging
            .service(get_featurecounts)
            .service(get_reduction)
            .service(get_metacolumn)
            .service(get_dataset_desc)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            //.service(get_)
            .default_service(
                web::route().to(|| HttpResponse::NotFound()),  //header("Location", "/").finish()
            )
    })
    .bind(config_file.bind)? /////////////// for dev, "127.0.0.1:8080"  ; 127.0.0.1:5199 for beagle deployment
    .run()
    .await
}
