use std::convert::TryInto;
use std::ffi::CStr;
use std::path::PathBuf;
use roselib::files::stb::DataTable;
use roselib::io::RoseFile;
use crate::utils::FfiString;

#[no_mangle]
pub unsafe extern fn data_table_new() -> *mut DataTable {
    Box::into_raw(Box::new(DataTable::new()))
}

#[no_mangle]
pub unsafe extern fn data_table_free(data_table: *mut DataTable) {
    Box::from_raw(data_table); // Drop
}

#[no_mangle]
pub unsafe extern fn data_table_read(data_table: *mut DataTable, path: *const libc::c_char) -> bool {
    let mut stb = Box::from_raw(data_table);

    let path_str = CStr::from_ptr(path).to_str().unwrap_or_default();
    let p = PathBuf::from(path_str);

    let res = stb.read_from_path(&p).is_ok();

    std::mem::forget(stb);
    res
}

#[no_mangle]
pub unsafe extern fn data_table_rows(data_table: *mut DataTable) -> libc::c_int {
    let stb: Box<DataTable> = Box::from_raw(data_table);
    let rows = stb.rows() as libc::c_int;
    std::mem::forget(stb);
    rows
}

#[no_mangle]
pub unsafe extern fn data_table_cols(data_table: *mut DataTable) -> libc::c_int {
    let stb: Box<DataTable> = Box::from_raw(data_table);
    let cols = stb.cols() as libc::c_int;
    std::mem::forget(stb);
    cols
}

#[no_mangle]
pub unsafe extern fn data_table_get_header(data_table: *mut DataTable, idx: libc::c_int, _out: *mut FfiString) -> bool {
    let stb: Box<DataTable> = Box::from_raw(data_table);

    let new_idx = idx.try_into().unwrap_or(0 as usize);

    let mut s = Box::from_raw(_out);
    let mut result = false;

    let val = stb.header(new_idx);
    if let Some(v) = val {
        s.set_string(&v);
        result = true;
    }

    std::mem::forget(s);
    std::mem::forget(stb);
    result
}

#[no_mangle]
pub unsafe extern fn data_table_get_data(data_table: *mut DataTable, row: libc::c_int, col: libc::c_int, _out: *mut FfiString) -> bool {
    let stb: Box<DataTable> = Box::from_raw(data_table);

    let new_row = row.try_into().unwrap_or(0 as usize);
    let new_col = col.try_into().unwrap_or(0 as usize);

    let mut s = Box::from_raw(_out);
    let mut result = false;

    let val = stb.value(new_row, new_col);
    if let Some(v) = val {
        s.set_string(&v);
        result = true;
    }

    std::mem::forget(s);
    std::mem::forget(stb);
    result
}
