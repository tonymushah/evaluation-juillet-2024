pub mod models;

use diesel::prelude::*;
use diesel_schemas::tables::location;
use mada_immo_utils::get_date_fin_from_nb_mois;
use models::{InsertClient, InsertLocation};
use serde::{Deserialize, Deserializer};
use time::{format_description, Date, PrimitiveDateTime, Time};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CSVLocationInsertError {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error("Can't get the day from {date} {nb_mois}")]
    DateFinGetError { date: Date, nb_mois: u8 },
}

#[derive(Debug, Clone, Deserialize)]
pub struct CSVLocation {
    pub reference: String,
    #[serde(alias = "Date debut", deserialize_with = "serealize_date_debut")]
    pub date_debut: Date,
    #[serde(alias = "duree mois")]
    pub duree_mois: u8,
    pub client: String,
}

impl CSVLocation {
    pub fn insert(&self, con: &mut PgConnection) -> Result<Uuid, CSVLocationInsertError> {
        use self::location::dsl::*;
        InsertClient::insert(self.client.clone(), con)?;
        Ok(diesel::insert_into(location)
            .values(InsertLocation {
                bien: self.reference.clone(),
                client: self.client.clone(),
                date_debut: PrimitiveDateTime::new(self.date_debut, Time::MIDNIGHT),
                date_fin: PrimitiveDateTime::new(
                    get_date_fin_from_nb_mois(self.date_debut, self.duree_mois).ok_or(
                        CSVLocationInsertError::DateFinGetError {
                            date: self.date_debut,
                            nb_mois: self.duree_mois,
                        },
                    )?,
                    Time::MIDNIGHT,
                ),
            })
            .returning(id_location)
            .get_result(con)?)
    }
}

const CSV_LOCATION_DATE_DEBUT_FORMAT: &str = "[day]/[month]/[year]";

fn serealize_date_debut<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    let format_des = format_description::parse(CSV_LOCATION_DATE_DEBUT_FORMAT)
        .map_err(serde::de::Error::custom)?;
    Date::parse(&date_str, &format_des).map_err(serde::de::Error::custom)
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

    use super::CSVLocation;

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
            BufReader::new(File::open("../../data/Donnees-csv-saison3 - Location.csv").unwrap());
        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVLocation>().flatten() {
            println!("{:?}", bien)
        }
    }
    #[test]
    fn insert_test() {
        let reader =
            BufReader::new(File::open("../../data/Donnees-csv-saison3 - Location.csv").unwrap());
        let pool = etablish_connection();
        let mut con = pool.get().unwrap();

        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVLocation>().flatten() {
            println!("{}", bien.insert(&mut con).unwrap())
        }
    }
}
