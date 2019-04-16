//! ROSE Online Heightmaps
use failure::Error;
use io::{ReadRoseExt, RoseFile, WriteRoseExt};
use std::f32;

/// Heightmap File
pub type HIM = Heightmap;

/// Heightmap
#[derive(Debug, Serialize, Deserialize)]
pub struct Heightmap {
    pub width: i32,
    pub height: i32,
    pub grid_count: i32,
    pub scale: f32,

    pub heights: Vec<Vec<f32>>,

    pub min_height: f32,
    pub max_height: f32,
}

impl RoseFile for Heightmap {
    fn new() -> Heightmap {
        Heightmap {
            width: 0,
            height: 0,
            grid_count: 0,
            scale: 0.0,
            heights: Vec::new(),

            min_height: f32::NAN,
            max_height: f32::NAN,
        }
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.width = reader.read_i32()?;
        self.height = reader.read_i32()?;
        self.grid_count = reader.read_i32()?;
        self.scale = reader.read_f32()?;

        self.heights = vec![vec![0.0; self.width as usize]; self.height as usize];
        for h in 0..self.height {
            for w in 0..self.width {
                let height = reader.read_f32()?;

                self.heights[h as usize][w as usize] = height;

                if self.min_height.is_nan() || height < self.min_height {
                    self.min_height = height;
                }

                if self.max_height.is_nan() || height > self.max_height {
                    self.max_height = height;
                }
            }
        }

        // TODO: File contains more data

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, _writer: &mut W) -> Result<(), Error> {
        // TODO: Implement writer
        unimplemented!();
    }
}
