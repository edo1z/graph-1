use crate::maze::posi::Posi;

#[derive(Debug)]
pub struct Dist(pub Vec<Vec<i64>>);
impl Dist {
    pub fn new(row: i64, col: i64) -> Self {
        Dist(vec![vec![-1; col as usize]; row as usize])
    }
    pub fn get(&self, posi: &Posi) -> i64 {
        self.0[posi.row as usize][posi.col as usize]
    }
    pub fn set(&mut self, posi: &Posi, val: i64) {
        self.0[posi.row as usize][posi.col as usize] = val;
    }
}
