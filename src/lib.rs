mod install;

use log::{error, warn};
use mdbook::book::Book;
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct Catppuccin;

impl Catppuccin {
    pub fn new() -> Self {
        Catppuccin
    }
}

impl Preprocessor for Catppuccin {
    fn name(&self) -> &str {
        "catppuccin"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
        let src_dir = ctx.root.join(&ctx.config.book.src);

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}
