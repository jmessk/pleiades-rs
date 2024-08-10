use std::borrow::Cow;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Id {
    pub id: Cow<'static, str>,
}

impl Id {
    pub fn as_str(&self) -> &str {
        &self.id
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Id { id: Cow::Owned(id) }
    }
}

impl From<&'static str> for Id {
    fn from(id: &'static str) -> Self {
        Id {
            id: Cow::Borrowed(id),
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
