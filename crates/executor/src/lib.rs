#![no_std]

use cybot::{self, UartCom};
use cyproto_core::{Bytes, Command, Response, SCAN_MAX};

#[repr(C)]
#[derive(Default)]
pub enum CyprotoError {
    #[default]
    None,
    BufferOverflow,
    Postcard,
}

#[repr(C)]
#[derive(Default)]
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
#[derive(Default)]
pub struct TurnCommand {
    pub angle: f32,
    pub speed: u16,
}

#[repr(C)]
pub struct TurnDone {
    pub total_angle: f32,
}

#[repr(C)]
#[derive(Default)]
pub struct ScanCommand {
    pub start_angle: u8,
    pub end_angle: u8,
    pub fidelity: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct ObjectData {
    pub distance: f32,
    pub angle: u8,
    pub width: f32,
}

#[repr(C)]
pub struct ScanDone {
    pub size: usize,
    pub objects: *const ObjectData,
}

#[repr(C)]
pub enum CommandRequest {
    Error(CyprotoError),
    Drive(DriveCommand),
    Turn(TurnCommand),
    Scan(ScanCommand),
}

static mut UART: Option<UartCom> = None;

fn get_uart() -> &'static UartCom {
    cybot::cortex_m::interrupt::free(|_| {
        if unsafe { UART.is_none() } {
            let uart = UartCom::take().unwrap();
            unsafe { UART = Some(uart) }
        }
        unsafe { UART.as_ref().unwrap_unchecked() }
    })
}

fn recieve<T: serde::de::DeserializeOwned>() -> Result<T, CyprotoError> {
    let uart = get_uart();
    let mut buffer = Bytes::new();

    let mut next = uart.uart_recieve();
    while next != 0 {
        buffer.push(next).map_err(|_| CyprotoError::BufferOverflow)?;
        next = uart.uart_recieve();
    }
    buffer.push(next).map_err(|_| CyprotoError::BufferOverflow)?;

    postcard::from_bytes_cobs(&mut buffer).map_err(|_| CyprotoError::Postcard)
}

fn send<T: serde::Serialize>(val: &T) -> Result<(), postcard::Error> {
    let uart = get_uart();
    let buf: Bytes = postcard::to_vec_cobs(val)?;

    for byte in buf {
        uart.uart_send(byte);
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn cyproto_read_command() -> CommandRequest {
    let res: Result<Command, _> = recieve();

    let req = match res {
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
        Ok(Command::Scan { range, fidelity }) => {
            CommandRequest::Scan(ScanCommand {
                start_angle: *range.start(),
                end_angle: *range.end(),
                fidelity,
            })
        }
        Err(_) => {
            CommandRequest::Error(CyprotoError::Postcard)
        }
    };

    req
}

#[no_mangle]
pub extern "C" fn cyproto_buffer_size() -> usize {
    return cyproto_core::BYTES_MAX;
}

#[no_mangle]
pub extern "C" fn cyproto_scan_size(cmd: ScanCommand) -> usize {
    return ((cmd.end_angle - cmd.start_angle) * (1 / cmd.fidelity)).into();
}

#[no_mangle]
pub extern "C" fn cyproto_drive_done(val: DriveDone, buf: *mut u8) -> usize {
    let buf_size = cyproto_buffer_size();
    let mut buf = unsafe { core::slice::from_raw_parts_mut(buf, buf_size) };

    let DriveDone { total_distance, .. } = val;
    let res = Response::DriveDone { total_distance };

    postcard::to_slice_cobs(&res, &mut buf)
        .map(|v| v.len())
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn cyproto_turn_done(val: TurnDone) -> CyprotoError {
    let TurnDone { total_angle } = val;
    let res = Response::TurnDone { total_angle };

    send(&res)
        .map_err(|_| CyprotoError::Postcard)
        .err()
        .unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn cyproto_scan_done(val: ScanDone) -> CyprotoError {
    if val.size > SCAN_MAX {
        return CyprotoError::BufferOverflow;
    }
    let data = unsafe { core::slice::from_raw_parts(val.objects, val.size) };
    let data = data.iter()
        .map(|s| cyproto_core::ObjectData {
            angle: s.angle,
            distance: s.distance,
            width: s.width,
        });
    let data = heapless::Vec::<_, SCAN_MAX>::from_iter(data);

    send(&Response::ScanDone { data })
        .map_err(|_| CyprotoError::Postcard)
        .err()
        .unwrap_or_default()
}
