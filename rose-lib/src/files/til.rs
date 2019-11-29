//! ROSE Online Terrain Tilemap
use std::iter;

use failure::Error;

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};

/// Tile file
pub type TIL = Tilemap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tilemap {
    pub width: i32,
    pub height: i32,

    pub tiles: Vec<Vec<Tile>>,
}

impl RoseFile for Tilemap {
    fn new() -> Tilemap {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.width = reader.read_i32()?;
        self.height = reader.read_i32()?;

        self.tiles.resize(
            self.width as usize,
            iter::repeat(Tile::new())
                .take(self.width as usize)
                .collect(),
        );

        for h in 0..self.height {
            for w in 0..self.width {
                let mut t = Tile::new();
                t.brush_id = reader.read_u8()?;
                t.tile_idx = reader.read_u8()?;
                t.tile_set = reader.read_u8()?;
                t.tile_id = reader.read_i32()?;

                self.tiles[h as usize][w as usize] = t;
            }
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, _writer: &mut W) -> Result<(), Error> {
        unimplemented!();
    }
}

/// Tile
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Tile {
    pub brush_id: u8,
    pub tile_idx: u8,
    pub tile_set: u8,
    pub tile_id: i32,
}

impl Tile {
    fn new() -> Tile {
        Self::default()
    }
}
