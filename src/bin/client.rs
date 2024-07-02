use std::env;

use backend::servers::client::{AuthService, LocationService, PayementService};
use evaluation_juillet_2024::{self as backend, token::ClientHmac};
use mada_immo_client_tonic::{
    auth_server::AuthServer, location_server::LocationServer, payement_server::PayementServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = env::var("CLIENT_BACK_ADDR")
        .map(|r| {
            println!("{r}");
            r
        })?
        .parse()?;
    let pool = backend::etablish_connection();
    let hmac = ClientHmac::extract_client();
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(AuthServer::new(AuthService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(LocationServer::new(LocationService {
            pool: pool.clone(),
            hmac: hmac.clone(),
        })))
        .add_service(tonic_web::enable(PayementServer::new(PayementService {
            pool: pool.clone(),
            hmac: hmac.clone(),
        })))
        .serve(addr)
        .await?;
    Ok(())
}
