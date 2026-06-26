use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "mark-twain-cli")]
#[command(author = "Otrobonita Studios")]
#[command(version = "0.1.0")]
#[command(about = "CLI client for the Mark Twain Vector Database API", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true, help = "Base URL of the Mark Twain API (overrides MARK_TWAIN_API_URL env)")]
    pub url: Option<String>,

    #[arg(short = 'k', long, global = true, help = "Authorization token (overrides RESEARCH_API_KEY env)")]
    pub api_key: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Perform a semantic search in the vector database")]
    Search {
        #[arg(short, long, help = "The search query/phrase")]
        query: String,

        #[arg(short, long, default_value_t = 5, help = "Max number of results to return")]
        limit: usize,

        #[arg(short, long, help = "Use exact keyword matching (full-text search) instead of semantic search")]
        exact: bool,
    },

    #[command(name = "analyze-style", about = "Analyze the style of a text snippet against Mark Twain's profile")]
    AnalyzeStyle {
        #[arg(short, long, help = "The text to analyze")]
        text: String,
    },

    #[command(about = "Start the interactive menu mode (default behavior when no command is provided)")]
    Interactive,
}
