use crate::graph::Graph;
use std::collections::VecDeque;

pub trait Bfs {
    fn bfs(&self) -> Vec<i32>;
}
impl Bfs for Graph {
    fn bfs(&self) -> Vec<i32> {
        let mut queue = VecDeque::from([0]);
        let mut dist: Vec<i32> = vec![-1; self.vertices.len()];
        dist[0] = 0;
        while !queue.is_empty() {
            let idx = queue.pop_front().unwrap();
            for &i in self.vertices[idx].iter() {
                if dist[i] >= 0 {
                    continue;
                }
                dist[i] = dist[idx] + 1;
                queue.push_back(i);
            }
        }
        return dist;
    }
}

#[test]
fn test_bfs() {
    let vertices = vec![
        vec![1, 2],
        vec![0, 3],
        vec![0, 5],
        vec![1, 4, 5],
        vec![3],
        vec![2, 3],
    ];
    let graph = Graph { vertices };
    assert_eq!(graph.bfs(), vec![0, 1, 1, 2, 3, 2]);
}
