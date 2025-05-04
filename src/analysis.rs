// src/analysis.rs
// Module: analysis
// Purpose: Analyze structural properties of the player graph: degree distribution, centrality, path lengths, and similarity.

use petgraph::algo::dijkstra;
use std::collections::{HashMap, HashSet};
use crate::graph_builder::PlayerGraph;
use rand::prelude::*; // Brings .choose() method into scope

/// Returns a histogram of node degrees in the graph
pub fn analyze_degrees(graph: &PlayerGraph) -> HashMap<usize, usize> {
    let mut degree_count: HashMap<usize, usize> = HashMap::new();
    // Count number of edges (degree) per node
    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        *degree_count.entry(degree).or_insert(0) += 1;
    }
    degree_count
}

/// Computes closeness centrality for each player
/// Input: Graph reference
/// Output: HashMap of player name to centrality score
pub fn compute_centrality(graph: &PlayerGraph) -> HashMap<String, f64> {
    let mut scores: HashMap<String, f64> = HashMap::new();
    for node in graph.node_indices() {
        // Dijkstra computes shortest paths from `node` to all others
        let result = dijkstra(graph, node, None, |_| 1);
        let total_distance: usize = result.values().sum();
        // Closeness = (n-1) / sum of shortest distances
        let closeness = if total_distance > 0 {
            (result.len() - 1) as f64 / total_distance as f64
        } else {
            0.0
        };
        scores.insert(graph[node].clone(), closeness);
    }
    scores
}

/// Randomly samples 100 player pairs and computes average shortest path
pub fn compute_shortest_paths(graph: &PlayerGraph) {
    let nodes: Vec<_> = graph.node_indices().collect();
    let mut rng = rand::thread_rng();
    let mut sampled_pairs = Vec::new();

    // Sample 100 unique random (a, b) pairs
    while sampled_pairs.len() < 100 {
        let a = *nodes.choose(&mut rng).unwrap();
        let b = *nodes.choose(&mut rng).unwrap();
        if a != b {
            sampled_pairs.push((a, b));
        }
    }

    let mut total_length = 0;
    let mut count = 0;

    for (a, b) in sampled_pairs {
        // Use A* to find shortest path between nodes a and b
        if let Some(path) = petgraph::algo::astar(
            graph,
            a,
            |finish| finish == b,
            |_| 1,
            |_| 0,
        ) {
            total_length += path.0; // path.0 = total path cost (length)
            count += 1;
        }
    }

    // Print average if at least one valid path found
    if count > 0 {
        let avg_length = total_length as f64 / count as f64;
        println!("Average shortest path over 100 sampled pairs: {:.4}", avg_length);
    } else {
        println!("No valid paths found in sample.");
    }
}

/// Finds the most structurally similar player pair using Jaccard similarity
pub fn analyze_similarity(graph: &PlayerGraph) {
    let mut max_sim = 0.0;
    let mut most_similar = ("", "");

    let nodes: Vec<_> = graph.node_indices().collect();
    for i in 0..nodes.len() {
        for j in i+1..nodes.len() {
            let u = nodes[i];
            let v = nodes[j];
            let u_neighbors: HashSet<_> = graph.neighbors(u).collect();
            let v_neighbors: HashSet<_> = graph.neighbors(v).collect();

            // Compute Jaccard similarity = |A ∩ B| / |A ∪ B|
            let intersection = u_neighbors.intersection(&v_neighbors).count();
            let union = u_neighbors.union(&v_neighbors).count();

            let jaccard = if union > 0 {
                intersection as f64 / union as f64
            } else {
                0.0
            };

            // Track most similar pair
            if jaccard > max_sim {
                max_sim = jaccard;
                most_similar = (&graph[u], &graph[v]);
            }
        }
    }

    println!(
        "Most similar players: {} and {} (Jaccard similarity = {:.4})",
        most_similar.0, most_similar.1, max_sim
    );
}
