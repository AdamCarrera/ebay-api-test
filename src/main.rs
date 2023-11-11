#[allow(unused)]
use crate::ebay_api::ebay_api::SearchConfig;
use toml;
use serde_derive::Deserialize;

pub mod ebay_api;

// Structure to hold important secret information
#[derive(Debug, Deserialize)]
struct ApiKeys {
    api_keys: ApiKeysInner,
}

#[derive(Debug, Deserialize)]
struct ApiKeysInner {
    ebay: String,
}

// Read the config file to retrieve secret information
fn read_config() -> Result<ApiKeys, Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&config_str)?)
}

#[allow(unused)]
fn main() {
    // Read API Key from Config File
    let api_keys = match read_config() {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("Error reading configuration: {}", e);
            return;
        }
    };

    // Define request parameters
    // query: what we are searching for
    let query: serde_json::Value = serde_json::Value::String(String::from("laptop"));

    // config: stuff we need to request - access token, headers, parameters, etc
    let config = SearchConfig::new(query, api_keys.api_keys.ebay);

    // post the query and print the results to the terminal
    let result = ebay_api::ebay_api::post_query(config);
    let outcome = match result {
        Ok(file) => file,
        Err(error) => panic!("Problem with the request: {:?}", error),
    };
}
