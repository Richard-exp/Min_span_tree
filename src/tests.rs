use crate::{bayes_rule::*};
#[test]
pub fn new_decision_tree() {
    let hypotheses: Vec<Hypothesis> = vec![
        Hypothesis {
            name: "H1".into(),
            prob: 0.1,
            s_cond_prob: 0.8,
            f_cond_prob: 0.2,
        },
        Hypothesis {
            name: "H2".into(),
            prob: 0.2,
            s_cond_prob: 0.7,
            f_cond_prob: 0.3,
        },
        Hypothesis {
            name: "H3".into(),
            prob: 0.7,
            s_cond_prob: 0.1,
            f_cond_prob: 0.9,
        },
    ];

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

    for (e, i) in ((hypotheses.len()..(hypotheses.len() * 3)).step_by(2)).enumerate() {
        println!("{} {}", e, i);
        assert_eq!(
            hypotheses[e].s_cond_prob,
            d_tree.edge_weight(EdgeIndex::new(i)).unwrap().clone()
        );
        assert_eq!(
            hypotheses[e].f_cond_prob,
            d_tree.edge_weight(EdgeIndex::new(i + 1)).unwrap().clone()
        );
    }
}
