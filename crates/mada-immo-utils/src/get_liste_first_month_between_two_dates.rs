use time::{Date, Month};

fn get_start_month(date: Date) -> Option<Date> {
    Date::from_calendar_date(date.year(), date.month(), 1).ok()
}

/// BUG this shit won't on january to february
fn get_next_month(date: Date) -> Option<Date> {
    let year = if date.month() == Month::December {
        date.year() + 1
    } else {
        date.year()
    };
    Date::from_calendar_date(year, date.month().next(), date.day()).ok()
}

pub fn get_liste_first_month_between_two_dates(start_date: Date, end_date: Date) -> Vec<Date> {
    if start_date > end_date {
        Default::default()
    } else {
        let mut dates = Vec::new();
        let Some(mut start) = get_start_month(start_date) else {
            return Default::default();
        };
        dates.push(start);
        while let Some(next) = get_next_month(start).filter(|next| next < &end_date) {
            dates.push(next);
            start = next;
        }
        dates
    }
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use crate::get_liste_first_month_between_two_dates::{
        get_liste_first_month_between_two_dates, get_start_month,
    };
    #[test]
    fn test_start_month() {
        let input_date = date!(2024 - 02 - 3);
        let output_date = date!(2024 - 02 - 01);
        assert_eq!(get_start_month(input_date), Some(output_date))
    }
    #[test]
    fn test_glfmbtd_normal() {
        let input_date = date!(2024 - 02 - 3);
        let output_date = date!(2024 - 04 - 30);
        let res = get_liste_first_month_between_two_dates(input_date, output_date);
        assert_eq!(res.len(), 3);
        assert_eq!(
            &res,
            &[
                date!(2024 - 02 - 01),
                date!(2024 - 03 - 01),
                date!(2024 - 04 - 01)
            ]
        )
    }
    #[test]
    fn test_glfmbtd_year_outer() {
        let input_date = date!(2024 - 11 - 3);
        let output_date = date!(2025 - 2 - 28);
        let res = get_liste_first_month_between_two_dates(input_date, output_date);
        assert_eq!(res.len(), 4);
        assert_eq!(
            &res,
            &[
                date!(2024 - 11 - 01),
                date!(2024 - 12 - 01),
                date!(2025 - 01 - 01),
                date!(2025 - 02 - 01),
            ]
        )
    }
}
