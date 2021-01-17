use remote_trait_object::*;

#[service]
pub trait RenderTest: Service {
    fn renderer(&self) -> ServiceRef<dyn RenderTestRenderer>;
}

#[service]
pub trait RenderTestRenderer: Service {
    fn set_probes(&mut self, on: bool);
}
