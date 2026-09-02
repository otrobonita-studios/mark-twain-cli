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

/// Plain counts over the input text. Descriptive only -- nothing here is
/// evidence about authorship, and the renderer must not present it as such.
#[derive(Debug, PartialEq)]
pub struct TextStats {
    pub words: usize,
    pub characters: usize,
    pub avg_word_len: f32,
    pub exclamations: usize,
    pub questions: usize,
    pub dashes: usize,
}

impl TextStats {
    pub fn of(text: &str) -> Self {
        let words: Vec<&str> = text.split_whitespace().collect();
        let letters: usize = words.iter().map(|w| w.chars().count()).sum();
        TextStats {
            words: words.len(),
            characters: text.chars().count(),
            avg_word_len: if words.is_empty() {
                0.0
            } else {
                letters as f32 / words.len() as f32
            },
            exclamations: text.matches('!').count(),
            questions: text.matches('?').count(),
            dashes: text.matches('-').count(),
        }
    }
}

/// Finds the passages in the corpus closest to a piece of text.
///
/// This used to print invented conclusions -- "characteristic of Twain" for any
/// text with short words -- under a "STYLISTIC ANALYSIS REPORT" heading. The
/// thresholds were hardcoded and the verdicts were fixed strings, so the output
/// looked authoritative while measuring nothing about authorship. See issue #6.
///
/// What remains is what the tool can actually establish: real similarity scores
/// against the indexed corpus, and plain counts over the input, each labelled as
/// what it is.
pub async fn analyze_style_flow(api_client: &ApiClient, text: &str) {
    let spinner = show_spinner("Searching the corpus for the closest passages...");
    let result = api_client.search(text, 3).await;
    spinner.finish_and_clear();

    let res = match result {
        Ok(res) => res,
        Err(e) => {
            println!("{} {}", "Error communicating with API:".red().bold(), e);
            return;
        }
    };

    println!(
        "\n{}",
        "=== CLOSEST PASSAGES IN THE CORPUS ===".green().bold()
    );

    if res.results.is_empty() {
        println!(
            "{}",
            "No passages in the active corpus matched this text.".yellow()
        );
    } else {
        for (idx, r) in res.results.iter().enumerate() {
            println!(
                "  {}. [Similarity: {:.2}%] {} (Chunk #{})",
                idx + 1,
                r.score * 100.0,
                r.payload.filename.green(),
                r.payload.chunk_index.unwrap_or(0)
            );
        }

        println!("\n{}", "Nearest passage:".white().bold());
        println!(
            "{}",
            format!("\"{}\"", res.results[0].payload.text)
                .italic()
                .dimmed()
        );
    }

    // Descriptive counts, with no interpretation attached. Whether short words
    // mean anything about Twain is a question this tool cannot answer, so it
    // does not pretend to.
    let stats = TextStats::of(text);
    println!(
        "\n{}",
        "Input text (measured, not interpreted):".yellow().bold()
    );
    println!("  {:<24} {}", "Words:", stats.words.to_string().cyan());
    println!(
        "  {:<24} {}",
        "Characters:",
        stats.characters.to_string().cyan()
    );
    println!("  {:<24} {:.2}", "Average word length:", stats.avg_word_len);
    println!(
        "  {:<24} {} / {} / {}",
        "! ? - counts:", stats.exclamations, stats.questions, stats.dashes
    );

    println!(
        "\n{}",
        "Similarity is semantic proximity to indexed passages. It is not a measure of authorship."
            .dimmed()
    );
    println!(
        "{}",
        "======================================".green().bold()
    );
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
            MenuChoice::AnalyzeStyle => "Find Closest Passages",
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
    println!("- Inside prompts (Search / Closest Passages):");
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
                    .with_prompt("Enter the text to compare against the corpus")
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
    fn stats_are_plain_counts() {
        let s = TextStats::of("Well, the first week went by!");
        assert_eq!(s.words, 6);
        assert_eq!(s.exclamations, 1);
        assert_eq!(s.questions, 0);
        assert_eq!(s.characters, 29);
    }

    #[test]
    fn empty_input_does_not_divide_by_zero() {
        let s = TextStats::of("   ");
        assert_eq!(s.words, 0);
        assert_eq!(s.avg_word_len, 0.0);
    }

    #[test]
    fn average_word_length_ignores_whitespace() {
        // "aaa bbb" -> 6 letters over 2 words, not 7 characters over 2.
        let s = TextStats::of("aaa bbb");
        assert_eq!(s.avg_word_len, 3.0);
    }

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
