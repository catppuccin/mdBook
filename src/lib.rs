pub mod toml;

use version_compare::{compare, Cmp};

use log::error;
use mdbook::book::Book;
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct Catppuccin;

pub enum TomlPath {
    Path(&'static str),
    None,
}

const LATEST_ASSETS_VERSION: &str = env!("CARGO_PKG_VERSION");
const TOML_KEY: &str = "preprocessor.catppuccin.assets_version";

fn get_major_version(version: &str) -> i32 {
    version
        .split_once('.')
        .unwrap()
        .0
        .parse::<i32>()
        .unwrap_or_else(|_| {
            panic!("Major version from '{version}' cannot be converted to an integer")
        })
}

impl Catppuccin {
    pub fn new() -> Self {
        Catppuccin
    }
}

impl Default for Catppuccin {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for Catppuccin {
    fn name(&self) -> &str {
        "catppuccin"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        match ctx.config.get(TOML_KEY).and_then(|key| key.as_str()) {
            Some(current_assets_version) => {
                if let Ok(cmp) = compare(LATEST_ASSETS_VERSION, current_assets_version) {
                    match cmp {
                        Cmp::Lt => error!(
                            r#"mdbook-catppuccin with version '{LATEST_ASSETS_VERSION}' is out of date with current asset version '{current_assets_version}'.
Please upgrade by running 'cargo install --force mdbook-catppuccin' and then re-run 'mdbook-catppuccin install'"#
                        ),
                        Cmp::Gt => {
                            error!(
                                r#"Out-Of-Date Asset Version '{current_assets_version}' Found:
Please update your version of 'mdbook-catppuccin' to '{LATEST_ASSETS_VERSION}'
Then run 'mdbook-catppuccin install' to install the lastest assets."#
                            );

                            let current_major_version = get_major_version(current_assets_version);
                            let latest_major_version = get_major_version(LATEST_ASSETS_VERSION);
                            if latest_major_version > current_major_version {
                                error!(
                                    r#"BREAKING CHANGES - Major Version Mismatch Detected.
Please see the version compatibility table to understand if you need to upgrade your 'index.hbs'
https://github.com/catppuccin/mdBook?tab=readme-ov-file#version-compatibility"#
                                )
                            }
                        }
                        _ => {}
                    }
                }
            }
            None => {
                error!("Unable to check assets_version, key '{TOML_KEY}' is not a string");
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}
