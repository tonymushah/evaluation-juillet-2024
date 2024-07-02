use mada_immo_admin_tonic::{auth_server::Auth, LoginRequest, LoginResponse};
use tonic::Request;

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> TonicRpcResult<LoginResponse> {
        todo!()
    }
}
