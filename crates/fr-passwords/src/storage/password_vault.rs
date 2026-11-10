use std::path::PathBuf;

use crate::model::change_kind::ChangeKind;
use crate::model::credential::Credential;
use crate::model::password_list::PasswordList;
use crate::storage::load_vault::load_vault;
use crate::storage::save_vault::save_vault;
use crate::storage::vault_path::vault_path;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ImportCounts {
    pub found: usize,
    pub added: usize,
    pub updated: usize,
}

#[derive(Debug, Clone, Default)]
pub struct PasswordVault {
    list: PasswordList,
    path: Option<PathBuf>,
}

impl PasswordVault {
    pub fn open() -> Self {
        Self::at(vault_path())
    }

    pub fn at(path: Option<PathBuf>) -> Self {
        let list = match &path {
            Some(path) => load_vault(path),
            None => PasswordList::new(),
        };
        Self { list, path }
    }

    pub fn list(&self) -> &PasswordList {
        &self.list
    }

    pub fn matching(&self, origin: &str) -> Vec<&Credential> {
        self.list.matching(origin)
    }

    pub fn save_login(&mut self, credential: Credential) -> ChangeKind {
        let change = self.list.save(credential);
        if change != ChangeKind::Unchanged {
            self.persist();
        }
        change
    }

    pub fn remove(&mut self, index: usize) -> bool {
        let removed = self.list.remove(index);
        if removed {
            self.persist();
        }
        removed
    }

    pub fn import(&mut self, incoming: Vec<Credential>) -> ImportCounts {
        let mut counts = ImportCounts {
            found: incoming.len(),
            ..ImportCounts::default()
        };
        for credential in incoming {
            match self.list.save(credential) {
                ChangeKind::Added => counts.added += 1,
                ChangeKind::Updated => counts.updated += 1,
                ChangeKind::Unchanged => {}
            }
        }
        if counts.added + counts.updated > 0 {
            self.persist();
        }
        counts
    }

    fn persist(&self) {
        let Some(path) = &self.path else {
            return;
        };
        if let Err(error) = save_vault(path, &self.list) {
            eprintln!("cannot save passwords to {}: {error}", path.display());
        }
    }
}
