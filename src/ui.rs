use crate::api::ApiClient;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// Prints the splash screen with ASCII art "MT"
pub fn print_splash() {
    println!("{}", "█   █  ███  ████  █   █    █████ █   █  ███  ███ █   █     ███  █     ███".cyan().bold());
    println!("{}", "██ ██ █   █ █   █ █  █       █   █   █ █   █  █  ██  █    █     █      █".cyan().bold());
    println!("{}", "█ █ █ █████ ████  ███        █   █ █ █ █████  █  █ █ █    █     █      █".cyan().bold());
    println!("{}", "█   █ █   █ █  █  █  █       █   ██ ██ █   █  █  █  ██    █     █      █".cyan().bold());
    println!("{}", "█   █ █   █ █   █ █   █      █   █   █ █   █ ███ █   █     ███  █████ ███".cyan().bold());
    println!();
    println!("{}", "Interactive client for the Vector Research API".italic().white());
    println!("--------------------------------------------------");
}

// Runs a spinner for a future or task
pub fn show_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .expect("Invalid progress template"),
    );
    pb
}

// Helper to run a mocked/interactive style analysis using the search API to find stylistic matches
pub async fn analyze_style_flow(api_client: &ApiClient, text: &str) {
    let spinner = show_spinner("Analyzing stylistic fingerprint against Mark Twain's profile...");
    
    // Find the closest semantic/stylistic matches
    match api_client.search(text, 3).await {
        Ok(res) => {
            spinner.finish_and_clear();
            println!("\n{}", "=== STYLISTIC ANALYSIS REPORT ===".green().bold());
            
            // Calculate simple stylistic markers
            let words: Vec<&str> = text.split_whitespace().collect();
            let word_count = words.len();
            let avg_word_len = if word_count > 0 {
                words.iter().map(|w| w.len()).sum::<usize>() as f32 / word_count as f32
            } else {
                0.0
            };
            
            let exclamations = text.matches('!').count();
            let questions = text.matches('?').count();
            let hyphens = text.matches('-').count();

            println!("{:<30} {}", "Input Word Count:".white(), word_count.to_string().cyan());
            println!("{:<30} {:.2}", "Average Word Length:".white(), avg_word_len);
            
            // Formulate style notes based on punctuation and word count
            let mut style_notes = Vec::new();
            if avg_word_len > 6.0 {
                style_notes.push("High vocabulary density and complex syllable structures.");
            } else {
                style_notes.push("Simple, direct, and colloquial phrasing (characteristic of Twain).");
            }
            if exclamations > 0 || questions > 0 {
                style_notes.push("Dramatic dialogic markers with highly active conversational tone.");
            }
            if hyphens > 1 {
                style_notes.push("Frequent compounding and structural pauses.");
            }

            println!("\n{}", "Linguistic Fingerprints:".yellow().bold());
            for note in style_notes {
                println!("  * {}", note);
            }

            if !res.results.is_empty() {
                let best_match = &res.results[0];
                println!("\n{}", "Top Stylistic Matches in Corpus:".yellow().bold());
                for (idx, result) in res.results.iter().enumerate() {
                    println!(
                        "  {}. [Similarity: {:.2}%] - Source: {} (Chunk #{})",
                        idx + 1,
                        result.score * 100.0,
                        result.payload.filename.green(),
                        result.payload.chunk_index
                    );
                }
                println!("\n{}", "Nearest Matching Fragment:".white().bold());
                println!("{}", format!("\"{}\"", best_match.payload.text).italic().dimmed());
            } else {
                println!("\n{}", "No direct matches found in the active corpus to compare style.".red());
            }
            println!("{}", "=================================".green().bold());
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("{} {}", "Error communicating with API:".red().bold(), e);
        }
    }
}

// Runs the interactive TUI flow
pub async fn run_interactive_loop(api_client: &ApiClient) {
    print_splash();
    println!();

    let choices = &[
        "1. View Database Metadata",
        "2. Semantic Search",
        "3. Analyze Text Style",
        "4. Exit",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(&choices[..])
            .interact_opt();

        match selection {
            Ok(Some(0)) => {
                let spinner = show_spinner("Fetching database metadata...");
                match api_client.get_metadata().await {
                    Ok(meta) => {
                        spinner.finish_and_clear();
                        println!("\n{}", "=== DATABASE METADATA ===".cyan().bold());
                        println!("{:<20} {}", "Collection Name:", meta.collection.green());
                        println!("{:<20} {}", "Status:", meta.status.green());
                        if let Some(vc) = meta.vectors_count {
                            println!("{:<20} {}", "Vectors Count:", vc.to_string().yellow());
                        }
                        println!("{:<20} {}", "Points Count:", meta.points_count.to_string().yellow());
                        println!("{:<20} {}", "Vector Dimension:", meta.vector_size);
                        println!("{:<20} {}", "Distance Metric:", meta.distance);
                        println!("{:<20} {}", "Embedding Model:", meta.embedding_model.magenta());
                        println!("{}", "=========================".cyan().bold());
                    }
                    Err(e) => {
                        spinner.finish_and_clear();
                        println!("{} {}", "Error:".red().bold(), e);
                    }
                }
                println!();
            }
            Ok(Some(1)) => {
                let query: Result<String, _> = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter search query")
                    .interact_text();

                if let Ok(q) = query {
                    let spinner = show_spinner("Running semantic similarity search...");
                    match api_client.search(&q, 5).await {
                        Ok(res) => {
                            spinner.finish_and_clear();
                            println!("\n{} '{}':", "Search Results for".bold(), q.cyan());
                            if res.results.is_empty() {
                                println!("{}", "No matching results found.".yellow());
                            } else {
                                for (idx, r) in res.results.iter().enumerate() {
                                    println!(
                                        "\n[Result #{}] Score: {:.4} | Source: {} (Chunk #{})",
                                        idx + 1,
                                        r.score.to_string().green(),
                                        r.payload.filename.yellow(),
                                        r.payload.chunk_index
                                    );
                                    println!("{}", r.payload.text.dimmed());
                                    println!("{}", "-".repeat(50).black());
                                }
                            }
                        }
                        Err(e) => {
                            spinner.finish_and_clear();
                            println!("{} {}", "Error:".red().bold(), e);
                        }
                    }
                }
                println!();
            }
            Ok(Some(2)) => {
                let text: Result<String, _> = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the text to analyze")
                    .interact_text();

                if let Ok(t) = text {
                    analyze_style_flow(api_client, &t).await;
                }
                println!();
            }
            Ok(Some(3)) | Ok(None) => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
            _ => {
                println!("{}", "Invalid option selected.".red());
            }
        }
    }
}
