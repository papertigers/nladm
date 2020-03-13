mod brightness;
mod cli;
mod config;
mod effects;
mod state;
mod user;

fn main() {
    let _ = cli::execute().map_err(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
