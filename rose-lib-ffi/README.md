# Roselib FFI
Foreign function interface for the [roselib](../rose-lib) library.

This library wraps [roselib](../rose-lib) and exposes some of its functionality using Rust FFI. The
functionality is exposed as a C-API with a generated header file and corresponding 
static/dynamic libraries. 

Please refer to the README in [roselib](../rose-lib) for more information about the purpose of this
library.

**NOTE** The current focus on this library is for wrapping only *read* functionality.

## Usage
Please refer to the test file(s) for general usage.

To use as part of a C/C++ program, first build the library with `cargo build`. This will create
the libraries in the `target/` directory and the header file in `include/`. Include/link these
in your project as needed.

## Example
```c
#include <string.h>
#include "roselib.h"

DataTable* stb = data_table_new();

bool res = data_table_read(stb, "list_zone.stb");
assert(res == true);

int rows = data_table_rows(stb);
assert(rows == 121);

int cols = data_table_cols(stb);
assert(cols == 38);

FfiString* data = ffi_string_new();

res = data_table_get_header(stb, 2, data);
assert(res == true);
assert(strcmp(res.chars, "ZON") == 0);

res = data_table_get_data(stb, 2, 2, data);
assert(res == true);
assert(strcmp(res.chars, ""3DDATA\\Maps\\Junon\\JPT01\\JPT01.zon"") == 0);

ffi_string_free(data);
data_table_free(stb);
```