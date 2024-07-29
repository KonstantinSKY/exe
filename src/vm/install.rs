use crate::prelude::*;

pub fn run(){

    H1!("INSTALL VIRTUAL BOX");


    h2!("Checking linux system kernel");
    exe!("mhwd-kernel -li");

    h2!("Installing Virtual box packages");
    crate::linux::manjaro::packages::install("virtualbox virtualbox-ext-oracle");

    h2!("Adding user to virtualboxusers");
    exe!("sudo usermod -aG vboxusers $USER");

    h2!("showing users in group"); 
    exe!("grep vboxusers /etc/group"; true);

    h2!("Rebooting system");
    exe!("reboot"); 


}