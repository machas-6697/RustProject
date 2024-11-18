use warp::{Filter, http::Method};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use serde::Serialize;
use futures_util::stream::StreamExt;
use tokio::io::AsyncWriteExt;
use bytes::Buf;

#[derive(Debug)]
struct CustomError;

impl warp::reject::Reject for CustomError {}

#[derive(Serialize)]
struct WordFrequency {
    word: String,
    count: usize,
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method(Method::POST)
        .allow_header("Content-Type");

    let static_files = warp::get().and(warp::fs::dir(Path::new("static")));

    let upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(10_000_000))
        .and_then(handle_file_upload);

    let routes = static_files.or(upload_route).with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_file_upload(mut form: warp::multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let dir = Path::new("C:\\MY-SPACE\\Rust\\project");
    let mut file_path = None;

    while let Some(Ok(part)) = form.next().await {
        if part.name() == "file" {
            let file_name = part.filename().map(String::from).unwrap_or_else(|| "uploaded_file.txt".to_string());
            let destination = dir.join(file_name);

            let mut file = tokio::fs::File::create(&destination).await.map_err(|_| warp::reject::custom(CustomError))?;
            file_path = Some(destination.clone());

            let mut stream = part.stream();
            while let Some(Ok(mut chunk)) = stream.next().await {
                let mut bytes = vec![0; chunk.remaining()];
                chunk.copy_to_slice(&mut bytes);
                file.write_all(&bytes).await.map_err(|_| warp::reject::custom(CustomError))?;
            }
        }
    }

    let path = file_path.ok_or_else(|| warp::reject::custom(CustomError))?;
    let word_frequencies = process_file(path).map_err(|_| warp::reject::custom(CustomError))?;

    Ok(warp::reply::json(&word_frequencies))
}

fn process_file(file_path: std::path::PathBuf) -> io::Result<Vec<WordFrequency>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut word_count: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let word = word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
            if !word.is_empty() {
                *word_count.entry(word).or_insert(0) += 1;
            }
        }
    }

    let word_frequencies: Vec<WordFrequency> = word_count.into_iter()
        .map(|(word, count)| WordFrequency { word, count })
        .collect();

    Ok(word_frequencies)
}
