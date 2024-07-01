use diesel::prelude::*;
use diesel_schemas::tables::{client, location};
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = location)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertLocation {
    pub bien: String,
    pub client: String,
    pub date_debut: PrimitiveDateTime,
    pub date_fin: PrimitiveDateTime,
}

impl InsertLocation {
    pub fn insert(&self, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::location::dsl::*;
        diesel::insert_into(location)
            .values(self)
            .returning(id_location)
            .get_result(con)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertClient {
    pub email: String,
    pub nom: String,
}

impl InsertClient {
    pub fn insert(email_: String, con: &mut PgConnection) -> QueryResult<()> {
        use self::client::dsl::*;
        let inner = client
            .filter(email.eq(&email_))
            .select(email)
            .get_result::<String>(con);
        if inner.is_ok() {
            Ok(())
        } else if let Err(diesel::result::Error::NotFound) = &inner {
            diesel::insert_into(client)
                .values(Self {
                    email: email_,
                    nom: Default::default(),
                })
                .execute(con)?;
            Ok(())
        } else {
            inner.map(|_| {})
        }
    }
}
