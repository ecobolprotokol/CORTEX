pub fn is_safe(signal_magnitude: f32, average_error: f32) -> bool {
    if average_error < 1e-6 {
        return signal_magnitude < 0.5;
    }
    signal_magnitude < average_error * 3.0
}
