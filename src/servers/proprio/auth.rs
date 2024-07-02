use mada_immo_csv_import::bien::models::InsertProprietaire;
use mada_immo_proprio_tonic::{auth_server::Auth, LoginRequest, LoginResponse};
use tonic::{Request, Response};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> TonicRpcResult<LoginResponse> {
        let pool = self.pool.clone();
        let tel = request.get_ref().telephone.clone();

        let token = crate::spawn_blocking(move || -> crate::Result<String> {
            use diesel::prelude::*;
            use diesel_schemas::tables::proprietaire::dsl::*;
            let mut con = pool.get()?;
            let res = proprietaire
                .filter(telephone.eq(tel.clone()))
                .select(telephone)
                .get_result::<String>(&mut con);
            if let Err(diesel::result::Error::NotFound) = &res {
                Ok(diesel::insert_into(proprietaire)
                    .values(InsertProprietaire {
                        telephone: tel,
                        nom: String::default(),
                    })
                    .returning(telephone)
                    .get_result(&mut con)?)
            } else {
                Ok(res?)
            }
        })
        .await??;
        Ok(Response::new(LoginResponse { token }))
    }
}
