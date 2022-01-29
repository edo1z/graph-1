use graph::dfs::Graph;

fn main() {
    let vertices_num = 6;
    let pairs = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 4), (4, 6), (5, 6)];
    let graph = Graph::new(vertices_num, &pairs);
    println!("{}", if graph.dfs() { "Yes" } else { "No" });
}

