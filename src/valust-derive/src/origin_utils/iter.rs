pub trait IteratorExt: Iterator {
    #[allow(unused)]
    fn unzip3<A, B, C, FromA, FromB, FromC>(self) -> (FromA, FromB, FromC)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        FromC: Default + Extend<C>,
        Self: Sized + Iterator<Item = (A, B, C)>;

    fn collect_result_with<T, R: Default>(
        self,
        f: impl FnMut(T, R) -> R,
    ) -> (R, Option<syn::Error>)
    where
        Self: Sized + Iterator<Item = Result<T, syn::Error>>;

    fn collect_result<T>(self) -> (Vec<T>, Option<syn::Error>)
    where
        Self: Sized + Iterator<Item = Result<T, syn::Error>>,
    {
        self.collect_result_with(|i, mut r: Vec<T>| {
            r.push(i);
            r
        })
    }
}

impl<I: Iterator> IteratorExt for I {
    fn unzip3<A, B, C, FromA, FromB, FromC>(self) -> (FromA, FromB, FromC)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        FromC: Default + Extend<C>,
        Self: Sized + Iterator<Item = (A, B, C)>,
    {
        let (mut a, mut b, mut c): (FromA, FromB, FromC) = Default::default();

        fn extend<'a, A, B, C>(
            a: &'a mut impl Extend<A>,
            b: &'a mut impl Extend<B>,
            c: &'a mut impl Extend<C>,
        ) -> impl FnMut((), (A, B, C)) + 'a {
            move |(), (t, u, v)| {
                a.extend(Some(t));
                b.extend(Some(u));
                c.extend(Some(v))
            }
        }

        self.fold((), extend(&mut a, &mut b, &mut c));

        (a, b, c)
    }

    fn collect_result_with<T, R: Default>(
        self,
        mut f: impl FnMut(T, R) -> R,
    ) -> (R, Option<syn::Error>)
    where
        Self: Sized + Iterator<Item = Result<T, syn::Error>>,
    {
        self.fold((R::default(), None), |(o, e), x| match x {
            Ok(ok) => (f(ok, o), e),
            Err(err) => (
                o,
                e.map(|mut e| {
                    e.combine(err);
                    e
                }),
            ),
        })
    }
}
