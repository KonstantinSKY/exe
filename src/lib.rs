#![warn(clippy::pedantic)]  

pub mod styles;
pub mod deploy; 
pub mod rust;
pub mod sh;
pub mod linux;

use clap::Arg;

#[must_use] 
pub fn arg_no_confirm() -> Arg {
    Arg::new("noconfirm")
    .help("Skip confirmation flag")
    .short('n')
    .long("noconfirm")
    .action(clap::ArgAction::SetTrue)
}

#[must_use] 
pub fn arg_version() -> Arg {
    Arg::new("version")
    .help("Show versions")
    .short('v')
    .long("version")
    .action(clap::ArgAction::SetTrue)
}
