use serde::Deserialize;

const CONTEXT_JSON: &str = include_str!("../../context.json");

#[derive(Deserialize)]
struct ContextItem {
    topic: String,
    keywords: Vec<String>,
    content: String,
}

pub fn search_context_from_json(user_prompt: &str) -> String {
    let items: Vec<ContextItem> =
        serde_json::from_str(CONTEXT_JSON).expect("Format JSON tidak valid");

    let lower_prompt = user_prompt.to_lowercase();
    let mut matched = vec![];

    for item in &items {
        let topic_match = lower_prompt.contains(&item.topic.replace("_", " "))
            || lower_prompt.contains(&item.topic);

        let keyword_match = item
            .keywords
            .iter()
            .any(|keyword| lower_prompt.contains(keyword));

        if !(topic_match || keyword_match) {
            continue;
        }
        if !matched.contains(&item.content) {
            matched.push(item.content.clone());
        }
    }

    if matched.is_empty() {
        items
            .iter()
            .map(|i| i.content.clone())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        matched.join(" ")
    }
}

