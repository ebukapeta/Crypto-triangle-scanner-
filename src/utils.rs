pub fn percent_string(p: f64) -> String {
    format!("{:.3}%", p * 100.0)
}