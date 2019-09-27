use crate::utils::*;
use roselib::files::zms::Mesh;
use roselib::io::RoseFile;
use std::ffi::CStr;
use std::path::PathBuf;

#[no_mangle]
pub unsafe extern "C" fn mesh_new() -> *mut Mesh {
    Box::into_raw(Box::new(Mesh::new()))
}

#[no_mangle]
pub unsafe extern "C" fn mesh_free(mesh: *mut Mesh) {
    Box::from_raw(mesh); //Drop
}

#[no_mangle]
pub unsafe extern "C" fn mesh_read(mesh: *mut Mesh, path: *const libc::c_char) -> bool {
    let mut zms = Box::from_raw(mesh);

    let path_str = CStr::from_ptr(path).to_str().unwrap_or_default();
    let p = PathBuf::from(path_str);

    let res = zms.read_from_path(&p).is_ok();

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_bones(mesh: *mut Mesh) -> u32 {
    let zms = Box::from_raw(mesh);
    let res = zms.bones.len() as u32;
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_vertices(mesh: *mut Mesh) -> u32 {
    let zms = Box::from_raw(mesh);
    let res = zms.vertices.len() as u32;
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_indices(mesh: *mut Mesh) -> u32 {
    let zms = Box::from_raw(mesh);
    let res = zms.indices.len() as u32;
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_materials(mesh: *mut Mesh) -> u32 {
    let zms = Box::from_raw(mesh);
    let res = zms.materials.len() as u32;
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_positions_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.positions_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_normals_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.normals_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_colors_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.colors_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_bones_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.bones_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_tangents_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.tangents_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_uv1_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.uv1_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_uv2_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.uv2_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_uv3_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.uv3_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_uv4_enabled(mesh: *mut Mesh) -> bool {
    let zms = Box::from_raw(mesh);
    let res = zms.uv4_enabled();
    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_bone(mesh: *mut Mesh, idx: u32, _out: *mut i16) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let res = if idx < zms.bones.len() {
        *_out = zms.bones[idx];
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_position(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector3_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.positions_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].position;
        (*_out).x = v.x;
        (*_out).y = v.y;
        (*_out).z = v.z;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_normal(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector3_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.normals_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].normal;
        (*_out).x = v.x;
        (*_out).y = v.y;
        (*_out).z = v.z;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_color(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiColor4,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.colors_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let c = zms.vertices[idx].color;
        (*_out).r = c.r;
        (*_out).g = c.g;
        (*_out).b = c.b;
        (*_out).a = c.a;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_bone_weight(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector4_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.bones_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].bone_weights;
        (*_out).w = v.w;
        (*_out).x = v.x;
        (*_out).y = v.y;
        (*_out).z = v.z;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_bone_indices(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector4_i16,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.bones_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].bone_indices;
        (*_out).w = v.w;
        (*_out).x = v.x;
        (*_out).y = v.y;
        (*_out).z = v.z;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_tangent(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector3_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.tangents_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].tangent;
        (*_out).x = v.x;
        (*_out).y = v.y;
        (*_out).z = v.z;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_uv1(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.uv1_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].uv1;
        (*_out).x = v.x;
        (*_out).y = v.y;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_uv2(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.uv2_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].uv2;
        (*_out).x = v.x;
        (*_out).y = v.y;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_uv3(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.uv3_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].uv3;
        (*_out).x = v.x;
        (*_out).y = v.y;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_vertex_uv4(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let enabled = zms.uv4_enabled();
    let res = if enabled && idx < zms.vertices.len() {
        let v = zms.vertices[idx].uv4;
        (*_out).x = v.x;
        (*_out).y = v.y;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_indices(
    mesh: *mut Mesh,
    idx: u32,
    _out: *mut FfiVector3_i16,
) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let res = if idx < zms.indices.len() {
        let i = zms.indices[idx];
        (*_out).x = i.x;
        (*_out).y = i.y;
        (*_out).z = i.z;
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}

#[no_mangle]
pub unsafe extern "C" fn mesh_get_material(mesh: *mut Mesh, idx: u32, _out: *mut i16) -> bool {
    let zms = Box::from_raw(mesh);
    let idx = idx as usize;

    let res = if idx < zms.materials.len() {
        *_out = zms.materials[idx];
        true
    } else {
        false
    };

    std::mem::forget(zms);
    res
}
