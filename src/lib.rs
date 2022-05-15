use ruby_sys::{class, fixnum, string, util};
use ruby_sys::types::{Value, SignedValue, CallbackPtr, c_long};
use std::ffi::{CStr, CString};
use std::ptr;

fn str_to_cstring(str: &str) -> CString {
    CString::new(str).unwrap()
}

fn value_to_string(value: Value) -> String {
    unsafe {
        let str = string::rb_string_value_cstr(&value);
        CStr::from_ptr(str).to_string_lossy().into_owned()
    }
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

extern fn rb_object_initialize(obj: Value, name: Value) {
    unsafe {
        let name_id = util::rb_intern(str_to_cstring("name").as_ptr());
        class::rb_ivar_set(obj, name_id, name);
    }
    println!("rb_object_initialize: {}", value_to_string(name));
}

unsafe extern fn rb_name(obj: Value) -> Value {
    let name_id = util::rb_intern(str_to_cstring("name").as_ptr());
    let ivar_name = class::rb_ivar_get(obj, name_id);
    let name = value_to_string(ivar_name);
    string::rb_utf8_str_new(str_to_cstring(&name).as_ptr(), name.len() as c_long)
}

unsafe extern fn rb_say(obj: Value) -> Value {
    let name_id = util::rb_intern(str_to_cstring("name").as_ptr());
    let ivar_name = class::rb_ivar_get(obj, name_id);
    let name = value_to_string(ivar_name);
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
        class::rb_define_method(rb_class, str_to_cstring("initialize").as_ptr(), rb_object_initialize as CallbackPtr, 1);
        class::rb_define_method(rb_class, str_to_cstring("name").as_ptr(), rb_name as CallbackPtr, 0);
        class::rb_define_method(rb_class, str_to_cstring("say").as_ptr(), rb_say as CallbackPtr, 0);
    }
}
