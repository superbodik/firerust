use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::tree::document::Document;

#[derive(Debug, Clone)]
pub struct SharedDocument {
    inner: Rc<RefCell<Document>>,
}

impl SharedDocument {
    pub fn new(document: Document) -> Self {
        Self {
            inner: Rc::new(RefCell::new(document)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, Document> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Document> {
        self.inner.borrow_mut()
    }

    pub fn revision(&self) -> u64 {
        self.inner.borrow().revision()
    }

    pub fn is_same(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}
