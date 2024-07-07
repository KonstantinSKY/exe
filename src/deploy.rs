use clap::{Command, ArgMatches};

use crate::{
    sh::exec::exe,
    styles::{h1, h2},
};


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
    h1("Deploy exe binary");

    h2("Building exe binary file");
    exe("cargo build --release", true);


    h2("Copying exe to ./local/bin");
    exe("cp target/release/exe $HOME/.local/bin/exe -v", n); 

    h2("Copying exe to Tools/bin");
    exe("cp target/release/exe $HOME/Tools/bin/exe -v", n); 

    h2("Commit and pushing Tools directory");
    exe("git -C $HOME/Tools add . -v", n);
    exe("git -C $HOME/Tools commit -av -m 'exe util update'", n);
    exe("git -C $HOME/Tools push -v", n);
}