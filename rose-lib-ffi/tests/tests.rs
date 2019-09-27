use roselib_ffi::*;
use std::ffi::CString;
use std::path::{Path, PathBuf};

fn data_dir() -> PathBuf {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = project_dir.parent().unwrap_or(Path::new(""));
    let mut data_dir = PathBuf::from(workspace_dir);
    data_dir.push("rose-lib");
    data_dir.push("tests");
    data_dir.push("data");
    data_dir
}

fn test_file(name: &str) -> CString {
    CString::new(data_dir().join(name).to_string_lossy().as_bytes()).unwrap_or_default()
}

#[test]
fn read_stb() {
    unsafe {
        let filepath = test_file("list_zone.stb");
        let stb = data_table_new();

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

#[test]
fn read_zmd() {
    unsafe {
        let filepath = test_file("male.zmd");
        let zmd = skeleton_new();

        let res = skeleton_read(zmd, filepath.into_raw());
        assert_eq!(res, true);

        let bones = skeleton_bones(zmd);
        assert_eq!(bones, 21);

        let dummies = skeleton_dummies(zmd);
        assert_eq!(dummies, 7);

        // Bones
        {
            let name = ffi_string_new();
            let res = skeleton_get_bone_name(zmd, 0, name);
            assert_eq!(res, true);
            ffi_string_free(name);

            let mut parent: i32 = -1;
            let res = skeleton_get_bone_parent(zmd, 0, &mut parent as *mut i32);
            assert_eq!(res, true);
            assert_eq!(parent, 0);

            let mut position = FfiVector3_f32::default();
            let res = skeleton_get_bone_position(zmd, 0, &mut position as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert!(position.x <= 0.0 && position.x > -0.1);
            assert!(position.y <= 0.1 && position.y > 0.0);
            assert!(position.z <= 73.1 && position.z > 73.0);

            let mut rotation = FfiQuaternion::default();
            let res = skeleton_get_bone_rotation(zmd, 0, &mut rotation as *mut FfiQuaternion);
            assert_eq!(res, true);
            assert!(rotation.w <= 0.1 && rotation.w > 0.0);
            assert!(rotation.x <= -0.7 && rotation.x > -0.8);
            assert!(rotation.y <= 0.0 && rotation.y > -0.1);
            assert!(rotation.z <= -0.7 && rotation.z > -0.8);
        }

        // Dummies
        {
            let name = ffi_string_new();
            let res = skeleton_get_dummy_name(zmd, 0, name);
            assert_eq!(res, true);
            ffi_string_free(name);

            let mut parent: i32 = -1;
            let res = skeleton_get_dummy_parent(zmd, 0, &mut parent as *mut i32);
            assert_eq!(res, true);
            assert_eq!(parent, 12);

            let mut position = FfiVector3_f32::default();
            let res = skeleton_get_dummy_position(zmd, 0, &mut position as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert!(position.x <= 8.3 && position.x > 8.2);
            assert!(position.y <= 1.6 && position.y > 1.5);
            assert!(position.z <= 3.4 && position.z > 3.3);

            let mut rotation = FfiQuaternion::default();
            let res = skeleton_get_dummy_rotation(zmd, 0, &mut rotation as *mut FfiQuaternion);
            dbg!(&rotation);
            assert_eq!(res, true);
            assert!(rotation.w <= 1.0 && rotation.w > 0.9);
            assert!(rotation.x <= 0.1 && rotation.x > 0.0);
            assert!(rotation.y <= 0.1 && rotation.y > 0.0);
            assert!(rotation.z <= 0.0 && rotation.z > -0.1);
        }

        skeleton_free(zmd);
    }
}
