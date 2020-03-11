//! ROSE Scene
use std::convert::{Into, TryFrom, TryInto};
use std::path::PathBuf;

use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};
use crate::utils::{BoundingBox, BoundingCylinder, Color3, Quaternion, Vector3};

/// Scene file
pub type ZSC = Scene;

/// Scene
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Scene {
    pub meshes: Vec<PathBuf>,
    pub materials: Vec<SceneMaterial>,
    pub effects: Vec<PathBuf>,
    pub objects: Vec<SceneObject>,
}

impl RoseFile for Scene {
    fn new() -> Scene {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let mesh_count = reader.read_u16()?;
        for _ in 0..mesh_count {
            let path = PathBuf::from(reader.read_cstring()?);
            self.meshes.push(path);
        }

        let material_count = reader.read_u16()?;
        for _ in 0..material_count {
            let mut mat = SceneMaterial::default();
            mat.path = PathBuf::from(reader.read_cstring()?);
            mat.is_skin = reader.read_bool16()?;
            mat.alpha_enabled = reader.read_bool16()?;
            mat.two_sided = reader.read_bool16()?;
            mat.alpha_test_enabled = reader.read_bool16()?;
            mat.alpha_ref_enabled = reader.read_bool16()?;
            mat.z_write_enabled = reader.read_bool16()?;
            mat.z_test_enabled = reader.read_bool16()?;
            mat.blend_mode = SceneBlendMode::try_from(reader.read_u16()?)?;
            mat.specular_enabled = reader.read_bool16()?;
            mat.alpha = reader.read_f32()?;
            mat.glow_type = SceneGlowType::try_from(reader.read_u16()?)?;
            mat.glow_color = reader.read_color3()?;
            self.materials.push(mat);
        }
        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_u16(self.meshes.len() as u16)?;
        for mesh_path in &self.meshes {
            writer.write_cstring(mesh_path.to_str().unwrap())?;
        }

        writer.write_u16(self.materials.len() as u16)?;
        for mat in &self.materials {
            writer.write_cstring(mat.path.to_str().unwrap())?;
            writer.write_bool16(mat.is_skin)?;
            writer.write_bool16(mat.alpha_enabled)?;
            writer.write_bool16(mat.two_sided)?;
            writer.write_bool16(mat.alpha_test_enabled)?;
            writer.write_bool16(mat.alpha_ref_enabled)?;
            writer.write_bool16(mat.z_write_enabled)?;
            writer.write_bool16(mat.z_test_enabled)?;
            writer.write_u16(mat.blend_mode.try_into()?)?;
            writer.write_bool16(mat.specular_enabled)?;
            writer.write_f32(mat.alpha)?;
            writer.write_u16(mat.glow_type.try_into()?)?;
            writer.write_color3(&mat.glow_color)?;
        }

        // TODO: Read effects
        // TODO: Read objects

        Ok(())
    }
}

/// Scene Material
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SceneMaterial {
    pub path: PathBuf,
    pub is_skin: bool,
    pub alpha_enabled: bool,
    pub two_sided: bool,
    pub alpha_test_enabled: bool,
    pub alpha_ref_enabled: bool,
    pub z_write_enabled: bool,
    pub z_test_enabled: bool,
    pub blend_mode: SceneBlendMode,
    pub specular_enabled: bool,
    pub alpha: f32,
    pub glow_type: SceneGlowType,
    pub glow_color: Color3,
}

/// Scene Object
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SceneObject {
    pub bounding_cylinder: BoundingCylinder,
    pub bounding_box: BoundingBox<f32>,
    pub parts: Vec<SceneObjectPart>,
    pub effects: Vec<SceneObjectEffect>,
}

/// Scene Object Part
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SceneObjectPart {
    mesh_id: i32,
    material_id: i32,
    position: Vector3<f32>,
    rotation: Quaternion,
    scale: Vector3<f32>,
    axis_rotation: Quaternion,
    bone_index: i16,
    dummy_index: i16,
    parent: i16,
    collision: SceneCollisionType,
    motion_path: PathBuf,
    range: i16,
    use_lightmap: bool,
}

/// Scene Object Effect
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SceneObjectEffect {
    position: Vector3<f32>,
    rotation: Quaternion,
    scale: Vector3<f32>,
    parent: i16,
    data: Vec<u8>
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SceneBlendMode {
    None = 0,
    Custom = 1,
    Normal = 2,
    Lighten = 3,
}

impl Default for SceneBlendMode {
    fn default() -> Self {
        SceneBlendMode::None
    }
}

impl Into<u16> for SceneBlendMode {
    fn into(self) -> u16 {
        match self {
            SceneBlendMode::None => 0,
            SceneBlendMode::Custom => 1,
            SceneBlendMode::Normal => 2,
            SceneBlendMode::Lighten => 3,
        }
    }
}

impl TryFrom<u16> for SceneBlendMode {
    type Error = failure::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SceneBlendMode::None),
            1 => Ok(SceneBlendMode::Custom),
            2 => Ok(SceneBlendMode::Normal),
            3 => Ok(SceneBlendMode::Lighten),
            _ => {
                bail!("Invalid SceneBlendMode: {}", value);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SceneGlowType {
    None = 0,
    NotSet = 1,
    Simple = 2,
    Light = 3,
    Texture = 4,
    TextureLight = 5,
    Alpha = 6,
}

impl Default for SceneGlowType {
    fn default() -> Self {
        SceneGlowType::None
    }
}

impl Into<u16> for SceneGlowType {
    fn into(self) -> u16 {
        match self {
            SceneGlowType::None => 0,
            SceneGlowType::NotSet => 1,
            SceneGlowType::Simple => 2,
            SceneGlowType::Light => 3,
            SceneGlowType::Texture => 4,
            SceneGlowType::TextureLight => 5,
            SceneGlowType::Alpha => 6,
        }
    }
}

impl TryFrom<u16> for SceneGlowType {
    type Error = failure::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SceneGlowType::None),
            1 => Ok(SceneGlowType::NotSet),
            2 => Ok(SceneGlowType::Simple),
            3 => Ok(SceneGlowType::Light),
            4 => Ok(SceneGlowType::Texture),
            5 => Ok(SceneGlowType::TextureLight),
            6 => Ok(SceneGlowType::Alpha),
            _ => {
                bail!("Invalid SceneGlowType: {}", value);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SceneCollisionType {
    None = 0,
    BoundingBox = 3,
    Polygon = 1 << 2,
    NotMovable = 1 << 3,
    NotPickable = 1 << 4,
    HeightOnly = 1 << 5,
    NoCameraCollision = 1 << 6,
}

impl Default for SceneCollisionType {
    fn default() -> Self {
        SceneCollisionType::None
    }
}