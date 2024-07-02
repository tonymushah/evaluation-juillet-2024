use mada_immo_client_tonic::{
    location_server::Location, ListLocationRequest, ListLocationResponse,
};
use tonic::Request;

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct LocationService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Location for LocationService {
    async fn list(
        &self,
        request: Request<ListLocationRequest>,
    ) -> TonicRpcResult<ListLocationResponse> {
        todo!()
    }
}
