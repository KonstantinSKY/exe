use crate::prelude::*;

pub fn run(){
    let cmd = "git -C $HOME/Configs";
    H1!("VSCODE Settings Sync to GitHub");
    exe!(&format!("{cmd} pull -v"), true);
    exe!(&format!("{cmd} add . -v"), true);
    exe!(&format!("{cmd} commit -av -m 'vscode setting update'"), true);
    exe!(&format!("{cmd} push -v"), true);
}