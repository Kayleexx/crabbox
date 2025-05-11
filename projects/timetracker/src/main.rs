mod commands;
mod storage;
mod time_utils;

use clap::Parser;
use commands::Cli;

fn main() {
    let cli = Cli::parse();
    commands::handle_command(cli);
}

