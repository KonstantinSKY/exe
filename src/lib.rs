#![warn(clippy::pedantic)]  

pub mod styles;
pub mod rust;
pub mod sh;
pub mod mods;

use clap::Arg;

#[must_use] 
pub fn arg_no_confirm() -> Arg {
    Arg::new("noconfirm")
    .help("Skip confirmation flag")
    .short('n')
    .long("noconfirm")
    .action(clap::ArgAction::SetTrue)
}
