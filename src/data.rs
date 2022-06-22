use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde_json::{de, ser};

use crate::fetch;
use crate::public_holiday::PublicHoliday;

pub struct DataCache {
    locale: String,
    storage_path: PathBuf,
}

impl DataCache {
    // init instead of new, because it is fallible
    pub fn init(locale: String) -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "example", "holidays")
            .context("failed to access data storage")?;
        let cache_dir = project_dirs.cache_dir();
        let storage_path = cache_dir.join(&locale);

        fs::create_dir_all(&storage_path)?;

        Ok(Self {
            locale,
            storage_path,
        })
    }

    fn file_path(&self, year: i32) -> PathBuf {
        self.storage_path.join(year.to_string())
    }

    fn data_exists(&self, year: i32) -> bool {
        self.storage_path.join(year.to_string()).exists()
    }

    fn save(&mut self, year: i32, public_holidays: Vec<PublicHoliday>) -> Result<()> {
        let data = ser::to_string(&public_holidays)?;

        let mut file = self.open_file_read_only(false, year)?;
        file.write_all(data.as_bytes())
            .context("could not save data in storage")?;

        Ok(())
    }

    pub fn read(&mut self, year: i32) -> Result<Vec<PublicHoliday>> {
        if !self.data_exists(year) {
            let result = fetch::public_holidays(year, &self.locale)?;
            self.save(year, result)?;
        }

        let file = self.open_file_read_only(true, year)?;
        de::from_reader(file).context("could not read data storage")
    }

    fn open_file_read_only(&self, read_only: bool, year: i32) -> Result<File> {
        OpenOptions::new()
            .read(read_only)
            .write(!read_only)
            .create(!read_only)
            .open(self.file_path(year))
            .context("could not access data storage")
    }
}
