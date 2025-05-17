use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct ContextItem {
    topic: String,
    content: String,
}

pub fn search_context_from_json(user_prompt: &str) -> String {
    let file_content = fs::read_to_string("context.json").expect("Failed to read context.json");
    let items: Vec<ContextItem> = serde_json::from_str(&file_content).expect("Invalid JSON format");

    let lower_prompt = user_prompt.to_lowercase();
    let mut matched = vec![];

    for item in &items {
        if lower_prompt.contains(&item.topic.replace("_", " ")) || lower_prompt.contains(&item.topic) {
            matched.push(item.content.clone());
        }
    }

    // Fallback jika tidak ada kecocokan
    if matched.is_empty() {
        items.iter().map(|i| i.content.clone()).collect::<Vec<_>>().join(" ")
    } else {
        matched.join(" ")
    }
}
