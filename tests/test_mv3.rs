use imageproc;
use imageproc::image;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn softmax(x: &Vec<f32>) -> Vec<f32> {
    let max_x = x.iter().fold(f32::NEG_INFINITY, |acc, &x| acc.max(x));
    let exp_x = x.iter().map(|x| (x - max_x).exp()).collect::<Vec<f32>>();
    let sum_exp_x = exp_x.iter().sum::<f32>();
    exp_x.iter().map(|x| x / sum_exp_x).collect::<Vec<f32>>()
}

#[allow(dead_code)]
fn argmax(x: &Vec<f32>) -> usize {
    let mut max_index = 0;
    let mut max_value = f32::NEG_INFINITY;
    for (i, &value) in x.iter().enumerate() {
        if value > max_value {
            max_index = i;
            max_value = value;
        }
    }
    max_index
}

#[allow(dead_code)]
fn forward_model(model_path: &str, expected_class: usize, expected_str: &str) {
    let base_path = dinghy_test::test_project_path();

    // Load image
    let image_path = base_path.join("resource/image/dog.png");
    let image = image::open(image_path).unwrap();
    let image = image.to_rgb8();

    // Resize image
    let input_image_width = 224;
    let input_image_height = 224;
    let resized_image = image::imageops::resize(
        &image,
        input_image_width,
        input_image_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Normalize image
    let mean = [0.485, 0.456, 0.406];
    let std = [0.229, 0.224, 0.225];
    let raw_resized_image = resized_image.into_raw();
    let mut normalized_image = vec![0.0; raw_resized_image.len()];
    let pixel_count = raw_resized_image.len() / 3;
    // HWC -> CHW
    for i in 0..pixel_count {
        for c in 0..3 {
            normalized_image[c * pixel_count + i] =
                ((raw_resized_image[i * 3 + c] as f32 / 255.0 - mean[c])
                    / std[c]) as f32;
        }
    }

    let model_path = base_path.join(model_path);
    let mut module =
        berry_executorch::Module::new(&model_path.display().to_string())
            .expect("Failed to create a new module");
    module.load().unwrap();

    // Forward model
    let input_sizes =
        vec![1, 3, input_image_height as i32, input_image_width as i32];
    let output = module
        .forward(&normalized_image, &input_sizes)
        .expect("Failed to forward the module");

    // Softmax
    let softmax_output = softmax(&output.data);
    let class = argmax(&softmax_output);

    // Load class file
    let class_file_path =
        base_path.join("resource/model/mv3/imagenet_classes.txt");
    let class_file =
        File::open(class_file_path).expect("Failed to open class file");
    let mut class_names = vec![];
    let reader = BufReader::new(class_file);
    for line in reader.lines() {
        let line = line.expect("Failed to read class file");
        class_names.push(line);
    }
    assert_eq!(expected_class, class);
    assert_eq!(expected_str, class_names[class].to_uppercase());
}

#[cfg(feature = "android")]
mod android {
    use super::*;

    #[test]
    fn test_mv3_portable() {
        let model_path = "resource/model/mv3/mv3.pte";
        let expected_class = 207;
        let expected_str = "GOLDEN RETRIEVER";
        forward_model(model_path, expected_class, expected_str);
    }

    #[test]
    fn test_mv3_xnnpack_fp32() {
        let model_path = "resource/model/mv3/mv3_xnnpack_fp32.pte";
        let expected_class = 207;
        let expected_str = "GOLDEN RETRIEVER";
        forward_model(model_path, expected_class, expected_str);
    }

    #[test]
    fn test_mv2_vulkan() {
        let model_path = "resource/model/mv3/mv2_vulkan.pte";
        let expected_class = 257;
        let expected_str = "GREAT PYRENEES";
        forward_model(model_path, expected_class, expected_str);
    }
}

#[cfg(feature = "apple")]
mod apple {
    use super::*;

    #[test]
    fn test_mv3_portable() {
        let model_path = "resource/model/mv3/mv3.pte";
        let expected_class = 207;
        let expected_str = "GOLDEN RETRIEVER";
        forward_model(model_path, expected_class, expected_str);
    }

    #[test]
    fn test_mv3_xnnpack_fp32() {
        let model_path = "resource/model/mv3/mv3_xnnpack_fp32.pte";
        let expected_class = 207;
        let expected_str = "GOLDEN RETRIEVER";
        forward_model(model_path, expected_class, expected_str);
    }

    #[test]
    fn test_mv3_mps() {
        let model_path = "resource/model/mv3/mv3_mps.pte";
        let expected_class = 207;
        let expected_str = "GOLDEN RETRIEVER";
        forward_model(model_path, expected_class, expected_str);
    }

    #[test]
    fn test_mv3_coreml_all() {
        let model_path = "resource/model/mv3/mv3_coreml_all.pte";
        let expected_class = 207;
        let expected_str = "GOLDEN RETRIEVER";
        forward_model(model_path, expected_class, expected_str);
    }
}
