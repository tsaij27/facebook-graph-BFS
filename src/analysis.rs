use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;
use rand::seq::SliceRandom;

pub struct GraphAnalyzer<'a> {
    graph: &'a UnGraph<u32, ()>,
}

impl<'a> GraphAnalyzer<'a> {
    pub fn new(graph: &'a UnGraph<u32, ()>) -> Self {
        Self { graph }
    }

    pub fn average_shortest_path_length(&self) -> f64 {
        let mut total_distance = 0;
        let mut path_count = 0;

        for source in self.graph.node_indices() {
            let distances = dijkstra(self.graph, source, None, |_| 1);
            for distance in distances.values() {
                total_distance += distance;
                path_count += 1;
            }
        }

        if path_count > 0 {
            total_distance as f64 / path_count as f64
        } else {
            0.0
        }
    }

    pub fn analyze_random_pairs(&self, num_pairs: usize) -> f64 {
        let nodes: Vec<_> = self.graph.node_indices().collect();
        let mut rng = rand::thread_rng();
        let mut total_distance = 0;
        let mut count = 0;

        for _ in 0..num_pairs {
            if let Some(pair) = nodes.choose_multiple(&mut rng, 2).collect::<Vec<_>>().get(0..2) {
                let source = pair[0];
                let target = pair[1];
                let distances = dijkstra(self.graph, *source, Some(*target), |_| 1);

                if let Some(distance) = distances.get(target) {
                    total_distance += distance;
                    count += 1;
                }
            }
        }

        if count > 0 {
            total_distance as f64 / count as f64
        } else {
            0.0
        }
    }
}
