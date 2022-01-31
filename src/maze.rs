pub type MazeVec = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub start: Vec<usize>,
    pub goal: Vec<usize>,
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
    ) -> Vec<Vec<usize>> {
        let mut start_posi = vec![];
        let mut goal_posi = vec![];
        for (h, row) in maze_vec.iter().enumerate() {
            for (w, c) in row.iter().enumerate() {
                if c == start_char {
                    start_posi.push(h);
                    start_posi.push(w);
                }
                if c == goal_char {
                    goal_posi.push(h);
                    goal_posi.push(w);
                }
            }
        }
        vec![start_posi, goal_posi]
    }
    pub fn new(maze_vec: MazeVec, start_char: char, goal_char: char, wall_char: char) -> Self {
        let posi = Maze::start_goal_position(&maze_vec, &start_char, &goal_char);
        Self {
            width: maze_vec[0].len(),
            height: maze_vec.len(),
            start: posi[0].clone(),
            goal: posi[1].clone(),
            start_char,
            goal_char,
            wall_char,
            maze_vec,
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
