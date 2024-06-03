pub struct RuntimeBuilder {
    runtime: Runtime,
}

impl RuntimeBuilder {
    pub fn domain(domain: impl Into<String>) -> RuntimeBuilder {
        RuntimeBuilder {
            runtime: Runtime {
                domain: domain.into(),
                features: Vec::new(),
            },
        }
    }

    pub fn add(mut self, feature: impl Into<String>) -> RuntimeBuilder {
        self.runtime.features.push(feature.into());
        self
    }

    pub fn features<T: IntoIterator<Item = impl Into<String>>>(
        mut self,
        features: T,
    ) -> RuntimeBuilder {
        self.runtime.features = features.into_iter().map(Into::into).collect();
        self
    }
}

pub struct Runtime {
    domain: String,
    features: Vec<String>,
}

impl Runtime {
    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }
}
