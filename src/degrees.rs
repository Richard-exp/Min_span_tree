use petgraph::{
    graph::Graph,
    Direction::{Incoming, Outgoing},
    Undirected,
};

pub fn calc_in_degrees_dir(g: &Graph<String, i32>) {
    let mut degrees = 0;
    for i in 0..g.node_count() {
        let node = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();
        let node_degree = g.neighbors_directed(node, Incoming).count();
        degrees = degrees + node_degree;
        println!("#{} node has in-degree(-): {}", i + 1, node_degree);
    }
    println!("Total nodes in-degrees(-): {}\n", degrees);
}

pub fn calc_out_degrees_dir(g: &Graph<String, i32>) {
    let mut degrees = 0;
    for i in 0..g.node_count() {
        let node = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();
        let node_degree = g.neighbors_directed(node, Outgoing).count();
        degrees = degrees + node_degree;
        println!("#{} node has out-degree(+): {}", i + 1, node_degree);
    }
    println!("Total nodes out-degrees(+): {}\n", degrees);
}

pub fn calc_degrees(g: &Graph<String, i32, Undirected>) {
    let mut degrees = 0;
    for i in 0..g.node_count() {
        let node = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();
        let node_degree = g.neighbors(node).count();
        degrees = degrees + node_degree;
        println!("#{} node has degree: {}\n", i + 1, node_degree);
    }
    println!("Total nodes degree: {}\n", degrees);
}
