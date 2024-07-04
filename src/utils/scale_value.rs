pub fn scale_value(input: f32, max: f32) -> f32 {
    if input < 0.0 || input > 1.0 {
        panic!("Input value must be between 0 and 1");
    }
    if max <= 1.0 {
        panic!("Max value must be greater than 1");
    }
    1.0 + (input * (max - 1.0))
}
