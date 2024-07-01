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
    pub loyer: f64,
    #[serde(alias = "Proprietaire")]
    pub proprietaire: String,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use csv::Reader;

    use super::CSVBien;

    #[test]
    fn parse_test() {
        let reader =
            BufReader::new(File::open("../../data/Donnees-csv-saison3 - Biens.csv").unwrap());
        let mut reader = Reader::from_reader(reader);
        for bien in reader.deserialize::<CSVBien>().flatten() {
            println!("{:?}", bien)
        }
    }
}
