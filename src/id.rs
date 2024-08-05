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

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        Id { id: id.to_string() }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
