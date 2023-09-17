use std::{io, process};

use clap::{command, crate_version, Arg, ArgMatches, Command};
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

pub fn make_app() -> Command {
    command!()
        .name("mdbook-catppuccin")
        .about("A mdbook preprocessor that implements catppuccin flavours as default themes")
        .version(crate_version!())
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            Command::new("install")
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
    let renderer = sub_args
        .get_one::<&str>("renderer")
        .expect("Required argument");

    if pre.supports_renderer(renderer) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

mod install {
    use std::fs::File;
    use std::io::{ErrorKind, Write};
    use std::path::Path;
    use std::{fs, path::PathBuf};

    use clap::ArgMatches;
    use log::{error, info, warn};
    use toml_edit::{Document, Value};

    use mdbook_catppuccin::{
        toml::{ArrayExt, DocumentExt, TableExt},
        TomlPath,
    };

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const CATPPUCCIN_ASSETS: &[(&str, TomlPath, &[u8])] = &[
        (
            "catppuccin.css",
            TomlPath::Path("additional-css"),
            include_bytes!("./assets/catppuccin.css"),
        ),
        (
            "catppuccin-admonish.css",
            TomlPath::Path("additional-css"),
            include_bytes!("./assets/catppuccin-admonish.css"),
        ),
        (
            "index.hbs",
            TomlPath::None,
            include_bytes!("./assets/index.hbs"),
        ),
    ];

    pub(crate) fn handle_install(sub_args: &ArgMatches) {
        let dir = sub_args.get_one::<String>("dir").unwrap();
        let project_dir = PathBuf::from(dir);
        let toml_config = project_dir.join("book.toml");

        if !toml_config.exists() {
            error!(
                "Configuration TOML file '{}' is missing!",
                toml_config.display()
            );
            return;
        }

        let (toml, mut document) = read_configuration_file(&toml_config);
        let theme_dir = populate_theme_directory(&document, &project_dir);
        copy_assets(&mut document, &theme_dir);
        update_configuration_file(document, toml, toml_config);

        info!("mdbook-catppuccin is now installed. Build your book with `mdbook build` to try out your new catppuccin colour palettes!");
    }

    fn read_configuration_file(toml_config: &PathBuf) -> (String, Document) {
        info!("Reading configuration file '{}'", toml_config.display());
        let toml = fs::read_to_string(&toml_config).expect("Can't read configuration file");
        let document = toml
            .parse::<Document>()
            .expect("Configuration is not valid TOML");
        (toml, document)
    }

    fn populate_theme_directory(document: &Document, project_dir: &Path) -> PathBuf {
        let mut theme_dir = project_dir.join("theme");

        if let Ok(output_html) = document.get_output_html() {
            if let Ok(theme) = output_html.theme() {
                theme_dir = project_dir.join(theme)
            }
        } else {
            warn!("Unexpected configuration, defaulting to default 'theme' directory for transfering assets");
        }

        if let Err(err) = fs::create_dir_all(&theme_dir) {
            if err.kind() == ErrorKind::PermissionDenied {
                warn!("Permission to create '{}' denied", theme_dir.display())
            }
        }

        theme_dir
    }

    fn copy_assets(document: &mut Document, theme_dir: &Path) {
        if let Ok(preprocessor) = document.insert_into_preprocessor("catppuccin") {
            let value = toml_edit::value(Value::from(VERSION.trim()).decorated(
                " ",
                " # DO NOT EDIT: Managed by `mdbook-catppuccin install`",
            ));
            preprocessor["assets_version"] = value;
        } else {
            warn!("Unexpected configuration, not updating pre-processor configuration");
        };

        for (name, entry, content) in CATPPUCCIN_ASSETS {
            let path = theme_dir.join(name);
            let path_str = path.to_str().expect("Non-UTF8 Filepath");

            if let TomlPath::Path(path) = entry {
                if let Ok(additional_key) = document.insert_into_output_html(path) {
                    if !additional_key.contains_str(path_str) {
                        info!("Adding '{path_str}' to '{path}'");
                        additional_key.push(path_str);
                    }
                } else {
                    warn!("Unexpected configuration, not updating '{path}'");
                }
            }

            if *name == "index.hbs" {
                let file_exists = Path::new(path_str).try_exists();
                if let Ok(val) = file_exists {
                    if val {
                        info!(
                            "'{}' already exists and therefore will not be overwritten",
                            path.display()
                        );
                        break;
                    }
                } else {
                    error!("Unexpected error, cannot determine if 'index.hbs' exists");
                    break;
                }
            }

            info!("Copying '{name}' to '{path_str}'");
            let mut file = File::create(path_str).expect("Can't open file for writing");
            file.write_all(content)
                .expect("Can't write content to file");
        }
    }

    fn update_configuration_file(document: Document, toml: String, toml_config: PathBuf) {
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
    }
}
