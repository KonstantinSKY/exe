use files::slink;

use crate::prelude::*;
use crate::{home_path, linux::os, sh};

pub fn install() {
    let config_link_path = home_path!(".config/alacritty");
    let config_source_path = home_path!(CONFIGS_DIR, "alacritty");

    os::install("alacritty");
    sh::files::slink(&config_source_path, &config_link_path);
    exe!("mkdir -pv .local/bin");
    slink(
        &home_path!(CONFIGS_DIR, "terminal"),
        &home_path!(".local/bin/terminal"),
    );
    exe!("alacritty &");
}
