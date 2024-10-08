/* ----------------------------------------------------------------------------
* Safe Rust Interface for executorch
------------------------------------------------------------------------------- */

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub sizes: Vec<i32>,
    pub dim: i32,
}

impl Tensor {
    pub fn new(data: Vec<f32>, sizes: Vec<i32>, dim: i32) -> Self {
        Self { data, sizes, dim }
    }
}
