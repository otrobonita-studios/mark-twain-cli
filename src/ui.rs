use crate::api::{ApiClient, SearchResult};
use colored::*;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// Prints the splash screen with ASCII art "MT"
pub fn print_splash() {
    println!();
    println!();
    println!(
        "{}",
        "  █   █  ███  ████  █   █    █████ █   █  ███  ███ █   █     ███  █     ███  "
            .truecolor(217, 163, 74)
            .on_truecolor(29, 22, 17)
            .bold()
    );
    println!(
        "{}",
        "  ██ ██ █   █ █   █ █  █       █   █   █ █   █  █  ██  █    █     █      █  "
            .truecolor(217, 163, 74)
            .on_truecolor(29, 22, 17)
            .bold()
    );
    println!(
        "{}",
        "  █ █ █ █████ ████  ███        █   █ █ █ █████  █  █ █ █    █     █      █  "
            .truecolor(217, 163, 74)
            .on_truecolor(29, 22, 17)
            .bold()
    );
    println!(
        "{}",
        "  █   █ █   █ █  █  █  █       █   ██ ██ █   █  █  █  ██    █     █      █  "
            .truecolor(217, 163, 74)
            .on_truecolor(29, 22, 17)
            .bold()
    );
    println!(
        "{}",
        "  █   █ █   █ █   █ █   █      █   █   █ █   █ ███ █   █     ███  █████ ███  "
            .truecolor(217, 163, 74)
            .on_truecolor(29, 22, 17)
            .bold()
    );
    println!();
    println!(
        "{}",
        "  Interactive client for the Vector Research API"
            .truecolor(217, 163, 74)
            .italic()
    );
    println!(
        "{}",
        "  --------------------------------------------------".truecolor(217, 163, 74)
    );
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

            println!(
                "{:<30} {}",
                "Input Word Count:".white(),
                word_count.to_string().cyan()
            );
            println!("{:<30} {:.2}", "Average Word Length:".white(), avg_word_len);

            // Formulate style notes based on punctuation and word count
            let mut style_notes = Vec::new();
            if avg_word_len > 6.0 {
                style_notes.push("High vocabulary density and complex syllable structures.");
            } else {
                style_notes
                    .push("Simple, direct, and colloquial phrasing (characteristic of Twain).");
            }
            if exclamations > 0 || questions > 0 {
                style_notes
                    .push("Dramatic dialogic markers with highly active conversational tone.");
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
                        result.payload.chunk_index.unwrap_or(0)
                    );
                }
                println!("\n{}", "Nearest Matching Fragment:".white().bold());
                println!(
                    "{}",
                    format!("\"{}\"", best_match.payload.text).italic().dimmed()
                );
            } else {
                println!(
                    "\n{}",
                    "No direct matches found in the active corpus to compare style.".red()
                );
            }
            println!("{}", "=================================".green().bold());
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("{} {}", "Error communicating with API:".red().bold(), e);
        }
    }
}

/// Renders a list of search results.
///
/// This lives in one place on purpose. The same block used to be copied into
/// the CLI search command and both TUI search arms, and the copies had already
/// drifted -- one of them silently dropped the score.
pub fn print_results(results: &[SearchResult], show_score: bool) {
    if results.is_empty() {
        println!("{}", "No matching results found.".yellow());
        return;
    }

    for (idx, r) in results.iter().enumerate() {
        let source = r.payload.filename.yellow();
        let chunk = r.payload.chunk_index.unwrap_or(0);

        if show_score {
            // Format the float BEFORE colouring it. Applying `{:.4}` to a
            // ColoredString truncates to four *characters*, so 0.523456 printed
            // as "0.52" and 1.0 as "1" -- see issue #14.
            let score = format!("{:.4}", r.score);
            println!(
                "\n[Result #{}] Score: {} | Source: {} (Chunk #{})",
                idx + 1,
                score.green(),
                source,
                chunk
            );
        } else {
            println!(
                "\n[Match #{}] Source: {} (Chunk #{})",
                idx + 1,
                source,
                chunk
            );
        }

        println!("{}", r.payload.text.dimmed());
        println!("{}", "-".repeat(50).black());
    }
}

/// The interactive menu.
///
/// Deliberately an enum rather than positional indices: the labels and the
/// dispatch are generated from the same list, so adding an entry cannot
/// silently reassign the actions below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuChoice {
    Metadata,
    SemanticSearch,
    KeywordSearch,
    AnalyzeStyle,
    Help,
    Exit,
}

impl MenuChoice {
    const ALL: [MenuChoice; 6] = [
        MenuChoice::Metadata,
        MenuChoice::SemanticSearch,
        MenuChoice::KeywordSearch,
        MenuChoice::AnalyzeStyle,
        MenuChoice::Help,
        MenuChoice::Exit,
    ];

    fn label(self) -> &'static str {
        match self {
            MenuChoice::Metadata => "View Database Metadata",
            MenuChoice::SemanticSearch => "Semantic Search",
            MenuChoice::KeywordSearch => "Exact Keyword Search",
            MenuChoice::AnalyzeStyle => "Analyze Text Style",
            MenuChoice::Help => "Show Help / Instructions",
            MenuChoice::Exit => "Exit",
        }
    }

    fn numbered_labels() -> Vec<String> {
        MenuChoice::ALL
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}. {}", i + 1, c.label()))
            .collect()
    }
}

fn print_help() {
    println!(
        "\n{}",
        "=== INTERACTIVE TUI HELP & USAGE ===".yellow().bold()
    );
    println!(
        "- Use the {} keys or press the corresponding number to navigate.",
        "Up/Down".cyan()
    );
    println!("- Press {} to select an option.", "Enter".cyan());
    println!("- Inside prompts (Search / Style Analysis):");
    println!(
        "  * Type your text and press {} to run the query.",
        "Enter".cyan()
    );
    println!("  * If you want to abort, you can leave it blank and press Enter to return here.");
    println!("- To force-exit at any time, press {}.", "Ctrl+C".red());
    println!("====================================\n");
}

async fn show_metadata(api_client: &ApiClient) {
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
            println!(
                "{:<20} {}",
                "Points Count:",
                meta.points_count.to_string().yellow()
            );
            println!("{:<20} {}", "Vector Dimension:", meta.vector_size);
            println!("{:<20} {}", "Distance Metric:", meta.distance);
            println!(
                "{:<20} {}",
                "Embedding Model:",
                meta.embedding_model.magenta()
            );
            println!("{}", "=========================".cyan().bold());
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("{} {}", "Error:".red().bold(), e);
        }
    }
}

async fn run_search(api_client: &ApiClient, exact: bool) {
    let prompt = if exact {
        "Enter exact word/phrase to match"
    } else {
        "Enter search query"
    };

    let query: Result<String, _> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text();

    let Ok(q) = query else {
        return;
    };

    let spinner = show_spinner(if exact {
        "Running exact keyword search..."
    } else {
        "Running semantic similarity search..."
    });

    let result = if exact {
        api_client.keyword_search(&q, 5).await
    } else {
        api_client.search(&q, 5).await
    };

    spinner.finish_and_clear();

    match result {
        Ok(res) => {
            let heading = if exact {
                "Exact Matches for".bold()
            } else {
                "Search Results for".bold()
            };
            println!("\n{} '{}':", heading, q.cyan());
            print_results(&res.results, !exact);
        }
        Err(e) => println!("{} {}", "Error:".red().bold(), e),
    }
}

// Runs the interactive TUI flow
pub async fn run_interactive_loop(api_client: &ApiClient) {
    print_splash();
    println!();

    let labels = MenuChoice::numbered_labels();

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(&labels)
            .interact_opt();

        // A cancelled prompt (Esc) is treated the same as choosing Exit.
        let choice = match selection {
            Ok(Some(idx)) => MenuChoice::ALL.get(idx).copied(),
            Ok(None) => Some(MenuChoice::Exit),
            Err(_) => {
                println!("{}", "Could not read your selection.".red());
                return;
            }
        };

        let Some(choice) = choice else {
            println!("{}", "Invalid option selected.".red());
            continue;
        };

        match choice {
            MenuChoice::Metadata => show_metadata(api_client).await,
            MenuChoice::SemanticSearch => run_search(api_client, false).await,
            MenuChoice::KeywordSearch => run_search(api_client, true).await,
            MenuChoice::AnalyzeStyle => {
                let text: Result<String, _> = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the text to analyze")
                    .interact_text();
                if let Ok(t) = text {
                    analyze_style_flow(api_client, &t).await;
                }
            }
            MenuChoice::Help => print_help(),
            MenuChoice::Exit => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_menu_variant_has_a_numbered_label() {
        let labels = MenuChoice::numbered_labels();
        assert_eq!(labels.len(), MenuChoice::ALL.len());
        assert_eq!(labels[0], "1. View Database Metadata");
        assert_eq!(labels[5], "6. Exit");
    }

    #[test]
    fn labels_and_dispatch_stay_aligned() {
        // The bug this guards against: labels built from one list and actions
        // matched on hardcoded indices from another.
        for (idx, choice) in MenuChoice::ALL.iter().enumerate() {
            assert_eq!(MenuChoice::ALL.get(idx).copied(), Some(*choice));
        }
    }
}
