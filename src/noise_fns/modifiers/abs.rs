use crate::{noise_fns::NoiseFn, MultiFractal, Seedable};

/// Noise function that outputs the absolute value of the output value from the
/// source function.
pub struct Abs<T, const DIM: usize>
where
    T: NoiseFn<DIM>,
{
    /// Outputs a value.
    pub source: T,
}

impl<T, const DIM: usize> Abs<T, DIM>
where
    T: NoiseFn<DIM>,
{
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T, const DIM: usize> NoiseFn<DIM> for Abs<T, DIM>
where
    T: NoiseFn<DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source.get(point)).abs()
    }
}

impl<T, const DIM: usize> Seedable for Abs<T, DIM>
where
    T: NoiseFn<DIM> + Seedable,
{
    fn new(seed: u32) -> Self {
        Self {
            source: T::new(seed),
        }
    }

    fn set_seed(self, seed: u32) -> Self {
        Self::new(self.source.set_seed(seed))
    }

    fn seed(&self) -> u32 {
        self.source.seed()
    }
}

impl<T, const DIM: usize> MultiFractal for Abs<T, DIM>
where
    T: NoiseFn<DIM> + MultiFractal,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self::new(self.source.set_octaves(octaves))
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self::new(self.source.set_frequency(frequency))
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self::new(self.source.set_lacunarity(lacunarity))
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self::new(self.source.set_persistence(persistence))
    }
}
