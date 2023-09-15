use cargo_scout_audit::startup::{run_scout, CargoSubCommand, Cli};
use clap::Parser;

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match cli.subcmd {
        CargoSubCommand::ScoutAudit(opts) => run_scout(opts).unwrap(),
    }
}
