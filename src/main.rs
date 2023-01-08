use graphula::adj::matrix::Matrix;

fn main() {
    // Node 0 - Me
    // Node 1 - Ruslan
    // Node 2 - Maksim
    // Node 4 - Marat
    // Node 3 - Peter
    // Node 5 - Sveta
    let mut graph: Matrix = Matrix::new(6, 0);
    graph.add_directed_edge(0, 1);
    graph.add_directed_edge(0, 2);
    graph.add_directed_edge(1, 4);
    graph.add_directed_edge(2, 3);
    graph.add_directed_edge(3, 5);
    graph.bfs(0, 4);
}
