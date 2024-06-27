use crate::{
    exec,
    styles::{h1, h2},
};

pub fn run(noconfirm_flag: bool) {
    let n = noconfirm_flag;

    h1("Rust & Ecosystem Installation and Setting");

    h2("getting installation file");
    exec(
        "curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh",
        n,
    );

    h2("Restarting your shell");
    exec("echo $PATH", true);
    
    h2("Checking $PATH variable");
    exec("source $HOME/.cargo/env", n);

    h2("Checking rustup Version");
    exec("rustup --version", true);

    h2("Checking Cargo Version");
    exec("cargo --version", true);

    h2("Update rustup to latest Version");

    exec("rustup update", n);

    h2("Open Local Documentation");
    exec("rustup doc", n);
}
