use center::*;
use libloading::Library;

fn main() {
    let mut line = String::new();
    let stdin = std::io::stdin();

    loop {
        load_and_run_library();

        stdin.read_line(&mut line).unwrap();
    }
}

fn load_and_run_library() {
    unsafe {
        let lib = Library::new("target/debug/hotloaded_dll.dll").unwrap();
        //     let awesome_function: Symbol<unsafe extern fn(f64) -> f64> =
        //     lib.get(b"awesome_function\0").unwrap();
        // awesome_function(0.42);

        let f = lib
            .get::<extern "C" fn() -> *const TraitHolder>(b"trait_maker\0")
            .unwrap();

        let trait_holder: TraitHolder = *Box::from_raw(f() as *mut TraitHolder);
        let id = trait_holder.inner.identify();
        println!("Id was {}", id);
        trait_holder.inner.say_hello();
    };
}
