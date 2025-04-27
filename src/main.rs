use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    google: bool,
    #[arg(short, long)]
    perplexity: bool,
    query: Vec<String>,
}

fn search(search_url: String) {
    match Command::new("chromium-browser").arg(search_url).output() {
        Ok(_) => println!("Search successful"),
        Err(e) => println!("Search encountered error: {}", e),
    }
}

fn main() {
    let args = Args::parse();
    if args.google {
        let search_url = format!("https://www.google.com/search?q={}", args.query.join(" "));
        search(search_url);
    } else if args.perplexity {
        search(format!("https://www.perplexity.ai/search?q={}", args.query.join(" ")));
    }
}

