//! ROSE Online Skeleton
use failure::Error;

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};
use crate::utils::{Quaternion, Vector3};

/// Skeleton file
pub type ZMD = Skeleton;

const ZMD_IDENTIFIER_2: &str = "ZMD0002";
const ZMD_IDENTIFIER_3: &str = "ZMD0003";

/// Skeleton
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
    pub dummy_bones: Vec<Bone>,
}

impl RoseFile for Skeleton {
    fn new() -> Skeleton {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        let identifier = reader.read_string(7)?;
        let version = match identifier.as_str() {
            ZMD_IDENTIFIER_2 => 2,
            ZMD_IDENTIFIER_3 => 3,
            _ => bail!("Unsupported ZMD version: {}", identifier),
        };

        let bone_count = reader.read_u32()?;
        for _ in 0..bone_count {
            let mut bone = Bone::new();
            bone.parent = reader.read_i32()?;
            bone.name = reader.read_cstring()?;
            bone.position = reader.read_vector3_f32()?;
            bone.rotation = reader.read_quaternion_wxyz()?;
            self.bones.push(bone);
        }

        let dummy_count = reader.read_u32()?;
        for _ in 0..dummy_count {
            let mut bone = Bone::new();
            bone.name = reader.read_cstring()?;
            bone.parent = reader.read_i32()?;
            bone.position = reader.read_vector3_f32()?;

            if version == 3 {
                bone.rotation = reader.read_quaternion_wxyz()?;
            }

            self.dummy_bones.push(bone);
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_string(ZMD_IDENTIFIER_3, 7)?;

        writer.write_u32(self.bones.len() as u32)?;
        for bone in self.bones.iter() {
            writer.write_i32(bone.parent)?;
            writer.write_cstring(&bone.name)?;
            writer.write_vector3_f32(&bone.position)?;
            writer.write_quaternion_wxyz(&bone.rotation)?;
        }

        writer.write_u32(self.dummy_bones.len() as u32)?;
        for dummy in self.dummy_bones.iter() {
            writer.write_cstring(&dummy.name)?;
            writer.write_i32(dummy.parent)?;
            writer.write_vector3_f32(&dummy.position)?;
            writer.write_quaternion_wxyz(&dummy.rotation)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Bone {
    pub parent: i32,
    pub name: String,
    pub position: Vector3<f32>,
    pub rotation: Quaternion,
}

impl Bone {
    pub fn new() -> Bone {
        Self::default()
    }
}
