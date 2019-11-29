//! ROSE Online Motion
use std::convert::TryFrom;

use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};
use crate::utils::{Quaternion, Vector2, Vector3};

/// Motion File
pub type ZMO = Motion;

/// Motion
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Motion {
    pub identifier: String,
    pub fps: u32,
    pub frames: u32,

    pub channels: Vec<Channel>,
}

impl RoseFile for Motion {
    fn new() -> Motion {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.identifier = reader.read_string(8)?;
        if self.identifier != "ZMO0002" {
            bail!(format!("Unsupported Motion version: {}", self.identifier));
        }

        self.fps = reader.read_u32()?;
        self.frames = reader.read_u32()?;
        let channel_count = reader.read_u32()?;

        for _ in 0..channel_count {
            let channel_type = ChannelType::try_from(reader.read_u32()?)?;
            let channel_index = reader.read_u32()?;

            let mut channel = Channel::from(channel_type);
            channel.index = channel_index;

            self.channels.push(channel);
        }

        for _ in 0..self.frames {
            for channel in &mut self.channels {
                match channel.typ {
                    ChannelType::Position => {
                        let v = channel.position_frames().unwrap();
                        v.push(reader.read_vector3_f32()?);
                    }
                    ChannelType::Rotation => {
                        let v = channel.rotation_frames().unwrap();
                        v.push(reader.read_quaternion()?);
                    }
                    ChannelType::Normal => {
                        let v = channel.normal_frames().unwrap();
                        v.push(reader.read_vector3_f32()?);
                    }
                    ChannelType::Alpha => {
                        let v = channel.alpha_frames().unwrap();
                        v.push(reader.read_f32()?);
                    }
                    ChannelType::UV1 => {
                        let v = channel.uv1_frames().unwrap();
                        v.push(reader.read_vector2_f32()?);
                    }
                    ChannelType::UV2 => {
                        let v = channel.uv2_frames().unwrap();
                        v.push(reader.read_vector2_f32()?);
                    }
                    ChannelType::UV3 => {
                        let v = channel.uv3_frames().unwrap();
                        v.push(reader.read_vector2_f32()?);
                    }
                    ChannelType::UV4 => {
                        let v = channel.uv4_frames().unwrap();
                        v.push(reader.read_vector2_f32()?);
                    }
                    ChannelType::Texture => {
                        let v = channel.texture_frames().unwrap();
                        v.push(reader.read_f32()?);
                    }
                    ChannelType::Scale => {
                        let v = channel.scale_frames().unwrap();
                        v.push(reader.read_f32()?);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_string(&self.identifier, 8)?;
        writer.write_u32(self.fps)?;
        writer.write_u32(self.frames)?;
        writer.write_u32(self.channels.len() as u32)?;

        for channel in &self.channels {
            writer.write_u32(channel.typ as u32)?;
            writer.write_u32(channel.index)?;
        }

        for i in 0..self.frames {
            let i = i as usize;
            for channel in &mut self.channels {
                match channel.typ {
                    ChannelType::Position => {
                        let v = channel.position_frames().unwrap()[i];
                        writer.write_vector3_f32(&v)?;
                    }
                    ChannelType::Rotation => {
                        let q = channel.rotation_frames().unwrap()[i];
                        writer.write_quaternion(&q)?;
                    }
                    ChannelType::Normal => {
                        let v = channel.normal_frames().unwrap()[i];
                        writer.write_vector3_f32(&v)?;
                    }
                    ChannelType::Alpha => {
                        let f = channel.alpha_frames().unwrap()[i];
                        writer.write_f32(f)?;
                    }
                    ChannelType::UV1 => {
                        let v = channel.uv1_frames().unwrap()[i];
                        writer.write_vector2_f32(&v)?;
                    }
                    ChannelType::UV2 => {
                        let v = channel.uv2_frames().unwrap()[i];
                        writer.write_vector2_f32(&v)?;
                    }
                    ChannelType::UV3 => {
                        let v = channel.uv3_frames().unwrap()[i];
                        writer.write_vector2_f32(&v)?;
                    }
                    ChannelType::UV4 => {
                        let v = channel.uv4_frames().unwrap()[i];
                        writer.write_vector2_f32(&v)?;
                    }
                    ChannelType::Texture => {
                        let f = channel.texture_frames().unwrap()[i];
                        writer.write_f32(f)?;
                    }
                    ChannelType::Scale => {
                        let f = channel.scale_frames().unwrap()[i];
                        writer.write_f32(f)?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub typ: ChannelType,
    pub index: u32,
    pub frames: ChannelData,
}

impl Channel {
    pub fn position_frames(&mut self) -> Option<&mut Vec<Vector3<f32>>> {
        if let ChannelData::Position(v) = &mut self.frames {
            Some(v)
        } else {
            None
        }
    }

    pub fn rotation_frames(&mut self) -> Option<&mut Vec<Quaternion>> {
        if let ChannelData::Rotation(q) = &mut self.frames {
            Some(q)
        } else {
            None
        }
    }

    pub fn normal_frames(&mut self) -> Option<&mut Vec<Vector3<f32>>> {
        if let ChannelData::Normal(f) = &mut self.frames {
            Some(f)
        } else {
            None
        }
    }

    pub fn alpha_frames(&mut self) -> Option<&mut Vec<f32>> {
        if let ChannelData::Alpha(f) = &mut self.frames {
            Some(f)
        } else {
            None
        }
    }

    pub fn uv1_frames(&mut self) -> Option<&mut Vec<Vector2<f32>>> {
        if let ChannelData::UV1(v) = &mut self.frames {
            Some(v)
        } else {
            None
        }
    }

    pub fn uv2_frames(&mut self) -> Option<&mut Vec<Vector2<f32>>> {
        if let ChannelData::UV2(v) = &mut self.frames {
            Some(v)
        } else {
            None
        }
    }

    pub fn uv3_frames(&mut self) -> Option<&mut Vec<Vector2<f32>>> {
        if let ChannelData::UV3(v) = &mut self.frames {
            Some(v)
        } else {
            None
        }
    }

    pub fn uv4_frames(&mut self) -> Option<&mut Vec<Vector2<f32>>> {
        if let ChannelData::UV1(v) = &mut self.frames {
            Some(v)
        } else {
            None
        }
    }

    pub fn texture_frames(&mut self) -> Option<&mut Vec<f32>> {
        if let ChannelData::Texture(f) = &mut self.frames {
            Some(f)
        } else {
            None
        }
    }

    pub fn scale_frames(&mut self) -> Option<&mut Vec<f32>> {
        if let ChannelData::Scale(f) = &mut self.frames {
            Some(f)
        } else {
            None
        }
    }
}

impl From<ChannelType> for Channel {
    fn from(typ: ChannelType) -> Self {
        let mut channel = Self::default();
        channel.typ = typ;

        channel.frames = match channel.typ {
            ChannelType::Position => ChannelData::Position(Vec::new()),
            ChannelType::Rotation => ChannelData::Rotation(Vec::new()),
            ChannelType::Normal => ChannelData::Normal(Vec::new()),
            ChannelType::Alpha => ChannelData::Alpha(Vec::new()),
            ChannelType::UV1 => ChannelData::UV1(Vec::new()),
            ChannelType::UV2 => ChannelData::UV2(Vec::new()),
            ChannelType::UV3 => ChannelData::UV3(Vec::new()),
            ChannelType::UV4 => ChannelData::UV4(Vec::new()),
            ChannelType::Texture => ChannelData::Texture(Vec::new()),
            ChannelType::Scale => ChannelData::Scale(Vec::new()),
            _ => ChannelData::None,
        };

        channel
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ChannelData {
    None,
    Position(Vec<Vector3<f32>>),
    Rotation(Vec<Quaternion>),
    Normal(Vec<Vector3<f32>>),
    Alpha(Vec<f32>),
    UV1(Vec<Vector2<f32>>),
    UV2(Vec<Vector2<f32>>),
    UV3(Vec<Vector2<f32>>),
    UV4(Vec<Vector2<f32>>),
    Texture(Vec<f32>),
    Scale(Vec<f32>),
}

impl Default for ChannelData {
    fn default() -> ChannelData {
        ChannelData::None
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum ChannelType {
    None = 1,
    Position = 1 << 1,
    Rotation = 1 << 2,
    Normal = 1 << 3,
    Alpha = 1 << 4,
    UV1 = 1 << 5,
    UV2 = 1 << 6,
    UV3 = 1 << 7,
    UV4 = 1 << 8,
    Texture = 1 << 9,
    Scale = 1 << 10,
}

impl Default for ChannelType {
    fn default() -> ChannelType {
        ChannelType::None
    }
}

impl TryFrom<u32> for ChannelType {
    type Error = failure::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x001 => Ok(ChannelType::None),
            0x002 => Ok(ChannelType::Position),
            0x004 => Ok(ChannelType::Rotation),
            0x008 => Ok(ChannelType::Normal),
            0x010 => Ok(ChannelType::Alpha),
            0x020 => Ok(ChannelType::UV1),
            0x040 => Ok(ChannelType::UV2),
            0x080 => Ok(ChannelType::UV3),
            0x100 => Ok(ChannelType::UV4),
            0x200 => Ok(ChannelType::Texture),
            0x400 => Ok(ChannelType::Scale),
            _ => {
                bail!("Invalid ChannelType: {}", value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel() {
        let mut channel = Channel::default();
        assert_eq!(channel.position_frames(), None);
        assert_eq!(channel.rotation_frames(), None);
        assert_eq!(channel.normal_frames(), None);
        assert_eq!(channel.alpha_frames(), None);
        assert_eq!(channel.uv1_frames(), None);
        assert_eq!(channel.uv2_frames(), None);
        assert_eq!(channel.uv3_frames(), None);
        assert_eq!(channel.uv4_frames(), None);
        assert_eq!(channel.texture_frames(), None);
        assert_eq!(channel.scale_frames(), None);
    }
}
