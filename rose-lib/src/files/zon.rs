//! ROSE Online Zone
use std::io::SeekFrom;
use std::iter;

use failure::Error;
use num;
use num::FromPrimitive as num_from;
use io::{RoseFile, ReadRoseExt, WriteRoseExt};
use utils::{Vector2, Vector3};


/// Zone File
pub type ZON = Zone;

/// Zone Type
#[derive(Debug, Serialize, Deserialize, FromPrimitive, PartialEq)]
pub enum ZoneType {
    Grass = 0,
    Mountain = 1,
    MountainVillage = 2,
    BoatVillage = 3,
    Login = 4,
    MountainGorge = 5,
    Beach = 6,
    JunonDungeon = 7,
    LunaSnow = 8,
    Birth = 9,
    JunonField = 10,
    LunaDungeon = 11,
    EldeonField = 12,
    EldeonField2 = 13,
    JunonPyramids = 14,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, PartialEq)]
pub enum ZoneBlockType {
    BasicInfo = 0,
    EventPoints = 1,
    Textures = 2,
    Tiles = 3,
    Economy = 4,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, PartialEq)]
pub enum ZoneTileRotation {
    Unkown = 0,
    None = 1,
    FlipHorizontal = 2,
    FlipVertical = 3,
    Flip = 4,
    Clockwise90 = 5,
    CounterClockwise90 = 6,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ZonePosition {
    pub position: Vector2<f32>,
    pub is_used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneEventPoint {
    pub position: Vector3<f32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneTile {
    pub layer1: i32,
    pub layer2: i32,
    pub offset1: i32,
    pub offset2: i32,
    pub blend: bool,
    pub rotation: ZoneTileRotation,
    pub tile_type: i32,
}

/// Zone
#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub width: i32,
    pub height: i32,
    pub grid_count: i32,
    pub grid_size: f32,
    pub start_position: Vector2<i32>,
    pub positions: Vec<Vec<ZonePosition>>,
    pub event_points: Vec<ZoneEventPoint>,
    pub textures: Vec<String>,
    pub tiles: Vec<ZoneTile>,
    pub name: String,
    pub is_underground: bool,
    pub background_music: String,
    pub sky: String,
    pub economy_tick_rate: i32,
    pub population_base: i32,
    pub population_growth_rate: i32,
    pub metal_consumption: i32,
    pub stone_consumption: i32,
    pub wood_consumption: i32,
    pub leather_consumption: i32,
    pub cloth_consumption: i32,
    pub alchemy_consumption: i32,
    pub chemical_consumption: i32,
    pub medicine_consumption: i32,
    pub food_consumption: i32,
}

impl ZonePosition {
    fn new() -> ZonePosition {
        ZonePosition {
            position: Vector2::<f32>::new(),
            is_used: false,
        }
    }
}

impl ZoneEventPoint {
    fn new() -> ZoneEventPoint {
        ZoneEventPoint {
            position: Vector3::<f32>::new(),
            name: String::new(),
        }
    }
}

impl ZoneTile {
    fn new() -> ZoneTile {
        ZoneTile {
            layer1: -1,
            layer2: -1,
            offset1: -1,
            offset2: -1,
            blend: false,
            rotation: ZoneTileRotation::None,
            tile_type: -1,
        }
    }
}
impl RoseFile for Zone {
    fn new() -> Zone {
        Zone {
            zone_type: ZoneType::Grass,
            width: 0,
            height: 0,
            grid_count: 0,
            grid_size: 0.0,
            start_position: Vector2::<i32>::new(),
            positions: Vec::new(),
            event_points: Vec::new(),
            textures: Vec::new(),
            tiles: Vec::new(),
            name: String::new(),
            is_underground: false,
            background_music: String::new(),
            sky: String::new(),
            economy_tick_rate: 0,
            population_base: 0,
            population_growth_rate: 0,
            metal_consumption: 0,
            stone_consumption: 0,
            wood_consumption: 0,
            leather_consumption: 0,
            cloth_consumption: 0,
            alchemy_consumption: 0,
            chemical_consumption: 0,
            medicine_consumption: 0,
            food_consumption: 0,
        }
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let block_count = reader.read_i32()?;

        // Zone block type/offset pairs
        let mut blocks: Vec<(i32, i32)> = Vec::new();
        for _ in 0..block_count {
            let block_type = reader.read_i32()?;
            let offset = reader.read_i32()?;
            blocks.push((block_type, offset));
        }

        for block in blocks {
            let block_type = num_from::from_i32(block.0);
            let block_offset = block.1;

            if block_type.is_none() {
                bail!("Invalid block type: {}", block.0);
            }

            reader.seek(SeekFrom::Start(block_offset as u64))?;

            match block_type.unwrap() {
                ZoneBlockType::BasicInfo => {
                    self.zone_type = num_from::from_i32(reader.read_i32()?).unwrap();
                    self.width = reader.read_i32()?;
                    self.height = reader.read_i32()?;
                    self.grid_count = reader.read_i32()?;
                    self.grid_size = reader.read_f32()?;
                    self.start_position = reader.read_vector2_i32()?;

                    for _ in 0..self.height {
                        let row = iter::repeat(ZonePosition::new())
                            .take(self.width as usize)
                            .collect();
                        self.positions.push(row);
                    }

                    for w in 0..self.width {
                        for h in 0..self.height {
                            let mut pos = ZonePosition::new();
                            pos.is_used = reader.read_bool()?;
                            pos.position = reader.read_vector2_f32()?;
                            self.positions[h as usize][w as usize] = pos;
                        }
                    }
                },
                ZoneBlockType::EventPoints => {
                    let count = reader.read_i32()?;
                    for _ in 0..count {
                        let mut p = ZoneEventPoint::new();
                        p.position = reader.read_vector3_f32()?;
                        p.name = reader.read_string_u8()?;
                        self.event_points.push(p);
                    }
                },
                ZoneBlockType::Textures => {
                    let count = reader.read_i32()?;
                    for _ in 0..count {
                        self.textures.push(reader.read_string_u8()?);
                    }
                },
                ZoneBlockType::Tiles => {
                    let count = reader.read_i32()?;
                    for _ in 0..count {
                        let mut t = ZoneTile::new();
                        t.layer1 = reader.read_i32()?;
                        t.layer2 = reader.read_i32()?;
                        t.offset1 = reader.read_i32()?;
                        t.offset2 = reader.read_i32()?;
                        t.blend = reader.read_i32()? != 0;
                        t.rotation = num_from::from_i32(reader.read_i32()?).unwrap();
                        t.tile_type = reader.read_i32()?;
                        self.tiles.push(t);
                    }
                },
                ZoneBlockType::Economy => {
                    self.name = reader.read_string_u8()?;
                    self.is_underground = reader.read_i32()? != 0;
                    self.background_music = reader.read_string_u8()?;
                    self.sky = reader.read_string_u8()?;
                    self.economy_tick_rate = reader.read_i32()?;
                    self.population_base = reader.read_i32()?;
                    self.population_growth_rate = reader.read_i32()?;
                    self.metal_consumption = reader.read_i32()?;
                    self.stone_consumption = reader.read_i32()?;
                    self.wood_consumption = reader.read_i32()?;
                    self.leather_consumption = reader.read_i32()?;
                    self.cloth_consumption = reader.read_i32()?;
                    self.alchemy_consumption = reader.read_i32()?;
                    self.chemical_consumption = reader.read_i32()?;
                    self.medicine_consumption = reader.read_i32()?;
                    self.food_consumption = reader.read_i32()?;
                },
            }
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        unimplemented!();
    }
}
