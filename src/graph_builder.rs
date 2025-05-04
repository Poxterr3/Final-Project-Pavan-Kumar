// src/graph_builder.rs
// Module: graph_builder
// Purpose: Constructs the player network graph where nodes represent players and edges represent shared team-season membership.

use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use crate::data_loader::PlayerSeason;

/// Type alias for the graph used across the project
/// Each node is a player (String), and edges count number of shared team-season stints
pub type PlayerGraph = Graph<String, usize, Undirected>;

/// Builds a player graph based on shared team and season
/// Input: slice of PlayerSeason structs
/// Output: PlayerGraph
pub fn build_player_graph(players: &[PlayerSeason]) -> PlayerGraph {
    let mut graph = PlayerGraph::new_undirected();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();
    let mut teammates_map: HashMap<(String, String), usize> = HashMap::new();

    // Group players by (team, season) to find co-teammates
    let mut team_season_map: HashMap<(String, String), Vec<String>> = HashMap::new();
    for ps in players {
        team_season_map
            .entry((ps.team.clone(), ps.season.clone()))
            .or_default()
            .push(ps.player_name.clone());
    }

    // For each (team, season) group, connect each pair of players
    for (_team_season, player_list) in team_season_map {
        for i in 0..player_list.len() {
            let p1 = &player_list[i];
            // Add node to graph if not already present
            let idx1 = *node_indices.entry(p1.clone()).or_insert_with(|| graph.add_node(p1.clone()));

            for j in i + 1..player_list.len() {
                let p2 = &player_list[j];
                let idx2 = *node_indices.entry(p2.clone()).or_insert_with(|| graph.add_node(p2.clone()));

                // Ensure (p1, p2) order is canonical to avoid duplicate entries
                let key = if p1 < p2 { (p1.clone(), p2.clone()) } else { (p2.clone(), p1.clone()) };
                let count = teammates_map.entry(key.clone()).or_insert(0);
                *count += 1;

                // Add edge or increment existing edge weight
                if graph.find_edge(idx1, idx2).is_none() {
                    graph.add_edge(idx1, idx2, 1);
                } else {
                    let edge = graph.find_edge(idx1, idx2).unwrap();
                    let edge_weight = graph.edge_weight_mut(edge).unwrap();
                    *edge_weight += 1;
                }
            }
        }
    }

    graph
}
