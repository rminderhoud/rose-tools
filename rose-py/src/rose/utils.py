import struct

def list_2d(width, length, default=None):
    """ Create a 2-dimensional list of width x length """
    return [[default] * width for i in range(length)]

class Vector2:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y

    def __repr__(self):
        return "Vector2({},{})".format(self.x, self.y)

class Vector3:
    def __init__(self):
        self.x = 0
        self.y = 0
        self.z = 0

    def __repr__(self):
        return f"Vector3({self.x}, {self.y}, {self.z}"

class Color4:
    def __init__(self):
        self.r = 0
        self.g = 0
        self.b = 0
        self.a = 0

class BoundingBox:
    def __init__(self):
        self.min = Vector3()
        self.max = Vector3()

def read_i8(f):
    return struct.unpack("b", f.read(1))[0]

def read_i16(f):
    return struct.unpack("<h", f.read(2))[0]

def read_i32(f):
    return struct.unpack("<i", f.read(4))[0]

def read_u8(f):
    return struct.unpack("B", f.read(1))[0]

def read_u16(f):
    return struct.unpack("<H", f.read(2))[0]

def read_u32(f):
    return struct.unpack("<I", f.read(4))[0]

def read_f32(f):
    return struct.unpack("<f", f.read(4))[0]

def read_bool(f):
    return struct.unpack("?", f.read(1))[0]

def read_bstr(f):
    """ Read byte-prefixed string """
    size = struct.unpack("B", f.read(1))[0]
    if size == 0:
        return ""

    bstring = f.read(size)
    return bstring.decode("EUC-KR")

def read_sstr(f):
    """ read u16-prefix string """
    return read_fstr(f, read_i16(f))

def read_str(f):
    """ Read null-terminated string """
    bstring = bytes("", encoding="EUC-KR")
    while True:
        byte = f.read(1)
        if byte == b"\x00":
            break
        else:
            bstring += byte
    return bstring.decode("EUC-KR")

def read_fstr(f, size):
    """ Read fixed-size string """
    return f.read(size).decode("EUC-KR")

def read_vector3_f32(f):
    """ Read vector3 of floats """
    v = Vector3()
    v.x = read_f32(f)
    v.y = read_f32(f)
    v.z = read_f32(f)
    return v

class RoseParseError(Exception):
    ...