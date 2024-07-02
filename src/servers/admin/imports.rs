use mada_immo_admin_tonic::{
    imports_server::Imports, Empty, ImportBienRequest, ImportCommissionRequest,
    ImportLocationRequest,
};
use tonic::{Request, Response, Status, Streaming};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct ImportsService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Imports for ImportsService {
    async fn bien(&self, request: Request<Streaming<ImportBienRequest>>) -> TonicRpcResult<Empty> {
        todo!()
    }
    async fn location(
        &self,
        request: Request<Streaming<ImportLocationRequest>>,
    ) -> TonicRpcResult<Empty> {
        todo!()
    }
    async fn commission(
        &self,
        request: Request<Streaming<ImportCommissionRequest>>,
    ) -> TonicRpcResult<Empty> {
        todo!()
    }
}
