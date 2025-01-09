use std::fmt::{Debug, Display, Formatter};

pub trait ErrorShow: Debug + Display {}

impl<T: Debug + Display> ErrorShow for T {}

#[derive(Debug, Default)]
pub struct ValidationError {
    pub validates: Vec<ValidateError>,
    pub transforms: Vec<TransformError>,
}

impl ValidationError {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(self) -> Result<(), ValidationError> {
        if self.validates.is_empty() && self.transforms.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub fn push_validate_error(&mut self, err: ValidateError) {
        self.validates.push(err);
    }

    pub fn push_transform_error(&mut self, err: TransformError) {
        self.transforms.push(err);
    }

    pub fn extend_error(&mut self, parent: &str, rhs: Self) {
        self.validates
            .extend(rhs.validates.into_iter().map(|mut x| {
                x.path = format!("{}.{}", parent, x.path);
                x
            }));
        self.transforms
            .extend(rhs.transforms.into_iter().map(|mut x| {
                x.path = format!("{}.{}", parent, x.path);
                x
            }));
    }
}

#[derive(Debug)]
pub struct ValidateError {
    pub field: &'static str,
    pub path: String,
    pub value: String,
    pub cause: Box<dyn ErrorShow + 'static>,
    pub message: Option<&'static str>,
    pub expression: &'static str,
    pub type_name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ValidateFail;

impl Display for ValidateFail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "validate expression evaluate to `false`")
    }
}

#[derive(Debug)]
pub struct TransformError {
    pub field: &'static str,
    pub path: String,
    pub value: String,
    pub cause: Box<dyn ErrorShow + 'static>,
    pub message: Option<&'static str>,
    pub expression: &'static str,
    pub source_type_name: &'static str,
    pub target_type_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct CustomError(pub &'static str);

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
