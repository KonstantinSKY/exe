use crate::{home_dir, prelude::*};


pub fn run(){
    H1!("Manjaro Linux Setup");
    H1!("Manjaro i3 Create symbolic links for Configs");

//     h2!("Editing global i3 config for removing config wizard");
//     //TODO backup /etc/i3/config --sudo
//     exe!("sudo sed -i 's/exec i3-config-wizard//g' /etc/i3/config");

//     h2!("Creating .i3 directory for configs if absent");
//     exe!("mkdir -vp $HOME/i3; la -la $HOME/i3");
//     configs_path = Path::new(home_dir!()).join(CONFIG_DIR);
//     slink(config_path.join("i3.cfg"),  "$i3/config" "i3 config"

// slink "$CP/i3.profile" "$HOME/.profile" ".profile"

// # slink "$CP/bash_prfle" "$HOME/.profile" ".profile"

// slink "$CP/mimeapps.list.cfg" "$HOME/.config/mimeapps.list" "mimeapps.list"

// #Gtk
// # slink "$CP/gtkrc-2.cfg" "$HOME/.gtkrc-2.0" "gtk-2 config"
// # slink "$CP/gtk-3.settings.ini" "$HOME/.config/gtk-3.0/settings.ini" "gtk-3 config"

// #Qt
// slink "$CP/qt5ct.conf" "$HOME/.config/qt5ct/qt5ct.conf" "Qt5 config"

// #urxvt terminal
// slink "$CP/urxvt.Xresources.cfg" "$HOME/.Xresources" "URxvt terminal config"


//     i3_setup();
}



fn i3_setup(){
    H1!("Manjaro i3 GUI Setup");
}