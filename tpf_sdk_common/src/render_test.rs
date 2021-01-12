use pyo3::prelude::*;
use remote_trait_object::*;

#[service]
pub trait RenderTest: Service {
    fn renderer(&self) -> ServiceRef<dyn RenderTestRenderer>;
}

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

#[service]
pub trait RenderTestRenderer: Service {
    fn set_probes(&mut self, on: bool);
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
