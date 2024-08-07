use crate::prelude::*;
use crate::{home_path, linux::os, sh};


pub fn install (){
    let config_link_path = home_path!(".config/alacritty");
    let config_source_path = home_path!(CONFIGS_DIR, "terminals/alacritty");

    os::install("alacritty");
    sh::files::slink(&config_source_path, &config_link_path);
    exe!("alacritty");
}