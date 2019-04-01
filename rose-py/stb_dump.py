#!/usr/bin/python3

import sys
from rosepy.stb import *

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: stl_dump <path_to_file>")
        sys.exit(1)

    t = STB()
    
    with open(sys.argv[1], 'rb') as f:
        t.load(f)
