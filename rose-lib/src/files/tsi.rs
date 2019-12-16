//! ROSE Online Sprite Information
use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{PathRoseExt, ReadRoseExt, RoseFile, WriteRoseExt};
use crate::utils::Vector2;

/// Sprite Information File
pub type TSI = SpriteInformation;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpriteInformation {
    pub sprite_sheets: Vec<SpriteSheet>,
}

impl SpriteInformation {
    pub fn total_sprites(&self) -> usize {
        let mut total = 0;
        for sheet in self.sprite_sheets.iter() {
            total += sheet.sprites.len();
        }

        total
    }
}

impl RoseFile for SpriteInformation {
    fn new() -> SpriteInformation {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let sheet_count = reader.read_u16()?;
        for _ in 0..sheet_count {
            let mut sheet = SpriteSheet::new();
            sheet.path = PathBuf::from(reader.read_string_u16()?);
            sheet.color_key = reader.read_u32()?;

            self.sprite_sheets.push(sheet);
        }

        let _total_sprite_count = reader.read_u16()?;
        for sheet_idx in 0..sheet_count {
            let sheet_idx = sheet_idx as usize;

            let sprite_count = reader.read_u16()?;
            for _ in 0..sprite_count {
                let _sheet_id = reader.read_u16()?;

                let mut sprite = Sprite::new();
                sprite.start_point = reader.read_vector2_u32()?;
                sprite.end_point = reader.read_vector2_u32()?;
                sprite.color = reader.read_u32()?;
                sprite.name = reader.read_string(32)?;

                self.sprite_sheets[sheet_idx].sprites.push(sprite);
            }
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        let sheet_count = self.sprite_sheets.len();

        writer.write_u16(u16::try_from(sheet_count)?)?;

        for sheet in self.sprite_sheets.iter() {
            writer.write_string_u16(&sheet.path.to_rose_path())?;
            writer.write_u32(sheet.color_key)?;
        }

        writer.write_u16(u16::try_from(self.total_sprites())?)?;
        for (sheet_idx, sheet) in self.sprite_sheets.iter().enumerate() {
            writer.write_u16(u16::try_from(sheet.sprites.len())?)?;

            for sprite in sheet.sprites.iter() {
                writer.write_u16(u16::try_from(sheet_idx)?)?;
                writer.write_vector2_u32(&sprite.start_point)?;
                writer.write_vector2_u32(&sprite.end_point)?;
                writer.write_u32(sprite.color)?;
                writer.write_string(&sprite.name, 32)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub path: PathBuf,
    pub color_key: u32,
    pub sprites: Vec<Sprite>,
}

impl SpriteSheet {
    pub fn new() -> SpriteSheet {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub start_point: Vector2<u32>,
    pub end_point: Vector2<u32>,
    pub color: u32,
}

impl Sprite {
    pub fn new() -> Sprite {
        Self::default()
    }
}
