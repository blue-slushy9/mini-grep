use mini_grep;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = mini_grep::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(_e) = mini_grep::run(config) {
        // TODO: handle error
    }
}
