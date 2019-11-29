//! Sqlite virtual table module for ROSE Data Files (STB)
use std::os::raw::c_int;
use std::path::{Path, PathBuf};
use std::str;

use lazy_static::lazy_static;
use rusqlite::types::Null;
use rusqlite::vtab::{
    dequote, read_only_module, sqlite3_vtab, sqlite3_vtab_cursor, Context, CreateVTab, IndexInfo,
    Module, VTab, VTabConnection, VTabCursor, Values,
};
use rusqlite::{Connection, Error, Result};

use crate::files::STB;
use crate::io::RoseFile;
use crate::sqlite::DEFAULT_SCHEMAS;

lazy_static! {
    static ref STBTAB_MODULE: Module<STBTab> = read_only_module::<STBTab>(1);
}

/// Register the "stb" module.
///
/// # Usage
/// ```sql
/// CREATE VIRTUAL TABLE vtab USING stb(
///   filename=FILENAME -- Path the the STB file
///   [, schema=SCHEMA] -- Alternative STB schema. 'CREATE TABLE x(col1 TEXT NOT NULL, col2 INT, ...);'
///   [, use_default=YES|NO] - Search for a default pre-generated header based on file name if "yes". Default "yes".
///   [, use_header=YES|NO] -- Used headers to generate schema if "yes". Default "no".
/// );
/// ```
///
/// ```norun
/// let db = Connection::open_in_memory().unwrap();
/// stbtab::load_module(&db).unwrap();
///
/// db.execute_batch("CREATE VIRTUAL TABLE list_zone USING stb(filename='list_zone.stb')").unwrap();
/// {
///    let mut s = db.prepare("SELECT rowid, * FROM list_zone").unwrap();
///    let ids: Vec<String> = s
///        .query_map(NO_PARAMS, |row| row.get::<_, String>(2))
///        .unwrap()
///        .map(|r| r.unwrap())
///        .collect();
///
///    assert_eq!(ids[0], "Canyon City of Zant");
/// }
/// db.execute_batch("DROP TABLE list_zone").unwrap();
/// ```
///
/// This module will first try to use the schema passed into `schema`.
/// If no schema is provided then it will fall back to using a default schema.
/// If `use_default` is `NO` or a default schema could not be found then it will
/// fall back to using the headers to generate a schema with each field being of
/// `TEXT` affinity.
///
/// **NOTE:** Using `use_header` on the original ROSE files can result in Korean
/// or other language headers.
///
pub fn load_module(conn: &Connection) -> Result<()> {
    let aux: Option<()> = None;
    conn.create_module("stb", &STBTAB_MODULE, aux)
}

#[repr(C)]
#[derive(Default)]
struct STBTab {
    base: sqlite3_vtab,
    filename: PathBuf,
}

impl STBTab {
    fn parse_parameter(c_slice: &[u8]) -> Result<(&str, &str)> {
        let arg = str::from_utf8(c_slice)?.trim();
        let mut split = arg.split('=');
        if let Some(key) = split.next() {
            if let Some(value) = split.next() {
                let param = key.trim();
                let value = dequote(value);
                return Ok((param, value));
            }
        }
        Err(Error::ModuleError(format!("illegal argument: '{}'", arg)))
    }
}

impl VTab for STBTab {
    type Aux = ();
    type Cursor = STBTabCursor;

    fn connect(
        _: &mut VTabConnection,
        _aux: Option<&()>,
        args: &[&[u8]],
    ) -> Result<(String, STBTab)> {
        if args.len() < 4 {
            return Err(Error::ModuleError("No STB file specified".to_owned()));
        }

        let mut vtab = STBTab::default();
        let mut schema = String::new();

        let mut use_default = true;
        let mut use_header = false;

        let args = &args[3..];
        for c_slice in args {
            let (param, value) = STBTab::parse_parameter(c_slice)?;
            match param {
                "filename" => vtab.filename = PathBuf::from(value),
                "schema" => schema = value.to_string(),
                "use_default" => use_default = value == "YES" || value == "yes",
                "use_header" => use_header = value == "YES" || value == "yes",
                _ => {
                    return Err(Error::ModuleError(format!(
                        "Unrecognized parameter: '{}'",
                        param
                    )));
                }
            }
        }

        if !vtab.filename.exists() {
            return Err(Error::ModuleError(format!(
                "File does not exist: {}",
                vtab.filename.display()
            )));
        }

        if !schema.is_empty() {
            return Ok((schema, vtab));
        }

        if !use_default && !use_header {
            return Err(Error::ModuleError(
                "A schema was not provided but `use_default` and `use_header` are disabled"
                    .to_string(),
            ));
        }

        if use_default {
            let filename = vtab
                .filename
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_lowercase();

            for default_schema in DEFAULT_SCHEMAS.iter() {
                if filename == default_schema.0 {
                    return Ok((default_schema.1.to_string(), vtab));
                }
            }
        }

        if !use_header {
            return Err(Error::ModuleError(
                "A schema was not provided, a default schema could not be found and `use_header` is not enabled."
                    .to_string(),
            ));
        }

        let mut stb = STB::from_path(&vtab.filename).map_err(|e| {
            Error::ModuleError(format!(
                "Failed to open the STB file at {}: {}",
                &vtab.filename.display(),
                e
            ))
        })?;

        let mut sorted_headers: Vec<(usize, String)> = stb
            .headers
            .iter()
            .enumerate()
            .map(|t| (t.0, String::from(t.1)))
            .collect();
        sorted_headers.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut dupe_count = 0;
        for i in 0..sorted_headers.len() - 1 {
            let (_, cur) = &sorted_headers[i];
            let (next_idx, next) = &sorted_headers[i + 1];
            if cur == next {
                dupe_count += 1;
                stb.headers[*next_idx].push_str(&format!("_{}", dupe_count));
            } else {
                dupe_count = 0;
            }
        }

        let mut schema = String::from("CREATE TABLE x(");
        for (idx, header) in stb.headers.iter().enumerate() {
            schema.push_str(&format!("\"{}\" TEXT", header));
            if idx < stb.headers.len() - 1 {
                schema.push(',');
            }
        }
        schema.push_str(");");
        Ok((schema, vtab))
    }

    fn best_index(&self, _info: &mut IndexInfo) -> Result<()> {
        Ok(())
    }

    fn open(&self) -> Result<STBTabCursor> {
        STBTabCursor::open(&self.filename)
    }
}

impl CreateVTab for STBTab {}

#[repr(C)]
#[derive(Default)]
struct STBTabCursor {
    base: sqlite3_vtab_cursor,
    stb: STB,
    current_row: usize,
}

impl STBTabCursor {
    pub fn open(path: &Path) -> Result<STBTabCursor> {
        let mut vtab = STBTabCursor::default();

        if !path.exists() {
            return Err(Error::ModuleError(format!(
                "File does not exist: {}",
                path.display()
            )));
        }

        vtab.stb.read_from_path(path).map_err(|e| {
            Error::ModuleError(format!(
                "Failed to open the STB file at {}: {}",
                path.display(),
                e
            ))
        })?;

        Ok(vtab)
    }
}
impl VTabCursor for STBTabCursor {
    fn filter(
        &mut self,
        _idx_num: c_int,
        _idx_str: Option<&str>,
        _args: &Values<'_>,
    ) -> Result<()> {
        self.current_row = 0;
        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.current_row += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.current_row >= self.stb.rows()
    }

    fn column(&self, ctx: &mut Context, col: c_int) -> Result<()> {
        if col < 0 || col as usize >= self.stb.cols() {
            return Err(Error::ModuleError(format!(
                "Column index out of bounds: {}",
                col
            )));
        }

        if self.current_row >= self.stb.rows() {
            return Err(Error::ModuleError(format!(
                "Row index out of bounds: {}",
                self.current_row
            )));
        }

        let val = &self.stb.data[self.current_row as usize][col as usize];
        if val.is_empty() {
            return ctx.set_result(&Null);
        }

        return ctx.set_result(&val);
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.current_row as i64)
    }
}

#[cfg(test)]
mod test {
    use rusqlite::{params, Connection, Error, NO_PARAMS};
    use std::path::PathBuf;

    use crate::files::STB;
    use crate::io::RoseFile;
    use crate::sqlite::stbtab;

    #[test]
    fn test_stbtab_module() {
        let db = Connection::open_in_memory().unwrap();
        stbtab::load_module(&db).unwrap();

        let mut stb_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        stb_file.push("tests");
        stb_file.push("data");
        stb_file.push("list_zone.stb");

        let stb = STB::from_path(&stb_file).unwrap();

        let q_use_default = format!(
            "CREATE VIRTUAL TABLE list_zone USING stb(filename='{}')",
            stb_file.to_str().unwrap()
        );
        let q_use_header = format!(
            "CREATE VIRTUAL TABLE list_zone USING stb(filename='{}', use_default=NO, use_header=YES)",
            stb_file.to_str().unwrap()
        );

        let q_schema = format!(
            "CREATE VIRTUAL TABLE list_zone USING stb(filename='{}', schema='{}')",
            stb_file.to_str().unwrap(),
            "CREATE TABLE x(field1 TEXT)"
        );

        test_stbtab(&db, &q_use_default, stb.headers.len() + 1, stb.rows());
        test_stbtab(&db, &q_use_header, stb.headers.len() + 1, stb.rows());
        test_stbtab(&db, &q_schema, 2, stb.rows());

        test_stbtab_cursor(&db, &q_use_header, &stb);
    }

    fn test_stbtab(db: &Connection, query: &str, cols: usize, rows: usize) {
        db.execute_batch(query).unwrap();
        {
            let mut s = db.prepare("SELECT rowid, * FROM list_zone").unwrap();

            let headers = s.column_names();
            assert_eq!(headers.len(), cols);

            let ids: Vec<i32> = s
                .query_map(NO_PARAMS, |row| row.get::<_, i32>(0))
                .unwrap()
                .map(|r| r.unwrap())
                .collect();
            assert_eq!(ids.len(), rows);
        }
        db.execute_batch("DROP TABLE list_zone").unwrap();
    }

    fn test_stbtab_cursor(db: &Connection, query: &str, stb: &STB) {
        db.execute_batch(query).unwrap();
        {
            for row_idx in 0..stb.rows() {
                let row = db.query_row(
                    "SELECT row_idx, * FROM list_zone WHERE rowid = ?",
                    params![row_idx as u32],
                    |row| {
                        assert_eq!(row_idx as u32, row.get_unwrap(0));
                        for col_idx in 1..row.column_count() {
                            let val = row.get_unwrap::<_, String>(col_idx);
                            assert_eq!(stb.value(row_idx, col_idx - 1).unwrap(), val);
                        }
                        Ok(())
                    },
                );
            }
        }
        db.execute_batch("DROP TABLE list_zone").unwrap();
    }
}
