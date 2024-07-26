
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
    exe!("cargo build --release"; true);

    h2!("Force Copying exe to ./local/bin");
    let source_path = Path::new("target/release/exe");

    let destination_path = home_path!(".local/bin/exe");
    crate::sh::files::force_copy(source_path, &destination_path); 

    h2!("Copying exe to Project exe Directory");
    let project_path = home_path!("Projects/exe");
    let bin_path = home_path!(project_path.clone(), "bin/exe");

    exe!("cp {destination_path:?} {bin_path:?} -v"; true); 

    h2!("Commit and pushing Tools directory");
    let cmd = &format!("git -C {project_path:?}");
    exe!("{cmd} pull -v"; n);
    exe!("{cmd} add . -v"; n);
    exe!("{cmd} commit -av -m 'exe util update'"; n);
    exe!("{cmd} push -v"; n);
}