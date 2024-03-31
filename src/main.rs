use warp::filters::query;
use warp::Filter;
use serde::{Serialize, Deserialize};
use warp::http::StatusCode;
use openai_api_rs::v1::api::Client as OpenAiClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;
use std::env;

use pgvector::Vector;
use postgres::{Client, NoTls};
use serde_json::Value;
use std::error::Error;


fn insert_embeddings(input: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut client = Client::configure()
        .host("localhost")
        .dbname("postgres")
        .user("postgres")
        .password("postgres")
        .connect(NoTls)?;

    client.execute("CREATE EXTENSION IF NOT EXISTS vector", &[])?;
    client.execute("DROP TABLE IF EXISTS documents", &[])?;
    client.execute("CREATE TABLE documents (id serial PRIMARY KEY, content text, embedding vector(1536))", &[])?;

    let embeddings = fetch_embeddings(input)?;

    for (content, embedding) in input.iter().zip(embeddings) {
        let embedding = Vector::from(embedding);
        client.execute("INSERT INTO documents (content, embedding) VALUES ($1, $2)", &[&content, &embedding])?;
    }

    Ok(())
}


fn semantic_search(input: &str) -> Result<String, Box<dyn Error>> {
    let mut client = Client::configure()
        .host("localhost")
        .dbname("postgres")
        .user("postgres")
        .password("postgres")
        .connect(NoTls)?;

    // Fetch the embedding for the input string
    let embeddings = fetch_embeddings(&[input])?;
    let embedding = &embeddings[0];

    let mut result = String::new();
    // // Use the fetched embedding to query the database
    // for row in client.query("SELECT content FROM documents ORDER BY embedding <=> $1 LIMIT 3", &[&embedding])? {
    //     let content: &str = row.get(0);
    //     result.push_str(content);
    //     result.push('\n');
    // }

    // let document_id = 1;
    // for row in client.query("SELECT content FROM documents ORDER BY embedding <=> (SELECT embedding FROM documents LIMIT 1) LIMIT 3", &[])? {
    //     let content: &str = row.get(0);
    //     println!("{}", content);
    // }

    // Ok(result)

    // let query = "SELECT content FROM documents ORDER BY embedding <-> $1 LIMIT 3";

    // for row in client.query("SELECT content FROM documents ORDER BY embedding <-> $1 LIMIT 3", &[&embedding])? {
    //     let content: &str = row.get(0);
    //     // result.push_str(content);
    //     // result.push('\n');
    //     println!("{}", content);
    // }

    // println!("{:?}", embedding);

    let query = "SELECT content FROM documents ORDER BY embedding <-> $1 LIMIT 3";
    // let rows = client.query(query, &[&embedding])?;
    let rows = client.query(query, &[embedding])?;

    for row in rows {
        let content: &str = row.get(0);
        println!("{:?}", content);
    }

    Ok(result)
}


fn fetch_embeddings(input: &[&str]) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").or(Err("Set OPENAI_API_KEY"))?;

    let response: Value = ureq::post("https://api.openai.com/v1/embeddings")
        .set("Authorization", &format!("Bearer {}", api_key))
        .send_json(ureq::json!({
            "input": input,
            "model": "text-embedding-ada-002",
        }))?
        .into_json()?;

    let embeddings = response["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            v["embedding"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap() as f32)
                .collect()
        })
        .collect();

    Ok(embeddings)
}



#[derive(Deserialize, Serialize)]
struct Data {
    data: Vec<String>,
}

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
}

fn get_chat_completion(input: &str, system: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = OpenAiClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::system,
            content: chat_completion::Content::Text(system.to_string()),
            name: None,
        },
        chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(input.to_string()),
            name: None,
        }],
    );
    let result = client.chat_completion(req)?;
    match &result.choices[0].message.content {
        Some(content) => Ok(content.clone()),
        None => Err("No content found".into()),
    }
}

// #[tokio::main]
// async fn main() {

//     let system = "You are a personal assistant called eve"; // system + semantic + history

//     // let context = semantic_search(input);

//     // let history = // parse the history conversation

//     let save_route = warp::path("save")
//         .and(warp::post())
//         .and(warp::header::<String>("authorization"))
//         .and(warp::body::json())
//         .map(move |token: String, data: Data| {
//             let secret = "your_token_here"; // replace with your actual token
//             if token == secret {
//                 // warp::reply::with_status(warp::reply::json(&data), StatusCode::OK)
//                 match fetch_embeddings(&data["data"].iter().map(AsRef::as_ref).collect::<Vec<_>>()) {
//                     Ok(embeddings) => {
//                         let response = embeddings.iter().map(|v| v.iter().map(|&f| f as f64).collect::<Vec<_>>()).collect::<Vec<_>>();
//                         warp::reply::with_status(warp::reply::json(&response), StatusCode::OK)
//                     },
//                     Err(_) => {
//                         let error_message = ErrorMessage {
//                             error: "Failed to fetch embeddings".to_string(),
//                         };
//                         warp::reply::with_status(warp::reply::json(&error_message), StatusCode::INTERNAL_SERVER_ERROR)
//                     }
//                 }
//             } else {
//                 let error_message = ErrorMessage {
//                     error: "Unauthorized".to_string(),
//                 };
//                 warp::reply::with_status(warp::reply::json(&error_message), StatusCode::UNAUTHORIZED)
//             }
//         });

//     let run_route = warp::path("run")
//         .and(warp::post())
//         .and(warp::header::<String>("authorization"))
//         .and(warp::body::json())
//         .map(move |token: String, mut data: Data| {

//             let secret = "your_token_here"; // replace with your actual token
//             if token == secret {


//                 if let Some(first_element) = data.data.get_mut(0) {
//                     *first_element = get_chat_completion(first_element, &system).unwrap_or_else(|_| "Error getting chat completion".to_string());
//                 }
//                 warp::reply::with_status(warp::reply::json(&data), StatusCode::OK)

//             } else {
//                 let error_message = ErrorMessage {
//                     error: "Unauthorized".to_string(),
//                 };
//                 warp::reply::with_status(warp::reply::json(&error_message), StatusCode::UNAUTHORIZED)
//             }
//         });

//     let routes = save_route.or(run_route);


//     warp::serve(routes)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let input = [
//         "The dog is barking",
//         "The cat is purring",
//         "The bear is growling",
//     ];

//     insert_embeddings(&input)?;

//     println!("Embeddings inserted successfully");

//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let input = "The cat is meowing";
    let result = semantic_search(input)?;

    println!("Results for '{}':\n{}", input, result);

    Ok(())
}