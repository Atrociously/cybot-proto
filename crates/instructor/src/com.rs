use std::io::{BufRead, self, Write, Read};

use cyproto_core::{Response, Command};

pub fn read_response(stream: &mut crate::Socket) -> Result<Option<Response>, Box<dyn std::error::Error>> {
    let mut reader = io::BufReader::new(&mut stream.0);
    let temp_buf = &mut [0u8; 1];
    let mut buffer = Vec::new();

    let amt = reader.read(temp_buf)?; // try to read a single byte from the stream
    if amt == 0 {
        // if there was nothing in the stream to read we responde with nothing
        return Ok(None);
    }
    // otherwise read until the end of the packet
    buffer.extend_from_slice(temp_buf);
    reader.read_until(0, &mut buffer)?;
    let response: Response = postcard::from_bytes_cobs(&mut buffer)?;
    Ok(Some(response))
}

pub fn send_command(stream: &mut crate::Socket, command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = postcard::to_stdvec_cobs(&command)?;
    stream.0.write_all(&encoded)?;
    Ok(())
}
