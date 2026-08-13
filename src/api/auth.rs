pub fn authenticate(request_key: Option<&str>, expected_key: Option<&str>) -> bool {
    match (request_key, expected_key) {
        (Some(req), Some(exp)) => req == exp,
        (None, None) => true,
        _ => false,
    }
}
