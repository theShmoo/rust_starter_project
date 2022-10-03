mod app;
mod lib;
mod ui;

fn main() {

    let args = lib::cli::parse();
    let config = lib::config::parse(args.path);
    app::hello();
    ui::stopwatch::run();
}
