use ruby_sys::class::{rb_define_module,rb_define_singleton_method};
use ruby_sys::types::{Value, SignedValue, CallbackPtr};
use ruby_sys::fixnum;
use ruru::util::str_to_cstring;

extern fn rb_sum(_mod: Value, a :Value, b: Value) -> Value {
    let a = unsafe { fixnum::rb_num2int(a) as i64 };
    let b = unsafe { fixnum::rb_num2int(b) as i64 };
    let sum = a + b;

    unsafe { fixnum::rb_int2inum(sum as SignedValue) }
}

#[no_mangle]
pub extern fn Init_wasabi() {
    unsafe {
        let rb_mod = rb_define_module(str_to_cstring("Wasabi").as_ptr());
        rb_define_singleton_method(rb_mod, str_to_cstring("sum").as_ptr(), rb_sum as CallbackPtr, 2);
    }
}
