use roselib_ffi::*;
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::ptr;

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

        assert_eq!(data_table_rows(stb), 121);
        assert_eq!(data_table_cols(stb), 38);

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

        assert_eq!(skeleton_bones(zmd), 21);
        assert_eq!(skeleton_dummies(zmd), 7);

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
            assert_ne!(position, FfiVector3_f32::default());

            let mut rotation = FfiQuaternion::default();
            let res = skeleton_get_bone_rotation(zmd, 0, &mut rotation as *mut FfiQuaternion);
            assert_eq!(res, true);
            assert_ne!(rotation, FfiQuaternion::default());
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
            assert_ne!(position, FfiVector3_f32::default());

            let mut rotation = FfiQuaternion::default();
            let res = skeleton_get_dummy_rotation(zmd, 0, &mut rotation as *mut FfiQuaternion);
            assert_eq!(res, true);
            assert_ne!(rotation, FfiQuaternion::default());
        }

        skeleton_free(zmd);
    }
}

#[test]
fn read_zms() {
    unsafe {
        let filepath1 = test_file("headbad01.zms");
        let filepath2 = test_file("stone014.zms");
        let filepath3 = test_file("cart01_ability01.zms");

        // Mesh 1
        {
            let zms = mesh_new();

            let res = mesh_read(zms, filepath1.into_raw());
            assert_eq!(res, true);

            assert_eq!(mesh_positions_enabled(zms), true);
            assert_eq!(mesh_normals_enabled(zms), true);
            assert_eq!(mesh_colors_enabled(zms), false);
            assert_eq!(mesh_bones_enabled(zms), true);
            assert_eq!(mesh_tangents_enabled(zms), false);
            assert_eq!(mesh_uv1_enabled(zms), true);
            assert_eq!(mesh_uv2_enabled(zms), false);
            assert_eq!(mesh_uv3_enabled(zms), false);
            assert_eq!(mesh_uv4_enabled(zms), false);

            assert_eq!(mesh_bones(zms), 8);
            assert_eq!(mesh_vertices(zms), 336);
            assert_eq!(mesh_indices(zms), 578);
            assert_eq!(mesh_materials(zms), 6);

            let mut out: i16 = -1;
            let res = mesh_get_bone(zms, 0, &mut out as *mut i16);
            assert_eq!(res, true);
            assert_eq!(out, 4);

            let mut out = FfiVector3_f32::default();
            let res = mesh_get_vertex_position(zms, 0, &mut out as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector3_f32::default());

            let mut out = FfiVector3_f32::default();
            let res = mesh_get_vertex_normal(zms, 0, &mut out as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector3_f32::default());

            assert_eq!(false, mesh_get_vertex_color(zms, 0, ptr::null_mut()));

            let mut out = FfiVector4_f32::default();
            let res = mesh_get_vertex_bone_weight(zms, 0, &mut out as *mut FfiVector4_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector4_f32::default());

            let mut out = FfiVector4_i16::default();
            let res = mesh_get_vertex_bone_indices(zms, 0, &mut out as *mut FfiVector4_i16);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector4_i16::default());

            assert_eq!(false, mesh_get_vertex_tangent(zms, 0, ptr::null_mut()));

            let mut out = FfiVector2_f32::default();
            let res = mesh_get_vertex_uv1(zms, 0, &mut out as *mut FfiVector2_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector2_f32::default());

            assert_eq!(false, mesh_get_vertex_uv2(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_uv3(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_uv4(zms, 0, ptr::null_mut()));

            mesh_free(zms);
        }

        // Mesh 2
        {
            let zms = mesh_new();

            let res = mesh_read(zms, filepath2.into_raw());
            assert_eq!(res, true);

            assert_eq!(mesh_positions_enabled(zms), true);
            assert_eq!(mesh_normals_enabled(zms), true);
            assert_eq!(mesh_colors_enabled(zms), false);
            assert_eq!(mesh_bones_enabled(zms), false);
            assert_eq!(mesh_tangents_enabled(zms), false);
            assert_eq!(mesh_uv1_enabled(zms), true);
            assert_eq!(mesh_uv2_enabled(zms), true);
            assert_eq!(mesh_uv3_enabled(zms), false);
            assert_eq!(mesh_uv4_enabled(zms), false);

            assert_eq!(mesh_bones(zms), 0);
            assert_eq!(mesh_vertices(zms), 131);
            assert_eq!(mesh_indices(zms), 128);
            assert_eq!(mesh_materials(zms), 0);

            assert_eq!(false, mesh_get_bone(zms, 0, ptr::null_mut()));

            let mut out = FfiVector3_f32::default();
            let res = mesh_get_vertex_position(zms, 0, &mut out as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector3_f32::default());

            let mut out = FfiVector3_f32::default();
            let res = mesh_get_vertex_normal(zms, 0, &mut out as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector3_f32::default());

            assert_eq!(false, mesh_get_vertex_color(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_bone_weight(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_bone_indices(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_tangent(zms, 0, ptr::null_mut()));

            let mut out = FfiVector2_f32::default();
            let res = mesh_get_vertex_uv1(zms, 0, &mut out as *mut FfiVector2_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector2_f32::default());

            let mut out = FfiVector2_f32::default();
            let res = mesh_get_vertex_uv2(zms, 0, &mut out as *mut FfiVector2_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector2_f32::default());

            assert_eq!(false, mesh_get_vertex_uv3(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_uv4(zms, 0, ptr::null_mut()));

            mesh_free(zms);
        }

        // Mesh 3
        {
            let zms = mesh_new();

            let res = mesh_read(zms, filepath3.into_raw());
            assert_eq!(res, true);

            assert_eq!(mesh_positions_enabled(zms), true);
            assert_eq!(mesh_normals_enabled(zms), true);
            assert_eq!(mesh_colors_enabled(zms), false);
            assert_eq!(mesh_bones_enabled(zms), false);
            assert_eq!(mesh_tangents_enabled(zms), false);
            assert_eq!(mesh_uv1_enabled(zms), true);
            assert_eq!(mesh_uv2_enabled(zms), false);
            assert_eq!(mesh_uv3_enabled(zms), false);
            assert_eq!(mesh_uv4_enabled(zms), false);

            assert_eq!(mesh_bones(zms), 0);
            assert_eq!(mesh_vertices(zms), 544);
            assert_eq!(mesh_indices(zms), 532);
            assert_eq!(mesh_materials(zms), 2);

            assert_eq!(false, mesh_get_bone(zms, 0, ptr::null_mut()));

            let mut out = FfiVector3_f32::default();
            let res = mesh_get_vertex_position(zms, 0, &mut out as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector3_f32::default());

            let mut out = FfiVector3_f32::default();
            let res = mesh_get_vertex_normal(zms, 0, &mut out as *mut FfiVector3_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector3_f32::default());

            assert_eq!(false, mesh_get_vertex_color(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_bone_weight(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_bone_indices(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_tangent(zms, 0, ptr::null_mut()));

            let mut out = FfiVector2_f32::default();
            let res = mesh_get_vertex_uv1(zms, 0, &mut out as *mut FfiVector2_f32);
            assert_eq!(res, true);
            assert_ne!(out, FfiVector2_f32::default());

            assert_eq!(false, mesh_get_vertex_uv2(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_uv3(zms, 0, ptr::null_mut()));
            assert_eq!(false, mesh_get_vertex_uv4(zms, 0, ptr::null_mut()));

            mesh_free(zms);
        }
    }
}
