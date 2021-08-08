use reqwest::header;
use std::collections::HashMap;
use std::error::Error;

pub struct ServiceRequestOpts<'a> {
    token: &'a str,
    method: reqwest::Method,
    body: &'a str,
    headers: header::HeaderMap,
}
/*
A constant variables for the purpose of calling a designated API.
Example showed this as a struct, so I went with that.
*/
pub struct ServiceConfig<'a> {
    pub root_url: &'a str,
}

pub struct Service<'a> {
    pub root_url: &'a str,
    pub client: reqwest::Client,
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

pub struct ServiceResponse {
    pub res: reqwest::Response,
}
// A standardized way to strip out and return the body of response in a way that can be easily converted to a custom data object.
impl ServiceResponse {
    #[tokio::main]
    pub async fn strip(self) -> Result<HashMap<String, serde_json::Value>, Box<dyn Error>> {
        let text = self
            .res
            .json::<HashMap<String, serde_json::Value>>()
            .await?;
        Ok(text)
    }
}

// A setup function that accepts a config object, creates the reqwest::Client instance, and returns the service as a struct.
pub fn setup(config: ServiceConfig) -> Service {
    Service::new(&config)
}

//An async function that will call an api with a provided bearer token and parameters, then return the response as a reqwest::Response object
#[tokio::main]
pub async fn request(
    service: &Service,
    options: &ServiceRequestOpts,
) -> Result<reqwest::Response, Box<dyn Error>> {
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
            Ok(res)
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
            Ok(res)
        }
        _ => Err("REST method not supported".into()),
    }
}
