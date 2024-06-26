use crate::cleanup_cnl::cleanup_cnl;


#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnPart(String);

impl CnPart {
    pub fn new(cn: String) -> Self {
        Self(cn)
    }
}

impl From<&str> for CnPart {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for CnPart {
    fn from(value: String) -> Self {
        Self(value)
    }
}



impl<T> From<Option<T>> for CnPart where T: Into<CnPart> + Default {
    fn from(value: Option<T>) -> Self {
        value.unwrap_or_default().into()
    }
}


impl From<(bool, &str, &str)> for CnPart {
    fn from(value: (bool, &str, &str)) -> Self {
        match value.0 {
            true => Self(value.1.to_string()),
            false => Self(value.2.to_string()),
        }
    }
}

impl From<(Option<bool>, &str, &str)> for CnPart {
    fn from(value: (Option<bool>, &str, &str)) -> Self {
        match value.0 {
            Some(bool_value) => CnPart::from((bool_value, value.1, value.2)),
            None => CnPart::from((false, value.1, value.2)),
        }
    }
}

impl From<(bool, &str)> for CnPart {
    fn from(value: (bool, &str)) -> Self {
        Self::from((value.0, value.1, ""))
    }
}

impl From<(Option<bool>, &str)> for CnPart {
    fn from(value: (Option<bool>, &str)) -> Self {
        Self::from((value.0, value.1, ""))
    }
}


#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnBuilder(String);

impl CnBuilder {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn add<T>(mut self, item: T) -> Self
    where
        T: Into<CnPart>,
    {
        let item: CnPart = item.into();
        self.0.push_str(&format!(" {}", item.0));

        self
    }

    pub fn to_classlist(&self) -> String {
        cleanup_cnl(&self.0)
    }
}
