use petgraph::{
    algo,
    adj,
    graph::{Graph, NodeIndex},
    stable_graph::IndexType,
    Undirected,
};

mod adj_constructor;
mod degrees;
mod incidence_constructor;

fn main() {
    let mut g = Graph::<String, i32>::new();

    incidence_constructor::create_from_inc_dir(&mut g);

    let ways = algo::all_simple_paths::<Vec<_>, _>(&graph, a, d, 0, None)
  .collect::<Vec<_>>();

    for i in 0..g.node_count() {
        let node_1 = g.node_indices().find(|n| g[*n] == format!("{}", i + 1)).unwrap();
        for j in 0..g.node_count() {
            let node_2 = g.node_indices().find(|n| g[*n] == format!("{}", j + 1)).unwrap();
            println!("{:?}", g.edges_connecting(node_1, node_2));
            break;
        }
        break;
    }

    //println!("{:?}", g);
}
