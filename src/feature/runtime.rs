use std::borrow::Cow;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Runtime {
    /// base runtime
    #[builder(
        setter(into),
        mutators(
            pub fn add_feature<T: Into<Cow<'static, str>>>(&mut self, feature: T) {
                self.features.push(feature.into());
            }
    ))]
    base: Cow<'static, str>,

    /// additional features
    #[builder(via_mutators(init = Vec::new()))]
    features: Vec<Cow<'static, str>>,
}

impl Runtime {
    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn features(&self) -> &Vec<Cow<'static, str>> {
        &self.features
    }

    pub fn full(&self) -> String {
        format!("{}+{}", self.base, self.features.join("+"))
    }
}
