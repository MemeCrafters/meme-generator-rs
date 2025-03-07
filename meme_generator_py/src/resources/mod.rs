use pyo3::prelude::*;

use meme_generator::resources;

pub(crate) fn register_resources_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "resources")?;
    m.add_function(wrap_pyfunction!(check_resources, &m)?)?;
    m.add_function(wrap_pyfunction!(check_resources_in_background, &m)?)?;
    parent_module.add_submodule(&m)?;
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("meme_generator.resources", m)
    })?;
    Ok(())
}

#[pyfunction]
fn check_resources() {
    resources::check_resources_sync(None);
}

#[pyfunction]
fn check_resources_in_background() {
    resources::check_resources_in_background(None);
}
