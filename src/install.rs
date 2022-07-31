use clap::ArgMatches;
use toml_edit::{Document, Item, Table};

fn handle_install(sub_args: &ArgMatches) {

}

fn toml_preproccesor(doc: &mut Document) -> Result<&mut Item, ()> {
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
