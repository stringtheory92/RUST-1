// enum Asparagus {
//     color(String),
//     length(u8),
// }
#[derive(Debug)]
pub struct Asparagus {
    color: String,
    length: u8,
}

impl Asparagus {
    pub fn grow(&mut self) {
        self.length += 1;
    }
    pub fn new(color: String, length: u8) -> Self {
        Self {color, length}
    }
}