use toml_edit::{Array, Table};

pub(crate) trait TableExt {
    fn additional_css(&self) -> Result<&Array, &str>;
    fn additional_js(&self) -> Result<&Array, &str>;
}

impl TableExt for Table {
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
    /// # Errors
    ///
    /// This function will return an error if catppuccin.css does not found
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
