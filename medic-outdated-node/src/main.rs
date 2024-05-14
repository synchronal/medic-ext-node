use medic_outdated_node::check_outdated;
use medic_outdated_node::cli::CliArgs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CliArgs::new();
    check_outdated(cli)
}
