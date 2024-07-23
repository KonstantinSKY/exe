use std::process::Command;

use crate::prelude::*;
use exec::exe;
use xshell::{cmd as shell_cmd, Shell};

pub fn run() {
    H1!("NODE.JS ecosystem update");
    h2!("Node remote LTS versions");
    exe!("nvm --versions"; true);

    // let sh = Shell::new().expect("Failed to create shell");
    // let cmd1 = "export TEST_VAR='test_VAR222'; echo $TEST_VAR";
    // let cmd1 = "export NVM_DIR='$HOME.nvm'";
    // // let cmd2 =  "[ -s '$NVM_DIR/nvm.sh' ] && . '$NVM_DIR/nvm.sh'";
    // let cmd2 =  "echo $NVM_DIR/";
    // exe!("source .bashrc");
    // exe!("nvm");
    // exe!(" echo SDSFSDFSDFSD");
    // let output = Command::new("sh")
    // .arg("-c")
    // .arg("source /home/sky/.nvm/nvm.sh && command Dnvm --version")
    // .output()
    // .expect("failed to execute process");
    

    // println!("OUTPUT: {output:?}");
    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // exe!("export TEST_VAR='test_VAR'");

    // exe!("echo !!! $TEST_VAR");
//     let cmd3 = "echo $TEST_VAR";
    
//     match shell_cmd!(sh, "sh -c {cmd1}").read() {
//         Ok(output) => {
//             println!("{output}");
//         },
//         Err(e) => {
//             eprintln!("Failed to execute command: {e}");
//         },
//     }

//     match shell_cmd!(sh, "sh -c {cmd2}").read() {
//         Ok(output) => {
//             println!("{output}");
//         },
//         Err(e) => {
//             eprintln!("Failed to execute command: {e}");
//         },
//     }

//   match shell_cmd!(sh, "sh -c {cmd3}").read() {
//         Ok(output) => {
//             println!("Node Installed versions:");
//             println!("{output}");
//         },
//         Err(e) => {
//             eprintln!("Failed to execute command: {e}");
//         },
//     }


    // exe(nvm_init_and_ls, false);
    // h2!("Node Installed versions");
    // exe!("chmod +x $HOME/.nvm/nvm.sh"; true);
    // exe!("$HOME/.nvm/nvm.sh"; true);

    // h2!("Installing NodeJS latest LTS version, switch to and showing all installed versions");
    // exe!("nvm install --lts; nvm use --lts");

    // h2!("Node Installed versions:");
    // exe!("nvm ls"; true);
    // h2!("Current activated version:");
    // exe!("nvm current"; true);

    // H1!("NPM  versions");
    // h2!("Checking npm version for update");
    // exe!("npm -g outdated npm"; true);

    // h2!("Update npm for latest version");
    // exe!("npm install -g npm@latest; npm --version");
}
