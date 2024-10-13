use std::ffi::c_char;

/* ----------------------------------------------------------------------------
* Unsafe C Interface for executorch
------------------------------------------------------------------------------- */
/// CModule is a struct that is defined in C.
/// _private is an opaque field that is used to prevent direct access to the struct fields.
#[repr(C)]
#[derive(Debug)]
pub struct CModule {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CTensor {
    pub error: i32,
    pub data: *mut f32,
    pub dim: i32,
    pub sizes: *mut i32,
}

extern "C" {
    /// Create a new executorch module.
    ///
    /// # Safety
    /// CModule is placed in the heap and the caller is responsible for dropping it.
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
    /// output must be a valid pointer to a f32 array.
    pub fn c_forward(
        ptr: *mut CModule,
        input: *const f32,
        input_dim: i32,
        input_sizes: *const i32,
    ) -> CTensor;
}
