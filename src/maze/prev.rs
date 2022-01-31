use crate::maze::posi::Posi;

#[derive(Debug)]
pub struct Prev(pub Vec<Vec<Posi>>);
impl Prev {
    pub fn new(row: i64, col: i64) -> Self {
        Prev(vec![vec![Posi::new(-1, -1); col as usize]; row as usize])
    }
    pub fn get(&self, posi: &Posi) -> &Posi {
        &(self.0[posi.row as usize][posi.col as usize])
    }
    pub fn set(&mut self, posi: &Posi, val: Posi) {
        self.0[posi.row as usize][posi.col as usize] = val;
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_initial_state(posi: &Posi) -> bool {
        posi.row == -1 && posi.col == -1
    }
}
