use std::ffi::c_char;

/// CModule is a struct that is defined in C.
/// _private is an opaque field that is used to prevent direct access to the struct fields.
#[repr(C)]
pub struct CModule {
    _private: [u8; 0],
}

extern "C" {
    pub fn c_new_module(file_path: *const c_char) -> *mut CModule;
    pub fn c_drop_module(ptr: *mut CModule);
    pub fn c_load(ptr: *mut CModule) -> i32;
    pub fn c_forward(
        ptr: *mut CModule,
        input: *const f32,
        dim: i32,
        sizes: *mut f32,
        output: *mut *mut f32,
    ) -> i32;
}
