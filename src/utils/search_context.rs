use serde::Deserialize;

const CONTEXT_JSON: &str = include_str!("../../context.json"); 

#[derive(Deserialize)]
struct ContextItem {
    topic: String,
    content: String,
}

pub fn search_context_from_json(user_prompt: &str) -> String {
    let items: Vec<ContextItem> = serde_json::from_str(CONTEXT_JSON).expect("Invalid JSON format");

    let lower_prompt = user_prompt.to_lowercase();
    let mut matched = vec![];

    for item in &items {
        if lower_prompt.contains(&item.topic.replace("_", " ")) || lower_prompt.contains(&item.topic) {
            matched.push(item.content.clone());
        }
    }

    if matched.is_empty() {
        items.iter().map(|i| i.content.clone()).collect::<Vec<_>>().join(" ")
    } else {
        matched.join(" ")
    }
}