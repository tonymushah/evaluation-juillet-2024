use std::env;

use backend::servers::proprio::{AuthService, BiensService, ComptesService};
use evaluation_juillet_2024::{self as backend, token::ClientHmac};
use mada_immo_proprio_tonic::{
    auth_server::AuthServer, biens_server::BiensServer, comptes_server::ComptesServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = env::var("PROPRIO_BACK_ADDR")
        .map(|r| {
            println!("{r}");
            r
        })?
        .parse()?;
    let pool = backend::etablish_connection();
    let hmac = ClientHmac::extract_proprio();
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(AuthServer::new(AuthService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(BiensServer::new(BiensService {
            pool: pool.clone(),
            hmac: hmac.clone(),
        })))
        .add_service(tonic_web::enable(ComptesServer::new(ComptesService {
            pool: pool.clone(),
            hmac: hmac.clone(),
        })))
        .serve(addr)
        .await?;
    Ok(())
}
