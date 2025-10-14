pub fn truncate_text(text: &str, max_words: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() <= max_words {
        return text.to_string();
    }

    words[..max_words].join(" ") + "..."
}
