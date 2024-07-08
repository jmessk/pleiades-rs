#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Id {
    pub id: String,
}

impl Id {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Id { id }
    }
}
