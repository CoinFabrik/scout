// Helper function to capitalize the first letter of a string
pub fn capitalize(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect()
}

pub fn sanitize_category_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}
