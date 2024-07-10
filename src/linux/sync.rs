use crate::prelude::*;

pub fn run(){
    let cmd = "git -C $HOME/Work/Configs";
    H1!("All Configs  Sync to GitHub");
    h2!("Pulling from GitHub");
    exe!(&format!("{cmd} pull -v"), true);
    
    h2!("Pushing to GitHub");
    exe!(&format!("{cmd} add . -v"), true);
    exe!(&format!("{cmd} commit -av -m 'Configs updated'"), true);
    exe!(&format!("{cmd} push -v"), true);
}