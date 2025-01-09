pub mod error;
pub mod transformer;
pub mod validator;

#[cfg(feature = "derive")]
pub use valust_derive as derive;

pub trait Validate<Output> {
    fn validate(self) -> Result<Output, error::ValidationError>;
}

pub trait ValidateFrom<R> {
    fn validate_from(raw: R) -> Result<Self, error::ValidationError>
    where
        Self: Sized;
}

impl<T, U> ValidateFrom<T> for U
where
    T: Validate<U>,
{
    fn validate_from(raw: T) -> Result<Self, error::ValidationError> {
        raw.validate()
    }
}
