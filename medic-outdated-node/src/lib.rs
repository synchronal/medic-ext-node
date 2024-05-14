#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;
mod outdated;

use cli::CliArgs;
use medic_lib::std_to_string;
use outdated::OutdatedInfo;
use std::error::Error;
use std::process::Command;

pub fn check_outdated(_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("npm");
    command.args(["outdated", "--json"]);

    match command.output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let outdated: OutdatedInfo = serde_json::from_str(&stdout)?;
            for (name, versions) in &outdated.dependencies {
                println!(
                    "::outdated::name={name}::version={}::latest={}",
                    versions.current, versions.latest
                )
            }

            if outdated.any() {
                println!("::remedy::npm update");
            }
        }
        Err(_) => return Err("::failure::Unable to get outdated".into()),
    }

    Ok(())
}
