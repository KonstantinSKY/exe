use crate::prelude::*;

pub fn run(){
    let cmd = "git -C $HOME/Work/Configs";
    H1!("All Configs  Sync to GitHub");
    h2!("Pulling from GitHub");
    exe!("{cmd} pull -v"; true);
    
    h2!("Pushing to GitHub");
    exe!("{cmd} add . -v"; true);
    exe!("{cmd} commit -av -m 'Configs updated'"; true);
    exe!("{cmd} push -v"; true);
}