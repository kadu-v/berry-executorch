use imageproc;
use imageproc::image;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn softmax(x: &Vec<f32>) -> Vec<f32> {
    let max_x = x.iter().fold(f32::NEG_INFINITY, |acc, &x| acc.max(x));
    let exp_x = x.iter().map(|x| (x - max_x).exp()).collect::<Vec<f32>>();
    let sum_exp_x = exp_x.iter().sum::<f32>();
    exp_x.iter().map(|x| x / sum_exp_x).collect::<Vec<f32>>()
}

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

fn main() {
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Load image
    let image_path = base_path.join("resource/image/dog.png");
    let image = image::open(image_path).unwrap();
    let image = image.to_rgb8();
    println!("Image: {:?}", image.dimensions());

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

    // Load model
    #[allow(unused_mut)]
    let mut file_paths = vec!["resource/model/mv3/mv3.pte"];

    #[cfg(feature = "xnnpack")]
    {
        file_paths.push("resource/model/mv3/mv3_xnnpack_fp32.pte");
    }
    #[cfg(feature = "apple")]
    {
        file_paths.push("resource/model/mv3/mv3_mps.pte");
        file_paths.push("resource/model/mv3/mv3_coreml_all.pte");
    }
    for file_path in file_paths {
        println!("Model: {}", file_path);
        let model_path = base_path.join(file_path);
        let mut module =
            executorch_rs::Module::new(&model_path.display().to_string())
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
        println!(
            "Class No: \"{}\", Class Name: \"{}\"",
            class,
            class_names[class].to_uppercase()
        );
    }
}
