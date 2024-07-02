use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::{
    tables::{client, location, location_speculative},
    views::v_location_bien,
};
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Selectable, Queryable)]
#[diesel(table_name = v_location_bien)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LocationCommissionLoyer {
    pub id_location: Uuid,
    pub commission: BigDecimal,
    pub loyer: BigDecimal,
}

impl LocationCommissionLoyer {
    pub fn get_by_location(id_loc: Uuid, con: &mut PgConnection) -> QueryResult<Self> {
        use self::v_location_bien::dsl::*;
        v_location_bien
            .filter(id_location.eq(id_loc))
            .select(Self::as_select())
            .get_result(con)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = location_speculative)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertLocationSpeculable {
    pub location: Uuid,
    pub date_ref: Date,
    pub loyer_initial: BigDecimal,
    pub commission: BigDecimal,
    pub num_mois: i32,
    pub loyer_a_payer: BigDecimal,
    pub valeur_commission: BigDecimal,
}

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
