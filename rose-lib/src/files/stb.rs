//! ROSE Online Data Table
use failure::Error;
use io::{ReadRoseExt, RoseFile, WriteRoseExt};
use std::io::SeekFrom;

/// Data File
pub type STB = DataTable;

/// Data Table
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DataTable {
    pub identifier: String,
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl RoseFile for DataTable {
    fn new() -> DataTable {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.identifier = reader.read_string(4)?;

        let offset = reader.read_u32()?;
        let row_count = reader.read_u32()?;
        let col_count = reader.read_u32()?;

        let _row_height = reader.read_u32()?;

        let _root_col_width = reader.read_u16()?;
        for _ in 0..col_count {
            let _col_width = reader.read_u16()?;
        }

        let root_col_name = reader.read_string_u16()?;
        self.headers.push(root_col_name);

        for _ in 0..col_count - 1 {
            self.headers.push(reader.read_string_u16()?);
        }

        // Unknown string
        let _ = reader.read_string_u16()?;

        for _ in 0..row_count - 1 {
            let mut row: Vec<String> = Vec::new();
            row.push(reader.read_string_u16()?);
            self.data.push(row);
        }

        reader.seek(SeekFrom::Start(u64::from(offset)))?;

        for i in 0..row_count - 1 {
            for _ in 0..col_count - 1 {
                self.data[i as usize].push(reader.read_string_u16()?);
            }
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_string(&self.identifier, 4)?;

        // Write temporary offset
        writer.write_u32(0)?;

        writer.write_u32((self.data.len() + 1) as u32)?;
        writer.write_u32(self.headers.len() as u32)?;

        // Row height
        writer.write_u32(0)?;

        // Root column width
        writer.write_u16(0)?;
        for _ in 0..self.headers.len() {
            // Column width
            writer.write_u16(0)?;
        }

        for header in &self.headers {
            // Column names
            writer.write_string_u16(&header)?;
        }

        // Unknown string
        writer.write_string_u16("")?;

        for row in &self.data {
            writer.write_string_u16(&row[0])?;
        }

        let offset = writer.seek(SeekFrom::Current(0))?;

        for row in &self.data {
            /*
            for i in 1..row.len() {
                writer.write_string_u16(&row[i])?;
            }*/
            for cell in row.iter().skip(1) {
                writer.write_string_u16(cell)?;
            }
        }

        writer.seek(SeekFrom::Start(4))?;
        writer.write_u32(offset as u32)?;

        Ok(())
    }
}
