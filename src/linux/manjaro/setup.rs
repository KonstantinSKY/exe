use files::{slink, delete};

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

    delete(MANJARO_I3_FILES_TO_DELETE, true);
    
    h2!("Installing Trash-CLI");
    exe!("sudo pacman -S trash-cli --noconfirm");

    h2!("Installing Materia GRT Theme");
    exe!("sudo pacman -S materia-gtk-theme --noconfirm");
    
    h2!("Installing Materia Setting Manager");
    exe!("sudo pacman -S manjaro-settings-manage --noconfirm");

    h2!("Installing Materia GRT Theme");
    exe!("sudo pacman -S materia-gtk-theme --noconfirm");

    H1!("GRUB SETTING");
    
    h2!("Showing GRUB Config {GRUB_CONFIG}");
    exe!("cat");
    exe!("cat {GRUB_CONFIG}");
    exe!("cat {}",GRUB_CONFIG);
    
    exe!("cat {GRUB_CONFIG}"; true);
    // h2 Changing GRUB_TIMEOUT_STYLE to 'menu'
    // exe "sudo sed -i 's/^GRUB_TIMEOUT_STYLE=.*$/GRUB_TIMEOUT_STYLE=menu/' $GRUB_CONFIG"
    
    // h2 Showing updated $GRUB_CONFIG
    // show $GRUB_CONFIG
    
    // h2 Update GRUB to apply the changes
    // exe "sudo update-grub"
    
    // h1 Reboot system IF Nessesary   
    // h2 Rebooting system.
    // exe "sudo reboot"
    


    

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