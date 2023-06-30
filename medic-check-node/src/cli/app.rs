use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-node")]
/// Checks for ensuring that NodeJS/NPM dependencies are
/// properly installed.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks whether a corepack shim (such as pnpm) has been installed.
    CorepackShimInstalled(CorepackShimArgs),
    /// Checks that NPM exists in the PATH.
    NpmExists,
    /// Checks that all NPM dependencies are installed.
    PackagesInstalled(PackageArgs),
}

#[derive(Args, Debug)]
pub struct PackageArgs {
    /// Path to a node project
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub cd: Option<String>,

    /// Npm prefix
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::DirPath)]
    pub prefix: Option<String>,
}

#[derive(Args, Debug)]
pub struct CorepackShimArgs {
    /// Name of a shim
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub name: String,

    /// Version of the shim
    #[clap(value_parser)]
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub version: String,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}
