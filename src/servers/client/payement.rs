use mada_immo_client_tonic::{
    payement_server::Payement, Empty, FairePayementRequest, LoyersPayeRequest, LoyersPayeResponse,
    LoyersRequest, LoyersResponse,
};
use tonic::Request;

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct PayementService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Payement for PayementService {
    async fn loyers(&self, request: Request<LoyersRequest>) -> TonicRpcResult<LoyersResponse> {
        todo!()
    }
    async fn loyers_paye(
        &self,
        request: Request<LoyersPayeRequest>,
    ) -> TonicRpcResult<LoyersPayeResponse> {
        todo!()
    }
    async fn faire_payement(
        &self,
        request: Request<FairePayementRequest>,
    ) -> TonicRpcResult<Empty> {
        todo!()
    }
}
