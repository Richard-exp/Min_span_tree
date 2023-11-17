use petgraph::{
    graph::{node_index, Graph, NodeIndex},
    visit::{Control, DfsEvent},
    Undirected,
};

mod adj_constructor;
mod utils;
mod incidence_constructor;

fn main() {
    // let mut g = Graph::<String, i32>::new();
    let mut g = Graph::<String, i32, Undirected>::new_undirected();

    // incidence_constructor::create_from_inc_dir(&mut g, incidence_constructor::in_matrix_dir());
    incidence_constructor::create_from_inc(&mut g, incidence_constructor::weighted_in_matrix());
    // adj_constructor::create_from_adj(&mut g, adj_constructor::adj_matrix());
    // adj_constructor::create_from_adj_dir(&mut g, adj_constructor::adj_matrix_dir());

    // utils::calc_degrees(&g);

    // utils::calc_in_degrees_dir(&g);
    // utils::calc_out_degrees_dir(&g);
    
    //utils::find_eccentrities_dir(&mut g);
    utils::find_eccentrities(&mut g);

    println!("{:?}", g);
}
