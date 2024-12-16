use petgraph::graph::UnGraph;
use std::fs;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_combined_graph(folder_path: &str) -> io::Result<UnGraph<u32, ()>> {
    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut node_indices = HashMap::new();

    for entry in fs::read_dir(folder_path)? {
        let path = entry?.path();
        if path.extension().map_or(false, |ext| ext == "edges") {
            let file = fs::File::open(&path)?;
            let lines = io::BufReader::new(file).lines();

            for line in lines.flatten() {
                let nodes: Vec<u32> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect();
                if nodes.len() == 2 {
                    let u = *node_indices.entry(nodes[0]).or_insert_with(|| graph.add_node(nodes[0]));
                    let v = *node_indices.entry(nodes[1]).or_insert_with(|| graph.add_node(nodes[1]));
                    graph.add_edge(u, v, ());
                }
            }
        }
    }

    Ok(graph)
}
