pub mod row;

use row::Row;
use std::fmt::{Display, Formatter, Result};
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
        self.add_weighted_edge(m, n, -1);
    }

    /// Adds a directed edge with weight
    pub fn add_w_directed_edge(&mut self, n: usize, m: usize, w: i32) {
        self.add_weighted_edge(n, m, w);
        self.add_weighted_edge(m, n, -w);
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
    pub fn bfs(&self, starter_node: usize, target_node: usize) -> bool {
        let mut queue = vec![];
        queue.push(starter_node);
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
}
