
use crate::{
    sh::exec::{exe, print_cmd},
    styles::{h1, h2},
};
use crate::linux::os;


const EXTENTIONS: [&str; 10] = [
    "vscodevim.vim",
    "asapdotid.manjaro-dark",
    "rust-lang.rust-analyzer",
    "pkief.material-icon-theme",
    "bungcip.better-toml", 
    "serayuzgur.crates",
    "",
    "",
    "",
    "",
];



pub fn run(){
    h1 ("VS Code install and setup");

    h2 ("Installing VS Code");
    os::install("code");

    h2 ("Installing VS Code Extentions");
    for ext in EXTENTIONS{
        if ext == "" {continue;}
        exe(ext, false)
    }
    h2 ("Creating configs symbolic links");
}
