pub struct playingfield {
    field: Vec<u8>,
}

impl playingfield {
    pub fn new(size: usize) -> playingfield {
        return playingfield { field: vec![0; (size * size)] };
    }
    pub fn get_playfield(&self) -> Vec<u8> {
        return self.field.clone();
    }
}