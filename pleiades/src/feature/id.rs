use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Id(pub Cow<'static, str>);

impl Id {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Id(Cow::Owned(id))
    }
}

impl From<&'static str> for Id {
    fn from(id: &'static str) -> Self {
        Id(Cow::Borrowed(id))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
