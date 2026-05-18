pub fn empty_dash(value: &str) -> &str {
    if value.trim().is_empty() { "-" } else { value }
}
