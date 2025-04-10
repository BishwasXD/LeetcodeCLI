use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(Debug, Deserialize)]
struct GraphQLData {
    question: Question,
}

#[derive(Debug, Deserialize)]
struct Question {
    questionId: String,
    title: String,
    content: String,
    codeSnippets: Vec<CodeSnippet>,
}

#[derive(Debug, Deserialize)]
struct CodeSnippet {
    lang: String,
    langSlug: String,
    code: String,
}

#[derive(Debug, Deserialize)]
struct Info {
    title: String,
    content: String,
    code: String,
    language: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = r#"{
        "operationName": "getQuestionDetail",
        "variables": {
            "titleSlug": "add-two-numbers"
        },
        "query": "query getQuestionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { questionId title content codeSnippets { lang langSlug code } } }"
    }"#;

    let client = reqwest::Client::new();
    let response = client
        .post("https://leetcode.com/graphql")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let parsed: GraphQLResponse = response.json().await?;
    let info: Info = Info {
        title: parsed.data.question.title,
        content: parsed.data.question.content,
        code: get_lang_code(&parsed.data.question.codeSnippets, "Java"),
        language: "Java".to_owned(),
    };

    println!("THIS IS ACTUAL USEFUL INFO {:?}", info);
    Ok(())
}
fn get_lang_code(snippets: &[CodeSnippet], lang: &str) -> String {
    snippets
        .iter()
        .find(|s| s.lang == lang)
        .map(|s| s.code.clone())
        .unwrap_or_else(|| format!("No code snippet found for {}", lang))
}
