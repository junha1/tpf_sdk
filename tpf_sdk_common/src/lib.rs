pub mod render_test;

use remote_trait_object::*;
use render_test::*;

#[service]
pub trait Tpf: Service {
    fn render_test(&self) -> Option<ServiceRef<dyn RenderTest>>;
}
