use magnus::{define_module, function, method, prelude::*, Error, Value, RString};
use rb_allocator::ruby_global_allocator;

ruby_global_allocator!();

fn sum(a: i64, b: i64) -> i64 {
    a + b
}

fn call_to_s(obj: Value) -> RString {
    obj.to_r_string().unwrap()
}

#[magnus::wrap(class = "Wasabi::Object")]
struct WasabiObject {
    name: String
}

impl WasabiObject {
    fn new(name: String) -> Self {
        WasabiObject { name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn say(&self) -> String {
        format!("say, {}", self.name)
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Wasabi")?;
    module.define_singleton_method("sum", function!(sum, 2))?;
    module.define_singleton_method("call_to_s", function!(call_to_s, 1))?;

    let class = module.define_class("Object", Default::default())?;
    class.define_singleton_method("new", function!(WasabiObject::new, 1))?;
    class.define_method("name", method!(WasabiObject::name, 0))?;
    class.define_method("say", method!(WasabiObject::say, 0))?;
    Ok(())
}
