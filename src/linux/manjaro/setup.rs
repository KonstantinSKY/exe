use files::{slink, delete};
use super::packages::{enable_aur, install};
use super::config::Config;
use crate::{linux::manjaro::packages, prelude::*};


pub fn run(){
    H1!("Manjaro Linux Setup");
    let config = super::config::Config::new("manjaro");
    
    run!(set_time, "Setting system time");


    
    h2!("Removing unneeded packages");
    super::packages::remove(&config.packages.unneeded);

    delete(&config.delete_files, true);
    
    h2!("Installing first required package collection: {:?}",config.packages.requirements);
    install(&config.packages.requirements);
    
    H1!("Linux Kernel");
    h2!("Running manjaro setting manager for checking kernels");
    exe!("manjaro-settings-manager &");


    run!(||enable_aur(&config), "Enabling AUR and others pamac settings");
    packages::update();
    run!(|| i3(&config), "Setup I3 window manager");
    run!(grub, "GRUB Setup");

    h2!("Installing package collection: requirements2 : {:?}", config.packages.requirements_2);
    install(&config.packages.requirements_2);

    //TODO AUR Enabling
    h2!("Installing package collection: internet : {:?}", config.packages.internet);
    install(&config.packages.internet);

    h2!("Installing package collection: communication : {:?}", config.packages.communication);
    install(&config.packages.communication);
    
    H1!("Reboot system IF Necessary");
    
    h2!("Rebooting system.");
    exe!("sudo reboot");
    

}


pub fn set_time(){
    H1!("System time Setup ");
    exe!("timedatectl status"; true);
    h2!("Setting system clock auto sync");
    exe!("sudo timedatectl set-ntp true");
    h2!("Showing timedatectl status");
    exe!("timedatectl status"; true);
}

fn i3(cfg: &Config){
    H1!("Manjaro i3 Create symbolic links for Configs");

    //Manjaro i3 setting
    h2!("Editing global i3 config for removing config wizard");
    exe!("sudo sed -i 's/exec i3-config-wizard//g' /etc/i3/config; cat /etc");

    let local_config_dir_path = home_path!(&cfg.local_config_dir);
    let target_config_dir_path = home_path!(&cfg.target_config_dir);

    h2!("Creating i3 config  directory for configs if absent: {local_config_dir_path:?}");
    
    exe!("mkdir -vp {local_config_dir_path:?}; la -la {local_config_dir_path:?}");
    slink(&target_config_dir_path, &local_config_dir_path);
    exe!("rm ~/.i3 -r");

    // slink(&home_path!(CONFIGS_DIR, "bash_profile"), &home_path!(".profile"));
    slink(&home_path!(CONFIGS_DIR, "mimeapps.list"), &home_path!(".config","mimeapps.list"));

    h2!("Qt configs");
    slink(&home_path!(CONFIGS_DIR, "qt5ct.conf"), &home_path!(".config","qt5ct/qt5ct.conf"));

    h2!("urxvt terminal");
    slink(&home_path!(CONFIGS_DIR, "terminals/urxvt/Xresources"), &home_path!(".Xresources"));
//     i3_setup();
}

fn grub(){
    H1!("GRUB SETTINGS");

    h2!("Showing GRUB Config {GRUB_CONFIG}");
    exe!("cat {GRUB_CONFIG}");
    
    h2!("Changing GRUB_TIMEOUT_STYLE and select theme for loading menu");
    exe!("sudo sed -i 's/GRUB_TIMEOUT_STYLE=.*$/GRUB_TIMEOUT_STYLE=menu/' {GRUB_CONFIG}");
    exe!("sudo sed -i 's|GRUB_THEME=.*|GRUB_THEME={GRUB_MANJARO_THEME}|' {GRUB_CONFIG}");

    h2!("Showing updated GRUB Config {GRUB_CONFIG}");
    exe!("cat {GRUB_CONFIG}");
    
    h2!("Update GRUB to apply the changes");
    exe!("sudo update-grub");
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exe() {
        exe!("cat {GRUB_CONFIG}");
        exe!("cat {}", GRUB_CONFIG);
        exe!("cat {GRUB_CONFIG}"; true);
    }

}