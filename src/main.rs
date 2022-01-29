#[derive(Debug)]
struct Graph {
    vertices: Vec<Vec<usize>>,
}
impl Graph {
    fn new(vertices_num: usize, pairs: &Vec<(usize, usize)>) -> Self {
        let mut vertices = vec![vec![]; vertices_num];
        for pair in pairs.iter() {
            vertices[pair.0 - 1].push(pair.1 - 1);
            vertices[pair.1 - 1].push(pair.0 - 1);
        } 
        Graph{vertices}
    }

    fn dfs(&self) -> bool {
        let mut visited = vec![false; self.vertices.len()];
        self._dfs(&mut visited, 0);
        visited.iter().find(|&&x| !x) == None
    }
    fn _dfs(&self, visited: &mut Vec<bool>, idx:usize) {
        visited[idx] = true;
        for i in self.vertices[idx].iter() {
            if !visited[*i] {self._dfs(visited, i.clone())}
        }
    }
}

fn main() {
    let vertices_num = 6;
    let pairs = vec![
        (1,2), 
        (1,3),
        (2,4),
        (2,5),
        (3,4),
        (4,6),
        (5,6),
    ];
    let graph = Graph::new(vertices_num, &pairs);
    println!("{}", if graph.dfs() {"Yes"} else {"No"});
}

#[test]
fn test_graph() {
    let pairs = vec![
        (1,2), 
        (1,3),
        (2,4),
        (2,5),
        (3,4),
        (4,6),
        (5,6),
    ];
    let graph = Graph::new(6, &pairs);
    assert_eq!(graph.dfs(), true);
    let graph = Graph::new(7, &pairs);
    assert_eq!(graph.dfs(), false);
}

