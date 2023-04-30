#![no_std]
use serde::{Deserialize, Serialize};

pub const BYTES_MAX: usize = 512;
pub const SCAN_MAX: usize = BYTES_MAX / core::mem::size_of::<ObjectData>();

pub type Bytes = heapless::Vec<u8, SCAN_MAX>;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ObjectData {
    pub distance: f32,
    pub angle: u8,
    pub width: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Drive { distance: f32, speed: u16 },
    Turn { angle: f32, speed: u16 },
    Scan { start: u8, end: u8 },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    DriveDone {
        total_distance: f32,
        bump_detected: bool,
        cliff_detected: bool,
    },
    TurnDone { total_angle: f32 },
    ScanDone { data: heapless::Vec<ObjectData, SCAN_MAX> }
}
