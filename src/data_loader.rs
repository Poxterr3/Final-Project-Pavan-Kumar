// src/data_loader.rs
// Module: data_loader
// Purpose: Load NBA player-season data from CSV and convert it into structured PlayerSeason records with performance stats.

use csv::ReaderBuilder;

/// Represents a player's statistics for a single season.
/// Fields include name, team, season, points, assists, and rebounds per game.
#[derive(Debug, Clone)]
pub struct PlayerSeason {
    pub player_name: String,
    pub team: String,
    pub season: String,
    pub pts: f64,
    pub ast: f64,
    pub reb: f64,
}

/// Loads the CSV file and returns a vector of PlayerSeason records.
/// Filters out any rows missing name, team, or season.
pub fn load_players(path: &str) -> Vec<PlayerSeason> {
    // Initialize CSV reader with headers enabled
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .expect("Cannot open CSV file");

    let mut players = Vec::new();

    // Iterate over each row in the CSV
    for result in rdr.records() {
        if let Ok(record) = result {
            // Extract relevant fields by column index
            let player_name = record.get(1).unwrap_or("").to_string(); // player name
            let team = record.get(2).unwrap_or("").to_string();        // team abbreviation
            let season = record.get(21).unwrap_or("").to_string();     // season
            let pts = record.get(12).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0); // points per game
            let reb = record.get(13).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0); // rebounds per game
            let ast = record.get(14).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0); // assists per game

            // Only include rows with non-empty identifiers
            if !player_name.is_empty() && !team.is_empty() && !season.is_empty() {
                players.push(PlayerSeason {
                    player_name,
                    team,
                    season,
                    pts,
                    ast,
                    reb,
                });
            }
        }
    }

    players
}
