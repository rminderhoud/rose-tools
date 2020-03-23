//! ROSE Scene
use std::convert::{Into, TryFrom, TryInto};
use std::io::SeekFrom;
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
            mat.alpha_ref = reader.read_u16()?;
            mat.z_write_enabled = reader.read_bool16()?;
            mat.z_test_enabled = reader.read_bool16()?;
            mat.blend_mode = SceneBlendMode::try_from(reader.read_u16()?)?;
            mat.specular_enabled = reader.read_bool16()?;
            mat.alpha = reader.read_f32()?;
            mat.glow_type = SceneGlowType::try_from(reader.read_u16()?)?;
            mat.glow_color = reader.read_color3()?;
            self.materials.push(mat);
        }

        let effects_count = reader.read_u16()?;
        for _ in 0..effects_count {
            let path = PathBuf::from(reader.read_cstring()?);
            self.effects.push(path);
        }

        let object_count = reader.read_u16()?;
        for _ in 0..object_count {
            let mut object = SceneObject::default();
            object.bounding_cylinder.radius = reader.read_u32()? as f32;
            object.bounding_cylinder.center = reader.read_vector2_i32()?;

            let part_count = reader.read_u16()?;
            if part_count == 0 {
                self.objects.push(object);
                continue;
            }

            for _ in 0..part_count {
                let mut part = SceneObjectPart::default();
                part.mesh_id = reader.read_u16()?;
                part.material_id = reader.read_u16()?;

                loop {
                    let flag = SceneObjectProperty::try_from(reader.read_u8()?)?;
                    if flag == SceneObjectProperty::None {
                        break;
                    }
                    let size = reader.read_u8()?;

                    match flag {
                        SceneObjectProperty::None => break,
                        SceneObjectProperty::Position => part.position = reader.read_vector3_f32()?,
                        SceneObjectProperty::Rotation => part.rotation = reader.read_quaternion_wxyz()?,
                        SceneObjectProperty::Scale => part.scale = reader.read_vector3_f32()?,
                        SceneObjectProperty::AxisRotation => part.axis_rotation = reader.read_quaternion_wxyz()?,
                        SceneObjectProperty::BoneIndex => part.bone_index = reader.read_i16()?,
                        SceneObjectProperty::DummyIndex => part.dummy_index = reader.read_i16()?,
                        SceneObjectProperty::Parent => part.parent = reader.read_u16()?,
                        //SceneObjectProperty::Collision => part.collision = SceneCollisionType::try_from(reader.read_u16()?)?,
                        SceneObjectProperty::Collision => part.collision = reader.read_u16()?,
                        SceneObjectProperty::AnimationPath => part.animation_path = PathBuf::from(reader.read_string(size as u64)?),
                        SceneObjectProperty::Range => part.range = reader.read_u16()?,
                        SceneObjectProperty::UseLightmap => part.use_lightmap = reader.read_bool16()?,
                        SceneObjectProperty::Animation => {
                            bail!("Animation scene object property found but no handler.")
                        }
                    }
                }

                object.parts.push(part);
            }

            let object_effect_count = reader.read_u16()?;
            for _ in 0..object_effect_count {
                let mut object_effect = SceneObjectEffect::default();

                object_effect.effect_id = reader.read_u16()?;
                object_effect.effect_type = SceneEffectType::try_from(reader.read_u16()?)?;

                loop {
                    let flag = SceneObjectProperty::try_from(reader.read_u8()?)?;
                    if flag == SceneObjectProperty::None {
                        break;
                    }
                    let size = reader.read_u8()?;

                    match flag {
                        SceneObjectProperty::None => break,
                        SceneObjectProperty::Position => object_effect.position = reader.read_vector3_f32()?,
                        SceneObjectProperty::Rotation => object_effect.rotation = reader.read_quaternion_wxyz()?,
                        SceneObjectProperty::Scale => object_effect.scale = reader.read_vector3_f32()?,
                        SceneObjectProperty::Parent => object_effect.parent = reader.read_u16()?,
                        _ => {
                            reader.seek(SeekFrom::Current(size as i64))?;
                        }
                    }
                }

                object.effects.push(object_effect);
            }

            object.bounding_box.min = reader.read_vector3_f32()?;
            object.bounding_box.max = reader.read_vector3_f32()?;

            self.objects.push(object);
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
            writer.write_u16(mat.alpha_ref)?;
            writer.write_bool16(mat.z_write_enabled)?;
            writer.write_bool16(mat.z_test_enabled)?;
            writer.write_u16(mat.blend_mode.try_into()?)?;
            writer.write_bool16(mat.specular_enabled)?;
            writer.write_f32(mat.alpha)?;
            writer.write_u16(mat.glow_type.try_into()?)?;
            writer.write_color3(&mat.glow_color)?;
        }

        writer.write_u16(self.effects.len() as u16)?;
        for effect_path in &self.effects {
            writer.write_cstring(effect_path.to_str().unwrap())?;
        }

        writer.write_u16(self.objects.len() as u16)?;
        for object in &self.objects {
            writer.write_u32(object.bounding_cylinder.radius as u32)?;
            writer.write_vector2_i32(&object.bounding_cylinder.center)?;

            writer.write_u16(object.parts.len() as u16)?;
            if object.parts.len() == 0 {
                continue;
            }

            for part in &object.parts {
                writer.write_u16(part.mesh_id)?;
                writer.write_u16(part.material_id)?;

                writer.write_u8(SceneObjectProperty::Position.into())?;
                writer.write_u8(0)?;
                writer.write_vector3_f32(&part.position)?;

                writer.write_u8(SceneObjectProperty::Rotation.into())?;
                writer.write_u8(0)?;
                writer.write_quaternion_wxyz(&part.rotation)?;

                writer.write_u8(SceneObjectProperty::Scale.into())?;
                writer.write_u8(0)?;
                writer.write_vector3_f32(&part.scale)?;

                writer.write_u8(SceneObjectProperty::AxisRotation.into())?;
                writer.write_u8(0)?;
                writer.write_quaternion_wxyz(&part.axis_rotation)?;

                writer.write_u8(SceneObjectProperty::BoneIndex.into())?;
                writer.write_u8(0)?;
                writer.write_i16(part.bone_index)?;

                writer.write_u8(SceneObjectProperty::DummyIndex.into())?;
                writer.write_u8(0)?;
                writer.write_i16(part.dummy_index)?;

                writer.write_u8(SceneObjectProperty::Parent.into())?;
                writer.write_u8(0)?;
                writer.write_u16(part.parent)?;

                writer.write_u8(SceneObjectProperty::Collision.into())?;
                writer.write_u8(0)?;
                writer.write_u16(part.collision)?;

                let path = part.animation_path.to_str().unwrap();
                writer.write_u8(SceneObjectProperty::AnimationPath.into())?;
                writer.write_u8(path.len() as u8)?;
                writer.write_string(&path, path.len() as i32)?;

                writer.write_u8(SceneObjectProperty::Range.into())?;
                writer.write_u8(0)?;
                writer.write_u16(part.range)?;

                writer.write_u8(SceneObjectProperty::UseLightmap.into())?;
                writer.write_u8(0)?;
                writer.write_bool16(part.use_lightmap)?;

                writer.write_u8(SceneObjectProperty::None.into())?;
            }

            writer.write_u16(object.effects.len() as u16)?;
            for effect in &object.effects {
                writer.write_u16(effect.effect_id)?;
                writer.write_u16(effect.effect_type.into())?;

                writer.write_u8(SceneObjectProperty::Position.into())?;
                writer.write_u8(0)?;
                writer.write_vector3_f32(&effect.position)?;

                writer.write_u8(SceneObjectProperty::Rotation.into())?;
                writer.write_u8(0)?;
                writer.write_quaternion_wxyz(&effect.rotation)?;

                writer.write_u8(SceneObjectProperty::Scale.into())?;
                writer.write_u8(0)?;
                writer.write_vector3_f32(&effect.scale)?;

                writer.write_u8(SceneObjectProperty::Parent.into())?;
                writer.write_u8(0)?;
                writer.write_u16(effect.parent)?;

                writer.write_u8(SceneObjectProperty::None.into())?;
            }

            writer.write_vector3_f32(&object.bounding_box.min)?;
            writer.write_vector3_f32(&object.bounding_box.max)?;
        }


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
    pub alpha_ref: u16,
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SceneObjectPart {
    pub mesh_id: u16,
    pub material_id: u16,
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
    pub scale: Vector3<f32>,
    pub axis_rotation: Quaternion,
    pub bone_index: i16,
    pub dummy_index: i16,
    pub parent: u16,
    /*
    TODO: Convert collision to an enum? (collision_info)

    :ENUM[WORD] collision_type
    NONE = 0
    SPHERE = 1
    AXISALIGNEDBOUNDINGBOX = 2
    ORIENTEDBOUNDINGBOX = 3
    POLYGON = 4
    :ENDENUM

    :ENUM[WORD] collisionpick_type
        NONE = 0
        NOTMOVABLE = 8
        NOTPICKABLE = 16
        HEIGHTONLY = 32
        NOCAMERACOLLISION = 64
    :ENDENUM

    :TYPEDEF[WORD] collision_info
        collision_type | collisionpick_type
    */
    pub collision: u16,
    //collision: SceneCollisionType,
    pub animation_path: PathBuf,
    pub range: u16,
    pub use_lightmap: bool,
}

impl Default for SceneObjectPart {
    fn default() -> SceneObjectPart {
        SceneObjectPart {
            mesh_id: 0,
            material_id: 0,
            position: Vector3::default(),
            rotation: Quaternion::default(),
            scale: Vector3::default(),
            axis_rotation: Quaternion::default(),
            bone_index: -1,
            dummy_index: -1,
            parent: 0,
            collision: 0,
            //collision: SceneCollisionType,
            animation_path: PathBuf::new(),
            range: 0,
            use_lightmap: false,
        }
    }
}

/// Scene Object Effect
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SceneObjectEffect {
    pub effect_id: u16,
    pub effect_type: SceneEffectType,
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
    pub scale: Vector3<f32>,
    pub parent: u16,
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

impl TryFrom<u16> for SceneCollisionType {
    type Error = failure::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SceneCollisionType::None),
            3 => Ok(SceneCollisionType::BoundingBox),
            4 => Ok(SceneCollisionType::Polygon),
            8 => Ok(SceneCollisionType::NotMovable),
            16 => Ok(SceneCollisionType::NotPickable),
            32 => Ok(SceneCollisionType::HeightOnly),
            64 => Ok(SceneCollisionType::NoCameraCollision),
            _ => bail!("Invalid SceneCollisionType: {}", value),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum  SceneObjectProperty {
    None = 0,
    Position = 1,
    Rotation = 2,
    Scale = 3,
    AxisRotation = 4,
    BoneIndex = 5,
    DummyIndex = 6,
    Parent = 7,
    Animation = 8,
    Collision = 29,
    AnimationPath = 30,
    Range = 31,
    UseLightmap = 32,
}

impl Default for SceneObjectProperty {
    fn default() -> Self {
        SceneObjectProperty::None
    }
}

impl Into<u8> for SceneObjectProperty {
    fn into(self) -> u8 {
        match self {
            SceneObjectProperty::None => 0,
            SceneObjectProperty::Position => 1,
            SceneObjectProperty::Rotation => 2,
            SceneObjectProperty::Scale => 3,
            SceneObjectProperty::AxisRotation => 4,
            SceneObjectProperty::BoneIndex => 5,
            SceneObjectProperty::DummyIndex => 6,
            SceneObjectProperty::Parent => 7,
            SceneObjectProperty::Animation => 8,
            SceneObjectProperty::Collision => 29,
            SceneObjectProperty::AnimationPath => 30,
            SceneObjectProperty::Range => 31,
            SceneObjectProperty::UseLightmap => 32,
        }
    }
}

impl TryFrom<u8> for SceneObjectProperty {
    type Error = failure::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SceneObjectProperty::None),
            1 => Ok(SceneObjectProperty::Position),
            2 => Ok(SceneObjectProperty::Rotation),
            3 => Ok(SceneObjectProperty::Scale),
            4 => Ok(SceneObjectProperty::AxisRotation),
            5 => Ok(SceneObjectProperty::BoneIndex),
            6 => Ok(SceneObjectProperty::DummyIndex),
            7 => Ok(SceneObjectProperty::Parent),
            8 => Ok(SceneObjectProperty::Animation),
            29 => Ok(SceneObjectProperty::Collision),
            30 => Ok(SceneObjectProperty::AnimationPath),
            31 => Ok(SceneObjectProperty::Range),
            32 => Ok(SceneObjectProperty::UseLightmap),
            _ => {
                bail!("Invalid SceneObjectProperty: {}", value);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum  SceneEffectType {
    Normal = 0,
    DayNight = 1,
    LightContainer = 2,
}

impl Default for SceneEffectType {
    fn default() -> Self {
        SceneEffectType::Normal
    }
}

impl Into<u16> for SceneEffectType {
    fn into(self) -> u16 {
        match self {
            SceneEffectType::Normal => 0,
            SceneEffectType::DayNight => 1,
            SceneEffectType::LightContainer => 2,
        }
    }
}
impl TryFrom<u16> for SceneEffectType {
    type Error = failure::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SceneEffectType::Normal),
            1 => Ok(SceneEffectType::DayNight),
            2 => Ok(SceneEffectType::LightContainer),
            _ => Ok(SceneEffectType::Normal),
        }
    }
}