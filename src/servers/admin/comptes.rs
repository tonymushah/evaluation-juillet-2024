use mada_immo_admin_tonic::{
    comptes_server::Comptes, ChiffreAffaireRequest, ChiffreAffaireResponse, GainRequest,
    GainResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct ComptesService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Comptes for ComptesService {
    type ChiffreAffaireStream = ReceiverStream<Result<ChiffreAffaireResponse, Status>>;
    async fn chiffre_affaire(
        &self,
        request: Request<Streaming<ChiffreAffaireRequest>>,
    ) -> TonicRpcResult<Self::ChiffreAffaireStream> {
        todo!()
    }
    type GainStream = ReceiverStream<Result<GainResponse, Status>>;
    async fn gain(
        &self,
        request: Request<Streaming<GainRequest>>,
    ) -> TonicRpcResult<Self::GainStream> {
        todo!()
    }
}
