use std::io::{self, Read, Write};

use cyproto_core::{Command, Response};

pub fn read_response(
    stream: &mut crate::Socket,
) -> Result<Option<Response>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();


    // don't block until the first byte of data comes across the buffer
    let mut byte_buf = [0; 1];
    if let Err(err) = stream.0.read(&mut byte_buf) {
        if err.kind() == io::ErrorKind::WouldBlock {
            return Ok(None);
        } else {
            return Err(Box::new(err));
        }
    }
    buffer.push(byte_buf[0]);

    // the rest of the data should follow quickly after the first
    while byte_buf[0] != 0 {
        while let Err(err) = stream.0.read(&mut byte_buf) {
            if err.kind() == io::ErrorKind::WouldBlock {
                continue;
            } else {
                return Err(Box::new(err));
            }
        }
        buffer.push(byte_buf[0]);
    }
    println!("{:?}", buffer);
    let response: Response = postcard::from_bytes_cobs(&mut buffer)?;
    Ok(Some(response))
}

pub fn send_command(
    stream: &mut crate::Socket,
    command: Command,
) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = postcard::to_stdvec_cobs(&command)?;
    stream.0.write_all(&encoded)?;
    Ok(())
}
