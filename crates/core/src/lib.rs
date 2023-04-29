#![no_std]
use core::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

pub const BYTES_MAX: usize = 256;
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
    /// range can be between 0..=0 and 0..=180
    /// fidelity degrees the servo turns between scans
    ///
    /// Ex: a range of 0..=180 and a fidelity of 2
    /// would result in 90 scans starting at 0 and
    /// ending at 180 degrees with every other degree
    /// being scanned.
    Scan { range: RangeInclusive<u8>, fidelity: u8 },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    DriveDone { total_distance: f32 },
    TurnDone { total_angle: f32 },
    ScanDone { data: heapless::Vec<ObjectData, SCAN_MAX> }
}
