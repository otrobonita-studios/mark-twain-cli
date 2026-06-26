mod api;
mod cli;
mod ui;

use api::ApiClient;
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;

#[tokio::main]
async fn main() {
    // Enable colorful terminal support for Windows/Unix
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let args = Cli::parse();

    // 1. Resolve API URL: CLI override -> Environment -> Default
    let base_url = args
        .url
        .or_else(|| std::env::var("MARK_TWAIN_API_URL").ok())
        .unwrap_or_else(|| "https://mark.otrobonita.com".to_string());

    // 2. Resolve API Key: CLI override -> Environment -> None
    let api_key = args
        .api_key
        .or_else(|| std::env::var("RESEARCH_API_KEY").ok());

    let api_client = ApiClient::new(base_url.clone(), api_key);

    match args.command {
        Some(Commands::Search { query, limit, exact }) => {
            let spinner = if exact {
                ui::show_spinner(&format!("Searching for exact keyword matches for '{}'...", query))
            } else {
                ui::show_spinner(&format!("Searching for similarity to '{}'...", query))
            };

            let search_result = if exact {
                api_client.keyword_search(&query, limit).await
            } else {
                api_client.search(&query, limit).await
            };

            match search_result {
                Ok(res) => {
                    spinner.finish_and_clear();
                    if exact {
                        println!("\n{} '{}' (Exact Match, Limit {}):", "Search Results for".bold(), query.cyan(), limit);
                    } else {
                        println!("\n{} '{}' (Semantic, Limit {}):", "Search Results for".bold(), query.cyan(), limit);
                    }
                    if res.results.is_empty() {
                        println!("{}", "No matching results found.".yellow());
                    } else {
                        for (idx, r) in res.results.iter().enumerate() {
                            println!(
                                "\n[Result #{}] Score: {:.4} | Source: {} (Chunk #{})",
                                idx + 1,
                                r.score.to_string().green(),
                                r.payload.filename.yellow(),
                                r.payload.chunk_index.unwrap_or(0)
                            );
                            println!("{}", r.payload.text.dimmed());
                            println!("{}", "-".repeat(50).black());
                        }
                    }
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    println!("{} {}", "API Error:".red().bold(), e);
                }
            }
        }
        Some(Commands::AnalyzeStyle { text }) => {
            ui::analyze_style_flow(&api_client, &text).await;
        }
        Some(Commands::Interactive) | None => {
            ui::run_interactive_loop(&api_client).await;
        }
    }
}
