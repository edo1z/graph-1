pub mod bfs;
pub mod dist;
pub mod posi;
pub mod prev;

use posi::Posi;
pub type MazeVec = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Maze {
    pub width: i64,
    pub height: i64,
    pub start_posi: Posi,
    pub goal_posi: Posi,
    pub start_char: char,
    pub goal_char: char,
    pub wall_char: char,
    pub maze_vec: MazeVec,
}
impl Maze {
    pub fn maze_vec_from_string(maze_str: String) -> MazeVec {
        let mut maze_vec: MazeVec = vec![];
        for row in maze_str.lines() {
            let mut maze_row = vec![];
            for c in row.trim().chars() {
                maze_row.push(c);
            }
            if maze_row.len() > 0 {
                maze_vec.push(maze_row);
            }
        }
        maze_vec
    }
    pub fn start_goal_position(
        maze_vec: &MazeVec,
        start_char: &char,
        goal_char: &char,
    ) -> Vec<Posi> {
        let mut start_posi: Posi = Posi::new(-1, -1);
        let mut goal_posi: Posi = Posi::new(-1, -1);
        for (h, row) in maze_vec.iter().enumerate() {
            for (w, c) in row.iter().enumerate() {
                if c == start_char {
                    start_posi = Posi::new(h as i64, w as i64);
                }
                if c == goal_char {
                    goal_posi = Posi::new(h as i64, w as i64);
                }
            }
        }
        vec![start_posi, goal_posi]
    }
    pub fn new(maze_vec: MazeVec, start_char: char, goal_char: char, wall_char: char) -> Self {
        let posi = Maze::start_goal_position(&maze_vec, &start_char, &goal_char);
        Self {
            width: maze_vec[0].len() as i64,
            height: maze_vec.len() as i64,
            start_posi: posi[0],
            goal_posi: posi[1],
            start_char,
            goal_char,
            wall_char,
            maze_vec,
        }
    }

    pub fn is_enabled_posi(&self, posi: &Posi) -> bool {
        if posi.row < 0 || posi.col < 0 {
            false
        } else if self.width <= posi.col || self.height <= posi.row {
            false
        } else if self.maze_vec[posi.row as usize][posi.col as usize] == self.wall_char {
            false
        } else {
            true
        }
    }
}

#[test]
fn test_maze() {
    let maze_str = r#"
        .#....#G
        .#.#....
        ...#.##.
        #.##...#
        ...###.#
        .#.....#
        ...#.#..
        S.......
    "#;
    let maze_vec = Maze::maze_vec_from_string(String::from(maze_str));
    let maze = Maze::new(maze_vec, 'S', 'G', '#');
    println!("{:?}", maze);
}
