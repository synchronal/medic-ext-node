use medic_check_node::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::NpmExists => medic_check_node::npm_exists()?,
        Command::PackagesInstalled(args) => {
            medic_check_node::packages_installed(args.cd, args.prefix)?
        }
    }
    CheckOk
}
