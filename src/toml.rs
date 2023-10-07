use toml_edit::{Array, Document, Item, Table, Value};

pub trait DocumentExt {
    fn get_output_html(&self) -> Result<&Table, String>;
    fn insert_into_output_html(&mut self, entry: &str) -> Result<&mut Array, String>;
    fn insert_into_preprocessor(&mut self, entry: &str) -> Result<&mut Item, &str>;
}

impl DocumentExt for Document {
    fn get_output_html(&self) -> Result<&Table, String> {
        self.get("output")
            .ok_or_else(|| "No key 'output'".to_owned())?
            .as_table()
            .ok_or_else(|| "'output' is not a table".to_owned())?
            .get("html")
            .ok_or_else(|| "No key 'output.html'".to_owned())?
            .as_table()
            .ok_or_else(|| "'output.html' is not a table".into())
    }

    fn insert_into_output_html(&mut self, entry: &str) -> Result<&mut Array, String> {
        let empty_table = Item::Table(Table::default());
        let empty_array = Item::Value(Value::Array(Array::default()));

        self.entry("output")
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
            .ok_or(format!("Could not insert 'output.html.{}'", entry))
    }

    fn insert_into_preprocessor(&mut self, entry: &str) -> Result<&mut Item, &str> {
        let empty_table = Item::Table(Table::default());
        let table = self
            .entry("preprocessor")
            .or_insert(empty_table.clone())
            .as_table_mut()
            .ok_or("'No table 'preprocessor' found")?
            .entry(entry)
            .or_insert(empty_table);
        Ok(table)
    }
}

pub trait TableExt {
    fn theme(&self) -> Result<&'_ str, &str>;
}

impl TableExt for Table {
    fn theme(&self) -> Result<&'_ str, &str> {
        self.get("theme")
            .and_then(|item| item.as_str())
            .ok_or("'theme' not found")
    }
}

pub trait ArrayExt {
    fn contains_str(&self, value: &str) -> bool;
}

impl ArrayExt for Array {
    fn contains_str(&self, value: &str) -> bool {
        self.iter().any(|element| match element.as_str() {
            None => false,
            Some(element_str) => element_str == value,
        })
    }
}

pub trait ItemExt {
    fn get_assets_version(&self) -> Result<&str, &str>;
}

impl ItemExt for Item {
    fn get_assets_version(&self) -> Result<&str, &str> {
        self.get("assets_version")
            .ok_or("assets_version not found")?
            .as_str()
            .ok_or("assets_version is not a string")
    }
}
