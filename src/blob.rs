use anyhow::{bail, Context as _, Result};

use super::{Client, MecrmObject, ObjectBuilder};

pub struct Blob {
    id: String,
    data: Option<Vec<u8>>,
}

impl MecrmObject for Blob {}

impl Blob {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub async fn data(&self) -> Result<&Vec<u8>> {
        self.data.as_ref().context("Blob data is not set")
    }
}

pub struct BlobBuilder {
    client: Client,
    id: Option<String>,
    data: Option<Vec<u8>>,
}

impl ObjectBuilder for BlobBuilder {
    type Output = Blob;

    fn new(client: Client) -> BlobBuilder {
        BlobBuilder {
            client,
            id: None,
            data: None,
        }
    }

    // fn id(self, id: impl Into<String>) -> Blob {
    //     Blob {
    //         id: id.into(),
    //         data: None,
    //     }
    // }
}   

impl BlobBuilder {
    pub fn data(mut self, data: impl Into<Vec<u8>>) -> BlobBuilder {
        self.data = Some(data.into());
        self
    }

    pub async fn post(self) -> Result<Blob> {
        if self.data.is_none() {
            bail!("Blob data is not set");
        }

        Ok(Blob {
            id: "1234567890".to_string(),
            data: self.data,
        })
    }
}
