use std::str::FromStr;

pub fn parse_to<F: FromStr>(s: impl AsRef<str>) -> Result<F, F::Err> {
    s.as_ref().parse::<F>()
}

pub fn into<F: Into<T>, T>(f: F) -> T {
    f.into()
}

pub fn try_into<F: TryInto<T>, T>(f: F) -> Result<T, F::Error> {
    f.try_into()
}
