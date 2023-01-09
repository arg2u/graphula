use graphula::adj::matrix::Matrix;

fn main() {
    // Node 0 - Home
    // Node 1 - A
    // Node 2 - B
    // Node 3 - C
    // Node 4 - D
    // Node 5 - Work
    let mut graph: Matrix = Matrix::new(6, 0);
    graph.add_w_directed_edge(0, 1, 2);
    graph.add_w_directed_edge(0, 2, 1);
    graph.add_w_directed_edge(1, 3, 6);
    graph.add_w_directed_edge(1, 4, 2);
    graph.add_w_directed_edge(2, 3, 2);
    graph.add_w_directed_edge(2, 4, 7);
    graph.add_w_directed_edge(3, 5, 10);
    graph.add_w_directed_edge(4, 5, 2);
    let res = graph.dijsktra(0, 5);
    if res.is_some() {
        let res = res.unwrap();
        println!("Weights sum: {:?}", res.0);
        println!("Path: {:?}", res.1);
    }
}
