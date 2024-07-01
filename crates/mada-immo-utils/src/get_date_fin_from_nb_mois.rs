use time::Date;

pub fn get_date_fin_from_nb_mois(start_date: Date, duree_mois: u8) -> Option<Date> {
    let duree_mois = duree_mois - 1;
    let start_date_month = start_date.month().next();
    let start_date_year = start_date.year();
    let out_month = start_date_month.nth_next(duree_mois);
    let next_year = (duree_mois - (duree_mois % 12)) / 12;
    Date::from_calendar_date(start_date_year + next_year as i32, out_month, 1)
        .ok()?
        .previous_day()
}

#[cfg(test)]
mod tests {
    use super::get_date_fin_from_nb_mois;
    use time::macros::date;
    #[test]
    fn test_date_output() {
        let input_date = date!(2024 - 02 - 3);
        let output_date = date!(2024 - 04 - 30);
        assert_eq!(
            output_date,
            get_date_fin_from_nb_mois(input_date, 3).unwrap()
        )
    }
}
