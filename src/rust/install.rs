use crate::{
    sh::exec::exe,
    styles::{h1, h2},
};

const GET_SH_CMD: &str = "curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh";

pub fn run(noconfirm_flag: bool) {
    let n = noconfirm_flag;

    h1("Rust & Ecosystem Installation and Setting");

    h2("Getting installation file");
    exe(GET_SH_CMD, n);

    h2("Adding $PATH variable");
    exe("source $HOME/.cargo/env", n);
    
    h2("Showing $PATH variable");
    exe("echo $PATH", true);

    // h2("Checking installed Versions");
    // exe("rustc --version; rustup --version; cargo --version", true);
    versions();

    h2("Update rustup to latest Version");

    exe("rustup update", n);

    h2("Open Local Documentation");
    exe("rustup doc", n);
}

pub fn versions(){
    h2("Checking installed Versions");
    exe("rustc --version; rustup --version; cargo --version", true);
}