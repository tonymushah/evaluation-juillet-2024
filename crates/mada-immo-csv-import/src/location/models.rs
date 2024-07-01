use diesel::prelude::*;
use diesel_schemas::tables::client;
use time::PrimitiveDateTime;

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
