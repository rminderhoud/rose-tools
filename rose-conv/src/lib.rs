use std::str::FromStr;

use failure::{bail, Error};

use roselib::files::stl::*;
use roselib::files::*;
use roselib::io::RoseFile;

pub trait ToCsv {
    fn to_csv(&self) -> Result<String, Error>;
}

impl ToCsv for STB {
    fn to_csv(&self) -> Result<String, Error> {
        let mut writer = csv::Writer::from_writer(Vec::new());

        writer.write_record(&self.headers)?;
        for row in &self.data {
            writer.write_record(row)?;
        }

        let data = String::from_utf8(writer.into_inner()?)?;
        return Ok(data);
    }
}

impl ToCsv for STL {
    fn to_csv(&self) -> Result<String, Error> {
        let mut writer = csv::Writer::from_writer(Vec::new());

        let mut headers = Vec::new();
        let mut headers2 = Vec::new();

        headers.push(self.format.to_string());
        headers2.push("Row ID");

        // Second and third column contain our keys
        headers.push(String::new());
        headers.push(String::new());
        headers2.push("Key ID");
        headers2.push("Key Name");

        for table in &self.language_tables {
            headers.push(table.language.to_string());
            headers2.push("Text");

            if self.format == StringTableType::Item || self.format == StringTableType::Quest {
                headers.push(String::new());
                headers2.push("Description");
            }

            if self.format == StringTableType::Quest {
                headers.push(String::new());
                headers.push(String::new());
                headers2.push("Start Message");
                headers2.push("End Message");
            }
        }

        writer.write_record(&headers)?;
        writer.write_record(&headers2)?;

        for row_idx in 0..self.row_count() {
            let mut row = Vec::new();
            row.push(row_idx.to_string());

            let key = &self.keys[row_idx];
            row.push(key.id.to_string());
            row.push(key.name.clone());

            for table in &self.language_tables {
                let table_row = &table.rows[row_idx];
                match table_row {
                    StringTableRow::NormalRow(data) => row.push(data.text.clone()),
                    StringTableRow::ItemRow(data) => {
                        row.push(data.text.clone());
                        row.push(data.description.clone());
                    }
                    StringTableRow::QuestRow(data) => {
                        row.push(data.text.clone());
                        row.push(data.description.clone());
                        row.push(data.start_message.clone());
                        row.push(data.end_message.clone());
                    }
                }
            }

            writer.write_record(&row)?;
        }

        let data = String::from_utf8(writer.into_inner()?)?;
        return Ok(data);
    }
}

pub trait FromCsv {
    fn from_csv(s: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized;
}

impl FromCsv for STB {
    fn from_csv(s: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let mut stb = STB::new();

        let mut reader = csv::Reader::from_reader(s.as_bytes());
        for header in reader.headers()? {
            stb.headers.push(header.to_string())
        }

        for record in reader.records() {
            let mut row = Vec::new();
            for field in record?.iter() {
                row.push(field.to_string());
            }
            stb.data.push(row);
        }

        return Ok(stb);
    }
}

impl FromCsv for STL {
    fn from_csv(s: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        let mut stl = STL::new();
        let mut reader = csv::Reader::from_reader(s.as_bytes());
        stl.format = StringTableType::from_str(reader.headers()?.get(0).unwrap_or_default())?;

        let headers: Vec<&str> = reader.headers()?.iter().collect();

        // Get the number of columns after key columns
        let data_col_count = headers.len() - 3;
        if data_col_count <= 0 {
            bail!(
                "Invalid number of headers in the CSV file, expected at least 4, found {}",
                headers.len()
            );
        }

        let language_count = match stl.format {
            StringTableType::Normal => data_col_count,
            StringTableType::Item => data_col_count / 2,
            StringTableType::Quest => data_col_count / 4,
        };

        for language_idx in 0..language_count {
            stl.language_tables.push(StringTableLanguageTable {
                language: StringTableLanguage::from(language_idx as u32),
                rows: Vec::new(),
            });
        }

        // Skip the second header line
        let records = reader.records().skip(1);

        let mut record_idx = 0;
        for record in records {
            record_idx += 1;

            let record = record?;
            let fields: Vec<&str> = record.iter().collect();
            if fields.len() < 3 {
                bail!(
                    "STL row #{}, expected at least 4 columns, found {}.",
                    record_idx,
                    fields.len()
                );
            }

            // Ignore first column, it's only written for readability of the csv

            // Read key values from 2nd and 3rd column
            stl.keys.push(StringTableKey {
                id: str::parse(fields[1])?,
                name: fields[2].into(),
            });

            // Read tables from remaining columns depending on STL format
            match &stl.format {
                StringTableType::Normal => {
                    // Every column after 2 is a row from a different normal table
                    for idx in 3..fields.len() {
                        let row = StringTableRow::NormalRow(NormalRowData {
                            text: fields[idx].into(),
                        });
                        let language_idx = idx - 3;
                        stl.language_tables[language_idx].rows.push(row);
                    }
                }
                StringTableType::Item => {
                    // Every 2 columns after 2 is a row from a different item table
                    for idx in (3..fields.len()).step_by(2) {
                        let row = StringTableRow::ItemRow(ItemRowData {
                            text: fields[idx].into(),
                            description: fields[idx + 1].into(),
                        });
                        let language_idx = (idx - 3) / 2;
                        stl.language_tables[language_idx].rows.push(row);
                    }
                }
                StringTableType::Quest => {
                    // Every 2 columns after 2 is a row from a different item table
                    for idx in (3..fields.len()).step_by(4) {
                        let row = StringTableRow::QuestRow(QuestRowData {
                            text: fields[idx].into(),
                            description: fields[idx + 1].into(),
                            start_message: fields[idx + 2].into(),
                            end_message: fields[idx + 3].into(),
                        });
                        let language_idx = (idx - 3) / 4;
                        stl.language_tables[language_idx].rows.push(row);
                    }
                }
            }
        }
        Ok(stl)
    }
}

pub trait ToJson {
    fn to_json(&self) -> Result<String, Error>
    where
        Self: serde::ser::Serialize,
    {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

impl<F> ToJson for F where F: RoseFile {}

pub trait FromJson {
    fn from_json(s: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized + serde::de::DeserializeOwned,
    {
        Ok(serde_json::from_reader(s.as_bytes())?)
    }
}

impl<F> FromJson for F where F: RoseFile {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    macro_rules! test_csv {
        ($filetype: ident, $path: expr) => {{
            let orig_file = $filetype::from_path(&$path).unwrap();
            let json_string = orig_file.to_csv().unwrap();
            let new_file = $filetype::from_csv(&json_string).unwrap();
            assert_eq!(orig_file, new_file);
        }};
    }

    macro_rules! test_json {
        ($filetype: ident, $path: expr) => {{
            let orig_file = $filetype::from_path(&$path).unwrap();
            let json_string = orig_file.to_json().unwrap();
            let new_file = $filetype::from_json(&json_string).unwrap();
            assert_eq!(orig_file, new_file);
        }};
    }

    #[test]
    fn test_csv() {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.pop();
        root.push("rose-lib");
        root.push("tests");
        root.push("data");

        test_csv!(STB, root.join("list_zone.stb"));
        test_csv!(STL, root.join("str_itemtype.stl"));
        test_csv!(STL, root.join("list_faceitem_s.stl"));
        test_csv!(STL, root.join("list_quest_s.stl"));
    }

    #[test]
    fn test_json() {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.pop();
        root.push("rose-lib");
        root.push("tests");
        root.push("data");

        test_json!(IDX, root.join("data.idx"));
        test_json!(LIT, root.join("OBJECTLIGHTMAPDATA.LIT"));
        test_json!(ZSC, root.join("list_weapon.zsc"));
        test_json!(ZSC, root.join("part_npc.zsc"));
    }
}
