pub trait WeightedMeanExt: Iterator {
    fn weighted_mean<M>(self) -> M
    where
        M: WeightedMean<Self::Item>,
        Self: Sized,
    {
        M::weighted_mean(self)
    }
}

impl<I: Iterator> WeightedMeanExt for I {}

pub trait WeightedMean<B, A = Self> {
    fn weighted_mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = B>;
}

impl WeightedMean<(f32, f32)> for f32 {
    fn weighted_mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = (Self, Self)>,
    {
        let mut sum = 0.;
        let mut count = 0.;

        for (value, weight) in iter {
            sum += value * weight;
            count += weight;
        }

        if count != 0. {
            sum / count
        } else {
            0.0
        }
    }
}

impl<'a> WeightedMean<&'a (f32, f32)> for f32 {
    fn weighted_mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a (Self, Self)>,
    {
        iter.copied().weighted_mean()
    }
}

#[cfg(test)]
mod test {
    use super::WeightedMeanExt;

    #[test]
    fn use_case() {
		let data = [(1., 1.), (2., 1.), (3., 1.)];
        assert_eq!(2., data.iter().weighted_mean::<f32>());
		
		let data = [(25., 30.), (-12., 45.), (4., 25.)];
        assert_eq!(3.1, data.iter().weighted_mean::<f32>());
		
		let data = [Some((-1., 2.)), Some((2., 1.)), None, Some((1., 2.))];
		let iter = data.into_iter().flatten();
        assert_eq!(0.4, iter.weighted_mean::<f32>());
    }
}
