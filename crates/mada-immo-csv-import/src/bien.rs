pub mod models;

use bigdecimal::BigDecimal;
use diesel::{prelude::*, PgConnection, QueryResult};
use models::{InsertBien, InsertBienLoyer, InsertProprietaire, InsertRegion, InsertTypeBien};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CSVBien {
    pub reference: String,
    pub nom: String,
    #[serde(alias = "Description")]
    pub description: String,
    #[serde(alias = "Type")]
    pub type_: String,
    pub region: String,
    #[serde(alias = "loyer mensuel")]
    pub loyer: BigDecimal,
    #[serde(alias = "Proprietaire")]
    pub proprietaire: String,
}

impl CSVBien {
    pub fn insert(&self, con: &mut PgConnection) -> QueryResult<()> {
        let reg = InsertRegion::get_by_nom_or_insert(self.region.clone(), con)?;
        let type_ = InsertTypeBien::get_by_nom_or_insert(self.type_.clone(), con)?;
        InsertProprietaire::insert(self.proprietaire.clone(), con)?;
        {
            use diesel_schemas::tables::bien::dsl::*;
            diesel::insert_into(bien)
                .values(InsertBien {
                    id_bien: self.reference.clone(),
                    nom: self.nom.clone(),
                    region: reg,
                    description: self.description.clone(),
                    proprietaire: self.proprietaire.clone(),
                    type_bien: type_,
                })
                .execute(con)?;
        }
        InsertBienLoyer {
            bien: self.reference.clone(),
            valeur: self.loyer.clone(),
        }
        .insert(con)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use diesel::{
        r2d2::{ConnectionManager, Pool},
        PgConnection,
    };
    use dotenvy::dotenv;
    use std::{env, fs::File, io::BufReader};

    use csv::Reader;

    use super::CSVBien;

    pub type DbPool = Pool<ConnectionManager<PgConnection>>;

    //pub type DbPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

    fn etablish_connection() -> DbPool {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        Pool::builder()
            .build(manager)
            .expect("Failed to create a pool.")
    }

    #[test]
    fn parse_test() {
        let reader = BufReader::new(
            File::open("../../Resultat-Donnees-csv-import-2juillet-saison3 - Biens.csv").unwrap(),
        );
        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVBien>().flatten() {
            println!("{:?}", bien)
        }
    }
    #[test]
    fn insert_test() {
        let reader = BufReader::new(
            File::open("../../data/Resultat-Donnees-csv-import-2juillet-saison3 - Biens.csv")
                .unwrap(),
        );
        let pool = etablish_connection();
        let mut con = pool.get().unwrap();

        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVBien>().flatten() {
            bien.insert(&mut con).unwrap();
        }
    }
}
