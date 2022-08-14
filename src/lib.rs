pub mod toml;

use std::process;

use log::error;
use mdbook::book::Book;
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct Catppuccin;

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Catppuccin {
    pub fn new() -> Self {
        Catppuccin
    }
}

impl Preprocessor for Catppuccin {
    fn name(&self) -> &str {
        "catppuccin"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let html_config = ctx.config.html_config().unwrap_or_else(|| {
            error!("Could not parse 'output.html' field");
            process::exit(1);
        });
        let theme_dir = match html_config.theme {
            Some(ref theme) => ctx.root.join(theme),
            None => ctx.root.join("theme"),
        };

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}
