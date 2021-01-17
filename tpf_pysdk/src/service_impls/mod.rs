pub mod render_test;

use pyo3::prelude::*;
use render_test::*;
use std::sync::Arc;
use tpf_sdk_common::*;

#[pyclass]
pub struct TpfPy {
    // This is the initial serviec, and designed to be shared all the time.
    pub proxy: Arc<dyn Tpf>,
}

#[pymethods]
impl TpfPy {
    pub fn render_test(&self) -> Option<RenderTestPy> {
        self.proxy.render_test().map(|x| RenderTestPy {
            proxy: x.into_object(),
        })
    }
}
