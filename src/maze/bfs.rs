use crate::maze::dist::Dist;
use crate::maze::posi::Posi;
use crate::maze::prev::Prev;
use crate::maze::Maze;
use std::collections::VecDeque;

pub trait Bfs {
    fn bfs(&self) -> (Dist, Prev);
    fn bfs_dist_map(&self, dist: &Dist) -> String;
    fn bfs_best_route(&self, prev: &Prev) -> VecDeque<Posi>;
    fn bfs_best_route_map(&self, route: &VecDeque<Posi>) -> String;
}
impl Bfs for Maze {
    fn bfs(&self) -> (Dist, Prev) {
        let mut queue = VecDeque::from([self.start_posi.clone()]);
        let mut prev = Prev::new(self.width, self.height);
        let mut dist = Dist::new(self.width, self.height);
        dist.set(&self.start_posi, 0);
        while !queue.is_empty() {
            let posi = queue.pop_front().unwrap();
            for p in posi.next_positions().iter() {
                if self.is_enabled_posi(p) && dist.get(p) < 0 {
                    dist.set(p, dist.get(&posi) + 1);
                    prev.set(p, posi.clone());
                    queue.push_back(p.clone());
                }
            }
        }
        (dist, prev)
    }

    fn bfs_dist_map(&self, dist: &Dist) -> String {
        let max_len = dist.get(&self.goal_posi).to_string().len();
        let mut dist_map = String::from("");
        for (row, row_val) in self.maze_vec.iter().enumerate() {
            for (col, &c) in row_val.iter().enumerate() {
                if c != self.wall_char {
                    let dist = dist.get(&Posi::new(row as i64, col as i64));
                    let str = &format!("{dist:0max_len$} | ", dist = dist, max_len = max_len);
                    dist_map.push_str(str);
                } else {
                    let str = &format!("{mark:^max_len$} | ", mark = c, max_len = max_len);
                    dist_map.push_str(str);
                }
            }
            dist_map.push_str("\n");
        }
        dist_map
    }

    fn bfs_best_route(&self, prev: &Prev) -> VecDeque<Posi> {
        let mut route: VecDeque<Posi> = VecDeque::from([self.goal_posi.clone()]);
        let mut current_posi = &self.goal_posi;
        loop {
            let posi = prev.get(current_posi);
            if Prev::is_initial_state(posi) {
                break;
            }
            route.push_front(posi.clone());
            current_posi = posi;
        }
        route
    }

    fn bfs_best_route_map(&self, routes: &VecDeque<Posi>) -> String {
        let mut route_map_vec = vec![vec![' '; self.width as usize]; self.height as usize];
        let mut route_map = String::from("");
        for posi in routes.iter() {
            route_map_vec[posi.row as usize][posi.col as usize] = '★';
        }
        for (row, row_val) in self.maze_vec.iter().enumerate() {
            for (col, &c) in row_val.iter().enumerate() {
                if c == self.start_char || c == self.goal_char || c == self.wall_char {
                    route_map.push_str(&format!("{} | ", c));
                } else {
                    route_map.push_str(&format!("{} | ", route_map_vec[row][col]));
                }
            }
            route_map.push_str("\n");
        }
        route_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture::maze::maze_str;

    #[test]
    fn test_maze_bfs_dist_map() {
        let maze_str = maze_str::maze_str1();
        let maze_vec = Maze::maze_vec_from_string(maze_str);
        let maze = Maze::new(maze_vec, 'S', 'G', '#');
        let (dist, _prev) = maze.bfs();
        let dist_map = maze.bfs_dist_map(&dist);
        let ans = r#"09 | #  | 09 | 10 | 11 | 12 | #  | 16 | 
08 | #  | 08 | #  | 12 | 13 | 14 | 15 | 
07 | 06 | 07 | #  | 13 | #  | #  | 16 | 
#  | 05 | #  | #  | 12 | 11 | 10 | #  | 
03 | 04 | 05 | #  | #  | #  | 09 | #  | 
02 | #  | 04 | 05 | 06 | 07 | 08 | #  | 
01 | 02 | 03 | #  | 05 | #  | 07 | 08 | 
00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 
"#;
        assert_eq!(dist_map, ans.to_string());
    }

    #[test]
    fn test_maze_bfs_best_route_map() {
        let maze_str = maze_str::maze_str1();
        let maze_vec = Maze::maze_vec_from_string(maze_str);
        let maze = Maze::new(maze_vec, 'S', 'G', '#');
        let (_dist, prev) = maze.bfs();
        let route = maze.bfs_best_route(&prev);
        let route_map = maze.bfs_best_route_map(&route);
        let ans = r#"  | # | ★ | ★ | ★ |   | # | G | 
  | # | ★ | # | ★ | ★ | ★ | ★ | 
  | ★ | ★ | # |   | # | # |   | 
# | ★ | # | # |   |   |   | # | 
★ | ★ |   | # | # | # |   | # | 
★ | # |   |   |   |   |   | # | 
★ |   |   | # |   | # |   |   | 
S |   |   |   |   |   |   |   | 
"#;
        assert_eq!(route_map, ans.to_string());
    }
}
