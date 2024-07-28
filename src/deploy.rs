use crate::prelude::*;
use clap::{ArgMatches, Command};

#[must_use] //Self deploying commands
pub fn commands() -> Command {
    Command::new("deploy")
        .about("Build exe bin file, self deploying to ~/local/bin and renew in ~/Tools/bin")
}

pub fn handle(_arg_matches: &ArgMatches) {
    deploy(true);
}

#[must_use] //Self deploying commands
pub fn get_commands() -> Command {
    Command::new("get").about("Get exe binary file from GitHub and copy to ~/.local/bin")
}

// pub fn get_handle(_arg_matches: &ArgMatches) {
//     get();
// }

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

    exe!("cp {destination_path:?} {bin_path:?} -v"; n);

    h2!("Commit and pushing Tools directory");
    let cmd = &format!("git -C {project_path:?}");
    exe!("{cmd} pull -v"; n);
    exe!("{cmd} add . -v"; n);
    exe!("{cmd} commit -av -m 'exe util update'"; n);
    exe!("{cmd} push -v"; n);
}

pub fn get() {
    H1!("Get exe binary file from GitHub");

    let project_path = home_path!("Projects/exe");

    let git_hub = "https://raw.githubusercontent.com/KonstantinSKY/exe/main/bin/exe";
    let bin_path = home_path!(".local/bin");
    let tmp_path = bin_path.join("exe.tmp");
    let exe_path = bin_path.join("exe");

    h2!("Getting exe binary file from github");
    exe!("mkdir -p {bin_path:?} && wget {git_hub} -O {tmp_path:?}"; true);

    h2!("Force Copying exe.tmp to ./local/bin/exe");
    crate::sh::files::force_copy(&tmp_path, &exe_path);
    h2!("Removing temporary exe ");
    exe!("rm {tmp_path:?} -v"; true);
    
    h2!("Set permission for {exe_path:?}");
    exe!("chmod +x {exe_path:?}");

    h2!("Pulling exe project to {project_path:?}");
    exe!("git -C {project_path:?} pull -v");
}
