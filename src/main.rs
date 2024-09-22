use std::process;

use clap::{
    command, crate_authors, crate_description, crate_version, Arg, ArgAction, Command,
};
use log::error;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    error!(
        r#"
This preprocessor is no longer supported and no new versions will be released.
Instructions on how to apply Catppuccin to your mdBook can be found at https://github.com/catppuccin/mdBook.

Please uninstall this preprocessor by running 'cargo uninstall mdbook-catppuccin' and remove '[preprocessor.catppuccin]' from your 'book.toml' file.
For further information on why this package no longer works, please refer to https://github.com/catppuccin/mdBook/issues/107.

Exiting..."#
    );

    make_app().get_matches();

    process::exit(1);
}

pub fn make_app() -> Command {
    command!()
        .name("mdbook-catppuccin")
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            command!("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            command!("install")
                .arg(
                    Arg::new("dir")
                    .default_value(".")
                    .help("Root directory for the book, this should contain the configuration file `book.toml`")
                )
                .arg(
                    Arg::new("force")
                    .long("force")
                    .short('f')
                    .action(ArgAction::SetTrue)
                    .help("Forcefully overwrite the existing 'index.hbs' file")
                )
                .about("Install the necessary files needed and update the config to include them"),
        )
}
