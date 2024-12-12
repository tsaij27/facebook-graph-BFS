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

    println!("Performing graph analysis...");
    let analyzer = GraphAnalyzer::new(&graph);

    let avg_path_length = analyzer.average_shortest_path_length();
    println!("Average shortest path length: {:.2}", avg_path_length);

    let typical_distance = analyzer.analyze_random_pairs(100);
    println!("Typical distance between 100 random pairs of users: {:.2}", typical_distance);

    println!("Graph analysis complete!");
}
