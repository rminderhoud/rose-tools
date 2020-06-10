from .utils import *

class STL:
    def __init__(self):
        self.type = ""
        self.lang_count = 0

        self.keys = {}
        self.rows = []

    def load(self, filepath):
        with open(filepath, 'rb') as f:
            self.type = read_bstr(f)

            row_count = read_i32(f)
            for i in range(row_count):
                key = read_bstr(f)
                id = read_i32(f)
                self.keys[id] = key

            lang_count = read_i32(f)
            for i in range(lang_count):
                lang_offset = read_i32(f)
                next_lang_offset = f.tell()

                for j in range(row_count):
                    row_offset = read_i32(f)
                    next_row_offset = f.tell()

                    row = {}
                    row["text"] = read_bstr(f)
                    
                    if self.type == "ITST01" or self.type == "QEST01":
                        row["description"] = read_bstr(f)

                        if self.type == "QEST01":
                            row["start_message"] = read_bstr(f)
                            row["end_message"] = read_bstr(f)

                    self.rows.append(row)
                    
                    if j < row_count - 1:
                        f.seek(next_row_offset)

                if i < lang_count - 1:
                    f.seek(next_lang_offset)







