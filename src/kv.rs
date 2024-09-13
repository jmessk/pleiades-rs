use bytes::Bytes;
use std::borrow::Cow;

use crate::{client::Client, feature::id::Id};

pub struct Selector {
    pub(crate) client: Client,
}

impl Selector {
    #[allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]
    pub async fn new(self) -> anyhow::Result<()> {
        todo!()
    }

    #[allow(clippy::wrong_self_convention)]
    pub async fn from_id(self, id: impl Into<Id>) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub(crate) client: Client,
    id: Id,
}

impl Namespace {
    pub fn new_key<'a>(&self, key: impl Into<Cow<'a, str>>) -> Key<'a> {
        Key {
            client: self.client.clone(),
            id: self.id.clone(),
            key: key.into(),
        }
    }
}

#[derive(Debug)]
pub struct Key<'a> {
    pub(crate) client: Client,
    id: Id,
    key: Cow<'a, str>,
}

impl<'a> Key<'a> {
    pub async fn get(self) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }

    pub async fn set(self, value: impl Into<Vec<u8>>) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn delete(self) -> anyhow::Result<()> {
        todo!()
    }
}
