use crate::prelude::*;
use crate::linux::manjaro::packages::install;


pub fn run (){
    H1!("JavaScript Eco System");

h2!("Installing nodejs npm");
install("nodejs npm");

h2!("Showing node, npm versions");
exe!("node --version; npm --version"; true);

H1!("NVM - Node Version Manager");
h2!("Installing nvm");
// exe "wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash"
}