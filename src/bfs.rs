use crate::graph::Graph;
use std::collections::VecDeque;

pub trait Bfs {
    fn bfs(&self) -> (Vec<i32>, Vec<i32>);
    fn bfs_route(&self, prev: &Vec<i32>) -> VecDeque<usize>;
}
impl Bfs for Graph {
    fn bfs(&self) -> (Vec<i32>, Vec<i32>) {
        let mut queue = VecDeque::from([0]);
        let mut prev: Vec<i32> = vec![-1; self.vertices.len()];
        let mut dist: Vec<i32> = vec![-1; self.vertices.len()];
        dist[0] = 0;
        while !queue.is_empty() {
            let idx = queue.pop_front().unwrap();
            for &i in self.vertices[idx].iter() {
                if dist[i] >= 0 {
                    continue;
                }
                dist[i] = dist[idx] + 1;
                prev[i] = idx as i32;
                queue.push_back(i);
            }
        }
        return (dist, prev);
    }

    fn bfs_route(&self, prev: &Vec<i32>) -> VecDeque<usize> {
        let mut idx = prev.len() - 1;
        let mut route: VecDeque<usize> = VecDeque::from([idx]);
        loop {
            let next = prev[idx];
            if next < 0 {
                break;
            }
            route.push_front(next as usize);
            idx = next as usize;
        }
        route
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
    assert_eq!(
        graph.bfs(),
        (vec![0, 1, 1, 2, 3, 2], vec![-1, 0, 0, 1, 3, 2])
    );
}

#[test]
fn test_bfs_route() {
    let vertices = vec![
        vec![1, 2],
        vec![0, 3],
        vec![0, 5],
        vec![1, 4, 5],
        vec![3],
        vec![2, 3],
    ];
    let graph = Graph { vertices };
    let (_dist, prev) = graph.bfs();
    assert_eq!(VecDeque::from([0, 2, 5]), graph.bfs_route(&prev));
}
