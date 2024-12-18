use std::fmt::Display;

use algo::matrix::{self, Matrix};
use pyo3::prelude::*;

#[pyclass(name = "Matrix")]
pub struct PyMatrix {
    pub inner: Matrix<f64>,
}

impl Display for PyMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix: {}", self.inner)
    }
}

#[pymethods]
impl PyMatrix {
    #[new]
    fn new(inner: Vec<Vec<f64>>) -> Self {
        let row = inner.len();
        let col = inner[0].len();
        let data: Vec<f64> = inner.into_iter().flatten().collect();
        let inner = Matrix::new(data, row, col);
        Self { inner }
    }

    fn __repr__(&self) -> String {
        format!("{}", self)
    }

    fn __str__(&self) -> String {
        format!("{}", self)
    }
}

#[pyfunction]
fn multiply(data1: PyRef<PyMatrix>, data2: PyRef<PyMatrix>) -> pyo3::PyResult<PyMatrix> {
    let res = matrix::multiply(&data1.inner, &data2.inner).unwrap();
    Ok(PyMatrix { inner: res })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_class::<PyMatrix>()?;
    Ok(())
}
