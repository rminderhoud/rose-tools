//! ROSE Online Heightmaps
use std::f32;

use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};

/// Heightmap File
pub type HIM = Heightmap;

/// Heightmap
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Heightmap {
    pub width: i32,
    pub length: i32,
    pub grid_count: i32,
    pub scale: f32,

    pub heights: Vec<f32>,

    pub min_height: f32,
    pub max_height: f32,
}

impl Heightmap {
    pub fn height(&self, x: usize, y: usize) -> f32 {
        let width = self.width as usize;
        let length = self.length as usize;
        let index = (y * length) + x;
        if x > width || y > length || index > self.heights.len() {
            return 0.0;
        }
        return self.heights[index];
    }
}

impl RoseFile for Heightmap {
    fn new() -> Heightmap {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.width = reader.read_i32()?;
        self.length = reader.read_i32()?;
        self.grid_count = reader.read_i32()?;
        self.scale = reader.read_f32()?;

        self.heights = Vec::with_capacity((self.width * self.length) as usize);
        for _ in 0..self.length {
            for _ in 0..self.width {
                let height = reader.read_f32()?;

                self.heights.push(height);

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
