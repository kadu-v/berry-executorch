use berry_executorch::Module;

fn main() {
    let file_path = "./resource/model/simple/two.pte";
    let mut module =
        Module::new(file_path).expect("Failed to create a new module");
    module.load().unwrap();

    let input = vec![1.0];
    let input_sizes = vec![1];
    let output = module
        .forward(&input, &input_sizes)
        .expect("Failed to forward the module");
    println!("[1] + [1] = [{}]", output.data[0]);
}
