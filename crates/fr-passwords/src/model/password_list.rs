use crate::model::change_kind::ChangeKind;
use crate::model::credential::Credential;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PasswordList {
    items: Vec<Credential>,
}

impl PasswordList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[Credential] {
        &self.items
    }

    pub fn matching(&self, origin: &str) -> Vec<&Credential> {
        self.items
            .iter()
            .filter(|item| item.origin == origin)
            .collect()
    }

    pub fn save(&mut self, credential: Credential) -> ChangeKind {
        if credential.origin.is_empty() || credential.password.is_empty() {
            return ChangeKind::Unchanged;
        }
        let existing = self
            .items
            .iter_mut()
            .find(|item| item.origin == credential.origin && item.username == credential.username);
        match existing {
            Some(item) if item.password == credential.password => ChangeKind::Unchanged,
            Some(item) => {
                item.password = credential.password;
                ChangeKind::Updated
            }
            None => {
                self.items.push(credential);
                ChangeKind::Added
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if index >= self.items.len() {
            return false;
        }
        self.items.remove(index);
        true
    }
}
