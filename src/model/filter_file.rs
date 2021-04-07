use crate::model::ProgramFilter;
use crate::Error;

use std::convert::TryInto;
use std::fs::OpenOptions;
use std::path::Path;

use csv::{ReaderBuilder, Writer};

impl ProgramFilter {
    /// Write the filters from a file at the given path.
    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut writer = Writer::from_writer(file);

        let content = <Vec<[String; 2]>>::from(self.clone());

        for c in content {
            writer.write_record(&c)?;
        }

        writer.flush()?;

        Ok(())
    }

    /// Read the filters from a file at the given path.
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = OpenOptions::new().read(true).open(path)?;

        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

        let mut content: Vec<[String; 2]> = vec![];

        for record_res in reader.records() {
            let record = record_res?;

            if record.len() != 2 {
                return Err(Error::ParsingFile);
            }

            let record_arr = [
                record.get(0).unwrap().to_string(),
                record.get(1).unwrap().to_string(),
            ];

            content.push(record_arr);
        }

        content.try_into().map_err(|_| Error::ParsingFile)
    }
}
