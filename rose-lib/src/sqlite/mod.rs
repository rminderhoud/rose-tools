//! A module for interoperability with sqlite

pub mod stbtab;

const DEFAULT_SCHEMAS: [(&str, &str); 1] =
    [("list_zone.stb", include_str!("schemas/list_zone.sql"))];
