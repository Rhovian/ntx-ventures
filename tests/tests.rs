use ntx_ventures;
/*
This function tests the following cases:
-   Service struct is instantiated succesfully (setup fn)
-   Client instantiated succesfully (setup fn)
-   Client works correctly
    -   Simple http get request status code check
-   HTTP response can be converted into a custom data object (strip fn)
*/
#[tokio::test]
async fn setup_returns_service() -> Result<(), Box<dyn std::error::Error>> {
    let config = ntx_ventures::ServiceConfig {
        root_url: "https://random-data-api.com/api/beer/random_beer",
    };
    // test struct can be instantiated
    // client instantiated succesfully
    let result = ntx_ventures::setup(config);
    // test client works correctly
    let res = result
        .client
        .get(result.root_url)
        .send()
        .await?;
    // 200 status code
    
    assert!(res.status().is_success());
    // ServiceResponse struct is now instantiated
    let r = ntx_ventures::ServiceResponse { res };
    // https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
    tokio::task::spawn_blocking(|| {
        // unpack response body into a hashmap of <String, T>
        r.strip();
    })
    .await;

    Ok(())
}
