#[derive(Debug, Clone)]
pub struct SyntaxError {
    err: Option<syn::Error>,
}

impl SyntaxError {
    pub const fn new() -> Self {
        Self { err: None }
    }

    pub fn push(&mut self, e: syn::Error) {
        if let Some(err) = &mut self.err {
            err.combine(e);
        } else {
            self.err = Some(e);
        }
    }

    pub fn check(self) -> Result<(), syn::Error> {
        match self.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
