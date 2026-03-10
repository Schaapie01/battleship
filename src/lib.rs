pub struct playingfield {
    field: Vec<u8>,
}
pub struct artist {
    size: usize,
}

impl playingfield {
    pub fn new(size: usize) -> playingfield {
        return playingfield { field: vec![0; (size * size)] };
    }
    pub fn get_playfield(&self) -> Vec<u8> {
        return self.field.clone();
    }

}

pub fn draw_field(playfield: Vec<u8>, size: usize) {
    let mut spot_counter = 0;
    while spot_counter < playfield.len() {
        match playfield[spot_counter] {
            1 => print!("[X]"),
            2 => print!("[O]"),
            _ => print!("[_]"),
        }
        if (spot_counter + 1) % size == 0 {
            print!("\n");
        }
        spot_counter += 1;
    }
}