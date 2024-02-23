use rosc::{OscPacket, OscType};
use std::env;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

use crate::{BODY1_BASE_SPINE, BODY2_BASE_SPINE, BODY1_HEAD, BODY2_HEAD};

const ADDR: &str = "127.0.0.1:9000";

pub fn spawn_osc_handler() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddrV4::from_str(ADDR)?;
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    std::thread::spawn(move || {
        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    let (_, packet) = match rosc::decoder::decode_udp(&buf[..size]) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    };
                    handle_packet(packet);
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    continue;
                }
            }
        }
    });
    Ok(())
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            if msg.addr == "/body1/spine_mid" {
                let mut pos = match handle_3d_position_osc_msg(&msg.args) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                pos[0] = -pos[0];
                pos[2] = -pos[2];
                if pos != [0.0, 0.0, 0.0] {
                    *BODY1_BASE_SPINE.lock() = pos;
                }
            } else if msg.addr == "/body2/spine_mid" {
                let mut pos = match handle_3d_position_osc_msg(&msg.args) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                pos[0] = -pos[0];
                pos[2] = -pos[2];
                if pos != [0.0, 0.0, 0.0] {
                    *BODY2_BASE_SPINE.lock() = pos;
                }
            } else if msg.addr == "/body1/head" {
                let mut pos = match handle_3d_position_osc_msg(&msg.args) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                pos[0] = -pos[0];
                pos[2] = -pos[2];
                *BODY1_HEAD.lock() = pos;
            } else if msg.addr == "/body2/head" {
                let mut pos = match handle_3d_position_osc_msg(&msg.args) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                pos[0] = -pos[0];
                pos[2] = -pos[2];
                *BODY2_HEAD.lock() = pos;
            } else {
                println!("address not recognized: {:?}", msg);
            }
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

fn handle_3d_position_osc_msg(args: &Vec<OscType>) -> Result<[f32; 3], &'static str> {
    if args.len() != 3 {
        return Err("args length must be 3");
    }

    let mut out = [0.0; 3];
    for i in 0..3 {
        match args[i] {
            OscType::Float(f) => {
                out[i] = f;
            },
            _ => return Err("args must be floats"),
        }
    }
    Ok(out)
}