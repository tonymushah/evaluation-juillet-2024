use mada_immo_proprio_tonic::{
    biens_server::Biens, Bien, GetBienRequest, ListBienRequest, ListBienResponse,
    ListTypeBienRequest, ListTypeBienResponse,
};
use tonic::Request;

use crate::{servers::TonicRpcResult, token::ClientHmac, DbPool};

#[derive(Debug, Clone)]
pub struct BiensService {
    pub pool: DbPool,
    pub hmac: ClientHmac,
}

#[tonic::async_trait]
impl Biens for BiensService {
    async fn list_bien(
        &self,
        request: Request<ListBienRequest>,
    ) -> TonicRpcResult<ListBienResponse> {
        crate::tonic_not_implemented()
    }
    async fn get(&self, request: tonic::Request<GetBienRequest>) -> TonicRpcResult<Bien> {
        crate::tonic_not_implemented()
    }
    async fn list_type_bien(
        &self,
        request: Request<ListTypeBienRequest>,
    ) -> TonicRpcResult<ListTypeBienResponse> {
        crate::tonic_not_implemented()
    }
}
