use petgraph::{
    graph::{Graph, NodeIndex, node_index},
    visit::depth_first_search,
    visit::{DfsEvent, Control},
};

mod adj_constructor;
mod degrees;
mod incidence_constructor;

fn main() {
    let mut g = Graph::<String, i32>::new();
    incidence_constructor::create_from_inc_dir(&mut g);

    
    
    //println!("{:?}", g);
}
