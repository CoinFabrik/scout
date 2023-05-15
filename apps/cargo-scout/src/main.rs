use clap::Parser;

mod args;

fn main() {
    let args = args::Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
