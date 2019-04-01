from .utils import *

class Bone:
    def __init__(self):
        parent_id = -1
        name = ""
        position = Vector3(0.0, 0.0, 0.0)
        rotation = Quat(0.0, 0.0, 0.0, 0.0)

class ZMD:
    def __init__(self, filepath=None):
        self.bones = []

        if filepath:
            with open(filepath, "rb") as f:
                self.read(f)

    def read(self, f):
        identifier = read_fstr(f, 7)

        bone_count = read_u32(f)

        for i in range(bone_count):
            bone = Bone()
            bone.parent_id = read_i32(f)
            bone.name = read_str(f)
            bone.position = read_vector3_f32(f).scalar(0.01)
            bone.rotation = read_quat_wxyz(f)
            
            if i == 0:
                bone.parent_id = -1

            self.bones.append(bone)
