pub mod row;

use row::Row;
use std::fmt::{Display, Formatter, Result};
use std::i32::MAX;
use std::ops::{Index, IndexMut};
use std::vec;

pub struct Matrix {
    /// Number of nodes
    pub n: usize,
    data: Vec<Row>,
}

impl Index<usize> for Matrix {
    type Output = Row;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Matrix {
    /// Creates a new instance of Matrix
    /// with n nodes
    pub fn new(nodes: usize, filler: i32) -> Self {
        Self {
            n: nodes,
            data: vec![Row::new(nodes, filler); nodes],
        }
    }
}

impl Matrix {
    /// Adds a weighted edge
    pub fn add_weighted_edge(&mut self, n: usize, m: usize, val: i32) {
        self[n][m] = val;
    }

    /// Adds a directed edge without weight
    pub fn add_directed_edge(&mut self, n: usize, m: usize) {
        self.add_weighted_edge(n, m, 1);
    }

    /// Adds a directed edge with weight
    pub fn add_w_directed_edge(&mut self, n: usize, m: usize, w: i32) {
        self.add_weighted_edge(n, m, w);
    }

    /// Adds an undirected edge
    pub fn add_undirected_edge(&mut self, n: usize, m: usize) {
        self.add_weighted_edge(n, m, 1);
        self.add_weighted_edge(m, n, 1);
    }
}

impl Matrix {
    /// Returns true if some node has adjacent nodes
    pub fn has_adjs(&self, node: usize) -> bool {
        !self.get_adjs(node).is_empty()
    }

    /// Returns adjacent nodes
    pub fn get_adjs(&self, node: usize) -> Vec<usize> {
        let mut adjs = vec![];
        for j in 0..self.n {
            if self[node][j] > 0 {
                adjs.push(j);
            }
        }
        adjs
    }

    /// Returns adjacent nodes count
    pub fn adjs_count(&self, node: usize) -> usize {
        self.get_adjs(node).len()
    }
}

impl Matrix {
    // Calculates a node with the smallest distance for the Dijkstra algorithm
    pub fn min_distance(&self, dists: &mut Vec<i32>, done: &mut Vec<bool>) -> usize {
        let mut min = MAX;
        let mut min_node = 0;
        for v in 0..self.n {
            if !done[v] && dists[v] <= min {
                min = dists[v];
                min_node = v;
            }
        }
        return min_node;
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut matrix = String::new();
        if !self.data.is_empty() {
            for row in &self.data {
                matrix.push_str(&row.to_string());
                matrix.push('\n');
            }
        } else {
            matrix = String::from("Matrix is empty!");
        }
        write!(f, "{}", matrix)
    }
}

impl Matrix {
    pub fn bfs(&self, start_node: usize, target_node: usize) -> bool {
        let mut queue = vec![];
        queue.push(start_node);
        let mut searched = vec![];
        while !queue.is_empty() {
            let node = queue.remove(0);
            if !searched.contains(&node) {
                if node == target_node {
                    println!("The target node was found!");
                    println!("Searched: {:?}", searched);
                    return true;
                } else {
                    queue.extend(self.get_adjs(node));
                    searched.push(node);
                }
                println!("Node: {:?}", node);
                println!("Q: {:?}\n", queue);
            }
        }
        return false;
    }

    pub fn dijsktra(&self, start_node: usize, target_node: usize) -> Option<(i32, Vec<usize>)> {
        let mut dists: Vec<i32> = vec![MAX; self.n];
        let mut done: Vec<bool> = vec![false; self.n];
        dists[start_node] = 0;
        let mut path: Vec<usize> = vec![0; self.n];
        for _ in 0..(self.n - 1) {
            let min_node = self.min_distance(&mut dists, &mut done);
            done[min_node] = true;
            for adj_node in 0..self.n {
                let not_zero = self[min_node][adj_node] != 0;
                let new_dist = dists[min_node] + self[min_node][adj_node];
                if !done[adj_node]
                    && not_zero
                    && dists[min_node] != MAX
                    && new_dist < dists[adj_node]
                {
                    path[adj_node] = min_node;
                    dists[adj_node] = new_dist;
                }
            }
        }
        if dists[target_node] == MAX {
            return None;
        }
        let mut target_path = path[target_node];
        let mut final_path = vec![target_node];
        while !final_path.contains(&0) {
            final_path.push(target_path);
            target_path = path[target_path];
        }
        final_path.reverse();
        return Some((dists[target_node], final_path));
    }
}
