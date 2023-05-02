#![no_std]

#[cfg(feature = "panic-abort")]
extern crate panic_abort;

use cyproto_core::{Command, Response, SCAN_MAX};

#[repr(C)]
#[derive(Debug, Default)]
pub enum CyprotoError {
    #[default]
    None,
    BufferOverflow,
    Postcard,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct DriveCommand {
    pub distance: f32,
    pub speed: u16,
}

#[repr(C)]
pub struct DriveDone {
    pub total_distance: f32,
    pub bump_detected: bool,
    pub cliff_detected: bool,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct TurnCommand {
    pub angle: f32,
    pub speed: u16,
}

#[repr(C)]
pub struct TurnDone {
    pub total_angle: f32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct ScanCommand {
    pub start: u8,
    pub end: u8,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ObjectData {
    pub distance: f32,
    pub angle: u8,
    pub width: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScanDone {
    pub size: usize,
    pub objects: *const ObjectData,
}

#[repr(C)]
#[derive(Debug)]
pub enum CommandRequest {
    Error(CyprotoError),
    Drive(DriveCommand),
    Turn(TurnCommand),
    Scan(ScanCommand),
}

#[no_mangle]
pub extern "C" fn cyproto_parse_command(buf: *mut u8) -> CommandRequest {
    let buf_size = cyproto_buffer_size();
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, buf_size) };

    let res: Result<Command, _> = postcard::from_bytes_cobs(buf);
    match res {
        Ok(Command::Drive { distance, speed }) => {
            CommandRequest::Drive(DriveCommand {
                distance,
                speed,
            })
        }
        Ok(Command::Turn { angle, speed }) => {
            CommandRequest::Turn(TurnCommand {
                angle,
                speed
            })
        }
        Ok(Command::Scan { start, end }) => {
            CommandRequest::Scan(ScanCommand {
                start,
                end,
            })
        }
        Err(_) => {
            CommandRequest::Error(CyprotoError::Postcard)
        }
    }
}

/// Get the expected buffer size for serializing and deserializing data
/// make sure the buffer has exactly cyproto_buffer_size() elements
#[no_mangle]
pub const extern "C" fn cyproto_buffer_size() -> usize {
    return cyproto_core::BYTES_MAX;
}

/// Get the maximum number of scan objects that are allowed by the buffer size
/// make sure the buffer has exactly cyproto_buffer_size() elements
#[no_mangle]
pub extern "C" fn max_objects() -> usize {
    return cyproto_core::SCAN_MAX;
}

/// Serialize a drive result struct into the provided buffer
/// make sure the buffer has exactly cyproto_buffer_size() elements
#[no_mangle]
pub extern "C" fn cyproto_drive_done(val: DriveDone, buf: *mut u8) -> usize {
    let buf_size = cyproto_buffer_size();
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, buf_size) };

    let DriveDone { total_distance, cliff_detected, bump_detected } = val;
    let res = Response::DriveDone { total_distance, bump_detected, cliff_detected };

    postcard::to_slice_cobs(&res, buf)
        .map(|v| v.len())
        .unwrap_or(0)
}

/// Serialize a turn result struct into the provided buffer
/// make sure the buffer has exactly cyproto_buffer_size() elements
#[no_mangle]
pub extern "C" fn cyproto_turn_done(val: TurnDone, buf: *mut u8) -> usize {
    let buf_size = cyproto_buffer_size();
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, buf_size) };

    let TurnDone { total_angle } = val;
    let res = Response::TurnDone { total_angle };

    postcard::to_slice_cobs(&res, buf)
        .map(|v| v.len())
        .unwrap_or(0)
}

/// Serialize a scan result struct into the provided buffer
/// make sure the buffer has exactly cyproto_buffer_size() elements
#[no_mangle]
pub extern "C" fn cyproto_scan_done(val: ScanDone, buf: *mut u8) -> usize {
    let buf_size = cyproto_buffer_size();
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, buf_size) };

    if val.size > SCAN_MAX {
        return 0;
    }
    let data = unsafe { core::slice::from_raw_parts(val.objects, val.size) };
    let data = data.iter()
        .map(|s| cyproto_core::ObjectData {
            angle: s.angle,
            distance: s.distance,
            width: s.width,
        });
    let data = heapless::Vec::<_, SCAN_MAX>::from_iter(data);
    let res = Response::ScanDone { data };

    postcard::to_slice_cobs(&res, buf)
        .map(|v| v.len())
        .unwrap_or(0)
}
