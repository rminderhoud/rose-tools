use crate::utils::*;
use roselib::files::zmo::{ChannelType, Motion};
use roselib::io::RoseFile;
use std::convert::From;
use std::ffi::CStr;
use std::path::PathBuf;

/// Motion channel type
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum MotionChannelType {
    None = 1,
    Position = 1 << 1,
    Rotation = 1 << 2,
    Normal = 1 << 3,
    Alpha = 1 << 4,
    UV1 = 1 << 5,
    UV2 = 1 << 6,
    UV3 = 1 << 7,
    UV4 = 1 << 8,
    Texture = 1 << 9,
    Scale = 1 << 10,
}

impl From<ChannelType> for MotionChannelType {
    fn from(t: ChannelType) -> MotionChannelType {
        match t {
            ChannelType::None => MotionChannelType::None,
            ChannelType::Position => MotionChannelType::Position,
            ChannelType::Rotation => MotionChannelType::Rotation,
            ChannelType::Normal => MotionChannelType::Normal,
            ChannelType::Alpha => MotionChannelType::Alpha,
            ChannelType::UV1 => MotionChannelType::UV1,
            ChannelType::UV2 => MotionChannelType::UV2,
            ChannelType::UV3 => MotionChannelType::UV3,
            ChannelType::UV4 => MotionChannelType::UV4,
            ChannelType::Texture => MotionChannelType::Texture,
            ChannelType::Scale => MotionChannelType::Scale,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn motion_new() -> *mut Motion {
    Box::into_raw(Box::new(Motion::new()))
}

#[no_mangle]
pub unsafe extern "C" fn motion_free(motion: *mut Motion) {
    Box::from_raw(motion); //Drop
}

#[no_mangle]
pub unsafe extern "C" fn motion_read(motion: *mut Motion, path: *const libc::c_char) -> bool {
    let mut zmo = Box::from_raw(motion);

    let path_str = CStr::from_ptr(path).to_str().unwrap_or_default();
    let p = PathBuf::from(path_str);

    let res = zmo.read_from_path(&p).is_ok();

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_fps(motion: *mut Motion) -> u32 {
    let zmo = Box::from_raw(motion);
    let res = zmo.fps;
    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_frames(motion: *mut Motion) -> u32 {
    let zmo = Box::from_raw(motion);
    let res = zmo.frames;
    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_channels(motion: *mut Motion) -> u32 {
    let zmo = Box::from_raw(motion);
    let res = zmo.channels.len() as u32;
    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_channel_type(
    motion: *mut Motion,
    idx: u32,
    _out: *mut MotionChannelType,
) -> bool {
    let zmo = Box::from_raw(motion);
    let idx = idx as usize;

    let res = if idx < zmo.channels.len() {
        *_out = zmo.channels[idx].typ.into();
        true
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_position_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiVector3_f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].position_frames() {
            let frame = frame as usize;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            (*_out).z = v[frame].z;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_rotation_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiQuaternion,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].rotation_frames() {
            let frame = frame as usize;
            (*_out).w = v[frame].w;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            (*_out).z = v[frame].z;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_normal_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiVector3_f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].normal_frames() {
            let frame = frame as usize;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            (*_out).z = v[frame].z;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_alpha_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].alpha_frames() {
            let frame = frame as usize;
            *_out = v[frame];
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_uv1_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].uv1_frames() {
            let frame = frame as usize;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_uv2_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].uv2_frames() {
            let frame = frame as usize;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_uv3_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].uv3_frames() {
            let frame = frame as usize;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_uv4_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut FfiVector2_f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].uv4_frames() {
            let frame = frame as usize;
            (*_out).x = v[frame].x;
            (*_out).y = v[frame].y;
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_texture_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].texture_frames() {
            let frame = frame as usize;
            *_out = v[frame];
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}

#[no_mangle]
pub unsafe extern "C" fn motion_get_scale_frame(
    motion: *mut Motion,
    channel: u32,
    frame: u32,
    _out: *mut f32,
) -> bool {
    let mut zmo = Box::from_raw(motion);
    let channel = channel as usize;

    let res = if frame < zmo.frames && channel < zmo.channels.len() {
        if let Some(v) = zmo.channels[channel].scale_frames() {
            let frame = frame as usize;
            *_out = v[frame];
            true
        } else {
            false
        }
    } else {
        false
    };

    std::mem::forget(zmo);
    res
}
