use ruby_sys::class::rb_define_module;
use ruru::util::str_to_cstring;

#[no_mangle]
pub extern fn Init_wasabi() {
    unsafe {
        rb_define_module(str_to_cstring("Wasabi").as_ptr());
    }
}
