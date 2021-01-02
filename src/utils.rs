pub fn is_equal(v1: f32, v2: f32) -> bool {
    if (v1 - v2).abs() <= 1e-4 {
        return true;
    }
    return false;
}