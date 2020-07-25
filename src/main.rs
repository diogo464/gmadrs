mod commands;
use commands::*;

use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Params {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Uncompress(uncompress::Config),
    Compress(compress::Config),
    Extract(extract::Config),
    Create(create::Config),
    List(list::Config),
    Info(info::Config),
    Contents(contents::Config),
}

fn main() {
    let params = Params::parse();

    let result = match params.subcmd {
        SubCommand::Uncompress(c) => uncompress::run(c),
        SubCommand::Compress(c) => compress::run(c),
        SubCommand::Extract(c) => extract::run(c),
        SubCommand::Create(c) => create::run(c),
        SubCommand::List(c) => list::run(c),
        SubCommand::Info(c) => info::run(c),
        SubCommand::Contents(c) => contents::run(c),
    };
}
