pub mod common {
    use std::cell::RefCell;
    use std::path::Path;
    pub trait File<T> {
        fn unzip(&self, path: &Path) -> Vec<String>;

        fn merge(&self, data: RefCell<Option<T>>);
    }
}
