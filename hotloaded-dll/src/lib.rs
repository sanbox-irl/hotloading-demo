use center::{DynTrait, TraitHolder};

pub struct DynTraitImpl {
    value: u32,
}

impl DynTrait for DynTraitImpl {
    fn identify(&self) -> u32 {
        self.value
    }

    fn say_hello(&self) {
        println!("Hello loser, get in the {}", self.value);
    }
}

#[no_mangle]
pub extern "C" fn trait_maker() -> *const TraitHolder {
    // make our trait...
    let holder = TraitHolder {
        inner: Box::new(DynTraitImpl { value: 2 }),
    };

    // box it...
    let as_box = Box::new(holder);

    // return it...
    Box::into_raw(as_box)
}
