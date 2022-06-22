use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum HolidayType {
    Public,
    Bank,
    School,
    Authorities,
    Optional,
    Observance,
}

impl Display for HolidayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicHoliday {
    pub date: NaiveDate,

    #[serde(rename = "localName")]
    local_name: String,

    name: String,

    #[serde(rename = "countryCode")]
    country_code: String, // later enum?

    fixed: bool,

    global: bool,

    counties: Option<Vec<String>>,

    #[serde(rename = "launchYear")]
    launch_year: Option<usize>,

    types: Vec<HolidayType>,
}

impl PublicHoliday {
    #[allow(dead_code)]
    pub fn new(date: NaiveDate, name: String) -> Self {
        Self {
            date,
            local_name: name.clone(),
            name,
            country_code: String::new(),
            fixed: false,
            global: false,
            counties: None,
            launch_year: None,
            types: vec![],
        }
    }
}

impl Display for PublicHoliday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_line = format!("- {} - {} ({})", self.date, self.local_name, self.name);
        // use debug formatter since its less hassle
        let counties = match &self.counties {
            Some(counties) => Some(format!("{:?}", counties)),
            None => None,
        };
        // use debug formatter since its less hassle
        let types = format!("{:?}", self.types);

        if let Some(counties) = counties {
            write!(
                f,
                "{}\n  types: {}\n  counties: {}",
                first_line, types, counties
            )
        } else {
            write!(f, "{}\n  types: {}", first_line, types)
        }
    }
}
