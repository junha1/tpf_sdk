use pyo3::prelude::*;
use tpf_sdk_common::render_test::*;

#[pyclass]
pub struct RenderTestPy {
    pub proxy: Box<dyn RenderTest>,
}

#[pymethods]
impl RenderTestPy {
    pub fn renderer(&self) -> RenderTestRendererPy {
        RenderTestRendererPy {
            proxy: self.proxy.renderer().into_object(),
        }
    }
}

#[pyclass]
pub struct RenderTestRendererPy {
    proxy: Box<dyn RenderTestRenderer>,
}

#[pymethods]
impl RenderTestRendererPy {
    pub fn set_probes(&mut self, on: bool) {
        self.proxy.set_probes(on);
    }
}
