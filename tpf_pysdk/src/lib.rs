use foundry_process_sandbox::ipc::Ipc;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use remote_trait_object::*;
use std::sync::Arc;
use tpf_sdk_common as sdk_common;

#[pyclass]
struct Connection {
    context: Context,
    test_server: Arc<dyn sdk_common::Tpf>,
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.context.disable_garbage_collection()
    }
}

#[pymethods]
impl Connection {
    fn test_server(&self) -> sdk_common::TpfPy {
        sdk_common::TpfPy {
            proxy: self.test_server.clone(),
        }
    }
}

#[pyfunction]
fn create_connection(port: u16) -> PyResult<Connection> {
    let addr = format!("127.0.0.1:{}", port)
        .parse::<std::net::SocketAddr>()
        .unwrap();
    let (send, recv) = foundry_process_sandbox::ipc::stream_socket::Tcp::new_client(addr).split();
    let (context, x): (_, ServiceToImport<dyn sdk_common::Tpf>) =
        Context::with_initial_service_import(Config::default_setup(), send, recv);

    Ok(Connection {
        context,
        test_server: x.into_proxy(),
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn tpf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_connection, m)?)?;
    m.add_class::<Connection>()?;
    m.add_class::<sdk_common::TpfPy>()?;
    m.add_class::<sdk_common::render_test::RenderTestPy>()?;
    m.add_class::<sdk_common::render_test::RenderTestRendererPy>()?;

    Ok(())
}
