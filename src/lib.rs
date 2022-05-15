use ruby_sys::{class, fixnum, string, util};
use ruby_sys::types::{Value, SignedValue, CallbackPtr, c_long};
use std::ffi::CString;
use std::ptr;

fn str_to_cstring(str: &str) -> CString {
    CString::new(str).unwrap()
}

extern fn rb_sum(_mod: Value, a :Value, b: Value) -> Value {
    let a = unsafe { fixnum::rb_num2int(a) as i64 };
    let b = unsafe { fixnum::rb_num2int(b) as i64 };
    let sum = a + b;

    unsafe { fixnum::rb_int2inum(sum as SignedValue) }
}

extern fn rb_call_to_s(_mod: Value, obj: Value) -> Value {
    unsafe {
        let method_id = util::rb_intern(str_to_cstring("to_s").as_ptr());
        util::rb_funcallv(obj, method_id, 0, ptr::null())
    }
}

extern {
    static rb_cObject: Value;
}

unsafe extern fn rb_name(obj: Value) -> Value {
    let name = "foo".to_string(); // TODO: use instance variables
    string::rb_utf8_str_new(str_to_cstring(&name).as_ptr(), name.len() as c_long)
}

unsafe extern fn rb_say(obj: Value) -> Value {
    let name = "foo".to_string(); // TODO: use instance variables
    let message = format!("say, {}", name);
    string::rb_utf8_str_new(str_to_cstring(&message).as_ptr(), message.len() as c_long)
}

#[no_mangle]
pub extern fn Init_wasabi() {
    unsafe {
        let rb_mod = class::rb_define_module(str_to_cstring("Wasabi").as_ptr());
        class::rb_define_singleton_method(rb_mod, str_to_cstring("sum").as_ptr(), rb_sum as CallbackPtr, 2);
        class::rb_define_singleton_method(rb_mod, str_to_cstring("call_to_s").as_ptr(), rb_call_to_s as CallbackPtr, 1);
        let rb_class = class::rb_define_class_under(rb_mod, str_to_cstring("Object").as_ptr(), rb_cObject);
        class::rb_define_method(rb_class, str_to_cstring("name").as_ptr(), rb_name as CallbackPtr, 0);
        class::rb_define_method(rb_class, str_to_cstring("say").as_ptr(), rb_say as CallbackPtr, 0);
    }
}
