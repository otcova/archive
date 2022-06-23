pub trait MeanExt: Iterator {
    fn mean<M>(self) -> M
    where
        M: Mean<Self::Item>,
        Self: Sized,
    {
        M::mean(self)
    }
}

impl<I: Iterator> MeanExt for I {}

pub trait Mean<A = Self> {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = A>;
}

impl Mean for f64 {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = f64>,
    {
        let mut sum = 0.0;
        let mut count: usize = 0;

        for v in iter {
            sum += v;
            count += 1;
        }

        if count > 0 {
            sum / (count as f64)
        } else {
            0.0
        }
    }
}

impl<'a> Mean<&'a f64> for f64 {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a f64>,
    {
        iter.copied().mean()
    }
}

impl Mean for f32 {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = f32>,
    {
        let mut sum = 0.0;
        let mut count: usize = 0;

        for v in iter {
            sum += v;
            count += 1;
        }

        if count > 0 {
            sum / (count as f32)
        } else {
            0.0
        }
    }
}

impl<'a> Mean<&'a f32> for f32 {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a f32>,
    {
        iter.copied().mean()
    }
}

#[cfg(test)]
mod test {
    use super::MeanExt;

    #[test]
    fn use_case() {
        assert_eq!(2., [1., 2., 3.].iter().mean::<f64>());
        assert_eq!(2. / 3., [-1., 2., 1.].iter().mean::<f32>());
    }
}
