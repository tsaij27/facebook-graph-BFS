mod graph;
mod analysis;

use graph::load_combined_graph;
use analysis::GraphAnalyzer;

fn main() {
    let folder_path = "facebook"; 
    println!("Loading all .edges files from folder: '{}'", folder_path);

    let graph = load_combined_graph(folder_path).expect("Failed to load combined graph");


    println!(
        "Graph loaded successfully with {} nodes and {} edges.",
        graph.node_count(),
        graph.edge_count()
    );