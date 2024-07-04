pub fn ease_in_out_quint(x: f32) -> f32 {
    if x < 0.5 {
        16.0 * x * x * x * x * x
    } else {
        1.0 - f32::powf(-2.0 * x + 2.0, 5.0) / 2.0
    }
}
