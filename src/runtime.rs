use anyhow::Result;

pub struct RuntimeBuilder {
    base: Option<String>,
    features: Option<Vec<String>>,
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            base: None,
            features: None,
        }
    }

    pub fn base(mut self, domain: impl Into<String>) -> RuntimeBuilder {
        self.base = Some(domain.into());
        self
    }

    pub fn add(mut self, feature: impl Into<String>) -> RuntimeBuilder {
        self.features
            .get_or_insert_with(|| Vec::new())
            .push(feature.into());

        self
    }

    pub fn features<T, U>(mut self, features: T) -> RuntimeBuilder
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.features = Some(features.into_iter().map(|f| f.into()).collect());
        self
    }

    pub fn build(mut self) -> Result<Runtime> {
        let base = self
            .base
            .ok_or_else(|| anyhow::anyhow!("base is required"))?;

        let full = if let Some(features) = self.features.as_ref() {
            format!("{}+{}", base, features.join("+"))
        } else {
            base.clone()
        };

        Ok(Runtime {
            base: base,
            features: self.features.unwrap_or_default(),
            full: full,
        })
    }
}

#[derive(Debug)]
pub struct Runtime {
    base: String,
    features: Vec<String>,
    full: String,
}

impl Runtime {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// e.g. "openpose+gpu+cuda+python"
    pub fn full(&self) -> &str {
        &self.full
    }
}
