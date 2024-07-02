use mada_immo_client_tonic::{auth_server::Auth, LoginRequest, LoginResponse};
use mada_immo_csv_import::location::models::InsertClient;
use tonic::{Request, Response, Status};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> TonicRpcResult<LoginResponse> {
        let pool = self.pool.clone();
        let email_ = request.get_ref().email.clone();

        let token = crate::spawn_blocking(move || -> crate::Result<String> {
            use diesel::prelude::*;
            use diesel_schemas::tables::client::dsl::*;
            let mut con = pool.get()?;
            let res = client
                .filter(email.eq(email_.clone()))
                .select(email)
                .get_result::<String>(&mut con);
            if let Err(diesel::result::Error::NotFound) = &res {
                Ok(diesel::insert_into(client)
                    .values(InsertClient {
                        email: email_,
                        nom: String::default(),
                    })
                    .returning(email)
                    .get_result(&mut con)?)
            } else {
                Ok(res?)
            }
        })
        .await??;
        Ok(Response::new(LoginResponse { token }))
    }
}
