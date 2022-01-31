pub mod bfs;
pub mod dfs;

#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vec<usize>>,
}
impl Graph {
    pub fn new(vertices_num: usize, pairs: &Vec<(usize, usize)>) -> Self {
        let mut vertices = vec![vec![]; vertices_num];
        for pair in pairs.iter() {
            vertices[pair.0 - 1].push(pair.1 - 1);
            vertices[pair.1 - 1].push(pair.0 - 1);
        }
        Graph { vertices }
    }
}
