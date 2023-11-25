use petgraph::{
    algo::dijkstra,
    graph::{node_index, Graph, NodeIndex},
    stable_graph::IndexType,
    visit::{Control, DfsEvent},
    Undirected,
};

mod adj_constructor;
mod incidence_constructor;
mod utils;
mod dijkstra_algo;

fn main() {
    // let mut g = Graph::<String, i32>::new();
    // let mut g = Graph::<String, i32, Undirected>::new_undirected();

    // incidence_constructor::create_from_inc_dir(
    //     &mut g,
    //     incidence_constructor::weighted_in_matrix_dir(),
    // );
    // incidence_constructor::create_from_inc(&mut g, incidence_constructor::weighted_in_matrix());
    // incidence_constructor::create_from_inc_dir(&mut g, incidence_constructor::in_matrix_dir());
    // incidence_constructor::create_from_inc_dir(&mut g, incidence_constructor::in_matrix_dir());
    // adj_constructor::create_from_adj(&mut g, adj_constructor::adj_matrix());
    // adj_constructor::create_from_adj_dir(&mut g, adj_constructor::adj_matrix_dir());

    // utils::calc_degrees(&g);

    // utils::calc_in_degrees_dir(&g);
    // utils::calc_out_degrees_dir(&g);

    // utils::find_eccentrities_dir(&mut g);
    // utils::find_eccentrities(&mut g);

    // let start: NodeIndex = NodeIndex::new(0);
    // let finish: NodeIndex = NodeIndex::new(7);
    // let paths = dijkstra(&g, start, None, |e| *e.weight());
    // println!("From {:?} to all other nodes: {:?}", start, paths);
    // println!(
    //     "From {:?} to {:?} - the shortest path: {:?}",
    //     start,
    //     finish,
    //     paths.get(&finish).unwrap()
    // );

    // println!("\n{:?}", g);

    dijkstra_algo::dijkstra_run();
}
