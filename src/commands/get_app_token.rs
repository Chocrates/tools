use crate::utility;
use clap::Args;
use octocrab::*;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use rustls_pemfile as pemfile;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use load_file::{load_bytes};

#[derive(Args, Clone, Debug)]
pub struct GetAppToken {
    /// GitHub Organization that the application is installed in
    #[clap(short, long, value_parser)]
    organization: String,

    /// Path to private key file
    #[clap(short, long, value_parser)]
    private_key: String,

    /// GitHub Application ID
    #[clap(short, long, value_parser)]
    app_id: u64,
}


pub async fn exec(args: GetAppToken) -> Result<(), Box<dyn Error>> {
    let app_id = models::AppId::from(args.app_id);
     
    let _octocrab = Octocrab::builder().app(app_id, get_encoding_key_from_pem(get_key_from_pem(args.private_key)))
        .build()
        .expect("Unable to build Octocrab instance");
    let installation_result: Result<models::Installation> = _octocrab.apps().get_org_installation(args.organization).await?;
    let org_installation = installation_result.expect("Unable to get installation");
    println!("{:?}", org_installation);
    Ok(())
}

// fn get_key_from_pem(pem_path: String) -> [u8] {
//     let pem = std::fs::read_to_string(pem_path).expect("Unable to read pem file");
//     let key = pemfile::pkcs8_private_keys(&mut pem.as_bytes())
//         .expect("Unable to parse pem file")
//         .remove(0);
//     key.private_key().to_vec()
// }


// fn get_encoding_key_from_pem(array: [u8]) -> jsonwebtoken::EncodingKey {
//     jsonwebtoken::EncodingKey::from_secret(&array)
// }

fn get_encoding_key_from_pem_file(file: &String) -> EncodingKey {

    EncodingKey::from_ed_der

}