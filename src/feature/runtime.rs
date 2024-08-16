use std::borrow::Cow;

// #[derive(Debug, Clone, typed_builder::TypedBuilder)]
// pub struct Runtime {
//     /// base runtime
//     #[builder(
//         setter(into),
//         mutators(
//             pub fn add_feature<T: Into<Cow<'static, str>>>(&mut self, feature: T) {
//                 self.features.push(feature.into());
//             }
//     ))]
//     base: Cow<'static, str>,

//     /// additional features
//     #[builder(via_mutators(init = Vec::new()))]
//     features: Vec<Cow<'static, str>>,
// }

#[derive(Default)]
pub struct RuntimeBuilder {
    base: Option<Cow<'static, str>>,
    features: Option<Vec<Cow<'static, str>>>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base<T: Into<Cow<'static, str>>>(mut self, base: T) -> Self {
        self.base = Some(base.into());
        self
    }

    pub fn add_feature<T: Into<Cow<'static, str>>>(mut self, feature: T) -> Self {
        self.features
            .get_or_insert_with(Vec::new)
            .push(feature.into());
        self
    }

    pub fn build(self) -> Runtime {
        let base = self.base.expect("base runtime is required");

        match self.features {
            Some(features) => {
                let runtime = format!("{}+{}", base, features.join("+"));
                Runtime(runtime.into())
            }
            None => Runtime(base),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Runtime(Cow<'static, str>);

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for Runtime {
    fn from(base: &'static str) -> Self {
        Self(base.into())
    }
}

impl From<String> for Runtime {
    fn from(base: String) -> Self {
        Self(base.into())
    }
}

// impl Runtime {
//     pub fn base(&self) -> &str {
//         &self.base
//     }

//     pub fn features(&self) -> &Vec<Cow<'static, str>> {
//         &self.features
//     }

//     pub fn full(&self) -> String {
//         format!("{}+{}", self.base, self.features.join("+"))
//     }
// }
