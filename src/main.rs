

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "codew",
    version = "v0.0.1",
    about = "Open folder as a vscode multi-root workspace",
)]
struct Args {
    path: String,
}

fn main() {
    let arg: Args = Args::parse();
    println!("{}", arg.path);
}