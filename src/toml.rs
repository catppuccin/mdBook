use toml_edit::{Array, Document, Item, Table, Value};

pub trait DocumentExt {
    fn get_or_insert_into_output_html_mut(&mut self, entry: &str) -> Result<&mut Array, String>;
    fn get_or_insert_into_preprocessor_mut(&mut self, entry: &str) -> Result<&mut Item, &str>;
}

impl DocumentExt for Document {
    fn get_or_insert_into_output_html_mut(&mut self, entry: &str) -> Result<&mut Array, String> {
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
            .ok_or(format!("Could not insert 'output.html.{entry}'"))
    }

    fn get_or_insert_into_preprocessor_mut(&mut self, entry: &str) -> Result<&mut Item, &str> {
        let empty_table = Item::Table(Table::default());
        let table = self.entry("preprocessor")
            .or_insert(empty_table.clone())
            .as_table_mut()
            .ok_or("'No table 'preprocessor' found")?
            .entry(entry)
            .or_insert(empty_table);
        Ok(table)
    }
}

pub(crate) trait TableExt {
    fn contains(&self, key: &str) -> bool;
    fn additional_css(&self) -> Result<&Array, &str>;
    fn additional_js(&self) -> Result<&Array, &str>;
}

impl TableExt for Table {
    fn contains(&self, item: &str) -> bool {
        self.iter().any(|(key, _)| key == item)
    }

    fn additional_css(&self) -> Result<&Array, &str> {
        self.get("additional-css")
            .and_then(|item| item.as_array())
            .ok_or("'additional-css' not found")
    }

    fn additional_js(&self) -> Result<&Array, &str> {
        self.get("additional-js")
            .and_then(|item| item.as_array())
            .ok_or("'additional-js' not found")
    }
}

pub trait ArrayExt {
    fn contains_str(&self, value: &str) -> bool;

    /// Returns `Option<&str>` with the `&str` that contains the given value
    ///
    /// # Safety
    ///
    /// Rust implements short circuiting for logical operators. Therefore, we can be
    /// confident in calling _unwrap_unchecked()_ as the item is guaranteed to be a &str
    unsafe fn get_str_if_contains(&self, entry: &str) -> Option<&str>;
}

impl ArrayExt for Array {
    fn contains_str(&self, value: &str) -> bool {
        self.iter().any(|element| match element.as_str() {
            None => false,
            Some(element_str) => element_str == value,
        })
    }

    unsafe fn get_str_if_contains(&self, value: &str) -> Option<&str> {
        self.iter()
            .find(|element| element.is_str() && element.as_str().unwrap_unchecked().contains(value))
            .and_then(|str| str.as_str())
    }
}
