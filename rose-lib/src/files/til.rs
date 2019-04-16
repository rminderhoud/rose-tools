//! ROSE Online Terrain Tilemap
use failure::Error;
use io::{ReadRoseExt, RoseFile, WriteRoseExt};
use std::iter;

/// Tile file
pub type TIL = Tilemap;

/// Tile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub brush_id: u8,
    pub tile_idx: u8,
    pub tile_set: u8,
    pub tile_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tilemap {
    pub width: i32,
    pub height: i32,

    pub tiles: Vec<Vec<Tile>>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            brush_id: 0,
            tile_idx: 0,
            tile_set: 0,
            tile_id: 0,
        }
    }
}

impl RoseFile for Tilemap {
    fn new() -> Tilemap {
        Tilemap {
            width: 0,
            height: 0,
            tiles: Vec::new(),
        }
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
