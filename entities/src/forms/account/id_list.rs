use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::AccountId;

#[derive(Debug, Default, Serialize, Clone)]
pub struct IdList {
    id: Vec<AccountId>,
}

impl IdList {
    pub fn empty() -> Self {
        Self { id: vec![] }
    }
}

impl From<Vec<AccountId>> for IdList {
    fn from(list: Vec<AccountId>) -> Self {
        IdList { id: list }
    }
}

impl Deref for IdList {
    type Target = Vec<AccountId>;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl DerefMut for IdList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}
