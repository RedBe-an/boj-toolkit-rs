use reqwest::Client;
use std::collections::HashMap;

/**
Sends a GET request to the specified URL with optional headers and query parameters.

# Example
```rust
use api::get::get_data;
use reqwest::Error;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url: String = "https://solved.ac/api/v3/problem/show".to_string();

    // Accept: application/json
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("x-solvedac-language".to_string(), "ko".to_string());
    // headers.insert("Accept".to_string(), "application/json".to_string());
    
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".to_string();
    headers.insert("User-Agent".to_string(), user_agent);

    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("problemId".to_string(), "1000".to_string());

    match get_data(url, Some(headers), Some(query)).await {
        Ok(data) => {
            println!("받은 데이터: {:?}", data);
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

*/
#[allow(dead_code)]
pub async fn get_data(
    url: String,
    headers: Option<HashMap<String, String>>,
    query: Option<HashMap<String, String>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client: Client = Client::builder()
        .build()?;

    let mut req: reqwest::RequestBuilder = client.get(url);

    if let Some(headers) = headers {
        for (key, value) in headers {
            req = req.header(key, value);
        }
    }

    if let Some(query) = query {
        req = req.query(&query);
    }

    let res: reqwest::Response = req.send().await?;

    Ok(res)
}

