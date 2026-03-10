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
    let mut field  = playingfield::new(SIZE);
    draw_field(field.get_playfield(), SIZE);

    println!("Enter the row number");
    let mut row = String::new();
    io::stdin().read_line(&mut row).expect("failed  input");

    println!("Enter the column number");
    let mut col = String::new();
    io::stdin().read_line(&mut col).expect("failed  input");
    row = row.trim().to_string();
    col=col.trim().to_string();
    println!("You entered {row}, {col}");
    field.place(1, row.trim().parse::<u8>().unwrap(), col.trim().parse::<u8>().unwrap());
    draw_field(field.get_playfield(), SIZE);


    // let socket = UdpSocket::bind("127.0.0.1:7878").expect("couldn't bind to address");
    // socket.connect("127.0.0.1:34253").unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("failed  input");
    // socket.send(input.as_bytes()).unwrap();
}

