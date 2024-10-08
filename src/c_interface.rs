use std::ffi::c_char;

/// CModule is a struct that is defined in C.
/// _private is an opaque field that is used to prevent direct access to the struct fields.
#[repr(C)]
pub struct CModule {
    _private: [u8; 0],
}

extern "C" {
    ///
    pub fn c_new_module(file_path: *const c_char) -> *mut CModule;

    /// Drop the executorch module.
    ///
    /// # Safety
    /// ptr must be a valid pointer to a CModule.
    pub fn c_drop_module(ptr: *mut CModule);

    /// Force load the executorch module.
    ///
    /// # Safety
    /// ptr must be a valid pointer to a CModule.
    pub fn c_load(ptr: *mut CModule) -> i32;

    /// Forward the executorch module.
    ///
    /// # Safety
    /// ptr must be a valid pointer to a CModule.
    /// input must be a valid pointer to a f32 array.
    /// sizes must be a valid pointer to a f32 array and the length of the array must be equal to dim.
    /// sizes represents the shape of the input tensor.
    /// output must be a valid pointer to a f32 array and the output must be dropped by the caller.
    pub fn c_forward(
        ptr: *mut CModule,
        input: *const f32,
        dim: i32,
        sizes: *mut f32,
        output: *mut *mut f32,
    ) -> i32;
}
