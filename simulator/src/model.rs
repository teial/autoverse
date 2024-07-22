use ndarray::Array2;

pub mod lenia;

pub trait Model {
    fn kernel(&self) -> &Array2<f64>;
}
