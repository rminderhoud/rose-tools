# ROSE Converter
Convert ROSE Online files to/from various formats

```
ROSE Converter 0.1.0
Ralph Minderhoud <ralphminderhoud@gmail.com>
Convert ROSE Online files to/from various formats

USAGE:
    rose-conv.exe [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <out_dir>        Directory to output converted files [default: ./out/]

SUBCOMMANDS:
    map            Convert ROSE map files
    serialize      Serialize a ROSE File into JSON (CSV for STB/STL) [aliases: se]
    deserialize    Deserialize a ROSE file from JSON (CSV for STB/STL) [aliases: de]
    help           Prints this message or the help of the given subcommand(s)
```

## Supported formats
**Serialize (to json/csv)**
* idx
* lit
* stb
* til
* zon

**Deserialize (from json/csv)**
* idx
* lit
* stb