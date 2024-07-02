use mada_immo_admin_tonic::{
    location_server::Location, Empty, ListBienRequest, ListBienResponse, ListClientRequest,
    ListClientResponse, NewLocationRequest, NewLocationResponse,
};
use tonic::{Request, Response, Status, Streaming};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct LocationService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Location for LocationService {
    async fn new_location(
        &self,
        request: Request<NewLocationRequest>,
    ) -> TonicRpcResult<NewLocationResponse> {
        crate::tonic_not_implemented()
    }
    async fn list_bien(
        &self,
        request: tonic::Request<ListBienRequest>,
    ) -> TonicRpcResult<ListBienResponse> {
        crate::tonic_not_implemented()
    }
    async fn list_client(
        &self,
        request: Request<ListClientRequest>,
    ) -> TonicRpcResult<ListClientResponse> {
        crate::tonic_not_implemented()
    }
}
