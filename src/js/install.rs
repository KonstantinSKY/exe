use crate::linux::manjaro::packages::install;
use crate::prelude::*;

pub fn run() {
    H1!("JavaScript Eco System");

    h2!("Installing needed packages");
    install("nodejs npm yarn nvm");

    h2!("Activating NVM");

    let nvm_init = "source /usr/share/nvm/init-nvm.sh";

    h2!("Showing versions");
    exe!("echo 'Node:'; node --version; 
            echo 'npm:'; npm --version; 
            echo 'yarn:'; yarn --version; 
            {nvm_init}; 
            echo 'nvm:'; nvm --version"; true);

    H1!("NODE.JS VERSIONS");
    h2!("Node remote LTS versions");
    exe!("{nvm_init}; nvm ls-remote --lts"; true);

    h2!("Node Installed versions");
    exe!("{nvm_init}; nvm ls"; true);

    h2!("Installing NodeJS latest LTS version, switch to and showing all installed versions");
    exe!("{nvm_init}; nvm install --lts; nvm use --lts");

    h2!("Node Installed versions:");
    exe!("{nvm_init}; nvm ls"; true);
    h2!("Current activated version:");
    exe!("{nvm_init}; nvm current"; true);

    H1!("NPM versions");
    h2!("Checking npm version for update");
    exe!("npm -g outdated npm"; true);

    h2!("Update npm for latest version");
    exe!("sudo npm install -g npm@latest; npm --version");
}
