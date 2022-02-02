use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}

#[derive(Debug, Eq)]
pub struct State {
    pub idx: usize,
    pub total_cost: usize,
    pub prev_idx: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}

pub fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize, goal: usize) -> (Vec<usize>, Vec<usize>) {
    let mut costs = vec![usize::MAX; graph.len()];
    let mut prev_nodes: Vec<usize> = (0..graph.len()).map(|i| i).collect();
    let mut heap = BinaryHeap::new();
    costs[start] = 0;
    heap.push(State {
        idx: start,
        total_cost: 0,
        prev_idx: 0,
    });
    while let Some(State {
        idx,
        total_cost,
        prev_idx,
    }) = heap.pop()
    {
        prev_nodes[idx] = prev_idx;
        if idx == goal {
            break;
        }
        for edge in graph[idx].iter() {
            let next = State {
                idx: edge.to,
                total_cost: total_cost + edge.cost,
                prev_idx: idx,
            };
            if next.total_cost < costs[next.idx] {
                costs[next.idx] = next.total_cost;
                heap.push(next);
            }
        }
    }
    (costs, prev_nodes)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![Edge { to: 2, cost: 10 }, Edge { to: 1, cost: 1 }],
            vec![Edge { to: 3, cost: 2 }],
            vec![
                Edge { to: 1, cost: 1 },
                Edge { to: 3, cost: 3 },
                Edge { to: 4, cost: 1 },
            ],
            vec![Edge { to: 0, cost: 7 }, Edge { to: 4, cost: 2 }],
            vec![],
        ];
        let (costs, route) = dijkstra(&graph, 3, 0);
        assert_eq!(costs[0], 7);
        assert_eq!(costs[3], 0);
        assert_eq!(route[0], 3);
        assert_eq!(route[3], 0);
    }
}
