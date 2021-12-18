use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let map: Vec<Vec<usize>> = data
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut graph: Vec<Vec<Edge>> = Vec::new();

    for (i, line) in map.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            let mut node: Vec<Edge> = Vec::new();

            if j != 0 {
                node.push(Edge {
                    node: i * line.len() + j - 1,
                    cost: map[i][j - 1],
                })
            }

            if j != line.len() - 1 {
                node.push(Edge {
                    node: i * line.len() + j + 1,
                    cost: map[i][j + 1],
                })
            }

            if i != 0 {
                node.push(Edge {
                    node: (i - 1) * line.len() + j,
                    cost: map[i - 1][j],
                })
            }

            if i != map.len() - 1 {
                node.push(Edge {
                    node: (i + 1) * line.len() + j,
                    cost: map[i + 1][j],
                })
            }

            graph.push(node);
        }
    }

    let res = shortest_path(&graph, 0, map.len() * map[0].len() - 1).unwrap();

    println!("{:?}", res);
}
