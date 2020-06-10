from .utils import *

class STB:
    def __init__(self):
        self.identifier = ""

    def load(self, f):
        self.identifier = read_fstr(f, 4)
        
        f.seek(4, SEEK_CUR)

        row_count = read_i32(f)
        col_count = read_i32(f)
        row_height = read_i32(f)
        
        root_col_width = read_i16(f)
        for i in range(col_count):
            column_width = read_i16(f)
        
        root_col_name = read_sstr(f)
        for i in range(col_count):
            col_name = read_sstr(f)


        for i in range(row_count - 1):
            for j in range(1, col_count):
                cell = read_sstr(f)
                # TODO: Store this in some structure (list of lists?)
                print(cell)


