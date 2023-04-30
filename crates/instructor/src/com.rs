use std::io::{self, BufRead, Write};

use cyproto_core::{Command, Response};

pub fn read_response(
    stream: &mut crate::Socket,
) -> Result<Option<Response>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut reader = io::BufReader::with_capacity(1, &mut stream.0);

    match reader.read_until(0, &mut buffer) {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            return Ok(None);
        }
        Err(e) => return Err(Box::new(e)),
    }
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
