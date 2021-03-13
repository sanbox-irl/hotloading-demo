pub struct TraitHolder {
    pub inner: Box<dyn DynTrait>,
}

pub trait DynTrait {
    fn identify(&self) -> u32;

    fn say_hello(&self);
}
