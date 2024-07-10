use files::slink;

use crate::prelude::*;


pub fn run(){
    H1!("Manjaro Linux Setup");

    set_time();

    H1!("Manjaro i3 Create symbolic links for Configs");

    h2!("Editing global i3 config for removing config wizard");
    //TODO backup /etc/i3/config --sudo
    exe!("sudo sed -i 's/exec i3-config-wizard//g' /etc/i3/config; cat /etc");

    h2!("Creating .i3 directory for configs if absent");
    exe!("mkdir -vp $HOME/.i3; la -la $HOME/.i3");
    slink(&home_path!(CONFIGS_DIR, "i3.cfg"),  &home_path!(".i3/config"));

    slink(&home_path!(CONFIGS_DIR, "i3.profile"), &home_path!(".profile"));
    // slink(&home_path!(CONFIGS_DIR, "bash_profile"), &home_path!(".profile"));
    slink(&home_path!(CONFIGS_DIR, "mimeapps.list.cfg"), &home_path!(".config","mimeapps.list"));

    h2!("Qt configs");
    slink(&home_path!(CONFIGS_DIR, "qt5ct.conf"), &home_path!(".config","qt5ct/qt5ct.conf"));

    //urxvt terminal
    slink(&home_path!(CONFIGS_DIR, "urxvt.Xresources.cfg"), &home_path!(".Xresources"));
//     i3_setup();
    
    h2!("Removing unneeded packages");
    super::packages::remove(MANJARO_I3_PACKAGES_TO_REMOVE);

}



fn i3_setup(){
    H1!("Manjaro i3 GUI Setup");
}

pub fn set_time(){
    H1!("System time Setup ");
    h2!("Setting system clock auto sync");
    exe!("sudo timedatectl set-ntp true");
    h2!("Showing timedatectl status");
    exe!("timedatectl status");
}