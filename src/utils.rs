use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
    visit::depth_first_search,
    visit::{Control, DfsEvent},
    Direction::{Incoming, Outgoing},
    Undirected,
};

pub fn find_eccentrities_dir(g: &mut Graph<String, i32>) {
    let mut eccentricities: Vec<(String, i32)> = Vec::new();

    for i in 0..g.node_count() {
        let start = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();

        let paths = dijkstra(&*g, start, None, |e| *e.weight());
        println!(
            "From {:?} to all other nodes: {:?}",
            start.index() + 1,
            paths
        );
        // println!(
        //     "#{} From {:?} to {:?} - the shortest path: {:?}",
        //     j + 1,
        //     node_1,
        //     node_2,
        //     path
        // );

        eccentricities.push(
            paths
                .iter()
                .max_by_key(|(_, v)| *v)
                .map(|(n, v)| (format!("Node: {}", (*n).index() + 1), *v))
                .unwrap(),
        );
    }
    println!("\nEccentricities: {:?}", eccentricities);
    println!(
        "\nRadius: {:?}",
        eccentricities.iter().min_by_key(|&(_, v)| v).unwrap()
    );
    println!(
        "\nDiameter: {:?}",
        eccentricities.iter().max_by_key(|&(_, v)| v).unwrap()
    );
}

pub fn find_eccentrities(g: &mut Graph<String, i32, Undirected>) {
    let mut eccentricities: Vec<(String, i32)> = Vec::new();

    for i in 0..g.node_count() {
        let start = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();

        let paths = dijkstra(&*g, start, None, |e| *e.weight());
        println!(
            "From {:?} to all other nodes: {:?}",
            start.index() + 1,
            paths
        );
        // println!(
        //     "#{} From {:?} to {:?} - the shortest path: {:?}",
        //     j + 1,
        //     node_1,
        //     node_2,
        //     path
        // );

        eccentricities.push(
            paths
                .iter()
                .max_by_key(|(_, v)| *v)
                .map(|(n, v)| (format!("Node: {}", (*n).index() + 1), *v))
                .unwrap(),
        );
    }
    println!("\nEccentricities: {:?}", eccentricities);
    println!(
        "\nRadius: {:?}",
        eccentricities.iter().min_by_key(|&(_, v)| v).unwrap()
    );
    println!(
        "\nDiameter: {:?}",
        eccentricities.iter().max_by_key(|&(_, v)| v).unwrap()
    );
}

pub fn dfs(
    g: &mut Graph<String, i32, Undirected>,
    start: NodeIndex,
    goal: NodeIndex,
) -> Vec<NodeIndex> {
    let mut predecessor = vec![NodeIndex::end(); g.node_count()];
    depth_first_search(&*g, Some(start), |event| {
        if let DfsEvent::TreeEdge(u, v) = event {
            predecessor[v.index()] = u;
            if v == goal {
                return Control::Break(v);
            }
        }
        Control::Continue
    });

    let mut next = goal;
    let mut path = vec![next];
    while next != start {
        let pred = predecessor[next.index()];
        path.push(pred);
        next = pred;
    }
    path.reverse();
    path
}

pub fn dfs_dir(g: &mut Graph<String, i32>, start: NodeIndex, goal: NodeIndex) -> Vec<NodeIndex> {
    let mut predecessor = vec![NodeIndex::end(); g.node_count()];
    depth_first_search(&*g, Some(start), |event| {
        if let DfsEvent::TreeEdge(u, v) = event {
            predecessor[v.index()] = u;
            if v == goal {
                return Control::Break(v);
            }
        }
        Control::Continue
    });

    let mut next = goal;
    let mut path = vec![next];
    while next != start {
        let pred = predecessor[next.index()];
        path.push(pred);
        next = pred;
    }
    path.reverse();
    path
}

pub fn calc_in_degrees_dir(g: &Graph<String, i32>) {
    let mut degrees = 0;
    for i in 0..g.node_count() {
        let node = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();
        let node_degree = g.neighbors_directed(node, Incoming).count();
        degrees = degrees + node_degree;
        println!("#{} node has in-degree(-): {}", i + 1, node_degree);
    }
    println!("Total nodes in-degrees(-): {}\n", degrees);
}

pub fn calc_out_degrees_dir(g: &Graph<String, i32>) {
    let mut degrees = 0;
    for i in 0..g.node_count() {
        let node = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();
        let node_degree = g.neighbors_directed(node, Outgoing).count();
        degrees = degrees + node_degree;
        println!("#{} node has out-degree(+): {}", i + 1, node_degree);
    }
    println!("Total nodes out-degrees(+): {}\n", degrees);
}

pub fn calc_degrees(g: &Graph<String, f64, Undirected>) {
    let mut degrees = 0;
    for i in 0..g.node_count() {
        let node = g
            .node_indices()
            .find(|n| g[*n] == format!("{}", i + 1))
            .unwrap();
        let node_degree = g.neighbors(node).count();
        degrees = degrees + node_degree;
        println!("#{} node has degree: {}", i + 1, node_degree);
    }
    println!("Total nodes degree: {}\n", degrees);
}
