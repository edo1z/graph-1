#[derive(Debug, Clone, Copy)]
pub struct Posi {
    pub row: i64,
    pub col: i64,
}
impl Posi {
    pub fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
    pub fn up(&self) -> Posi {
        Posi::new(self.row + 1, self.col)
    }
    pub fn down(&self) -> Posi {
        Posi::new(self.row - 1, self.col)
    }
    pub fn right(&self) -> Posi {
        Posi::new(self.row, self.col + 1)
    }
    pub fn left(&self) -> Posi {
        Posi::new(self.row, self.col - 1)
    }
    pub fn next_positions(&self) -> Vec<Posi> {
        vec![self.up(), self.right(), self.down(), self.left()]
    }
}
