pub mod common {
    use std::cell::RefCell;
    use std::path::Path;

    use crate::epub::Epub;
    pub trait File<T> {
        fn unzip(&self, path: &Path) -> Epub;

        fn merge(&self, data: RefCell<Option<T>>);
    }
}
