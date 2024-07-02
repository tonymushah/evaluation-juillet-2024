use mada_immo_admin_tonic::{database_server::Database, Empty};
use tonic::{Request, Response};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct DatabaseService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Database for DatabaseService {
    async fn reset(&self, _reuest: Request<Empty>) -> TonicRpcResult<Empty> {
        let pool = self.pool.clone();
        crate::spawn_blocking(move || -> crate::Result<()> {
            let mut con = pool.get()?;
            crate::reset::reset_db(&mut con)?;
            Ok(())
        })
        .await??;
        Ok(Response::new(Empty {}))
    }
}
