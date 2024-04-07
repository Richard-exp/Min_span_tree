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
    pub h_num: usize,
    pub h_names: Vec<String>,
}

#[derive(Clone)]
pub struct Hypothesis {
    pub name: String,
    pub prob: f64,
    pub s_cond_prob: f64,
    pub f_cond_prob: f64,
}

impl Probability {
    pub fn new(hypotheses: Vec<Hypothesis>) -> Self {
        let mut prob = Self {
            forward_prop: Probability::new_forward_tree(hypotheses.clone()),
            backward_prop: Graph::<String, f64, Undirected>::new_undirected(),
            h_num: hypotheses.len(),
            h_names: hypotheses.iter().map(|h| h.name.to_owned()).collect(),
        };
        prob.new_backward_tree();
        prob
    }

    fn new_backward_tree(&mut self) {
        let prob_s = self.find_prob_s();

        let start_node = self.backward_prop.add_node("Start".into()); //0 index of node is reserved for Start
        let s_node = self.backward_prop.add_node("Success".into());
        self.backward_prop.add_edge(start_node, s_node, prob_s);
        let f_node = self.backward_prop.add_node("Failure".into());
        self.backward_prop
            .add_edge(start_node, f_node, 1.0 - prob_s);

        for (e, i) in ((self.h_num..(self.h_num * 3)).step_by(2)).enumerate() {
            let h_node = self
                .backward_prop
                .add_node(format!("Success_{}", self.h_names[e]));
            let prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(e))
                .unwrap()
                .clone();
            let cond_prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(i))
                .unwrap()
                .clone();
            self.backward_prop
                .add_edge(s_node, h_node, (prob * cond_prob) / prob_s);
        }

        for (e, i) in ((self.h_num..(self.h_num * 3)).step_by(2)).enumerate() {
            let h_node = self
                .backward_prop
                .add_node(format!("Success_{}", self.h_names[e]));
            let prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(e))
                .unwrap()
                .clone();
            let cond_prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(i + 1))
                .unwrap()
                .clone();
            self.backward_prop
                .add_edge(f_node, h_node, (prob * cond_prob) / (1.0 - prob_s));
        }
    }

    fn find_prob_s(&self) -> f64 {
        let mut prob_s = 0.0;
        for (e, i) in ((self.h_num..(self.h_num * 3)).step_by(2)).enumerate() {
            let prop = self
                .forward_prop
                .edge_weight(EdgeIndex::new(e))
                .unwrap()
                .clone();
            let cond_prop = self
                .forward_prop
                .edge_weight(EdgeIndex::new(i))
                .unwrap()
                .clone();
            prob_s += prop * cond_prop;
        }

        prob_s
    }

    fn new_forward_tree(hypotheses: Vec<Hypothesis>) -> Graph<String, f64, Undirected> {
        let mut f_tree = Graph::<String, f64, Undirected>::new_undirected();
        let start_node = f_tree.add_node("Start".into()); //0 index of node is reserved for Start
        for i in 0..hypotheses.len() {
            //Creating hypotheses nodes and edges (prob)
            let h_node = f_tree.add_node(hypotheses[i].name.clone());
            f_tree.add_edge(start_node, h_node, hypotheses[i].prob);
        }
        for i in 0..hypotheses.len() {
            //Creating evidence nodes and edges (e_cond_prob)
            let s_node = f_tree.add_node(format!("{}_Success", hypotheses[i].name));
            f_tree.add_edge(NodeIndex::new(i + 1), s_node, hypotheses[i].s_cond_prob);
            let f_node = f_tree.add_node(format!("{}_Failure", hypotheses[i].name));
            f_tree.add_edge(NodeIndex::new(i + 1), f_node, hypotheses[i].f_cond_prob);
        }

        f_tree
    }

    pub fn upt_forward_tree_pos(&mut self) {
        for (e, i) in (2..(2 + self.h_num)).enumerate() {
            let endpoints = self.forward_prop.edge_endpoints(EdgeIndex::new(e)).unwrap();
            self.forward_prop.update_edge(
                endpoints.0,
                endpoints.1,
                self.backward_prop
                    .edge_weight(EdgeIndex::new(i))
                    .unwrap()
                    .to_owned(),
            );
        }
    }

    pub fn upt_forward_tree_neg(&mut self) {
        for (e, i) in ((2 + self.h_num)..(2 + self.h_num * 2)).enumerate() {
            let endpoints = self.forward_prop.edge_endpoints(EdgeIndex::new(e)).unwrap();
            self.forward_prop.update_edge(
                endpoints.0,
                endpoints.1,
                self.backward_prop
                    .edge_weight(EdgeIndex::new(i))
                    .unwrap()
                    .to_owned(),
            );
        }
    }

    pub fn upt_backward_tree(&mut self) {
        let prob_s = self.find_prob_s();

        let endpoints = self
            .backward_prop
            .edge_endpoints(EdgeIndex::new(0))
            .unwrap();
        self.backward_prop
            .update_edge(endpoints.0, endpoints.1, prob_s);

        let endpoints = self
            .backward_prop
            .edge_endpoints(EdgeIndex::new(1))
            .unwrap();
        self.backward_prop
            .update_edge(endpoints.0, endpoints.1, 1.0 - prob_s);

        let mut i_b = 2..(2 + self.h_num);
        for (e, i_f) in ((self.h_num..(self.h_num * 3)).step_by(2)).enumerate() {
            let prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(e))
                .unwrap()
                .clone();
            let cond_prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(i_f))
                .unwrap()
                .clone();

            let endpoints = self
                .backward_prop
                .edge_endpoints(EdgeIndex::new(i_b.next().unwrap()))
                .unwrap();
            self.backward_prop
                .update_edge(endpoints.0, endpoints.1, (prob * cond_prob) / prob_s);
        }

        let mut i_b = (2 + self.h_num)..(2 + self.h_num * 2);
        for (e, i_f) in ((self.h_num..(self.h_num * 3)).step_by(2)).enumerate() {
            let prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(e))
                .unwrap()
                .clone();
            let cond_prob = self
                .forward_prop
                .edge_weight(EdgeIndex::new(i_f + 1))
                .unwrap()
                .clone();

            let endpoints = self
                .backward_prop
                .edge_endpoints(EdgeIndex::new(i_b.next().unwrap()))
                .unwrap();
            self.backward_prop.update_edge(
                endpoints.0,
                endpoints.1,
                (prob * cond_prob) / (1.0 - prob_s),
            );
        }
    }
}
