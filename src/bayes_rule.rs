pub use petgraph::{
    data::Build,
    graph::{node_index, EdgeIndex, Graph, Node, NodeIndex},
    visit::{EdgeIndexable, EdgeRef, IntoEdges},
    Undirected,
};

use crate::incidence_constructor;

pub struct Probability {
    pub forward_prop: Graph<String, f64, Undirected>,
    pub backward_prop: Graph<String, f64, Undirected>,
}

pub struct Hypothesis {
    pub name: String,
    pub prob: f64,
    pub s_cond_prob: f64,
    pub f_cond_prob: f64,
}


impl Probability {
    pub fn new(forward_prop: Graph<String, f64, Undirected>) -> Self {
        Self {
            forward_prop,
            backward_prop: Graph::<String, f64, Undirected>::new_undirected(),
        }
    }

    pub fn new_decision_tree(hypotheses: Vec<Hypothesis>) -> Graph<String, f64, Undirected> {
    
    let mut d_tree = Graph::<String, f64, Undirected>::new_undirected();
    let strat_node = d_tree.add_node("Start".into()); //0 index of node is reserved for Start
    for i in 0..hypotheses.len() {
        //Creating hypotheses nodes and edges (prob)
        let h_node = d_tree.add_node(hypotheses[i].name.clone());
        d_tree.add_edge(strat_node, h_node, hypotheses[i].prob);
    }
    for i in 0..hypotheses.len() {
        //Creating evidence nodes and edges (e_cond_prob)
        let s_node = d_tree.add_node(format!("{}_Success", hypotheses[i].name));
        d_tree.add_edge(NodeIndex::new(i + 1), s_node, hypotheses[i].s_cond_prob);
        let f_node = d_tree.add_node(format!("{}_Failure", hypotheses[i].name));
        d_tree.add_edge(NodeIndex::new(i + 1), f_node, hypotheses[i].f_cond_prob);
    }

    for (e, i) in hypotheses.iter().enumerate() {
        assert_eq!(
            i.prob,
            d_tree.edge_weight(EdgeIndex::new(e)).unwrap().clone()
        );
    }

   d_tree
}


    pub fn upt_forward_prop_pos(&mut self) {
        let mut endpoints = self
            .forward_prop
            .edge_endpoints((1 - 1).into())
            .expect("No such edge in forward_prop graph");
        self.forward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            self.backward_prop
                .edge_weight((2 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone(),
        );

        endpoints = self
            .forward_prop
            .edge_endpoints((4 - 1).into())
            .expect("No such edge in forward_prop graph");
        self.forward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            self.backward_prop
                .edge_weight((3 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone(),
        );

        endpoints = self
            .forward_prop
            .edge_endpoints((7 - 1).into())
            .expect("No such edge in forward_prop graph");
        self.forward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            self.backward_prop
                .edge_weight((4 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone(),
        );
    }

    pub fn upt_forward_prop_neg(&mut self) {
        let mut endpoints = self
            .forward_prop
            .edge_endpoints((1 - 1).into())
            .expect("No such edge in forward_prop graph");
        self.forward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            self.backward_prop
                .edge_weight((6 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone(),
        );

        endpoints = self
            .forward_prop
            .edge_endpoints((4 - 1).into())
            .expect("No such edge in forward_prop graph");
        self.forward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            self.backward_prop
                .edge_weight((7 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone(),
        );

        endpoints = self
            .forward_prop
            .edge_endpoints((7 - 1).into())
            .expect("No such edge in forward_prop graph");
        self.forward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            self.backward_prop
                .edge_weight((8 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone(),
        );
    }

    pub fn upt_backward_prop(&mut self) {
        let mut endpoints = self
            .backward_prop
            .edge_endpoints((1 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop
            .update_edge(endpoints.0, endpoints.1, self.find_prob_a().0);

        endpoints = self
            .backward_prop
            .edge_endpoints((5 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop
            .update_edge(endpoints.0, endpoints.1, self.find_prob_a().1);

        endpoints = self
            .backward_prop
            .edge_endpoints((2 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            (self
                .forward_prop
                .edge_weight((1 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone()
                * self
                    .forward_prop
                    .edge_weight((2 - 1).into())
                    .expect("No such edge in forward_prop graph")
                    .clone())
                / self.find_prob_a().0,
        );

        endpoints = self
            .backward_prop
            .edge_endpoints((3 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            (self
                .forward_prop
                .edge_weight((4 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone()
                * self
                    .forward_prop
                    .edge_weight((5 - 1).into())
                    .expect("No such edge in forward_prop graph")
                    .clone())
                / self.find_prob_a().0,
        );

        endpoints = self
            .backward_prop
            .edge_endpoints((4 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            (self
                .forward_prop
                .edge_weight((7 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone()
                * self
                    .forward_prop
                    .edge_weight((8 - 1).into())
                    .expect("No such edge in forward_prop graph")
                    .clone())
                / self.find_prob_a().0,
        );

        endpoints = self
            .backward_prop
            .edge_endpoints((6 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            (self
                .forward_prop
                .edge_weight((1 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone()
                * self
                    .forward_prop
                    .edge_weight((3 - 1).into())
                    .expect("No such edge in forward_prop graph")
                    .clone())
                / self.find_prob_a().1,
        );

        endpoints = self
            .backward_prop
            .edge_endpoints((7 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            (self
                .forward_prop
                .edge_weight((4 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone()
                * self
                    .forward_prop
                    .edge_weight((6 - 1).into())
                    .expect("No such edge in forward_prop graph")
                    .clone())
                / self.find_prob_a().1,
        );

        endpoints = self
            .backward_prop
            .edge_endpoints((8 - 1).into())
            .expect("No such edge in backward_prop graph");
        self.backward_prop.update_edge(
            endpoints.0,
            endpoints.1,
            (self
                .forward_prop
                .edge_weight((7 - 1).into())
                .expect("No such edge in forward_prop graph")
                .clone()
                * self
                    .forward_prop
                    .edge_weight((9 - 1).into())
                    .expect("No such edge in forward_prop graph")
                    .clone())
                / self.find_prob_a().1,
        );
    }

    fn find_prob_h(&self) -> (f64, f64, f64) {
        let e1 = self
            .backward_prop
            .edge_weight((1 - 1).into())
            .unwrap()
            .clone();
        let e2 = self
            .backward_prop
            .edge_weight((2 - 1).into())
            .unwrap()
            .clone();
        let e3 = self
            .backward_prop
            .edge_weight((3 - 1).into())
            .unwrap()
            .clone();
        let e5 = self
            .backward_prop
            .edge_weight((5 - 1).into())
            .unwrap()
            .clone();
        let e6 = self
            .backward_prop
            .edge_weight((6 - 1).into())
            .unwrap()
            .clone();
        let e7 = self
            .backward_prop
            .edge_weight((7 - 1).into())
            .unwrap()
            .clone();

        let prob_h1 = e1 * e2 + e5 * e6;
        let prob_h2 = e1 * e3 + e5 * e7;
        let prob_h3 = 1.0 - prob_h1 - prob_h2;

        (prob_h1, prob_h2, prob_h3)
    }

    fn find_prob_a(&self) -> (f64, f64) {
        let e1 = self
            .forward_prop
            .edge_weight((1 - 1).into())
            .unwrap()
            .clone();
        let e2 = self
            .forward_prop
            .edge_weight((2 - 1).into())
            .unwrap()
            .clone();
        let e4 = self
            .forward_prop
            .edge_weight((4 - 1).into())
            .unwrap()
            .clone();
        let e5 = self
            .forward_prop
            .edge_weight((5 - 1).into())
            .unwrap()
            .clone();
        let e7 = self
            .forward_prop
            .edge_weight((7 - 1).into())
            .unwrap()
            .clone();
        let e8 = self
            .forward_prop
            .edge_weight((8 - 1).into())
            .unwrap()
            .clone();

        let prob_a = e1 * e2 + e4 * e5 + e7 * e8;
        (prob_a, 1.0 - prob_a)
    }
}
