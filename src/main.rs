// use serde::Deserialize;
use std::collections::HashMap;

struct ServiceRequestOpts<'a> {
    token: &'a str,
    method: reqwest::Method,
    body: &'a str,
    headers: reqwest::header::HeaderMap,
}
/*
A constant variables for the purpose of calling a designated API.
Example showed this as a struct, so I went with that.
*/
struct ServiceConfig<'a> {
    root_url: &'a str,
}

#[derive(Debug)]
struct Service<'a> {
    root_url: &'a str,
    client: reqwest::Client,
}
// An async reqwest Http client that will be used for making all calls
impl<'a> Service<'a> {
    fn new(config: &ServiceConfig<'a>) -> Service<'a> {
        Service {
            root_url: config.root_url,
            client: reqwest::Client::new(),
        }
    }
}

struct ServiceResponse {
    res: reqwest::Response,
}
// A standardized way to strip out and return the body of response in a way that can be easily converted to a custom data object.
impl ServiceResponse {
    #[tokio::main]
    async fn strip(self) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let text = self
            .res
            .json::<HashMap<String, serde_json::Value>>()
            .await?;
        Ok(text)
    }
}

fn main() {
    // config incoming from API
    let config = ServiceConfig {
        root_url: "https://random-data-api.com/api/beer/random_beer",
    };
    let service = setup(config);
    // request options incoming from API
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str("My Rust Program 1.0").unwrap(),
    );
    let options = ServiceRequestOpts {
        token: "mLghicY9MM",
        method: reqwest::Method::GET,
        body: "lorem ipsum",
        headers,
    };

    let res = match request(&service, &options) {
        Ok(r) => r,
        Err(e) => return println!("Error requesting data: {:?}", e),
    };
    let response = match res {
        Some(r) => r,
        None => panic!(
            "this REST method is not supported: {:?}",
            options.method.as_str()
        ),
    };
    let r = ServiceResponse { res: response };
    let body = r.strip();
    println!("{:?}", body)
}

// A setup function that accepts a config object, creates the reqwest::Client instance, and returns the service as a struct.
fn setup(config: ServiceConfig) -> Service {
    Service::new(&config)
}

//An async function that will call an api with a provided bearer token and parameters, then return the response as a reqwest::Response object
#[tokio::main]
async fn request(
    service: &Service,
    options: &ServiceRequestOpts,
) -> Result<Option<reqwest::Response>, Box<dyn std::error::Error>> {
    match options.method.as_str() {
        "GET" => {
            // clone is bad (?)
            let res = service
                .client
                .get(service.root_url)
                .headers(options.headers.clone())
                .bearer_auth(options.token)
                .send()
                .await?;
            Ok(Some(res))
        }
        "POST" => {
            let res = service
                .client
                .post(service.root_url)
                .headers(options.headers.clone())
                .body(options.body.to_string())
                .bearer_auth(options.token)
                .send()
                .await?;
            Ok(Some(res))
        }
        _ => Ok(None),
    }
}
