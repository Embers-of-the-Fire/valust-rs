pub trait Transformer {
    type Input;
    type Output;

    fn transform(&self, input: Self::Input) -> Self::Output;
}

impl<I, O> Transformer for fn(I) -> O {
    type Input = I;
    type Output = O;

    fn transform(&self, input: Self::Input) -> Self::Output {
        self(input)
    }
}

pub trait FallibleTransformer {
    type Input;
    type Output;
    type Error;

    fn transform(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

impl<I, O, E> FallibleTransformer for fn(I) -> Result<O, E> {
    type Input = I;
    type Output = O;
    type Error = E;

    fn transform(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        self(input)
    }
}
