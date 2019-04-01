import os
import sys
import unittest

DIR = os.path.abspath(os.path.dirname(__file__))
ROOT_DIR = os.path.dirname(DIR)
DATA_DIR = os.path.join(DIR, "data")

# Manually manipulate path so avoid `bpy` imports in `io_rose` module
sys.path.append(os.path.join(ROOT_DIR, "io_rose"))

from rose.him import *
from rose.til import *
from rose.zmd import *
from rose.zms import *
from rose.zon import *

class RoseTests(unittest.TestCase):
    def test_him(self):
        him_file = os.path.join(DATA_DIR, "30_30.HIM")
        h = Him(him_file)

        self.assertEqual(h.width, 65)
        self.assertEqual(h.length, 65)
        self.assertEqual(h.grid_count, 4)
        self.assertEqual(h.patch_scale, 250.0)

        self.assertEqual(len(h.heights), 65)
        self.assertEqual(int(h.max_height), 8234)
        self.assertEqual(int(h.min_height), -500)

        self.assertEqual(len(h.patches), 16)
        for patch in h.patches:
            self.assertEqual(len(patch), 16)

        self.assertEqual(len(h.quad_patches), 85)
    
    def test_til(self):
        til_file = os.path.join(DATA_DIR, "30_30.TIL")
        t = Til(til_file)

        self.assertEqual(t.width, 16)
        self.assertEqual(t.length, 16)

        self.assertEqual(len(t.tiles), 16)
        for patch in t.tiles:
            self.assertEqual(len(patch), 16)
    
    def test_zmd(self):
        zmd_file = os.path.join(DATA_DIR, "MALE.ZMD")
        zmd = ZMD(zmd_file)

        self.assertEqual(len(zmd.bones), 21)

    def test_zms(self):
        zms7 = os.path.join(DATA_DIR, "FACE1_00100.ZMS")
        zms = ZMS(zms7)

        self.assertEqual(zms.identifier, "ZMS0007")

        self.assertEqual(zms.flags, 134)
        self.assertEqual(zms.positions_enabled(), True)
        self.assertEqual(zms.normals_enabled(), True)
        self.assertEqual(zms.bones_enabled(), False)
        self.assertEqual(zms.tangents_enabled(), False)
        self.assertEqual(zms.uv1_enabled(), True)
        self.assertEqual(zms.uv2_enabled(), False)
        self.assertEqual(zms.uv3_enabled(), False)
        self.assertEqual(zms.uv4_enabled(), False)
        
        self.assertEqual(len(zms.vertices), 183)
        self.assertEqual(len(zms.indices), 292)
        self.assertEqual(len(zms.bones), 0)
        self.assertEqual(len(zms.materials), 3)
        self.assertEqual(len(zms.strips), 0)
        self.assertEqual(zms.pool, 0)

        zms8 = os.path.join(DATA_DIR, "BODY1_00100.ZMS")
        zms = ZMS(zms8)

        self.assertEqual(zms.identifier, "ZMS0008")

        self.assertEqual(zms.flags, 182)
        self.assertEqual(zms.positions_enabled(), True)
        self.assertEqual(zms.normals_enabled(), True)
        self.assertEqual(zms.bones_enabled(), True)
        self.assertEqual(zms.tangents_enabled(), False)
        self.assertEqual(zms.uv1_enabled(), True)
        self.assertEqual(zms.uv2_enabled(), False)
        self.assertEqual(zms.uv3_enabled(), False)
        self.assertEqual(zms.uv4_enabled(), False)
        
        self.assertEqual(len(zms.vertices), 175)
        self.assertEqual(len(zms.indices), 258)
        self.assertEqual(len(zms.bones), 12)
        self.assertEqual(len(zms.materials), 0)
        self.assertEqual(len(zms.strips), 474)
        self.assertEqual(zms.pool, 0)

    def test_zon(self):
        zon_file = os.path.join(DATA_DIR, "JPT01.ZON")
        z = Zon(zon_file)
        
        self.assertEqual(z.zone_type, ZoneType.BoatVillage)
        self.assertEqual(z.width, 64)
        self.assertEqual(z.length, 64)
        self.assertEqual(z.grid_count, 4)
        self.assertEqual(z.grid_size, 250.0)
        
        self.assertEqual(len(z.positions), 64)
        for pos in z.positions:
            self.assertEqual(len(pos), 64)

        self.assertEqual(len(z.spawns), 6)
        self.assertEqual(len(z.textures), 49)
        self.assertEqual(len(z.tiles), 224)
        
        self.assertEqual(z.name, "0")
        self.assertEqual(z.is_underground, False)
        self.assertEqual(z.background_music_path, "button1")
        self.assertEqual(z.sky_path, "button2")
        self.assertEqual(z.economy_check_rate, 20)
        self.assertEqual(z.population_base, 6000)
        self.assertEqual(z.population_growth_rate, 50)
        self.assertEqual(z.metal_consumption, 15)
        self.assertEqual(z.stone_consumption, 15)
        self.assertEqual(z.wood_consumption, 5)
        self.assertEqual(z.leather_consumption, 10)
        self.assertEqual(z.cloth_consumption, 10)
        self.assertEqual(z.alchemy_consumption, 5)
        self.assertEqual(z.chemical_consumption, 5)
        self.assertEqual(z.industrial_consumption, 10)
        self.assertEqual(z.medicine_consumption, 5)
        self.assertEqual(z.food_consumption, 10)
