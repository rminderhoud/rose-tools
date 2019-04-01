from pathlib import Path

if "bpy" in locals():
    import importlib
else:
    from .rose.zmd import *

import bpy
import mathutils as bmath
from bpy.props import StringProperty, BoolProperty
from bpy_extras.io_utils import ImportHelper


class ImportZMD(bpy.types.Operator, ImportHelper):
    bl_idname = "rose.import_zmd"
    bl_label = "ROSE Armature (.zmd)"
    bl_options = {"PRESET"}

    filename_ext = ".zmd"
    filter_glob = StringProperty(default="*.zmd", options={"HIDDEN"})

    find_animations = BoolProperty(
        name = "Find Animations",
        description = ( "Recursively load any animations (ZMOs) from current "
                        "directory with this armature"),
        default = True,
    )
    
    keep_root_bone = BoolProperty(
        name = "Keep Root bone",
        description = ( "Prevent blender from automatically removing the root "
                        "bone" ),
        default = True,
    )

    animation_extensions = [".ZMO", ".zmo"]

    def execute(self, context):
        filepath = Path(self.filepath)
        filename = filepath.stem
        zmd = ZMD(str(filepath))

        armature = bpy.data.armatures.new(filename)
        obj = bpy.data.objects.new(filename, armature)

        scene = context.scene
        scene.objects.link(obj)
        scene.objects.active = obj

        # Bones can only be added to armature after it is added to scene
        self.bones_from_zmd(zmd, armature)
 
        scene.update()
        return {"FINISHED"}

    def bones_from_zmd(self, zmd, armature):
        bpy.ops.object.mode_set(mode='EDIT')
        
        # Create all bones first so parenting can be done later
        for rose_bone in zmd.bones:
            bone = armature.edit_bones.new(rose_bone.name)
            bone.use_connect = True

        for idx, rose_bone in enumerate(zmd.bones):
            bone = armature.edit_bones[idx]

            pos = bmath.Vector(rose_bone.position.as_tuple())
            rot = bmath.Quaternion(rose_bone.rotation.as_tuple(w_first=True))
            
            if rose_bone.parent_id == -1:
                bone.head = pos
                bone.tail = pos 
                
                if self.keep_root_bone:
                    bone.head.z += 0.00001 # Blender removes 0-length bones
            else:
                bone.parent = armature.edit_bones[rose_bone.parent_id]

                #pos = bone.head + pos
                #pos.rotate(rot)
                #bone.tail = pos
                
                parent_pos = bone.parent.tail.copy()
                #parent_pos.rotate(rot)
                #pos.rotate(rot)
                pos = parent_pos + pos
                bone.tail = pos

                #p_matrix = bmath.Matrix(bone.parent.matrix)
                #p_pos = p_matrix.to_translation()
                #p_rot = p_matrix.to_quaternion()
                #pos = p_pos + pos
                #rot = p_rot * rot
                #bone.tail = pos

                #p = zmd.bones[rose_bone.parent_id]
                #p_pos = bmath.Vector(p.position.as_tuple())
                #p_rot = bmath.Quaternion(p.rotation.as_tuple(w_first=True))
                #pos = p_pos + pos
                #rot = p_rot * rot
                #pos.rotate(rot)
                #bone.tail = pos

                #pos.rotate(rot)
                #bone.tail = pos

                # -- Matrix attempt
                #trans_mat = bmath.Matrix.Translation(pos)
                #axis, angle = rot.to_axis_angle()
                #rot_mat = bmath.Matrix.Rotation(angle, 4, axis)
                #bone.tail = (trans_mat * rot_mat).to_translation()

        bpy.ops.object.mode_set(mode='OBJECT')

        return armature
