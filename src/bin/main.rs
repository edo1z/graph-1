use graph::maze::{bfs::Bfs, Maze};

fn main() {
    let maze_str = r#"
        .#....#G
        .#.#....
        ...#.##.
        #.##...#
        ...###.#
        .#.....#
        ...#.#..
        S.......
    "#
    .to_string();
    let maze_vec = Maze::maze_vec_from_string(maze_str);
    let maze = Maze::new(maze_vec, 'S', 'G', '#');
    let (dist, prev) = maze.bfs();
    let dist_map = maze.bfs_dist_map(&dist);
    let route = maze.bfs_best_route(&prev);
    let route_map = maze.bfs_best_route_map(&route);
    println!("{dist_map}");
    println!("{route_map}");
}
