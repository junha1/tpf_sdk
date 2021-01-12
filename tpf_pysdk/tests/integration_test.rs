/*
use foundry_process_sandbox::ipc::Ipc;
use remote_trait_object::*;
use tpf_sdk_common::{Tpf, render_test::{RenderTest, RenderTestRenderer}};

struct TestTpf;
impl Service for TestTpf {}
impl Tpf for TestTpf {
    fn render_test(&self) -> Option<ServiceRef<dyn RenderTest>> {
        Some(ServiceRef::create_export(Box::new(TestRenderTest) as Box<dyn RenderTest>))
    }
}

struct TestRenderTest;
impl Service for TestRenderTest {}
impl RenderTest for TestRenderTest {
    fn renderer(&self) -> ServiceRef<dyn RenderTestRenderer> {
        ServiceRef::create_export(Box::new(TestRenderTestRenderer) as Box<dyn RenderTestRenderer>)
    }
}

struct TestRenderTestRenderer;
impl Service for TestRenderTestRenderer {}
impl RenderTestRenderer for TestRenderTestRenderer {
    fn set_probes(&mut self, on: bool) {
        println!("set_probes() / on: {}", on)
    }
}

fn run_server() {
    let addr = "127.0.0.1:4444".parse::<std::net::SocketAddr>().unwrap();
    loop {
        let (send, recv) =
            foundry_process_sandbox::ipc::stream_socket::Tcp::new_server(addr).split();
        let ctx = Context::with_initial_service_export(
            Config::default_setup(),
            send,
            recv,
            ServiceToExport::new(Box::new(TestTpf) as Box<dyn Tpf>),
        );
        ctx.wait(None).unwrap();
        println!("SERVER END");
    }
}

#[test]
fn hello() {
    std::thread::spawn(|| run_server());
    let addr = "127.0.0.1:4444".parse::<std::net::SocketAddr>().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    for _ in 0..10 {
        let (send, recv) =
            foundry_process_sandbox::ipc::stream_socket::Tcp::new_client(addr).split();

        let (ctx, x): (_, ServiceToImport<dyn Tpf>) =
            Context::with_initial_service_import(Config::default_setup(), send, recv);
        let x: Box<dyn Tpf> = x.into_proxy();
        let y: Box<dyn RenderTest> = x.render_test().unwrap().into_object();
        let mut z: Box<dyn RenderTestRenderer> = y.renderer().into_object();
        z.set_probes(true);
        drop(x);
        drop(ctx);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[test]
fn just() {
    run_server()
}

*/
