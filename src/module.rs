use crate::c_interface;
use crate::c_interface::{c_new_module, CModule};
use crate::error::ExecutorchError;
use crate::tensor::Tensor;
use std::ffi::NulError;
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
            Err(ExecutorchError::FailedToLoad(status))
        } else {
            Ok(())
        }
    }

    /// Forward the executorch module.
    pub fn forward(
        &self,
        input: &[f32],
        input_sizes: &[i32],
        output_sizes: &[i32],
    ) -> Result<Tensor, ExecutorchError> {
        let output_numel =
            output_sizes.iter().fold(1, |acc, &x| acc * x as usize);
        let mut found_output_dim = 0;
        let mut found_output_sizes = vec![0; output_numel];
        let mut output = vec![0.0; output_numel];
        let status = unsafe {
            c_interface::c_forward(
                self.c_module,
                input.as_ptr(),
                input_sizes.len() as i32,
                input_sizes.as_ptr(),
                output_sizes.len() as i32,
                output_sizes.as_ptr(),
                &mut found_output_dim,
                found_output_sizes.as_mut_ptr(),
                output.as_mut_ptr(),
            )
        };

        let output_dim = output_sizes.len() as i32;
        match status {
            -1 => Err(ExecutorchError::ShapeMismatch {
                expected: vec![output_dim],
                found: vec![0],
            }),
            -2 => Err(ExecutorchError::ShapeMismatch {
                expected: vec![output_dim],
                found: vec![found_output_dim],
            }),
            -3 => Err(ExecutorchError::ShapeMismatch {
                expected: output_sizes.to_vec(),
                found: found_output_sizes,
            }),
            _ => Ok(Tensor::new(output, found_output_sizes, output_dim)),
        }
    }
}
