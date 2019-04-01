import os
import unittest
from rose.him import *
from rose.til import *
from rose.zon import *

DIR = os.path.abspath(os.path.dirname(__file__))
DATA_DIR = os.path.join(DIR, "data")

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
