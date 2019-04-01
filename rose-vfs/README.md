# VFS Extractor
A tool to extract content from a ROSE Online VFS System

## Build
`cargo build --release`

## Usage
```
Extracts content from a ROSE Online VFS system

USAGE:
    vfs_extractor [FLAGS] [OPTIONS] <idx>

FLAGS:
    -d, --dry-run    Executes program without modifying any files
    -f, --flat       Ignore hierarchy and extract all files to top-level of out dir
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints extra information

OPTIONS:
    -i, --include <include>...    Only extract files with these extensions
    -o <out_dir>                  Directory to output extracted files [default: out]

ARGS:
    <idx>    Path to .idx file to load
```

Example:
```
./vfs_extractor /path/to/data.idx -f -i zms zmd zmo
./vfs_extractor C:\\path\to\data.idx -f -i zms zmd zmo
```

Output:
```
File loaded: "data.idx"
Version: 129_129
Loaded DATA.VFS: 3193 files indexed
1431 files extracted
Loaded MAP.VFS: 11053 files indexed
0 files extracted
Loaded GROUND.VFS: 2206 files indexed
1527 files extracted
Loaded 3DDATA.VFS: 3163 files indexed
1693 files extracted
Loaded BASIC.VFS: 93 files indexed
1 files extracted
Unable to open ROOT.VFS: No such file or directory (os error 2)
```
