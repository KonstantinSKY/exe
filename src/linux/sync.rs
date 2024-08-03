use crate::prelude::*;

pub fn run(){
    let config_path = home_path!(CONFIGS_DIR);
    let cmd = format!("git -C {config_path:?}");
    H1!("All Configs  Sync to GitHub");
    h2!("Pulling from GitHub");
    exe!("{cmd} pull -v"; true);
    
    h2!("Pushing to GitHub");
    exe!("{cmd} add . -v"; true);
    exe!("{cmd} commit -av -m 'Configs updated'"; true);
    exe!("{cmd} push -v"; true);
    
    // let config_source_path = crate::configs::get_config_path("pass");
    let config = crate::pass::config::Config::new("pass");
    let password_store_source_path = home_path!(&config.password_store_source);

    //password_storage
    let cmd = format!("git -C {password_store_source_path:?}");
    H1!("Password Store Sync");
    h2!("Pulling from");
    exe!("{cmd} pull -v"; true);
    
    h2!("Pushing");
    exe!("{cmd} add . -v"; true);
    exe!("{cmd} commit -av -m 'Password store updated'"; true);
    exe!("{cmd} push -v"; true);

}