//! ROSE Online 3D Meshes
use failure::Error;
use serde::{Deserialize, Serialize};

use crate::io::{ReadRoseExt, RoseFile, WriteRoseExt};
use crate::utils::{BoundingBox, Color4, Vector2, Vector3, Vector4};

/// Mesh File
pub type ZMS = Mesh;

/// Mesh
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Mesh {
    pub identifier: String,
    pub format: i32,

    pub bounding_box: BoundingBox<f32>,
    pub bones: Vec<i16>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Vector3<i16>>,
    pub materials: Vec<i16>,
    pub strips: Vec<i16>,

    // Pool properties for the vertex buffer [Static/Dynamic/System]
    pub pool: i16,
}

impl Mesh {
    pub fn positions_enabled(&self) -> bool {
        (VertexFormat::Position as i32 & self.format) != 0
    }

    pub fn normals_enabled(&self) -> bool {
        (VertexFormat::Normal as i32 & self.format) != 0
    }

    pub fn colors_enabled(&self) -> bool {
        (VertexFormat::Color as i32 & self.format) != 0
    }

    pub fn bones_enabled(&self) -> bool {
        ((VertexFormat::BoneWeight as i32 & self.format) != 0)
            && ((VertexFormat::BoneIndex as i32 & self.format) != 0)
    }

    pub fn tangents_enabled(&self) -> bool {
        (VertexFormat::Tangent as i32 & self.format) != 0
    }

    pub fn uv1_enabled(&self) -> bool {
        (VertexFormat::UV1 as i32 & self.format) != 0
    }

    pub fn uv2_enabled(&self) -> bool {
        (VertexFormat::UV2 as i32 & self.format) != 0
    }

    pub fn uv3_enabled(&self) -> bool {
        (VertexFormat::UV3 as i32 & self.format) != 0
    }

    pub fn uv4_enabled(&self) -> bool {
        (VertexFormat::UV4 as i32 & self.format) != 0
    }

    pub fn read_version6<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.format = reader.read_i32()?;
        self.bounding_box.min = reader.read_vector3_f32()?;
        self.bounding_box.max = reader.read_vector3_f32()?;

        let bone_count = reader.read_u32()?;
        for _ in 0..bone_count {
            let _index = reader.read_u32()?;
            let bone_index = reader.read_u32()?;
            self.bones.push(bone_index as i16);
        }

        let vert_count = reader.read_u32()?;
        for _ in 0..vert_count {
            self.vertices.push(Vertex::new());
        }

        for i in 0..vert_count as usize {
            let _index = reader.read_u32()?;
            self.vertices[i].position = reader.read_vector3_f32()?;
        }

        if self.normals_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].normal = reader.read_vector3_f32()?;
            }
        }

        if self.colors_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].color = reader.read_color4()?;
            }
        }

        if self.bones_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].bone_weights = reader.read_vector4_f32()?;

                let bone_indices = reader.read_vector4_u32()?;
                self.vertices[i].bone_indices.w = bone_indices.w as i16;
                self.vertices[i].bone_indices.x = bone_indices.x as i16;
                self.vertices[i].bone_indices.y = bone_indices.y as i16;
                self.vertices[i].bone_indices.z = bone_indices.z as i16;
            }
        }

        if self.tangents_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].tangent = reader.read_vector3_f32()?;
            }
        }

        if self.uv1_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].uv1 = reader.read_vector2_f32()?;
            }
        }

        if self.uv2_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].uv2 = reader.read_vector2_f32()?;
            }
        }

        if self.uv3_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].uv3 = reader.read_vector2_f32()?;
            }
        }

        if self.uv4_enabled() {
            for i in 0..vert_count as usize {
                let _index = reader.read_u32()?;
                self.vertices[i].uv4 = reader.read_vector2_f32()?;
            }
        }

        let index_count = reader.read_u32()?;
        for _ in 0..index_count {
            let _index = reader.read_u32()?;
            let indices_i32 = reader.read_vector3_u32()?;

            let mut indices_i16 = Vector3::<i16>::default();
            indices_i16.x = indices_i32.x as i16;
            indices_i16.y = indices_i32.y as i16;
            indices_i16.z = indices_i32.z as i16;

            self.indices.push(indices_i16);
        }

        let material_count = reader.read_u32()?;
        for _ in 0..material_count {
            let _index = reader.read_u32()?;
            self.materials.push(reader.read_u32()? as i16);
        }

        Ok(())
    }
}

impl RoseFile for Mesh {
    fn new() -> Mesh {
        Self::default()
    }

    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.identifier = reader.read_cstring()?;

        let version = match self.identifier.as_str() {
            "ZMS0006" => 6,
            "ZMS0007" => 7,
            "ZMS0008" => 8,
            _ => bail!(format!("Unsupported Mesh version: {}", self.identifier)),
        };

        if version == 6 {
            return self.read_version6(reader);
        }

        self.format = reader.read_i32()?;
        self.bounding_box.min = reader.read_vector3_f32()?;
        self.bounding_box.max = reader.read_vector3_f32()?;

        let bone_count = reader.read_i16()?;
        for _ in 0..bone_count {
            self.bones.push(reader.read_i16()?);
        }

        let vert_count = reader.read_i16()?;
        for _ in 0..vert_count {
            self.vertices.push(Vertex::new());
        }

        if self.positions_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].position = reader.read_vector3_f32()?;
            }
        }

        if self.normals_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].normal = reader.read_vector3_f32()?;
            }
        }

        if self.colors_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].color = reader.read_color4()?;
            }
        }

        if self.bones_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].bone_weights = reader.read_vector4_f32()?;
                self.vertices[i].bone_indices = reader.read_vector4_i16()?;
            }
        }

        if self.tangents_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].tangent = reader.read_vector3_f32()?;
            }
        }

        if self.uv1_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv1 = reader.read_vector2_f32()?;
            }
        }

        if self.uv2_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv2 = reader.read_vector2_f32()?;
            }
        }

        if self.uv3_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv3 = reader.read_vector2_f32()?;
            }
        }
        if self.uv4_enabled() {
            for i in 0..vert_count as usize {
                self.vertices[i].uv4 = reader.read_vector2_f32()?;
            }
        }

        let index_count = reader.read_i16()?;
        for _ in 0..index_count {
            self.indices.push(reader.read_vector3_i16()?);
        }

        let material_count = reader.read_i16()?;
        for _ in 0..material_count {
            self.materials.push(reader.read_i16()?);
        }

        let strip_count = reader.read_i16()?;
        for _ in 0..strip_count {
            self.strips.push(reader.read_i16()?);
        }

        if version >= 8 {
            self.pool = reader.read_i16()?;
        }

        Ok(())
    }

    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_cstring("ZMS0008")?;
        writer.write_i32(self.format)?;

        writer.write_vector3_f32(&self.bounding_box.min)?;
        writer.write_vector3_f32(&self.bounding_box.max)?;

        writer.write_i16(self.bones.len() as i16)?;
        for bone in &self.bones {
            writer.write_i16(*bone)?;
        }

        writer.write_i16(self.vertices.len() as i16)?;

        if self.positions_enabled() {
            //for ref vertex in &self.vertices {
            for vertex in &self.vertices {
                writer.write_vector3_f32(&vertex.position)?;
            }
        }

        if self.normals_enabled() {
            for vertex in &self.vertices {
                writer.write_vector3_f32(&vertex.normal)?;
            }
        }

        if self.colors_enabled() {
            for vertex in &self.vertices {
                writer.write_color4(&vertex.color)?;
            }
        }

        if self.bones_enabled() {
            for vertex in &self.vertices {
                writer.write_vector4_f32(&vertex.bone_weights)?;
                writer.write_vector4_i16(&vertex.bone_indices)?;
            }
        }

        if self.tangents_enabled() {
            for vertex in &self.vertices {
                writer.write_vector3_f32(&vertex.tangent)?;
            }
        }

        if self.uv1_enabled() {
            for vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv1)?;
            }
        }

        if self.uv2_enabled() {
            for vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv2)?;
            }
        }

        if self.uv3_enabled() {
            for vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv3)?;
            }
        }

        if self.uv4_enabled() {
            for vertex in &self.vertices {
                writer.write_vector2_f32(&vertex.uv4)?;
            }
        }

        writer.write_i16(self.indices.len() as i16)?;
        for index in &self.indices {
            writer.write_vector3_i16(index)?;
        }

        writer.write_i16(self.materials.len() as i16)?;
        for mat in &self.materials {
            writer.write_i16(*mat)?;
        }

        writer.write_i16(self.strips.len() as i16)?;
        for strip in &self.strips {
            writer.write_i16(*strip)?;
        }

        writer.write_i16(self.pool)?;

        Ok(())
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            identifier: String::from(""),
            format: -1,
            bounding_box: BoundingBox {
                min: Vector3::<f32>::new(),
                max: Vector3::<f32>::new(),
            },
            bones: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            materials: Vec::new(),
            strips: Vec::new(),
            pool: 0,
        }
    }
}

/// Mesh Vertex
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub color: Color4,
    pub bone_weights: Vector4<f32>,
    pub bone_indices: Vector4<i16>,
    pub tangent: Vector3<f32>,
    pub uv1: Vector2<f32>,
    pub uv2: Vector2<f32>,
    pub uv3: Vector2<f32>,
    pub uv4: Vector2<f32>,
}

impl Vertex {
    pub fn new() -> Vertex {
        Self::default()
    }
}

/// Mesh Vertex Flags
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum VertexFormat {
    Position = 1 << 1,
    Normal = 1 << 2,
    Color = 1 << 3,
    BoneWeight = 1 << 4,
    BoneIndex = 1 << 5,
    Tangent = 1 << 6,
    UV1 = 1 << 7,
    UV2 = 1 << 8,
    UV3 = 1 << 9,
    UV4 = 1 << 10,
}
