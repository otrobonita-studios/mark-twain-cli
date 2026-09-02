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
        Some(Commands::Search {
            query,
            limit,
            exact,
        }) => {
            let spinner = if exact {
                ui::show_spinner(&format!(
                    "Searching for exact keyword matches for '{}'...",
                    query
                ))
            } else {
                ui::show_spinner(&format!("Searching for similarity to '{}'...", query))
            };

            let search_result = if exact {
                api_client.keyword_search(&query, limit).await
            } else {
                api_client.search(&query, limit).await
            };

            spinner.finish_and_clear();

            match search_result {
                Ok(res) => {
                    let mode = if exact { "Exact Match" } else { "Semantic" };
                    println!(
                        "\n{} '{}' ({}, Limit {}):",
                        "Search Results for".bold(),
                        query.cyan(),
                        mode,
                        limit
                    );
                    ui::print_results(&res.results, !exact);
                }
                Err(e) => {
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
