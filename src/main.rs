#![allow(dead_code)]
use serde::Deserialize;

struct DataServiceRequestOpts<'a> {
    token: &'a str,
    method: reqwest::Method,
    body: &'a str,
}

struct ServiceConfig<'a> {
    root_url: &'a str,
}

#[derive(Debug)]
struct Service<'a> {
    root_url: &'a str,
    client: reqwest::Client,
}

impl<'a> Service<'a> {
    fn new(config: &ServiceConfig<'a>) -> Service<'a> {
        Service {
            root_url: config.root_url,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct ServiceResponse {
    res: String,
}

/*
impl<'a> ServiceResponse<'a> {
    pub async fn strip(&self, res: reqwest::Response) -> Result<Self, Box<dyn std::error::Error>> {
        let body = res.json::<ServiceResponse>().await?;
        Ok(body)
    }
}
*/
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

    let text = match deserialize(res) {
        Ok(r) => r,
        Err(e) => return println!("Error requesting data: {:?}", e),
    };
    println!("{}", text)
}

fn setup(config: ServiceConfig) -> Service {
    Service::new(&config)
}

#[tokio::main]
async fn deserialize(response: reqwest::Response) -> Result<String, Box<dyn std::error::Error>> {
    let text = response.text().await?;
    Ok(text)
}

#[tokio::main]
async fn request(
    service: &Service,
    _options: &DataServiceRequestOpts,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let res = service.client.get(service.root_url).send().await?;
    Ok(res)
}
