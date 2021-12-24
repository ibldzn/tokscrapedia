use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct App {
    /// Query to search
    #[clap(short, long)]
    pub query: String,

    /// Minimum price to filter the results
    #[clap(short = 'm', long)]
    pub min_price: Option<usize>,

    /// Maximum price to filter the results
    #[clap(short = 'M', long)]
    pub max_price: Option<usize>,

    /// Number of results to return
    #[clap(short = 'r', long, default_value_t = 200)]
    pub max_results: usize,

    /// CSV output file
    #[clap(short, long, default_value = "out.csv")]
    pub output: String,
}

impl App {
    pub fn parse_from_args() -> Self {
        Self::parse()
    }
}
