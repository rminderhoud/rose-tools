//! ROSE Online Lightmaps
//!
//! ROSE Online uses pre-baked lights that get rendered to a lightmap texture
//! for blending with terrain/object textures.
//!
use failure::Error;
use io::{RoseFile, ReadRoseExt, WriteRoseExt};

/// Lightmap file
pub type LIT = Lightmap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lightmap {
    pub objects: Vec<LightmapObject>,
    pub filenames: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapObject {
    pub id: i32,
    pub parts: Vec<LightmapPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightmapPart {
    pub name: String,
    pub id: i32,
    pub filename: String,
    pub lightmap_index: i32,
    pub pixels_per_part: i32,
    pub parts_per_width: i32,
    pub part_position: i32,
}

impl RoseFile for Lightmap {
    fn new() -> Lightmap {
        Lightmap {
            objects: Vec::new(),
            filenames: Vec::new(),
        }
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let object_count = reader.read_i32()?;

        for _ in 0..object_count {
            let mut object = LightmapObject::new();

            let part_count = reader.read_i32()?;
            object.id = reader.read_i32()?;

            for _ in 0..part_count {
                let mut part = LightmapPart::new();
                part.name = reader.read_string_u8()?;
                part.id = reader.read_i32()?;
                part.filename = reader.read_string_u8()?;
                part.lightmap_index = reader.read_i32()?;
                part.pixels_per_part = reader.read_i32()?;
                part.parts_per_width = reader.read_i32()?;
                part.part_position = reader.read_i32()?;

                object.parts.push(part);
            }

            self.objects.push(object);
        }

        let file_count = reader.read_i32()?;

        for _ in 0..file_count {
            self.filenames.push(reader.read_string_u8()?);
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_i32(self.objects.len() as i32)?;

        for ref object in &self.objects {
            writer.write_i32(object.parts.len() as i32)?;
            writer.write_i32(object.id)?;

            for ref part in &object.parts {
                writer.write_string_u8(&part.name)?;
                writer.write_i32(part.id)?;
                writer.write_string_u8(&part.filename)?;
                writer.write_i32(part.lightmap_index)?;
                writer.write_i32(part.pixels_per_part)?;
                writer.write_i32(part.parts_per_width)?;
                writer.write_i32(part.part_position)?;
            }
        }

        writer.write_i32(self.filenames.len() as i32)?;

        for ref filename in &self.filenames {
            writer.write_string_u8(&filename)?;
        }

        Ok(())
    }
}

impl LightmapObject {
    pub fn new() -> LightmapObject {
        LightmapObject {
            id: -1,
            parts: Vec::new(),
        }
    }
}

impl LightmapPart {
    pub fn new() -> LightmapPart {
        LightmapPart {
            name: String::new(),
            id: -1,
            filename: String::new(),
            lightmap_index: -1,
            pixels_per_part: 0,
            parts_per_width: 0,
            part_position: -1,
        }
    }
}
