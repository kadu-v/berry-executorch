use crate::c_interface;
use crate::c_interface::{c_new_module, CModule};
use crate::error::ExecutorchError;
use crate::tensor::Tensor;
use std::alloc::{dealloc, Layout};
use std::ffi::NulError;
use std::mem;
use std::result::Result;

/// Executorch module.
#[derive(Debug)]
pub struct Module {
    c_module: *mut CModule,
}

impl Module {
    /// Create a new executorch module.
    pub fn new(file_path: &str) -> Result<Self, NulError> {
        let c_file_path = std::ffi::CString::new(file_path)?;
        let c_module = unsafe { c_new_module(c_file_path.as_ptr()) };
        Ok(Self { c_module })
    }

    /// Load the executorch module.
    pub fn load(&mut self) -> Result<(), ExecutorchError> {
        let status = unsafe { c_interface::c_load(self.c_module) };
        if status != 0 {
            let error = ExecutorchError::from_i32(status);
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Forward the executorch module.
    pub fn forward(
        &self,
        input: &[f32],
        input_sizes: &[i32],
    ) -> Result<Tensor, ExecutorchError> {
        let c_tensor = unsafe {
            c_interface::c_forward(
                self.c_module,
                input.as_ptr(),
                input_sizes.len() as i32,
                input_sizes.as_ptr(),
            )
        };

        if c_tensor.error != 0 {
            let error = ExecutorchError::from_i32(c_tensor.error);
            return Err(error);
        }

        let dim = c_tensor.dim as usize;
        let sizes = unsafe { std::slice::from_raw_parts(c_tensor.sizes, dim) };
        let len = sizes.iter().product::<i32>() as usize;
        let data =
            unsafe { std::slice::from_raw_parts_mut(c_tensor.data, len) };

        // Copy the output tensor data to a new Vec<f32>.
        // TODO: hava a better way to avoid copying the data.
        let tensor = Tensor::new(data.to_vec(), sizes.to_vec(), dim as i32);

        // Drop the output tensor.
        unsafe {
            let data_layout = Layout::from_size_align_unchecked(
                mem::size_of::<f32>() * len,
                mem::align_of::<f32>(),
            );
            dealloc(c_tensor.data as *mut u8, data_layout);

            let sizes_layout = Layout::from_size_align_unchecked(
                mem::size_of::<i32>() * dim,
                mem::align_of::<i32>(),
            );
            dealloc(c_tensor.sizes as *mut u8, sizes_layout);
        }

        return Ok(tensor);
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            c_interface::c_drop_module(self.c_module);
        }
    }
}
