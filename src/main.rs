#![allow(dead_code)]
// use serde::Deserialize;
use std::collections::HashMap;

struct DataServiceRequestOpts<'a> {
    token: &'a str,
    method: reqwest::Method,
    body: &'a str,
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
    let options = DataServiceRequestOpts {
        token: "mLghicY9MM",
        method: reqwest::Method::GET,
        body: "lorem ipsum",
    };

    let res = match request(&service, &options) {
        Ok(r) => r,
        Err(e) => return println!("Error requesting data: {:?}", e),
    };
    let r = ServiceResponse { res };
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
    _options: &DataServiceRequestOpts,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let res = service.client.get(service.root_url).send().await?;
    Ok(res)
}
