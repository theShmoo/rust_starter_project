mod app;
mod lib;

fn main() {

    let args = lib::cli::parse();
    let config = lib::config::parse(args.path);
    println!("config parsed: {}", config.test);
    app::hello();
}
