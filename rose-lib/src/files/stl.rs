//! ROSE Online String Table
use std::fmt;
use std::io::SeekFrom;
use std::str;
use std::str::FromStr;

use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};

/// String Table File
pub type STL = StringTable;

/// String Table Type
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StringTableType {
    Normal,
    Item,
    Quest,
}

impl Default for StringTableType {
    fn default() -> StringTableType {
        StringTableType::Normal
    }
}

impl fmt::Display for StringTableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringTableType::Normal => write!(f, "NRST01"),
            StringTableType::Item => write!(f, "ITST01"),
            StringTableType::Quest => write!(f, "QEST01"),
        }
    }
}

impl str::FromStr for StringTableType {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<StringTableType, Self::Err> {
        match s {
            "NRST01" => Ok(StringTableType::Normal),
            "ITST01" => Ok(StringTableType::Item),
            "QEST01" => Ok(StringTableType::Quest),
            _ => bail!("Unknown STL format identifier"),
        }
    }
}

/// String Table Key
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StringTableKey {
    id: u32,
    name: String,
}

// String Table Normal Row Data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NormalRowData {
    text: String,
}

/// String Table Item Row Data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ItemRowData {
    text: String,
    description: String,
}

/// String Table Quest Row Data
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct QuestRowData {
    text: String,
    description: String,
    start_message: String,
    end_message: String,
}

/// String Table Row
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StringTableRow {
    NormalRow(NormalRowData),
    ItemRow(ItemRowData),
    QuestRow(QuestRowData),
}

impl fmt::Display for StringTableRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringTableRow::NormalRow(data) => write!(f, "{}", data.text),
            StringTableRow::ItemRow(data) => write!(f, "{}, {}", data.text, data.description),
            StringTableRow::QuestRow(data) => write!(
                f,
                "{}, {}, {}, {}",
                data.text, data.description, data.start_message, data.end_message
            ),
        }
    }
}

/// String Table Language
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StringTableLanguage {
    Unknown = 999,
    Korean = 0,
    English = 1,
    Japanese = 2,
    ChineseTraditional = 3,
    ChineseSimplified = 4,
}

impl Default for StringTableLanguage {
    fn default() -> StringTableLanguage {
        StringTableLanguage::Unknown
    }
}

impl fmt::Display for StringTableLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringTableLanguage::Unknown => write!(f, "Unknown"),
            StringTableLanguage::Korean => write!(f, "Korean"),
            StringTableLanguage::English => write!(f, "English"),
            StringTableLanguage::Japanese => write!(f, "Japanese"),
            StringTableLanguage::ChineseTraditional => write!(f, "Chinese (Traditional)"),
            StringTableLanguage::ChineseSimplified => write!(f, "Chinese (Simplified)"),
        }
    }
}

impl From<u32> for StringTableLanguage {
    fn from(i: u32) -> StringTableLanguage {
        match i {
            0 => StringTableLanguage::Korean,
            1 => StringTableLanguage::English,
            2 => StringTableLanguage::Japanese,
            3 => StringTableLanguage::ChineseTraditional,
            4 => StringTableLanguage::ChineseSimplified,
            _ => StringTableLanguage::Unknown,
        }
    }
}

/// String Table Language Table
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StringTableLanguageTable {
    pub language: StringTableLanguage,
    pub rows: Vec<StringTableRow>,
}

/// String Table
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct StringTable {
    pub format: StringTableType,
    pub keys: Vec<StringTableKey>,
    pub language_tables: Vec<StringTableLanguageTable>,
}

impl StringTable {
    pub fn language_count(&self) -> usize {
        return self.language_tables.len();
    }

    pub fn row_count(&self) -> usize {
        if self.language_count() <= 0 {
            return 0;
        }
        self.language_tables[0].rows.len()
    }
}

impl RoseFile for StringTable {
    fn new() -> StringTable {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let identifier = reader.read_string_u8()?;
        self.format = StringTableType::from_str(&identifier)?;

        let row_count = reader.read_u32()?;
        for _ in 0..row_count {
            let mut key = StringTableKey::default();
            key.name = reader.read_string_u8()?;
            key.id = reader.read_u32()?;
            self.keys.push(key);
        }

        let language_count = reader.read_u32()?;
        for language_idx in 0..language_count {
            let language_offset = reader.read_u32()?;
            let next_language_offset = reader.position()?;

            reader.seek(SeekFrom::Start(language_offset as u64))?;

            let mut language_table = StringTableLanguageTable::default();
            language_table.language = StringTableLanguage::from(language_idx);

            for row_idx in 0..row_count {
                let row_offset = reader.read_u32()?;
                let next_row_offset = reader.position()?;

                reader.seek(SeekFrom::Start(row_offset as u64))?;

                match self.format {
                    StringTableType::Normal => {
                        let text = reader.read_string_varbyte()?;

                        let row = StringTableRow::NormalRow(NormalRowData { text });
                        language_table.rows.push(row);
                    }
                    StringTableType::Item => {
                        let text = reader.read_string_varbyte()?;
                        let description = reader.read_string_varbyte()?;

                        let row = StringTableRow::ItemRow(ItemRowData { text, description });
                        language_table.rows.push(row);
                    }
                    StringTableType::Quest => {
                        let text = reader.read_string_varbyte()?;
                        let description = reader.read_string_varbyte()?;
                        let start_message = reader.read_string_varbyte()?;
                        let end_message = reader.read_string_varbyte()?;

                        let row = StringTableRow::QuestRow(QuestRowData {
                            text,
                            description,
                            start_message,
                            end_message,
                        });
                        language_table.rows.push(row);
                    }
                }

                if row_idx < (row_count - 1) {
                    reader.seek(SeekFrom::Start(next_row_offset))?;
                }
            }

            self.language_tables.push(language_table);

            if language_idx < (language_count - 1) {
                reader.seek(SeekFrom::Start(next_language_offset))?;
            }
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_string_u8(&self.format.to_string())?;

        writer.write_u32(self.row_count() as u32)?;
        for key in &self.keys {
            writer.write_string_u8(&key.name)?;
            writer.write_u32(key.id)?;
        }

        writer.write_u32(self.language_count() as u32)?;

        let language_offsets_position = writer.position()?;
        for _ in 0..self.language_count() {
            // Temporary language offsets to be updated later
            writer.write_u32(0)?;
        }

        let mut language_offsets = Vec::new();
        for language_idx in 0..self.language_count() {
            let row_offsets_position = writer.position()?;
            language_offsets.push(row_offsets_position);

            for _ in 0..self.row_count() {
                // Temporary row offsets to be updated later
                writer.write_u32(0)?;
            }

            let mut row_offsets = Vec::new();
            for row_idx in 0..self.row_count() {
                row_offsets.push(writer.position()?);

                let language_table = &self.language_tables[language_idx];
                match &language_table.rows[row_idx] {
                    StringTableRow::NormalRow(data) => {
                        writer.write_string_varbyte(&data.text)?;
                    }
                    StringTableRow::ItemRow(data) => {
                        writer.write_string_varbyte(&data.text)?;
                        writer.write_string_varbyte(&data.description)?;
                    }
                    StringTableRow::QuestRow(data) => {
                        writer.write_string_varbyte(&data.text)?;
                        writer.write_string_varbyte(&data.description)?;
                        writer.write_string_varbyte(&data.start_message)?;
                        writer.write_string_varbyte(&data.end_message)?;
                    }
                }
            }

            let position = writer.position()?;

            // Jump to the row offset section to write our row offsets
            writer.seek(SeekFrom::Start(row_offsets_position))?;
            for row_offset in row_offsets {
                writer.write_u32(row_offset as u32)?;
            }

            // Jump back to where we were writing
            writer.seek(SeekFrom::Start(position))?;
        }

        // Jump to our language offsets section and update our offsets
        writer.seek(SeekFrom::Start(language_offsets_position))?;
        for language_offset in language_offsets {
            writer.write_u32(language_offset as u32)?;
        }

        Ok(())
    }
}
