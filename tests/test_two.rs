use berry_executorch::Module;

#[test]
fn test_two() {
    let file_path = dinghy_test::test_project_path();
    let file_path = file_path.join("resource/model/simple/two.pte");
    let mut module = Module::new(&file_path.display().to_string())
        .expect("Failed to create a new module");
    module.load().unwrap();

    let input = vec![1.0];
    let input_sizes = vec![1];
    let output = module
        .forward(&input, &input_sizes)
        .expect("Failed to forward the module");
    assert_eq!(2.0, output.data[0]);
}
