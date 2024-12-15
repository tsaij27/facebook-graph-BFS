use graph_analysis::graph::GraphLoader;
use graph_analysis::analysis::GraphAnalyzer;

#[test]
fn test_graph_loading() {
    let graph = GraphLoader::load_graph("test_data/test.edges").expect("Failed to load graph");
    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 3);
}

#[test]
fn test_average_shortest_path() {
    let graph = GraphLoader::load_graph("test_data/test.edges").expect("Failed to load graph");
    let analyzer = GraphAnalyzer::new(&graph);
    let avg_length = analyzer.average_shortest_path_length();
    assert!(avg_length > 0.0);
}
