use petgraph::{
    graph::{Graph, NodeIndex},
    Undirected,
};

pub fn create_from_inc_dir(g: &mut Graph<String, i32>) {
    let in_matrix: [[i32; 5]; 7] = [
        [1, -1, 0, 0, 0],
        [0, 1, -1, 0, 0],
        [0, 0, 1, -1, 0],
        [0, -1, 0, 1, 0],
        [0, 0, 0, 1, -1],
        [0, 1, 0, 0, -1],
        [-1, 0, 0, 0, 1],
    ];

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
        g.add_edge(node_1, node_2, 1);
    }
}

pub fn create_from_inc(g: &mut Graph<String, i32, Undirected>) {
    let in_matrix: [[i32; 5]; 7] = [
        [1, 1, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 1, 1],
        [0, 1, 0, 0, 1],
        [0, 1, 0, 0, 1],
    ];

    for edge in 0..in_matrix.len() {
        let mut node_1 = NodeIndex::new(usize::MAX);
        let mut node_2: NodeIndex;
        for node in 0..in_matrix[edge].len() {
            if in_matrix[edge][node] != 1 {
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

            g.add_edge(node_1, node_2, 1);
        }
    }
}
