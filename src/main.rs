

use clap::Parser;
use codew::context::Context;

use crate::operations::workspace;

mod operations;

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
    println!("path: {}", arg.path);

    let codew_home_dir = Context::get_codew_home_dir();
    println!("codew home: {:?}", codew_home_dir.unwrap().as_os_str());

    workspace::open_workspace();
}
