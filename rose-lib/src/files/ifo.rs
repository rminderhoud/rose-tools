//! ROSE Online Map Data
use std::convert::TryFrom;
use std::io::SeekFrom;

use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};
use crate::utils::{Quaternion, Vector2, Vector3};

/// Map Data File
pub type IFO = MapData;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MapDataBlockType {
    MapInfo = 0,
    Object = 1,
    Npc = 2,
    Building = 3,
    Sound = 4,
    Effect = 5,
    Animation = 6,
    Water = 7,
    MonsterSpawn = 8,
    Ocean = 9,
    Warp = 10,
    CollisionObject = 11,
    EventObject = 12,
}

impl TryFrom<u32> for MapDataBlockType {
    type Error = failure::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MapDataBlockType::MapInfo),
            1 => Ok(MapDataBlockType::Object),
            2 => Ok(MapDataBlockType::Npc),
            3 => Ok(MapDataBlockType::Building),
            4 => Ok(MapDataBlockType::Sound),
            5 => Ok(MapDataBlockType::Effect),
            6 => Ok(MapDataBlockType::Animation),
            7 => Ok(MapDataBlockType::Water),
            8 => Ok(MapDataBlockType::MonsterSpawn),
            9 => Ok(MapDataBlockType::Ocean),
            10 => Ok(MapDataBlockType::Warp),
            11 => Ok(MapDataBlockType::CollisionObject),
            12 => Ok(MapDataBlockType::EventObject),
            _ => {
                bail!("Invalid Map Data Block Type: {}", value);
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct OceanPatch {
    pub start: Vector3<f32>,
    pub end: Vector3<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Ocean {
    pub size: f32,
    pub patches: Vec<OceanPatch>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectData {
    pub name: String,
    pub warp_id: i16,
    pub event_id: i16,
    pub object_type: i32,
    pub object_id: i32,
    pub map_position: Vector2<i32>,
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
    pub scale: Vector3<f32>,
}

impl ObjectData {
    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.name = reader.read_string_u8()?;
        self.warp_id = reader.read_i16()?;
        self.event_id = reader.read_i16()?;
        self.object_type = reader.read_i32()?;
        self.object_id = reader.read_i32()?;
        self.map_position = reader.read_vector2_i32()?;
        self.rotation = reader.read_quaternion()?;
        self.position = reader.read_vector3_f32()?;
        self.scale = reader.read_vector3_f32()?;
        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NpcData {
    pub data: ObjectData,
    pub ai: i32,
    pub file: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SoundData {
    pub data: ObjectData,
    pub file: String,
    pub range: i32,
    pub interval: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EffectData {
    pub data: ObjectData,
    pub file: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct EventData {
    data: ObjectData,
    function_name: String,
    file: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct WaterData {
    pub width: u32,
    pub height: u32,
    pub has_water: Vec<bool>,
    pub heights: Vec<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MonsterSpawnPoint {
    pub name: String,
    pub monster: u32,
    pub count: u32,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MonsterSpawn {
    pub data: ObjectData,
    pub name: String,
    pub basic_spawns: Vec<MonsterSpawnPoint>,
    pub tactical_spawns: Vec<MonsterSpawnPoint>,
    pub interval: u32,
    pub limit: u32,
    pub range: u32,
    pub tactical_variable: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MapData {
    pub map_pos: Vector2<i32>,
    pub zone_pos: Vector2<i32>,
    pub name: String,
    pub objects: Vec<ObjectData>,
    pub npcs: Vec<NpcData>,
    pub sounds: Vec<SoundData>,
    pub effects: Vec<EffectData>,
    pub animations: Vec<ObjectData>,
    pub waters: Vec<WaterData>,
    pub buildings: Vec<ObjectData>,
    pub warps: Vec<ObjectData>,
    pub oceans: Vec<Ocean>,
    pub monster_spawns: Vec<MonsterSpawn>,
    pub collision_objects: Vec<ObjectData>,
    pub events: Vec<EventData>,
}

impl RoseFile for MapData {
    fn new() -> Self {
        MapData::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let block_count = reader.read_u32()?;

        let mut blocks = Vec::with_capacity(block_count as usize);
        for _ in 0..block_count {
            let block_type = MapDataBlockType::try_from(reader.read_u32()?)?;
            let block_offset = reader.read_u32()?;
            blocks.push((block_type, block_offset));
        }

        for (block_type, block_offset) in blocks {
            reader.seek(SeekFrom::Start(block_offset as u64))?;

            // Special cases
            match block_type {
                MapDataBlockType::MapInfo => {
                    self.map_pos = reader.read_vector2_i32()?;
                    self.zone_pos = reader.read_vector2_i32()?;

                    // Unused 4x4 matrix of floats
                    for _ in 0..16 {
                        let _ = reader.read_f32()?;
                    }

                    self.name = reader.read_cstring()?;
                    continue;
                }
                MapDataBlockType::Ocean => {
                    let mut ocean = Ocean::default();
                    ocean.size = reader.read_f32()?;

                    let patch_count = reader.read_u32()?;
                    ocean.patches.reserve(patch_count as usize);

                    for _ in 0..patch_count {
                        let mut ocean_patch = OceanPatch::default();
                        ocean_patch.start = reader.read_vector3_f32()?;
                        ocean_patch.end = reader.read_vector3_f32()?;
                        ocean.patches.push(ocean_patch);
                    }

                    self.oceans.push(ocean);
                    continue;
                }
                _ => {}
            }

            let count = reader.read_u32()?;
            for _ in 0..count {
                let mut data = ObjectData::default();
                data.read(reader)?;

                match block_type {
                    MapDataBlockType::Object => {
                        self.objects.push(data);
                    }
                    MapDataBlockType::Npc => {
                        let mut npc_data = NpcData::default();
                        npc_data.data = data;
                        npc_data.ai = reader.read_i32()?;
                        npc_data.file = reader.read_string_u8()?;
                        self.npcs.push(npc_data);
                    }
                    MapDataBlockType::Building => {
                        self.buildings.push(data);
                    }
                    MapDataBlockType::Sound => {
                        let mut sound_data = SoundData::default();
                        sound_data.data = data;
                        sound_data.file = reader.read_string_u8()?;
                        sound_data.range = reader.read_i32()?;
                        sound_data.interval = reader.read_i32()?;
                        self.sounds.push(sound_data);
                    }
                    MapDataBlockType::Effect => {
                        let mut effect_data = EffectData::default();
                        effect_data.data = data;
                        effect_data.file = reader.read_string_u8()?;
                        self.effects.push(effect_data);
                    }
                    MapDataBlockType::Animation => {
                        self.animations.push(data);
                    }
                    MapDataBlockType::Water => {
                        let mut water_data = WaterData::default();
                        water_data.width = reader.read_u32()?;
                        water_data.height = reader.read_u32()?;

                        let size = water_data.width * water_data.height;
                        water_data.has_water.reserve(size as usize);
                        water_data.heights.reserve(size as usize);

                        for _ in 0..water_data.height {
                            for _ in 0..water_data.width {
                                water_data.has_water.push(reader.read_u8()? > 0);
                                water_data.heights.push(reader.read_f32()?);
                            }
                        }
                        self.waters.push(water_data);
                    }
                    MapDataBlockType::MonsterSpawn => {
                        let mut monster_spawn = MonsterSpawn::default();
                        monster_spawn.name = reader.read_string_u8()?;

                        let basic_count = reader.read_u32()?;
                        monster_spawn.basic_spawns.reserve(basic_count as usize);
                        for _ in 0..basic_count {
                            let mut spawn_point = MonsterSpawnPoint::default();
                            spawn_point.name = reader.read_string_u8()?;
                            spawn_point.monster = reader.read_u32()?;
                            spawn_point.count = reader.read_u32()?;
                            monster_spawn.basic_spawns.push(spawn_point);
                        }

                        let tactical_count = reader.read_u32()?;
                        monster_spawn
                            .tactical_spawns
                            .reserve(tactical_count as usize);
                        for _ in 0..tactical_count {
                            let mut spawn_point = MonsterSpawnPoint::default();
                            spawn_point.name = reader.read_string_u8()?;
                            spawn_point.monster = reader.read_u32()?;
                            spawn_point.count = reader.read_u32()?;
                            monster_spawn.tactical_spawns.push(spawn_point);
                        }

                        monster_spawn.interval = reader.read_u32()?;
                        monster_spawn.limit = reader.read_u32()?;
                        monster_spawn.range = reader.read_u32()?;
                        monster_spawn.tactical_variable = reader.read_u32()?;
                        self.monster_spawns.push(monster_spawn);
                    }
                    MapDataBlockType::Warp => {
                        self.warps.push(data);
                    }
                    MapDataBlockType::CollisionObject => {
                        self.collision_objects.push(data);
                    }
                    MapDataBlockType::EventObject => {
                        let mut event = EventData::default();
                        event.data = data;
                        event.function_name = reader.read_string_u8()?;
                        event.file = reader.read_string_u8()?;
                        self.events.push(event);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, _writer: &mut W) -> Result<(), Error> {
        unimplemented!();
    }
}
