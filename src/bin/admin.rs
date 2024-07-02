use std::env;

use backend::servers::admin::{
    AuthService, ComptesService, DatabaseService, ImportsService, LocationService,
};
use evaluation_juillet_2024::{self as backend};
use mada_immo_admin_tonic::{
    auth_server::AuthServer, comptes_server::ComptesServer, database_server::DatabaseServer,
    imports_server::ImportsServer, location_server::LocationServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = env::var("ADMIN_BACK_ADDR")
        .map(|r| {
            println!("{r}");
            r
        })?
        .parse()?;
    let pool = backend::etablish_connection();
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(AuthServer::new(AuthService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(ComptesServer::new(ComptesService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(ImportsServer::new(ImportsService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(DatabaseServer::new(DatabaseService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(LocationServer::new(LocationService {
            pool: pool.clone(),
        })))
        .serve(addr)
        .await?;
    Ok(())
}
