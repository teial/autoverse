use ndarray::Array2;

use super::Model;

pub struct Lenia {
    kernel: Array2<f64>,
}

impl Lenia {
    pub fn new(size: usize, sigma: f64) -> Self {
        let mut kernel = Array2::<f64>::zeros((size, size));
        let center = size as isize / 2;
        let sigma2 = 2.0 * sigma * sigma;
        for i in 0..size {
            for j in 0..size {
                let x = i as isize - center;
                let y = j as isize - center;
                kernel[[i, j]] = (-((x * x + y * y) as f64) / sigma2).exp();
            }
        }
        Self { kernel }
    }
}

impl Model for Lenia {
    fn kernel(&self) -> &Array2<f64> {
        &self.kernel
    }
}
