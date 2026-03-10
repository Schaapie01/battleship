pub mod lib;

use std::net::UdpSocket;
use std::io;
use battleship::{
    playingfield,
    artist
};

use crate::lib::draw_field;
const SIZE: usize = 3;

fn main() {
    let field  = playingfield::new(SIZE);
    draw_field(field.get_playfield(), SIZE);

    // let socket = UdpSocket::bind("127.0.0.1:7878").expect("couldn't bind to address");
    // socket.connect("127.0.0.1:34253").unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("failed  input");
    // socket.send(input.as_bytes()).unwrap();
}

