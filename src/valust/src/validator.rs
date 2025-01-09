pub trait Validator {
    type Input;

    fn validate(&self, input: Self::Input) -> bool;
}

impl<I> Validator for fn(I) -> bool {
    type Input = I;

    fn validate(&self, input: Self::Input) -> bool {
        self(input)
    }
}

pub trait FallibleValidator {
    type Input;
    type Error: std::error::Error;

    fn validate(&self, input: Self::Input) -> Result<bool, Self::Error>;
}

impl<I, E: std::error::Error> FallibleValidator for fn(I) -> Result<bool, E> {
    type Input = I;
    type Error = E;

    fn validate(&self, input: Self::Input) -> Result<bool, Self::Error> {
        self(input)
    }
}
