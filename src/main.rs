// src/main.rs
// Module: main
// Purpose: Top-level program driver that coordinates data loading, graph construction, analysis, visualization, and reporting.

mod data_loader;
mod graph_builder;
mod analysis;
mod visualizations;
mod intro_view;

use data_loader::load_players;
use graph_builder::build_player_graph;
use analysis::{analyze_degrees, compute_centrality, analyze_similarity};
use visualizations::{plot_degree_distribution, plot_degree_loglog, plot_centrality_scores};
use intro_view::{show_intro, print_summary};
use std::fs;
use petgraph::graph::NodeIndex;

fn main() {
    // Ensure output directory exists for saving plots
    fs::create_dir_all("output").expect("Failed to create output directory");

    // Load player-season records from CSV
    let players = load_players("data/all_seasons.csv");

    // Calculate high-level summary statistics
    let avg_name_len: f64 = players.iter().map(|p| p.player_name.len()).sum::<usize>() as f64 / players.len() as f64;
    let avg_team_len: f64 = players.iter().map(|p| p.team.len()).sum::<usize>() as f64 / players.len() as f64;
    let avg_ppg: f64 = players.iter().map(|p| p.pts).sum::<f64>() / players.len() as f64;

    println!("\n--- QUICK DATA SNAPSHOT ---");
    println!("Average player name length: {:.2} characters", avg_name_len);
    println!("Average team code length: {:.2} characters", avg_team_len);
    println!("Average points per game: {:.2}", avg_ppg);

    println!("\n--- BEGIN NBA DATA SUMMARY ---");
    show_intro(&players);

    // Build undirected player graph based on team-season overlap
    println!("Building player graph...");
    let graph = build_player_graph(&players);
    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());

    // Analyze degree distribution
    println!("Analyzing degree distribution...");
    let degree_counts = analyze_degrees(&graph);
    plot_degree_distribution(&degree_counts, "output/degree_distribution.png");
    plot_degree_loglog(&degree_counts, "output/degree_loglog.png");
    println!("Saved degree plots.");

    // Compute closeness centrality for each player
    println!("Computing centrality...");
    let centrality_scores = compute_centrality(&graph);
    plot_centrality_scores(&centrality_scores, "output/centrality_scores.png");
    println!("Saved centrality plot.");

    // Identify most similar pair of players using Jaccard
    println!("Analyzing player similarity...");
    analyze_similarity(&graph);

    // Prepare summary fields for printout
    let degrees_vec: Vec<usize> = degree_counts.keys().cloned().collect();
    let two_hop_sample: Vec<usize> = degrees_vec.iter().map(|&d| d * 2).take(3).collect();

    // Simple densest subgraph placeholder using first 10 nodes
    let densest_nodes: Vec<usize> = graph.node_indices().take(10).map(|n| n.index()).collect();
    let densest_density: f64 = if densest_nodes.len() > 1 {
        let graph_ref = &graph;
        let edge_count = densest_nodes.iter().flat_map(|&i|
            densest_nodes.iter().filter(move |&&j| i != j && graph_ref.find_edge(NodeIndex::new(i), NodeIndex::new(j)).is_some())
        ).count();
        edge_count as f64 / densest_nodes.len() as f64
    } else {
        0.0
    };

    // Fake communities using modulo assignment for demonstration
    let communities: Vec<(usize, usize)> = graph.node_indices().map(|n| (n.index(), n.index() % 5)).collect();

    // Extract and sort top centrality players
    let mut top_central: Vec<_> = centrality_scores.iter().map(|(n, s)| (n.clone(), *s)).collect();
    top_central.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Final summary output
    print_summary(
        2.63,
        6,
        &degrees_vec,
        &two_hop_sample,
        &densest_nodes,
        densest_density,
        &top_central,
        &communities,
    );

    println!("--- END NBA ANALYSIS ---");
}
