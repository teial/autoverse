use crate::Error;
use ndarray::Array2;
use ndarray_conv::{ConvFFTExt, ConvMode, PaddingMode};

pub struct Grid {
    grid: Array2<f64>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Array2::zeros((width, height)),
        }
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.grid[[x, y]]
    }

    pub fn update(&mut self, kernel: &Array2<f64>) -> Result<(), Error> {
        let convolved = self
            .grid
            .conv_fft(kernel, ConvMode::Same, PaddingMode::Zeros)
            .map_err(|_| Error::FFTFailure)?;
        let activated = convolved.mapv(|x| 1.0 / (1.0 + (-x).exp()));
        self.grid = 0.5 * &self.grid + 0.5 * &activated;
        Ok(())
    }
}
