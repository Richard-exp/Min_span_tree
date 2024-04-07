use petgraph::{
    graph::{Graph, NodeIndex},
    Undirected,
};

pub fn weighted_in_matrix_dir() -> [[i32; 8]; 13] {
    [
        [1, -8, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, -17, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, -11],
        [0, 0, 0, 0, 0, 0, 1, -13],
        [0, 0, 0, 1, 0, 0, -20, 0],
        [1, 0, 0, -6, 0, 0, 0, 0],
        [0, 0, -8, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, -11, 0],
        [0, 0, 0, 0, 0, -9, 1, 0],
        [0, 0, 0, 0, 0, 1, 0, -9],
        [0, 0, 0, 0, 1, -5, 0, 0],
        [0, 0, 1, 0, -9, 0, 0, 0],
        [0, 1, -7, 0, 0, 0, 0, 0],
    ]
}

pub fn weighted_in_matrix() -> [[f64; 10]; 9] {
    [
        [1.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0, 0.0, 0.95, 0.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.02, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.98, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.03, 0.0],
        [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.92],
    ]
}

pub fn in_matrix() -> [[i32; 5]; 7] {
    [
        [1, 1, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 1, 1],
        [0, 1, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ]
}

pub fn in_matrix_dir() -> [[i32; 5]; 7] {
    [
        [1, -1, 0, 0, 0],
        [0, 1, -1, 0, 0],
        [0, 0, 1, -1, 0],
        [0, -1, 0, 1, 0],
        [0, 0, 0, 1, -1],
        [0, 1, 0, 0, -1],
        [-1, 0, 0, 0, 1],
    ]
}

pub fn create_weighted_from_inc_dir(g: &mut Graph<String, i32>, in_matrix: [[i32; 8]; 13]) {
    for edge in 0..in_matrix.len() {
        let mut node_1 = NodeIndex::new(0);
        let mut node_2 = NodeIndex::new(0);
        for node in 0..in_matrix[edge].len() {
            if in_matrix[edge][node] == 0 {
                continue;
            }

            if in_matrix[edge][node] == 1 {
                if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", node + 1)) {
                    node_1 = node;
                } else {
                    node_1 = g.add_node(format!("{}", node + 1));
                }
            } else {
                if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", node + 1)) {
                    node_2 = node;
                } else {
                    node_2 = g.add_node(format!("{}", node + 1));
                }
            }
        }
        g.add_edge(
            node_1,
            node_2,
            -in_matrix[edge][g.node_weight(node_2).unwrap().parse::<usize>().unwrap() - 1],
        );
    }
}

pub fn create_weighted_from_inc(g: &mut Graph<String, f64, Undirected>, in_matrix: [[f64; 10]; 9]) {
    for edge in 0..in_matrix.len() {
        let mut node_1 = NodeIndex::new(usize::MAX);
        let mut node_2: NodeIndex;
        for node in 0..in_matrix[edge].len() {
            if in_matrix[edge][node] == 0.0 {
                continue;
            }

            if node_1 == NodeIndex::new(usize::MAX) {
                if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", node + 1)) {
                    node_1 = node;
                } else {
                    node_1 = g.add_node(format!("{}", node + 1));
                }
                continue;
            }

            if let Some(node) = g.node_indices().find(|n| g[*n] == format!("{}", node + 1)) {
                node_2 = node;
            } else {
                node_2 = g.add_node(format!("{}", node + 1));
            }

            g.add_edge(
                node_1,
                node_2,
                in_matrix[edge][g.node_weight(node_2).unwrap().parse::<usize>().unwrap() - 1],
            );
        }
    }
}

