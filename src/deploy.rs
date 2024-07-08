
use crate::prelude::*;
use clap::{Command, ArgMatches};

#[must_use]  //Self deploying commands
pub fn commands() -> Command {
    Command::new("deploy")
        .about("Build exe bin file, self deploying to ~/local/bin and renew in ~/Tools/bin")
}

pub fn handle(_arg_matches: &ArgMatches) {
    deploy(true);
}

fn deploy(no_confirm_flag: bool) {
    let n = no_confirm_flag;
    H1!("Deploy exe binary");

    h2!("Building exe binary file");
    exe!("cargo build --release", true);

    h2!("Copying exe to ./local/bin");
    exe!("cp target/release/exe $HOME/.local/bin/exe -v", n); 

    h2!("Copying exe to Tools/bin");
    exe!("cp target/release/exe $HOME/Tools/bin/exe -v", n); 

    h2!("Commit and pushing Tools directory");
    let cmd = "git -C $HOME/Tools";
    exe!(&format!("{cmd} pull -v"), n);
    exe!(&format!("{cmd} add . -v"), n);
    exe!(&format!("{cmd} commit -av -m 'exe util update'"), n);
    exe!(&format!("{cmd} push -v"), n);
}