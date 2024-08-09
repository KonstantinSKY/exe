use super::config::Config;
use super::packages::{enable_aur, install};
use crate::{linux::manjaro::packages, prelude::*};
use files::{delete, slink_pair};

pub fn run() {
    H1!("Manjaro Linux Setup");
    let config = super::config::Config::new("manjaro");

    run!(set_time, "Setting system time");

    h2!("Removing unneeded packages");
    super::packages::remove(&config.packages.unneeded);

    delete(&config.delete_files, true);

    h2!(
        "Installing first required package collection: {:?}",
        config.packages.requirements
    );
    install(&config.packages.requirements);

    H1!("Linux Kernel and Drivers");
    h2!("Running manjaro setting manager for checking kernels");
    exe!("manjaro-settings-manager &");

    h2!("Showing installed Drivers");
    exe!("mhwd -li"; true);
    h2!("Installing and update proprietary video drivers");
    exe!("sudo mhwd -a pci nonfree 0300");
    h2!("Checking using Video driver, may be u need to reboot before");
    exe!("lspci -k | grep -EA3 'VGA|3D|Display'"; true);


    run!(
        || enable_aur(&config),
        "Enabling AUR and others pamac settings"
    );
    packages::update();
    run!(|| i3(&config), "Setup I3 window manager");
    run!(|| grub(&config), "GRUB Setup");

    run!(crate::megasync::install::run, "Install and Setup Megasync");

    h2!(
        "Installing package collection: requirements2 : {:?}",
        config.packages.requirements_2
    );
    install(&config.packages.requirements_2);

    //TODO AUR Enabling
    h2!(
        "Installing package collection: internet : {:?}",
        config.packages.internet
    );
    install(&config.packages.internet);

    h2!(
        "Installing package collection: communication : {:?}",
        config.packages.communication
    );
    install(&config.packages.communication);

    H1!("Reboot system IF Necessary");

    h2!("Rebooting system.");
    exe!("sudo reboot");
}

pub fn set_time() {
    H1!("System time Setup ");
    exe!("timedatectl status"; true);
    h2!("Setting system clock auto sync");
    exe!("sudo timedatectl set-ntp true");
    h2!("Showing timedatectl status");
    exe!("timedatectl status"; true);
}

fn i3(cfg: &Config) {
    H1!("Manjaro i3 Create symbolic links for Configs");

    //Manjaro i3 setting
    h2!("Editing global i3 config for removing config wizard");
    let cf = &cfg.global_config_file;
    exe!("sudo sed -i 's/exec i3-config-wizard//g' {cf}; cat {cf}");

    let local_i3_config_dir_path = home_path!(&cfg.i3_config_dir[0]);

    h2!("Creating i3 config  directory for configs if absent: {local_i3_config_dir_path:?}");
    exe!("mkdir -vp {local_i3_config_dir_path:?}; la -la {local_i3_config_dir_path:?}");
    slink_pair(&cfg.i3_config_dir);
    exe!("rm ~/.i3 -r");

    h2!("Qt configs");
    slink_pair(&cfg.qt5_conf);
    //     i3_setup();
}

fn grub(cfg: &Config) {
    H1!("GRUB SETTINGS");
    let gc = &cfg.grub_config;
    let gt = &cfg.grub_theme;

    h2!("Showing GRUB Config {gc}");
    exe!("cat {gc}"; true);

    h2!("Changing GRUB_TIMEOUT_STYLE and select theme for loading menu");
    exe!("sudo sed -i 's/GRUB_TIMEOUT_STYLE=.*$/GRUB_TIMEOUT_STYLE=menu/' {gc}");
    exe!("sudo sed -i 's|GRUB_THEME=.*|GRUB_THEME={gt}|' {gc}");

    h2!("Showing updated GRUB Config {gc}");
    exe!("cat {gc}"; true);

    h2!("Update GRUB to apply the changes");
    exe!("sudo update-grub");
}
