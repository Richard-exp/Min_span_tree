use petgraph::{
    algo::dijkstra,
    graph::{node_index, Graph, NodeIndex},
    stable_graph::IndexType,
    visit::{Control, DfsEvent},
    Undirected,
};

mod adj_constructor;
mod bayes_rule;
mod dijkstra_algo;
mod incidence_constructor;
mod tests;
mod utils;

fn main() {

    let hypotheses = vec![
        bayes_rule::Hypothesis {
            name: "H1".into(),
            prob: 0.7,
            s_cond_prob: 0.05,
            f_cond_prob: 0.95,
        },
        bayes_rule::Hypothesis {
            name: "H2".into(),
            prob: 0.2,
            s_cond_prob: 0.02,
            f_cond_prob: 0.98,
        },
        bayes_rule::Hypothesis {
            name: "H3".into(),
            prob: 0.1,
            s_cond_prob: 0.03,
            f_cond_prob: 0.92,
        },
    ];
    let mut prop = bayes_rule::Probability::new(hypotheses);

    println!("{:?}\n", prop.forward_prop);
    println!("{:?}\n", prop.backward_prop);

    prop.upt_forward_tree_pos();
    prop.upt_backward_tree();
    println!("{:?}\n", prop.backward_prop);

    // for i in 0..4 {
    //     prop.upt_backward_prop();
    //     prop.upt_forward_prop_pos();
    //     println!("#{i} backward_prop: {:?}\n", prop.backward_prop);
    //     println!("#{i} forward_prop: {:?}\n", prop.forward_prop);
    // }

    // for i in 0..1 {
    //     prop.upt_backward_prop();
    //     prop.upt_forward_prop_neg();
    //     println!("#{i} backward_prop: {:?}\n", prop.backward_prop);
    //     println!("#{i} forward_prop: {:?}\n", prop.forward_prop);
    // }
}
