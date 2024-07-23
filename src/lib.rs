#![warn(clippy::pedantic)]

pub mod code;
pub mod deploy;
pub mod docker;
pub mod linux;
pub mod prelude;
pub mod rust;
pub mod sh;
pub mod styles;
pub mod configs;
pub mod nvim;
pub mod megasync;
pub mod pass;
pub mod ssh;
pub mod git;
pub mod js;
pub mod python;

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
