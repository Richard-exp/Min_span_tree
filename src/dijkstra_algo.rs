use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn dijkstra_run() {
    let s = Vertex::new("s");
    let t = Vertex::new("t");
    let x = Vertex::new("x");
    let y = Vertex::new("y");
    let z = Vertex::new("z");
    let s_1 = Vertex::new("s_1");
    let t_1 = Vertex::new("t_1");
    let x_1 = Vertex::new("x_1");
    let y_1 = Vertex::new("y_1");
    let z_1 = Vertex::new("z_1");

    let mut adjacency_list = HashMap::new();
    adjacency_list.insert(s, vec![(t, 10), (y, 5)]);
    adjacency_list.insert(t, vec![(y, 2), (x, 1)]);
    adjacency_list.insert(x, vec![(z, 4), (s_1, 1)]);
    adjacency_list.insert(y, vec![(t, 3), (x, 9), (z, 2)]);
    adjacency_list.insert(z, vec![(s, 7), (x, 6)]);
    adjacency_list.insert(s_1, vec![(y, 2), (t_1, 1)]);
    // adjacency_list.insert(t_1, vec![(, ), (, )]);
    // adjacency_list.insert(x_1, vec![(, ), (, )]);
    // adjacency_list.insert(y_1, vec![(, ), (, )]);
    // adjacency_list.insert(z_1, vec![(, ), (, )]);

    let distances = dijkstra(s, &adjacency_list);

    for (v, d) in &distances {
        println!(
            "name: {}, distance: {}, path: {:?}",
            v.name, d.distance, d.parents
        );
    }

    // assert_eq!(distances.get(&t).unwrap().distance, 8);
    // assert_eq!(distances.get(&s).unwrap().distance, 0);
    // assert_eq!(distances.get(&y).unwrap().distance, 5);
    // assert_eq!(distances.get(&x).unwrap().distance, 9);
    // assert_eq!(distances.get(&z).unwrap().distance, 7);
}

fn dijkstra<'a>(
    start: Vertex<'a>,
    adjacency_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> HashMap<Vertex<'a>, Parents<'a>> {
    let mut distances: HashMap<Vertex<'a>, Parents<'a>> = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    // let mut par: HashMap<Vertex<'a>, Vec<Vertex<'a>>> = HashMap::new();

    distances.insert(
        start,
        Parents {
            parents: vec![],
            distance: 0,
        },
    );
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        if !visited.insert(vertex) {
            // Already visited this node
            continue;
        }

        if let Some(neighbors) = adjacency_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |current| new_distance < current.distance);

                if is_shorter {
                    distances.insert(
                        *neighbor,
                        Parents {
                            parents: Vec::new(),
                            distance: new_distance,
                        },
                    );
                    distances.get_mut(neighbor).map(|n| n.parents.push(vertex));
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

struct Parents<'a> {
    parents: Vec<Vertex<'a>>,
    distance: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    name: &'a str,
}

impl<'a> Vertex<'a> {
    fn new(name: &'a str) -> Vertex<'a> {
        Vertex { name }
    }
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}
