// src/visualizations.rs
// Module: visualizations
// Purpose: Generate visual representations of degree distribution and centrality scores as PNGs using the `plotters` crate.

use plotters::prelude::*;
use std::collections::HashMap;

/// Plots a binned degree distribution histogram from degree_counts.
/// Saves the result to the specified output_path.
pub fn plot_degree_distribution(degree_counts: &HashMap<usize, usize>, output_path: &str) {
    // Bin degrees into ranges of 10
    let mut binned: HashMap<usize, usize> = HashMap::new();
    for (&deg, &count) in degree_counts.iter() {
        let bin = (deg / 10) * 10;
        *binned.entry(bin).or_insert(0) += count;
    }

    let max_bin = *binned.keys().max().unwrap_or(&0);
    let max_count = *binned.values().max().unwrap_or(&0);

    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE).expect("Failed to fill background");

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Distribution (Binned)", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..max_bin + 10, 0..max_count + 5)
        .expect("Failed to build chart");

    chart.configure_mesh().draw().expect("Failed to draw mesh");

    // Draw bar rectangles for each bin
    chart.draw_series(
        binned.iter().map(|(&bin, &count)| {
            Rectangle::new(
                [(bin, 0), (bin + 9, count)],
                BLUE.filled(),
            )
        })
    ).expect("Failed to draw degree bars");

    root.present().expect("Failed to write degree_distribution.png");
}

/// Plots the degree distribution on a log-log scale.
/// Useful for checking power-law behavior.
pub fn plot_degree_loglog(degree_counts: &HashMap<usize, usize>, output_path: &str) {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Convert to log-scale data points
    let data: Vec<(f64, f64)> = degree_counts
        .iter()
        .filter(|&(&d, &c)| d > 0 && c > 0)
        .map(|(&d, &c)| ((d as f64).log10(), (c as f64).log10()))
        .collect();

    if data.is_empty() {
        println!("Warning: Log-log degree distribution data is empty, no plot generated.");
        return;
    }

    // Compute x and y axis ranges
    let x_min = data.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let x_max = data.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let y_min = data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let y_max = data.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let x_range = x_min..x_max;
    let y_range = y_min..y_max;

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Distribution (Log-Log Scale)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(data.iter().map(|(x, y)| Circle::new((*x, *y), 3, RED.filled())))
        .unwrap();
}

/// Plots top 20 players by closeness centrality.
/// Labels are rotated for readability and scaled to percentages.
pub fn plot_centrality_scores(centrality_scores: &HashMap<String, f64>, output_path: &str) {
    let mut scores: Vec<_> = centrality_scores.iter().collect();
    scores.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let top_scores = &scores[..scores.len().min(20)];

    let categories: Vec<String> = top_scores.iter().map(|(name, _)| (*name).clone()).collect();
    let cat_range = 0..categories.len();

    let max_score = top_scores
        .iter()
        .map(|(_, score)| **score * 100.0)
        .fold(f64::MIN, f64::max);

    let upper_bound = if max_score > 0.0 {
        (max_score * 1.1).ceil()
    } else {
        1.0
    };

    let root = BitMapBackend::new(output_path, (1200, 600)).into_drawing_area();
    root.fill(&WHITE).expect("Failed to fill background");

    let mut chart = ChartBuilder::on(&root)
        .caption("Top Player Centrality Scores (%)", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(120)
        .y_label_area_size(60)
        .build_cartesian_2d(cat_range.clone(), 0.0..upper_bound)
        .expect("Failed to build centrality chart");

    chart
        .configure_mesh()
        .x_labels(categories.len())
        .x_label_formatter(&|i| categories.get(*i).unwrap_or(&"".to_string()).to_string())
        .label_style(("sans-serif", 14))
        .x_label_style(("sans-serif", 13).into_font().transform(FontTransform::Rotate90))
        .draw()
        .expect("Failed to draw mesh");

    // Draw centrality bars
    chart
        .draw_series(
            top_scores.iter().enumerate().map(|(i, (_, score))| {
                let score_pct = *score * 100.0;
                Rectangle::new([(i, 0.0), (i, score_pct)], GREEN.filled())
            })
        )
        .expect("Failed to draw centrality bars");

    // Annotate with score values above each bar
    chart
        .draw_series(
            top_scores.iter().enumerate().map(|(i, (_, score))| {
                let score_pct = *score * 100.0;
                Text::new(
                    format!("{:.1}", score_pct),
                    (i, score_pct + 0.5),
                    ("sans-serif", 12).into_font().color(&BLACK),
                )
            })
        )
        .expect("Failed to draw value labels");

    root.present().expect("Failed to write centrality_scores.png");
}
