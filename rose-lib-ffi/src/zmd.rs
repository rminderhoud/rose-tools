use crate::utils::*;
use roselib::files::zmd::Skeleton;
use roselib::io::RoseFile;
use std::ffi::CStr;
use std::path::PathBuf;

#[no_mangle]
pub unsafe extern "C" fn skeleton_new() -> *mut Skeleton {
    Box::into_raw(Box::new(Skeleton::new()))
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_free(skeleton: *mut Skeleton) {
    Box::from_raw(skeleton); // Drop
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_read(skeleton: *mut Skeleton, path: *const libc::c_char) -> bool {
    let mut zmd = Box::from_raw(skeleton);

    let path_str = CStr::from_ptr(path).to_str().unwrap_or_default();
    let p = PathBuf::from(path_str);

    let res = zmd.read_from_path(&p).is_ok();

    std::mem::forget(zmd);
    res
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_bones(skeleton: *mut Skeleton) -> libc::c_int {
    let zmd = Box::from_raw(skeleton);
    let res = zmd.bones.len() as libc::c_int;
    std::mem::forget(zmd);
    res
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_dummies(skeleton: *mut Skeleton) -> libc::c_int {
    let zmd = Box::from_raw(skeleton);
    let res = zmd.dummy_bones.len() as libc::c_int;
    std::mem::forget(zmd);
    res
}

// -- Bone
#[no_mangle]
pub unsafe extern "C" fn skeleton_get_bone_name(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiString,
) -> bool {
    get_bone_name(skeleton, idx, _out, false)
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_get_bone_parent(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut libc::c_int,
) -> bool {
    get_bone_parent(skeleton, idx, _out, false)
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_get_bone_position(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiVector3_f32,
) -> bool {
    get_bone_position(skeleton, idx, _out, false)
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_get_bone_rotation(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiQuaternion,
) -> bool {
    get_bone_rotation(skeleton, idx, _out, false)
}

// -- Dummy
#[no_mangle]
pub unsafe extern "C" fn skeleton_get_dummy_name(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiString,
) -> bool {
    get_bone_name(skeleton, idx, _out, true)
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_get_dummy_parent(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut libc::c_int,
) -> bool {
    get_bone_parent(skeleton, idx, _out, true)
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_get_dummy_position(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiVector3_f32,
) -> bool {
    get_bone_position(skeleton, idx, _out, true)
}

#[no_mangle]
pub unsafe extern "C" fn skeleton_get_dummy_rotation(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiQuaternion,
) -> bool {
    get_bone_rotation(skeleton, idx, _out, true)
}

// Helper functions
unsafe fn get_bone_name(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiString,
    dummy: bool,
) -> bool {
    let zmd = Box::from_raw(skeleton);
    let mut s = Box::from_raw(_out);

    let idx = idx as usize;
    let mut res = false;

    if dummy && idx < zmd.dummy_bones.len() {
        s.set_string(&zmd.dummy_bones[idx].name);
        res = true;
    } else if idx < zmd.bones.len() {
        s.set_string(&zmd.bones[idx].name);
        res = true;
    }

    std::mem::forget(s);
    std::mem::forget(zmd);
    res
}

unsafe fn get_bone_parent(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut libc::c_int,
    dummy: bool,
) -> bool {
    let zmd = Box::from_raw(skeleton);

    let idx = idx as usize;
    let mut res = false;

    if dummy && idx < zmd.dummy_bones.len() {
        *_out = zmd.dummy_bones[idx].parent as libc::c_int;
        res = true;
    } else if idx < zmd.bones.len() {
        *_out = zmd.bones[idx].parent as libc::c_int;
        res = true;
    }

    std::mem::forget(zmd);
    res
}

unsafe fn get_bone_position(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiVector3_f32,
    dummy: bool,
) -> bool {
    let zmd = Box::from_raw(skeleton);

    let idx = idx as usize;
    let mut res = false;

    let vec = if dummy && idx < zmd.dummy_bones.len() {
        let v = zmd.dummy_bones[idx].position;
        Some(v)
    } else if idx < zmd.bones.len() {
        let v = zmd.bones[idx].position;
        Some(v)
    } else {
        None
    };

    if let Some(v) = vec {
        (*_out).x = v.x;
        (*_out).y = v.y;
        (*_out).z = v.z;
        res = true;
    }

    std::mem::forget(zmd);
    res
}

unsafe fn get_bone_rotation(
    skeleton: *mut Skeleton,
    idx: libc::c_int,
    _out: *mut FfiQuaternion,
    dummy: bool,
) -> bool {
    let zmd = Box::from_raw(skeleton);

    let idx = idx as usize;
    let mut res = false;

    let quat = if dummy && idx < zmd.dummy_bones.len() {
        let q = zmd.dummy_bones[idx].rotation;
        Some(q)
    } else if idx < zmd.bones.len() {
        let q = zmd.bones[idx].rotation;
        Some(q)
    } else {
        None
    };

    if let Some(q) = quat {
        (*_out).w = q.w;
        (*_out).x = q.x;
        (*_out).y = q.y;
        (*_out).z = q.z;
        res = true;
    }

    std::mem::forget(zmd);
    res
}
