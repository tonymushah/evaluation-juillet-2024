use mada_immo_admin_tonic::{database_server::Database, Empty};
use tonic::{Request, Response, Status, Streaming};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct DatabaseService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Database for DatabaseService {
    async fn reset(&self, reuest: Request<Empty>) -> TonicRpcResult<Empty> {
        todo!()
    }
}
