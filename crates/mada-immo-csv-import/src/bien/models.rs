use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::tables::{bien, bien_loyer, proprietaire, region, type_bien};
use uuid::Uuid;

#[derive(Debug, Clone, Insertable, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = bien)]
pub struct InsertBien {
    pub id_bien: String,
    pub nom: String,
    pub region: Uuid,
    pub description: String,
    pub proprietaire: String,
    pub type_bien: Uuid,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = bien_loyer)]
pub struct InsertBienLoyer {
    pub bien: String,
    pub valeur: BigDecimal,
}

impl InsertBienLoyer {
    pub fn insert(&self, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::bien_loyer::dsl::*;
        diesel::insert_into(bien_loyer)
            .values(self)
            .returning(id_bien_loyer)
            .get_result(con)
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = type_bien)]
pub struct InsertTypeBien {
    pub id_type_bien: Uuid,
    pub designation: String,
}

impl InsertTypeBien {
    fn insert_by_nom(nom_: String, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::type_bien::dsl::*;
        let to_insert = Self {
            id_type_bien: Uuid::new_v4(),
            designation: nom_,
        };
        diesel::insert_into(type_bien)
            .values(to_insert)
            .returning(id_type_bien)
            .get_result(con)
    }
    fn get_by_nom(nom_: String, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::type_bien::dsl::*;
        type_bien
            .filter(designation.eq(nom_))
            .select(id_type_bien)
            .get_result(con)
    }
    pub fn get_by_nom_or_insert(nom: String, con: &mut PgConnection) -> QueryResult<Uuid> {
        let res = Self::get_by_nom(nom.clone(), con);
        if let Ok(id) = &res {
            Ok(*id)
        } else if let Err(diesel::result::Error::NotFound) = &res {
            Self::insert_by_nom(nom, con)
        } else {
            res
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = proprietaire)]
pub struct InsertProprietaire {
    pub telephone: String,
    pub nom: String,
}

impl InsertProprietaire {
    pub fn insert(telephone_: String, con: &mut PgConnection) -> QueryResult<()> {
        use self::proprietaire::dsl::*;
        let inner = proprietaire
            .filter(telephone.eq(&telephone_))
            .select(telephone)
            .get_result::<String>(con);
        if inner.is_ok() {
            Ok(())
        } else if let Err(diesel::result::Error::NotFound) = &inner {
            diesel::insert_into(proprietaire)
                .values(Self {
                    telephone: telephone_,
                    nom: Default::default(),
                })
                .execute(con)?;
            Ok(())
        } else {
            inner.map(|_| {})
        }
    }
}

#[derive(Debug, Clone, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = region)]
pub struct InsertRegion {
    pub id_region: Uuid,
    pub nom: String,
}

impl InsertRegion {
    fn insert_by_nom(nom_: String, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::region::dsl::*;
        let to_insert = Self {
            id_region: Uuid::new_v4(),
            nom: nom_,
        };
        diesel::insert_into(region)
            .values(to_insert)
            .returning(id_region)
            .get_result(con)
    }
    fn get_by_nom(nom_: String, con: &mut PgConnection) -> QueryResult<Uuid> {
        use self::region::dsl::*;
        region
            .filter(nom.eq(nom_))
            .select(id_region)
            .get_result(con)
    }
    pub fn get_by_nom_or_insert(nom: String, con: &mut PgConnection) -> QueryResult<Uuid> {
        let res = Self::get_by_nom(nom.clone(), con);
        if let Ok(id) = &res {
            Ok(*id)
        } else if let Err(diesel::result::Error::NotFound) = &res {
            Self::insert_by_nom(nom, con)
        } else {
            res
        }
    }
}
