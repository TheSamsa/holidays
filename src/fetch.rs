use crate::public_holiday::PublicHoliday;
use reqwest::{blocking, Error};

const URL: &str = "https://date.nager.at/api/v3/PublicHolidays";

pub fn public_holidays(year: i32, locale: &String) -> Result<Vec<PublicHoliday>, Error> {
    let url = format!("{}/{}/{}", URL, year, locale);
    blocking::get(url)?.json()
}
