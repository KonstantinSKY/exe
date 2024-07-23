use crate::linux::manjaro::packages::install;
use crate::prelude::*;

pub fn run() {
    H1!("JavaScript Eco System");

    h2!("Installing needed packages");
    install("nodejs npm yarn nvm");

    h2!("Activating NVM");

    let nvm_init = "source /usr/share/nvm/init-nvm.sh";

    h2!("Showing versions");
    exe!("node --version; npm --version;  yarn --version; {nvm_init}; nvm --version"; true);

    // H1!("NVM - Node Version Manager");
    // h2!("Installing nvm");
    // exe!("wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash");

    // h2!("Exporting variables");
    // exe!("export NVM_DIR='$HOME/.nvm'");
    // exe!(r"[ -s '$NVM_DIR/nvm.sh' ] && \. '$NVM_DIR/nvm.sh'"; true);

    // H1!("NODE.JS VERSIONS");
    // h2!("Node remote LTS versions");
    // exe!("nvm ls-remote --lts"; true);

    // h2!("Node Installed versions");
    // exe!("nvm ls"; true);

    // h2!("Installing NodeJS latest LTS version, switch to and showing all installed versions");
    // exe!("nvm install --lts; nvm use --lts");

    // h2!("Node Installed versions:");
    // exe!("nvm ls"; true);
    // h2!("Current activated version:");
    // exe!("nvm current"; true);

    // H1!("NPM  versions");
    // h2!("Checking npm version for update");
    // exe!("npm -g outdated npm"; true);

    // h2!("Update npm for latest version");
    // exe!("npm install -g npm@latest; npm --version");
}
