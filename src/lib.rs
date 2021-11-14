mod client;
mod models;

use pyo3::prelude::*;
use pyo3::Python;

#[pymodule]
fn snekverse(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<client::Supernova>()?;
    m.add_class::<models::Department>()?;
    m.add_class::<models::Building>()?;
    m.add_class::<models::Place>()?;
    m.add_class::<models::Course>()?;
    m.add_class::<models::Class>()?;
    m.add_class::<models::ClassInstance>()?;
    m.add_class::<models::ClassShift>()?;

    Ok(())
}
