use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::tables::type_bien_commission;
use uuid::Uuid;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = type_bien_commission)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertTypeBienCommission {
    pub type_bien: Uuid,
    pub valeur: BigDecimal,
}

impl InsertTypeBienCommission {
    pub fn insert(&self, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::type_bien_commission::dsl::*;
        diesel::insert_into(type_bien_commission)
            .values(self)
            .returning(id_type_bien_commission)
            .get_result(con)
    }
}
