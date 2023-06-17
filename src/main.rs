

use std::{process::Command, path::Path};

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
    // let arg: Args = Args::parse();
    // println!("{}", arg.path);

    let path = Path::new("/Users/bisquit/.codew/workspaces/rpget.code-workspace");

    let home_dir = dirs::home_dir();
    if let Some(home_dir) = home_dir {
        let codew_home_dir = home_dir.join(".codew");
        println!("{}", codew_home_dir.to_str().unwrap());
    }

    // println!("{}", homedir.unwrap().into_os_string());

    // run code command
    // Command::new("code")
    //     .arg(path)
    //     .spawn()
    //     .expect("failed");
}