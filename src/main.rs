pub mod lib;

use std::net::UdpSocket;
use std::io;
use battleship::playingfield;
const SIZE: usize = 3;

fn main() {
    let mut field  = playingfield::new(SIZE);

    let mut spot_counter = 0;
    while spot_counter < field.get_playfield().len() {
        match field.get_playfield()[spot_counter] {
            1 => print!("[X]"),
            2 => print!("[O]"),
            _ => print!("[_]"),
        }
        if (spot_counter + 1) % SIZE == 0 {
            print!("\n");
        }
        spot_counter += 1;
    }

    // let socket = UdpSocket::bind("127.0.0.1:7878").expect("couldn't bind to address");
    // socket.connect("127.0.0.1:34253").unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("failed  input");
    // socket.send(input.as_bytes()).unwrap();
}

