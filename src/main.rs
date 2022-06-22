mod args;
mod data;
mod fetch;
mod public_holiday;

use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDate};
use public_holiday::PublicHoliday;
use structopt::StructOpt;

use args::Arguments;

use crate::data::DataCache;

const ITEMS: usize = 5;

fn main() -> Result<()> {
    let args = Arguments::from_args();
    let today = Local::today();
    let today = today.with_month(12).context("nope")?;
    let today = today.with_day(20).context("nope")?;
    let year = today.year();

    let mut cache = DataCache::init(args.locale.clone())?;

    let data = cache.read(year)?;
    let mut public_holidays = filter_items(data, today.naive_local(), ITEMS);

    // if the current year does not have enough public holidays left to display, look into the next year
    if public_holidays.len() < ITEMS {
        let mut next_years_cache = DataCache::init(args.locale)?;

        let mut next_years_public_holidays = filter_items(
            next_years_cache.read(year + 1)?,
            today.naive_local(),
            ITEMS - public_holidays.len(),
        );

        public_holidays.append(&mut next_years_public_holidays);
    }

    for ph in public_holidays {
        println!("{}", ph);
    }

    Ok(())
}

fn filter_items(
    public_holidays: Vec<PublicHoliday>,
    date: NaiveDate,
    items: usize,
) -> Vec<PublicHoliday> {
    public_holidays
        .into_iter()
        .filter(|ph| date <= ph.date)
        .take(items)
        .collect()
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use crate::{filter_items, public_holiday::PublicHoliday};

    #[test]
    fn test_filter_items() {
        let ph = vec![
            PublicHoliday::new(Local::today().naive_local(), String::from("one")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
        ];

        let phs = filter_items(ph, Local::today().pred().naive_local(), 5);
        assert_eq!(phs.len(), 2);
    }

    #[test]
    fn test_filter_items_filtered() {
        let ph = vec![
            PublicHoliday::new(Local::today().pred().naive_local(), String::from("one")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
        ];

        let phs = filter_items(ph, Local::today().naive_local(), 5);
        assert_eq!(phs.len(), 1);
    }

    #[test]
    fn test_filter_items_filtered_max_items() {
        let ph = vec![
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("one")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
            PublicHoliday::new(Local::today().succ().naive_local(), String::from("two")),
        ];

        let phs = filter_items(ph, Local::today().naive_local(), 5);
        assert_eq!(phs.len(), 5);
    }
}
