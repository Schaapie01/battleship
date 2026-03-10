pub struct playingfield {
    size: usize,
    field: Vec<u8>,
}
pub struct artist {
    size: usize,
}

impl playingfield {
    pub fn new(size: usize) -> playingfield {
        return playingfield { field: vec![0; (size * size)], size };
    }
    pub fn get_playfield(&self) -> Vec<u8> {
        return self.field.clone();
    }
    pub fn place(&mut self, player: u8, row: u8, col: u8) {
        let column = row * (self.size as u8) + col;
        if self.get_playfield()[column as usize] == 0 {
            match player {
                1 => {
                    self.field[column as usize] = 1;
                },
                2 => self.field[column as usize] = 2,
                _ => panic!("Invalid player")
            }
        }
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