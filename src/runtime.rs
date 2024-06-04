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
    inner: Runtime,
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            inner: Runtime {
                domain: None,
                features: Vec::new(),
            },
        }
    }

    pub fn build(self) -> Runtime {
        self.inner
    }

    pub fn domain(mut self, domain: impl Into<String>) -> RuntimeBuilder {
        self.inner.domain = Some(domain.into());
        self
    }

    pub fn add(mut self, feature: impl Into<String>) -> RuntimeBuilder {
        self.inner.features.push(feature.into());
        self
    }

    pub fn features<T, U>(mut self, features: T) -> RuntimeBuilder
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.inner.features = features.into_iter().map(Into::into).collect();
        self
    }
}
