//! Xtask automation commands for Nodoka project

mod screenshot;

use clap::{FromArgMatches, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// Generate comprehensive UI screenshots for documentation
    Screenshot(screenshot::Args),
}

fn main() -> anyhow::Result<()> {
    let command = clap::Command::new("xtask")
        .about("Nodoka project automation tasks")
        .subcommand_required(true)
        .subcommand(Commands::augment_subcommands(clap::Command::new("xtask")));

    let matches = command.get_matches();
    let (cmd_name, sub_matches) = matches.subcommand().expect("subcommand required");

    match cmd_name {
        "screenshot" => {
            let args = screenshot::Args::from_arg_matches(sub_matches)?;
            screenshot::run(args)?;
        }
        _ => unreachable!("exhausted subcommands"),
    }

    Ok(())
}
