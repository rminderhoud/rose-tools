import os
import unittest

from rose.him import *
from rose.til import *
from rose.zms import *
from rose.zon import *

DIR = os.path.abspath(os.path.dirname(__file__))
DATA_DIR = os.path.join(DIR, "data")


class RoseTests(unittest.TestCase):
    def test_him(self):
        him_file = os.path.join(DATA_DIR, "30_30.him")
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
        til_file = os.path.join(DATA_DIR, "30_30.til")
        t = Til(til_file)

        self.assertEqual(t.width, 16)
        self.assertEqual(t.length, 16)

        self.assertEqual(len(t.tiles), 16)
        for patch in t.tiles:
            self.assertEqual(len(patch), 16)

    def test_zms(self):
        zms_file1 = os.path.join(DATA_DIR, "headbad01.zms")
        zms_file2 = os.path.join(DATA_DIR, "stone014.zms")
        zms_file3 = os.path.join(DATA_DIR, "cart01_ability01.zms")

        zms1 = ZMS()
        zms1.load(zms_file1)

        self.assertEqual(zms1.identifier, "ZMS0008")
        self.assertEqual(zms1.format, 182)
        self.assertEqual(zms1.positions_enabled(), True)
        self.assertEqual(zms1.normals_enabled(), True)
        self.assertEqual(zms1.colors_enabled(), False)
        self.assertEqual(zms1.bones_enabled(), True)
        self.assertEqual(zms1.tangents_enabled(), False)
        self.assertEqual(zms1.uv1_enabled(), True)
        self.assertEqual(zms1.uv2_enabled(), False)
        self.assertEqual(zms1.uv3_enabled(), False)
        self.assertEqual(zms1.uv4_enabled(), False)

        self.assertEqual(len(zms1.bones), 8)
        self.assertEqual(len(zms1.vertices), 336)
        self.assertEqual(len(zms1.indices), 578)
        self.assertEqual(len(zms1.materials), 6)
        self.assertEqual(len(zms1.strips), 0)
        self.assertEqual(zms1.pool, 0)

        zms2 = ZMS()
        zms2.load(zms_file2)

        self.assertEqual(zms2.identifier, "ZMS0007")
        self.assertEqual(zms2.format, 390)
        self.assertEqual(zms2.positions_enabled(), True)
        self.assertEqual(zms2.normals_enabled(), True)
        self.assertEqual(zms2.colors_enabled(), False)
        self.assertEqual(zms2.bones_enabled(), False)
        self.assertEqual(zms2.tangents_enabled(), False)
        self.assertEqual(zms2.uv1_enabled(), True)
        self.assertEqual(zms2.uv2_enabled(), True)
        self.assertEqual(zms2.uv3_enabled(), False)
        self.assertEqual(zms2.uv4_enabled(), False)

        self.assertEqual(len(zms2.bones), 0)
        self.assertEqual(len(zms2.vertices), 131)
        self.assertEqual(len(zms2.indices), 128)
        self.assertEqual(len(zms2.materials), 0)
        self.assertEqual(len(zms2.strips), 0)
        self.assertEqual(zms2.pool, 0)

        zms3 = ZMS()
        zms3.load(zms_file3)

        self.assertEqual(zms3.identifier, "ZMS0008")
        self.assertEqual(zms3.format, 134)
        self.assertEqual(zms3.positions_enabled(), True)
        self.assertEqual(zms3.normals_enabled(), True)
        self.assertEqual(zms3.colors_enabled(), False)
        self.assertEqual(zms3.bones_enabled(), False)
        self.assertEqual(zms3.tangents_enabled(), False)
        self.assertEqual(zms3.uv1_enabled(), True)
        self.assertEqual(zms3.uv2_enabled(), False)
        self.assertEqual(zms3.uv3_enabled(), False)
        self.assertEqual(zms3.uv4_enabled(), False)

        self.assertEqual(len(zms3.bones), 0)
        self.assertEqual(len(zms3.vertices), 544)
        self.assertEqual(len(zms3.indices), 532)
        self.assertEqual(len(zms3.materials), 2)
        self.assertEqual(len(zms3.strips), 0)
        self.assertEqual(zms3.pool, 0)

    def test_zon(self):
        zon_file = os.path.join(DATA_DIR, "jpt01.zon")
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


if __name__ == "__main__":
    unittest.main()
