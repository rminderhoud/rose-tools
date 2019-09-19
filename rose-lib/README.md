# ROSE Online Rust Library (roselib)
A Rust library for working with ROSE Online's file formats.


## Library
This crate provides a Rust library that can be used in other projects. See
the documentation for more information

## Usage
Add `roselib` as a dependency in your `Cargo.toml`
```toml
[dependencies]
roselib="../path/to/rose-lib"

```
Use it in your project
```rust
extern crate roselib;

use std::path::Path;
use roselib::files::IDX;

let idx = IDX::from_path(Path::new("/path/to/index.idx")).unwrap();

for vfs in idx.file_systems {
  for vfs_file in vfs.files {
    println!("File: {}", vfs_file.filepath);
  }
}
```

### Supported File formats
* HIM - ROSE Heightmap [Read-only]
* IDX (VFS) - ROSE Virtual filesystem
* LIT - ROSE Lightmap
* STB - ROSE Data table
* TIL - ROSE Map Tile [Read-only]
* ZMD - ROSE Skeleton
* ZMS - ROSE 3D Mesh
* ZON - ROSE Zone data [Read-only]

## Compatibility
* This code has only been tested against rose_129_129en and is not guaranteed 
to work with other versions of ROSE Online (e.g. naRose, jRose, etc.)
* Older versions of ROSE Online used the EUC-KR encoding for strings. This lib
converts strings to UTF-8 lossily. See [here](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)
for more information.

## Acknowledgements
Inspired by Jack Wakefield's [Revise](https://github.com/jackwakefield/Revise) 
library and all the contributors at [osRose](http://forum.dev-osrose.com/).
