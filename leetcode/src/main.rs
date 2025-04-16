use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::{
    fmt::format,
    fs::{self, File},
    io::Write,
};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    id: String,
    title: String,
    content: String,
    code: String,
    language: String,
    lang_slug: String,
}

#[tokio::main]
async fn fetch_question(question: &str, lang_arg: &str) -> Result<(), reqwest::Error> {
    let body = format!(
        r#"{{
    "operationName": "getQuestionDetail",
    "variables": {{
        "titleSlug": "{}"
    }},
    "query": "query getQuestionDetail($titleSlug: String!) {{ question(titleSlug: $titleSlug) {{ questionId title content codeSnippets {{ lang langSlug code }} }} }}"
}}"#,
        question
    );

    let client = reqwest::Client::new();
    let response = client
        .post("https://leetcode.com/graphql")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let parsed: GraphQLResponse = response.json().await?;
    let info: Info = Info {
        id: parsed.data.question.questionId,
        title: parsed.data.question.title,
        content: parsed.data.question.content,
        code: get_lang_code(&parsed.data.question.codeSnippets, lang_arg),
        language: lang_arg.to_owned(),
        lang_slug: "python".to_owned(),
    };
    //TODO: CREATE A MAPPING OF LANG AND LANGSLUG

    let path: String = format!("{}.{}", question, lang_arg);
    let mut file = File::create(path).expect("Error creating a file");
    let file_content: String = format!(
        "/*{}{}{}*/{}{}",
        "/n",
        info.title,
        strip_html_tags(&info.content),
        "/n",
        info.code
    );
    file.write_all(file_content.as_bytes())
        .expect("Error writing in file");
    Ok(())
}
#[tokio::main]
async fn submit() -> Result<(), reqwest::Error> {
    println!("INSIDE SUBMIT");
    let token_path: &str = "/home/bishwas/LeetcodeCLI/leetcode/session.txt";
    let code_path: &str = "/home/bishwas/LeetcodeCLI/leetcode/src/add-two-numbers.py";
    let csrf_path: &str = "/home/bishwas/LeetcodeCLI/leetcode/csrf.txt";
    let session_token = fs::read_to_string(token_path).expect("Couldnt read session token");
    let csrf_token = fs::read_to_string(csrf_path).expect("Couldnt read csrf token");
    println!("{csrf_token} {session_token}");
    let code = fs::read_to_string(code_path).expect("Couldnt read the file");

let client = reqwest::Client::new();

let url = "https://leetcode.com/problems/reverse-integer/submit/";
let resp = client
    .post(url)  
    .header("Content-Type", "application/json")
    .header("Cookie", format!("LEETCODE_SESSION={}; csrftoken={}", session_token.trim(), csrf_token.trim()))
    .header("x-csrftoken", csrf_token.trim())
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
    .header("Referer", format!("https://leetcode.com/problems/{}/submit/", "python"))
    .json(&serde_json::json!({
        "query": r#"
            mutation submitSolution($lang: String!, $questionId: String!, $typedCode: String!, $username: String!) {
                submitSolution(lang: $lang, questionId: $questionId, typedCode: $typedCode, username: $username) {
                    submission_id
                }
            }
        "#,
        "variables": {
            "lang": "Python",
            "questionId": "7",  
            "typedCode": code,
            "username": "bishwasxdgautam"  
        }
    }))
    .send()
    .await?
    .text()
    .await?;

println!("RESPONSE RECEIVED IS: {}", resp);



    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let main_arg: &str = &args[1];
    let question: &str = if args.len() > 2 {
        &args[2]
    } else {
        "DEFAULT QUESTION"
    };
    let lang_arg: &str = if args.len() > 3 {
        &args[3]
    } else {
        "DEFAULT LANG"
    };

    match main_arg {
        "fetch" => fetch_question(question, lang_arg).expect("Error while fetching Question"),
        "submit" => submit().expect("ERROR SUBMITTING"),
        _ => panic!("command not recognized"),
    }
}
fn get_lang_code(snippets: &[CodeSnippet], lang: &str) -> String {
    snippets
        .iter()
        .find(|s| s.lang == lang)
        .map(|s| s.code.clone())
        .unwrap_or_else(|| format!("No code snippet found for {}", lang))
}

fn strip_html_tags(raw_html: &str) -> String {
    let re = Regex::new(r"</?[^>]+>").unwrap();
    re.replace_all(raw_html, "").to_string()
}
