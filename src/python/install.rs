use crate::linux::manjaro::packages::install;
use crate::prelude::*;

pub fn run() {
    H1!("Python Eco System");

    h2!("Installing needed packages");
    install("base-devel openssl zlib xz tk bzip2 libffi pyenv python-poetry");
    exe!("python --version; poetry --version;  pyenv --version;"; true);
    exe!("sh");
}
