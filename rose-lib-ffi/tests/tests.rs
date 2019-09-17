use std::ffi::CString;
use std::path::{Path, PathBuf};
use roselib_ffi::*;

fn data_dir() -> PathBuf {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = project_dir.parent().unwrap_or(Path::new(""));
    let mut data_dir = PathBuf::from(workspace_dir);
    data_dir.push("rose-lib");
    data_dir.push("tests");
    data_dir.push("data");
    data_dir
}

#[test]
fn read_stb() {
    unsafe {
        let stb = data_table_new();

        let filepath = CString::new(data_dir().join("list_zone.stb").to_string_lossy().as_bytes()).unwrap_or_default();
        let res = data_table_read(stb, filepath.into_raw());
        assert_eq!(res, true);

        let rows = data_table_rows(stb);
        assert_eq!(rows, 121);

        let cols = data_table_cols(stb);
        assert_eq!(cols, 38);

        let data = ffi_string_new();

        let res = data_table_get_header(stb, 2, data);
        assert_eq!(res, true);
        assert_eq!((*data).to_string(), "ZON");

        let res = data_table_get_data(stb, 2, 2, data);
        assert_eq!(res, true);
        assert_eq!((*data).to_string(), "3DDATA\\Maps\\Junon\\JPT01\\JPT01.zon");

        ffi_string_free(data);

        data_table_free(stb);
    }
}