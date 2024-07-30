use files::{slink, delete};
use super::packages::{enable_aur, install};

use crate::{linux::manjaro::packages, prelude::*};


pub fn run(){
    H1!("Manjaro Linux Setup");

    let config = super::config::get("manjaro");

    h2!("Stop PC beeper");
    exe!("echo 'blacklist pcspkr' | sudo tee -a /etc/modprobe.d/nobeep.conf");

    run!(set_time, "Setting system time");

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
    super::packages::remove(&config.packages.unneeded);

    delete(MANJARO_I3_FILES_TO_DELETE, true);
    
    h2!("Installing first required package collection: {:?}",config.packages.requirements);
    install(&config.packages.requirements);
    
    h2!("Making Trash folder");
    exe!("mkdir -pv ~/Work/Trash; trash --trash-dir ~/Work/Trash"); // To do config
    h2!("Checking Trash Directory");
    exe!("ls -la ~/Work/Trash; trash --directory"; true);


    H1!("Linux Kernel");
    h2!("Running manjaro setting manager for checking kernels");
    exe!("manjaro-settings-manager &");


    run!(enable_aur, "Enabling AUR and others pamac settings");
    packages::update();

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


// fn i3_setup(){
//     H1!("Manjaro i3 GUI Setup");
// }

pub fn set_time(){
    H1!("System time Setup ");
    exe!("timedatectl status"; true);
    h2!("Setting system clock auto sync");
    exe!("sudo timedatectl set-ntp true");
    h2!("Showing timedatectl status");
    exe!("timedatectl status"; true);
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