use std::{net::{TcpListener, TcpStream}, io::{self, BufRead, Write}, time::Duration};

use cyproto_core::{Response, Command, ObjectData};
use rand::Rng;

pub fn read_command(stream: &mut TcpStream) -> Result<Command, Box<dyn std::error::Error>> {
    let mut reader = io::BufReader::with_capacity(1, stream);

    let mut buffer = Vec::new();
    reader.read_until(0, &mut buffer)?;

    let command: Command = postcard::from_bytes_cobs(&mut buffer)?;
    Ok(command)
}

pub fn send_response(stream: &mut TcpStream, response: &Response) -> Result<(), Box<dyn std::error::Error>> {
    println!("{response:?}");
    let encoded = postcard::to_stdvec_cobs(response)?;
    stream.write_all(&encoded)?;
    Ok(())
}

fn main() {
    let mut rand = rand::thread_rng();
    let listener = TcpListener::bind("localhost:2888").unwrap();

    loop {
        let (mut stream, _) = listener.accept().unwrap();

        while let Ok(cmd) = read_command(&mut stream) {
            println!("{cmd:?}");
            std::thread::sleep(Duration::from_secs(1));
            match cmd {
                Command::Drive { distance, .. } => {
                    let failed = rand.gen_bool(0.1);
                    let range = if distance < 0. {
                        distance..=0.0
                    } else {
                        0.0..=distance
                    };
                    send_response(&mut stream, &Response::DriveDone {
                        total_distance: if failed { rand.gen_range(range) } else { distance },
                        bump_detected: if failed { rand.gen_bool(0.5) } else { false },
                        cliff_detected: if failed { rand.gen_bool(0.5) } else { false },
                    }).unwrap();
                }
                Command::Turn { angle, .. } => {
                    let failed = rand.gen_bool(0.1);
                    let range = if angle < 0. {
                        angle..=0.
                    } else {
                        0.0..=angle
                    };
                    send_response(&mut stream, &Response::TurnDone {
                        total_angle: if failed { rand.gen_range(range) } else { angle },
                    }).unwrap();
                }
                Command::Scan { start, end } => {
                    let num_objs: usize = rand.gen_range(0..=10);
                    let mut objs = heapless::Vec::new();
                    for _ in 0..num_objs {
                        objs.push(ObjectData {
                            distance: rand.gen_range(15.0..80.),
                            width: rand.gen_range(5.0..10.),
                            angle: rand.gen_range(start..=end)
                        }).unwrap();
                    }

                    send_response(&mut stream, &Response::ScanDone {
                        data: objs.into(),
                    }).unwrap();
                } 
            }
        }
    }
}
