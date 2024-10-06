use executorch_rs::c_interface::*;
use std::ptr;

fn main() {
    unsafe {
        let model_path = "./resource/model/two.pte";
        let c_model_path = std::ffi::CString::new(model_path).unwrap();
        let c_module = c_new_module(c_model_path.as_ptr());
        let status = c_load(c_module);
        assert_eq!(status, 0);

        let input = vec![1.0];
        let ssize = input.len() as i32;
        let mut output_ptr = ptr::null_mut();
        let status = c_forward(
            c_module,
            input.as_ptr(),
            input.len() as i32,
            &ssize as *const i32 as *mut f32,
            &mut output_ptr,
        );
        assert_eq!(status, 0);
        if output_ptr.is_null() {
            panic!("output_ptr is null");
        }
        println!("output: {:?}", *output_ptr);
    }
}
