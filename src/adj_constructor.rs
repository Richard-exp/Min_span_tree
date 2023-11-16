use petgraph::{
    graph::{Graph, NodeIndex},
    Undirected,
};

pub fn create_from_adj_dir(g: &mut Graph<String, i32>) {
    let a_matrix: [[i32; 5]; 5] = [
        [1, 1, 0, 0, 0],
        [0, 1, 1, 0, 1],
        [0, 0, 1, 1, 0],
        [0, 1, 0, 1, 1],
        [1, 0, 0, 0, 1],
    ];

    for i in 0..a_matrix.len() {
        for j in 0..a_matrix[i].len() {
            if a_matrix[i][j] != 1 {
                continue;
            }

            let node_1: NodeIndex;
            let node_2: NodeIndex;

            if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", i + 1)) {
                node_1 = node;
            } else {
                node_1 = g.add_node(format!("{}", i + 1));
            }

            if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", j + 1)) {
                node_2 = node;
            } else {
                node_2 = g.add_node(format!("{}", j + 1));
            }

            if i == j {
                continue;
            }

            g.add_edge(node_1, node_2, 1);
        }
    }
}

pub fn create_from_adj(g: &mut Graph<String, i32, Undirected>) {
    let a_matrix: [[i32; 5]; 5] = [
        [1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
    ];

    for i in 0..a_matrix.len() {
        for j in i..a_matrix[i].len() {
            if a_matrix[i][j] != 1 {
                continue;
            }

            let node_1: NodeIndex;
            let node_2: NodeIndex;

            if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", i + 1)) {
                node_1 = node;
            } else {
                node_1 = g.add_node(format!("{}", i + 1));
            }

            if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", j + 1)) {
                node_2 = node;
            } else {
                node_2 = g.add_node(format!("{}", j + 1));
            }

            if i == j {
                continue;
            }

            g.add_edge(node_1, node_2, 1);
        }
    }
}
