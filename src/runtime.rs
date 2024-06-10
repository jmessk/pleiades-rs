use anyhow::Result;

pub struct Runtime {
    domain: Option<String>,
    features: Vec<String>,
}

impl Runtime {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }
}

pub struct RuntimeBuilder {
    domain: Option<String>,
    features: Option<Vec<String>>,
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            domain: None,
            features: None,
        }
    }

    pub fn build(self) -> Result<Runtime> {
        Ok(Runtime {
            domain: self.domain,
            features: self.features.unwrap_or_default(),
        })
    }

    pub fn domain(mut self, domain: impl Into<String>) -> RuntimeBuilder {
        self.domain = Some(domain.into());
        self
    }

    pub fn add(mut self, feature: impl Into<String>) -> RuntimeBuilder {
        self.features
            .get_or_insert_with(Vec::new)
            .push(feature.into());

        self
    }

    pub fn features<T, U>(mut self, features: T) -> RuntimeBuilder
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.features = Some(features.into_iter().map(Into::into).collect());
        self
    }
}
