import enum

from rose.utils import *


class VertexFormat(enum.IntEnum):
    POSITION = 1 << 1
    NORMAL = 1 << 2
    COLOR = 1 << 3
    BONEWEIGHT = 1 << 4
    BONEINDEX = 1 << 5
    TANGENT = 1 << 6
    UV1 = 1 << 7
    UV2 = 1 << 8
    UV3 = 1 << 9
    UV4 = 1 << 10


class Vertex:
    def __init__(self):
        self.position = Vector3()
        self.normal = Vector3()
        self.color = Color4()
        self.bone_weights = Vector4()
        self.bone_indices = Vector4()
        self.tangent = Vector3()
        self.uv1 = Vector2()
        self.uv2 = Vector2()
        self.uv3 = Vector2()
        self.uv4 = Vector2()


class ZMS:
    def __init__(self):
        self.identifier = ""
        self.format = -1

        self.bounding_box = BoundingBox()
        self.bones = []
        self.vertices = []
        self.indices = []
        self.materials = []
        self.strips = []

        self.pool = 0

    def positions_enabled(self):
        return (VertexFormat.POSITION & self.format) != 0

    def normals_enabled(self):
        return (VertexFormat.NORMAL & self.format) != 0

    def colors_enabled(self):
        return (VertexFormat.COLOR & self.format) != 0

    def bones_enabled(self):
        return ((VertexFormat.BONEWEIGHT & self.format) != 0) and (
            (VertexFormat.BONEINDEX & self.format) != 0
        )

    def tangents_enabled(self):
        return (VertexFormat.TANGENT & self.format) != 0

    def uv1_enabled(self):
        return (VertexFormat.UV1 & self.format) != 0

    def uv2_enabled(self):
        return (VertexFormat.UV2 & self.format) != 0

    def uv3_enabled(self):
        return (VertexFormat.UV3 & self.format) != 0

    def uv4_enabled(self):
        return (VertexFormat.UV4 & self.format) != 0

    def load(self, filepath):
        with open(filepath, "rb") as f:
            self.identifier = read_str(f)

            version = None
            if self.identifier == "ZMS0007":
                version = 7
            elif self.identifier == "ZMS0008":
                version = 8

            if not version:
                raise RoseParseError(f"Unrecognized zms identifier {self.identifier}")

            self.format = read_i32(f)
            self.bounding_box.min = read_vector3_f32(f)
            self.bounding_box.max = read_vector3_f32(f)

            bone_count = read_i16(f)
            for _ in range(bone_count):
                self.bones.append(read_i16(f))

            vert_count = read_i16(f)
            for _ in range(vert_count):
                self.vertices.append(Vertex())

            if self.positions_enabled():
                for i in range(vert_count):
                    self.vertices[i].position = read_vector3_f32(f)

            if self.normals_enabled():
                for i in range(vert_count):
                    self.vertices[i].normal = read_vector3_f32(f)

            if self.colors_enabled():
                for i in range(vert_count):
                    self.vertices[i].color = read_color4(f)

            if self.bones_enabled():
                for i in range(vert_count):
                    self.vertices[i].bone_weights = read_vector4_f32(f)
                    self.vertices[i].bone_indices = read_vector4_i16(f)

            if self.tangents_enabled():
                for i in range(vert_count):
                    self.vertices[i].tangent = read_vector3_f32(f)

            if self.uv1_enabled():
                for i in range(vert_count):
                    self.vertices[i].uv1 = read_vector2_f32(f)

            if self.uv2_enabled():
                for i in range(vert_count):
                    self.vertices[i].uv2 = read_vector2_f32(f)

            if self.uv3_enabled():
                for i in range(vert_count):
                    self.vertices[i].uv3 = read_vector2_f32(f)

            if self.uv4_enabled():
                for i in range(vert_count):
                    self.vertices[i].uv4 = read_vector2_f32(f)

            index_count = read_i16(f)
            for _ in range(index_count):
                self.indices.append(read_vector3_i16(f))

            material_count = read_i16(f)
            for _ in range(material_count):
                self.materials.append(read_i16(f))

            strip_count = read_i16(f)
            for _ in range(strip_count):
                self.strips.append(read_i16(f))

            if version >= 8:
                self.pool = read_i16(f)
