use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
//     // let (query, filename) = parse_config(&args);
//
//     // --snip--
// }
//
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//
//     (query, filename)
// }
