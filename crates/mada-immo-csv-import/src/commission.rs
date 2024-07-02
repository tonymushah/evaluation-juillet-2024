pub mod models;

use bigdecimal::BigDecimal;
use diesel::PgConnection;
use models::InsertTypeBienCommission;
use serde::Deserialize;
use uuid::Uuid;

use crate::bien::models::InsertTypeBien;

#[derive(Debug, thiserror::Error)]
pub enum CSVCommissionParseError {
    #[error("{:?}", .0)]
    BigDecimal(#[from] bigdecimal::ParseBigDecimalError),
    #[error("{:?}", .0)]
    Float(#[from] std::num::ParseFloatError),
}

#[derive(Debug, thiserror::Error)]
pub enum CSVCommissionInsertError {
    #[error("{:?}", .0)]
    CommissionParse(#[from] CSVCommissionParseError),
    #[error("{:?}", 0)]
    Diesel(#[from] diesel::result::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct CSVCommission {
    #[serde(alias = "Type")]
    pub type_: String,
    #[serde(alias = "Commission")]
    pub commission: String,
}

impl CSVCommission {
    pub fn parse_commission_float(&self) -> Result<f64, CSVCommissionParseError> {
        Ok(self
            .commission
            .replace('%', "")
            .replace(',', ".")
            .parse::<f64>()?)
    }
    pub fn parse_commission(&self) -> Result<BigDecimal, CSVCommissionParseError> {
        Ok(self.parse_commission_float()?.try_into()?)
    }
    pub fn insert(&self, con: &mut PgConnection) -> Result<Uuid, CSVCommissionInsertError> {
        let type_bien = InsertTypeBien::get_by_nom_or_insert(self.type_.clone(), con)?;
        Ok(InsertTypeBienCommission {
            type_bien,
            valeur: self.parse_commission()?,
        }
        .insert(con)?)
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

    use super::CSVCommission;

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
        let reader =
            BufReader::new(File::open("../../data/Donnees-csv-saison3 - Commission.csv").unwrap());
        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVCommission>().flatten() {
            println!("{:?}", bien)
        }
    }
    #[test]
    fn insert_test() {
        let reader = BufReader::new(
            File::open("../../data/Resultat-Donnees-csv-import-2juillet-saison3 - Commission.csv")
                .unwrap(),
        );
        let pool = etablish_connection();
        let mut con = pool.get().unwrap();

        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVCommission>().flatten() {
            println!("{}", bien.insert(&mut con).unwrap())
        }
    }
}
