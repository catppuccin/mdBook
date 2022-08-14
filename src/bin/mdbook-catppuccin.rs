use std::{io, process};

use clap::{Arg, ArgMatches, Command};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use semver::{Version, VersionReq};

use mdbook_catppuccin::Catppuccin;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let matches = make_app().get_matches();

    let preprocessor = Catppuccin::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Some(sub_args) = matches.subcommand_matches("install") {
        install::handle_install(sub_args)
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

pub fn make_app() -> Command<'static> {
    Command::new("mdbook-catppuccin")
        .about("A mdbook preprocessor that implements catppuccin flavours as default themes")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            Command::new("install")
                .arg(
                    Arg::new("assets-dir")
                        .default_value(".")
                        .long("assets-dir")
                        .help("Relative directory for the assets, from the book directory root")
                )
                .arg(
                    Arg::new("dir")
                     .default_value(".")
                     .help("Root directory for the book, this should contain the configuration file `book.toml`")
                )
                .about("Install the necessary files needed and update the config to include them"),
        )
}

fn handle_preprocessing(pre: &Catppuccin) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &Catppuccin, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

mod install {
    use std::fs::File;
    use std::io::Write;
    use std::{fs, path::PathBuf};

    use clap::ArgMatches;
    use log::{error, info, warn};
    use mdbook_catppuccin::toml::ArrayExt;
    use toml_edit::{Array, Document, Item, Table, Value};

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const CATPPUCCIN_ASSETS: &[(&str, &str, &[u8])] = &[
        (
            "catppuccin.js",
            "additional-js",
            include_bytes!("./assets/catppuccin.js"),
        ),
        (
            "catppuccin.css",
            "additional-css",
            include_bytes!("./assets/catppuccin.css"),
        ),
    ];

    pub(crate) fn handle_install(sub_args: &ArgMatches) {
        let dir = sub_args.get_one::<String>("dir").unwrap();
        let assets_dir = sub_args.get_one::<String>("assets-dir").unwrap();
        let project_dir = PathBuf::from(dir);
        let toml_config = project_dir.join("book.toml");

        if !toml_config.exists() {
            error!(
                "Configuration TOML file '{}' is missing!",
                toml_config.display()
            );
            return;
        }

        info!("Reading configuration file '{}'", toml_config.display());
        let toml = fs::read_to_string(&toml_config).expect("Can't read configuration file");
        let mut document = toml
            .parse::<Document>()
            .expect("Configuration is not valid TOML");

        // match tbl_output_html(&document) {
        //     Ok(pog) => {
        //         match pog.additional_js() {
        //             Ok(css) => if !css.contains_str("catppuccin.css") {},
        //             Err(err) => error!("Unexpected Table: {err}"),
        //         }
        //         match pog.additional_css() {
        //             Ok(_) => todo!(),
        //             Err(err) => error!("Unexpected Table: {err}"),
        //         }
        //     }
        //     Err(unpog) => error!("Unexpected Configuration: {}", unpog),
        // }

        if let Ok(preprocessor) = tbl_preproccesor(&mut document) {
            let value = toml_edit::value(Value::from(VERSION.trim()).decorated(
                " ",
                " # DO NOT EDIT: Managed by `mdbook-catppuccin install`",
            ));
            preprocessor["assets_version"] = value;
        } else {
            warn!("Unexpected configuration, not updating pre-processor configuration");
        };

        for (name, entry, content) in CATPPUCCIN_ASSETS {
            let path = if assets_dir == "." {
                project_dir.join(name)
            } else {
                project_dir.join(assets_dir).join(name)
            };
            let path_str = path.to_str().expect("Non-UTF8 Filepath");

            if let Ok(asset) = tbl_additional_asset(&mut document, entry) {
                if !asset.contains_str(path_str) {
                    info!("Adding '{path_str}' to '{entry}'");
                    asset.push(path_str);
                }
            } else {
                warn!("Unexpected configuration, not updating '{entry}'");
            }

            info!(
                "Copying '{name}' to '{filepath}'",
                filepath = path.display()
            );
            let mut file = File::create(path).expect("Can't open file for writing");
            file.write_all(content)
                .expect("Can't write content to file");
        }

        let new_toml = document.to_string();
        if new_toml != toml {
            info!(
                "Saving changed configuration to '{}'",
                toml_config.display()
            );
            let mut file =
                File::create(toml_config).expect("Can't open configuration file for writing.");
            file.write_all(new_toml.as_bytes())
                .expect("Can't write configuration");
        } else {
            info!(
                "Configuration '{}' already up to date",
                toml_config.display()
            );
        }

        info!("mdbook-catppuccin is now installed. Build your book with `mdbook build` to see your new catppuccin colour palettes in action!");
    }

    fn read_configuration_file() {}

    fn copy_assets() {}

    fn update_configuration_file() {}

    fn tbl_additional_asset<'a>(
        doc: &'a mut Document,
        entry: &'a str,
    ) -> Result<&'a mut Array, ()> {
        let doc = doc.as_table_mut();
        let empty_table = Item::Table(Table::default());
        let empty_array = Item::Value(Value::Array(Array::default()));

        doc.entry("output")
            .or_insert(empty_table.clone())
            .as_table_mut()
            .and_then(|item| {
                item.entry("html")
                    .or_insert(empty_table)
                    .as_table_mut()?
                    .entry(entry)
                    .or_insert(empty_array)
                    .as_value_mut()?
                    .as_array_mut()
            })
            .ok_or(())
    }

    fn tbl_preproccesor(doc: &mut Document) -> Result<&mut Item, ()> {
        let doc = doc.as_table_mut();

        let empty_table = Item::Table(Table::default());
        let item = doc.entry("preprocessor").or_insert(empty_table.clone());
        let item = item
            .as_table_mut()
            .ok_or(())?
            .entry("catppuccin")
            .or_insert(empty_table);

        Ok(item)
    }
}
