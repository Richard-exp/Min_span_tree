use std::default;

use petgraph::{
    adj::NodeIndex,
    graph::{Graph, Node},
    matrix_graph::MatrixGraph,
    Undirected,
};
enum Nodes {
    one = 1,
    two,
    three,
    four,
    five,
}

fn main() {
    let mut g = Graph::<String, i32, Undirected>::new_undirected();

    let a_matrix: [[i32; 5]; 5] = [
        [1, 1, 0, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
    ];

    let in_matrix: [[i32; 7]; 5] = [
        [1, 0, 0, 0, 0, 0, 0],
        [1, 1, 0, 1, 0, 1, 1],
        [0, 1, 1, 0, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 1, 1, 1],
    ];

    let mut j_count: i32 = -1;
    for i in 0..a_matrix.len() {
        for j in 0..a_matrix[i].len() {
            if a_matrix[i][j] != 1 {
                continue;
            }
            println!("{:?}", g);

            if i
            let node_1 = g.add_node(format!("{}", i + 1));
            let mut node_2 = node_1;

            if j as i32 <= j_count || j == i{
                node_2 = g
                    .node_indices()
                    .find(|i| g[*i] == format!("{}", j + 1))
                    .unwrap();
                //println!("{}", node_1)
            } else {
                node_2 = g.add_node(format!("{}", j + 1));
            }

            g.add_edge(node_1, node_2, 1);
            j_count += 1;
        }
    }

    println!("{:?}", g);
}
