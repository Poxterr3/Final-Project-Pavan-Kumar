// src/intro_view.rs
// Module: intro_view
// Purpose: Print high-level dataset summaries and final analytical results to the console.

use crate::data_loader::PlayerSeason;
use std::collections::HashSet;

/// Prints unique player/team/season counts and sample records
pub fn show_intro(players: &[PlayerSeason]) {
    // Count unique players, teams, and seasons
    let unique_players: HashSet<_> = players.iter().map(|p| &p.player_name).collect();
    let unique_teams: HashSet<_> = players.iter().map(|p| &p.team).collect();
    let unique_seasons: HashSet<_> = players.iter().map(|p| &p.season).collect();

    println!("\n===== NBA Dataset Overview =====");
    println!("Total player-season records: {}", players.len());
    println!("Unique players: {}", unique_players.len());
    println!("Unique teams: {}", unique_teams.len());
    println!("Seasons covered: {}", unique_seasons.len());
    println!("Sample players:");

    // Print first 5 player-season records
    for p in players.iter().take(5) {
        println!("  {} | {} | {}", p.player_name, p.team, p.season);
    }
    println!("================================\n");
}

/// Prints a formatted summary of the network analysis results
/// Includes graph metrics and top central players
pub fn print_summary(
    avg_dist: f64,                  // average shortest path
    diameter: usize,               // network diameter
    degrees: &[usize],             // degree sample for validation
    two_hop: &[usize],             // synthetic 2-hop sample
    densest_nodes: &[usize],       // densest subgraph node set
    densest_density: f64,          // average internal connectivity of dense group
    top_centrality: &[(String, f64)], // top centrality players
    communities: &[(usize, usize)],   // toy community memberships
) {
    println!("===== NBA Network Analysis Summary =====");
    println!("Average shortest-path length: {:.3}", avg_dist);
    println!("Network diameter: {}", diameter);
    println!("Degree: sample {} nodes", degrees.len());
    println!("2-hop neighbors: sample {} nodes", two_hop.len());
    println!("Densest subgraph size: {} nodes", densest_nodes.len());
    println!("Densest subgraph density: {:.3}", densest_density);

    println!("Top centrality players:");
    for (name, score) in top_centrality.iter().take(10) {
        println!("  {}: {:.3}", name, score);
    }

    println!("Community assignments (node_index -> community_id):");
    for (node, comm) in communities.iter().take(10) {
        println!("  {} -> {}", node, comm);
    }

    println!("Check the `output/` directory for generated PNGs:");
    println!("  - degree_histogram.png");
    println!("  - degree_loglog.png");
    println!("  - top_centrality.png");
    println!("========================================");
}