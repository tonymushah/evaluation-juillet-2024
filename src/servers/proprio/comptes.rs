use mada_immo_proprio_tonic::{
    comptes_server::Comptes, ChiffreAffairesRequest, ChiffreAffairesResponse,
};
use tonic::Request;

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct ComptesService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Comptes for ComptesService {
    async fn chiffre_affaires(
        &self,
        request: Request<ChiffreAffairesRequest>,
    ) -> TonicRpcResult<ChiffreAffairesResponse> {
        todo!()
    }
}
