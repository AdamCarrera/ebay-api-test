//! # eBay API Module
//!
//! The `ebay_api` module provides functionality to interact with the eBay API for item search.
//!
//! ## Overview
//!
//! This module contains a `SearchConfig` structure to hold the data required to make a search request,
//! along with a function `post_query` to perform the actual API request.
//!
//! ## Example Usage
//!
//! ```rust
//! use ebay_api::{SearchConfig, post_query};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Replace the values with your actual eBay developer credentials and access token
//!     let app_id = "Your-App-ID".to_string();
//!     let cert_id = "Your-Cert-ID".to_string();
//!     let access_token = "Your-OAuth-Access-Token".to_string();
//!
//!     // Create a new SearchConfig with the search query and access token
//!     let search_query = serde_json::json!("your search query");
//!     let config = SearchConfig::new(search_query, access_token);
//!
//!     // Perform the API request and handle the result
//!     if let Err(err) = post_query(config).await {
//!         eprintln!("Error: {}", err);
//!     }
//! }
//! ```
//!
//! ## `SearchConfig` Structure
//!
//! The `SearchConfig` structure is used to configure the parameters for an eBay API search request.
//!
//! ### Fields
//!
//! - `app_id`: eBay developer application ID.
//! - `cert_id`: eBay developer certificate ID.
//! - `search_url`: The URL for the eBay API endpoint for item search.
//! - `headers`: HeaderMap containing necessary headers for the API request (content type, authorization).
//! - `search_parameters`: Map containing search parameters such as query and limit.
//!
//! ### Methods
//!
//! - `new(query: serde_json::Value, access_token: String) -> Self`: Creates a new `SearchConfig` instance.
//!
//! ## `post_query` Function
//!
//! The `post_query` function performs an asynchronous API request using the provided `SearchConfig`.
//!
//! ### Parameters
//!
//! - `config`: A `SearchConfig` instance containing the configuration for the API request.
//!
//! ### Returns
//!
//! - `Result<(), reqwest::Error>`: A Result indicating the success or failure of the API request.
//!
//! ### Example
//!
//! ```rust
//! use ebay_api::{SearchConfig, post_query};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Replace the values with your actual eBay developer credentials and access token
//!     let app_id = "Your-App-ID".to_string();
//!     let cert_id = "Your-Cert-ID".to_string();
//!     let access_token = "Your-OAuth-Access-Token".to_string();
//!
//!     // Create a new SearchConfig with the search query and access token
//!     let search_query = serde_json::json!("your search query");
//!     let config = SearchConfig::new(search_query, access_token);
//!
//!     // Perform the API request and handle the result
//!     if let Err(err) = post_query(config).await {
//!         eprintln!("Error: {}", err);
//!     }
//! }
//! ````

#[allow(unused)]
pub mod ebay_api {
    use std::collections::HashMap;
    use reqwest::header::{ self, HeaderMap };
    use serde_json::{ Value, json };

    #[derive(Debug)]
    /// Search Config Structure to hold the data we will use to
    /// make the request
    pub struct SearchConfig {
        pub app_id: String,
        pub cert_id: String,
        pub search_url: String,
        pub headers: header::HeaderMap,
        pub search_parameters: serde_json::Map<String, serde_json::Value>,
    }

    impl SearchConfig {
        /// Create New Search Config
        /// query -> search query, item you are searching for
        /// access_token -> OAuth access token from eBay

        pub fn new(query: serde_json::Value, access_token: String) -> Self {
            // Make an empty header map and insert the content type and authorization headers

            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json").to_owned()
            );

            let auth_header_value = format!("Bearer {}", access_token);
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&auth_header_value).unwrap()
            );

            let mut search_parameters: serde_json::Map<String, Value> = serde_json::Map::new();
            search_parameters.insert(String::from("q"), query);
            search_parameters.insert(String::from("limit"), json!("5"));

            SearchConfig {
                app_id: String::from("AdamCarr-mtgcardf-SBX-3ac219c73-c36c6538"),
                cert_id: String::from("SBX-ac219c739b47-816b-43f8-964f-6b1a"),
                headers,
                search_url: String::from(
                    "https://api.sandbox.ebay.com/buy/browse/v1/item_summary/search"
                ),
                search_parameters,
            }
        }
    }

    #[tokio::main]
    pub async fn post_query(config: SearchConfig) -> Result<(), reqwest::Error> {
        // Make a GET request with the url from SearchConfig

        let client = reqwest::Client::new();
        let response = client
            .get(config.search_url)
            .headers(config.headers)
            .query(&config.search_parameters)
            .send().await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let parsed_json: Value = serde_json::from_str(&body).expect("failed to parse json");
            let pretty_json = serde_json
                ::to_string_pretty(&parsed_json)
                .expect("failed to pretty json");

            println!("Response body: {}", pretty_json);
        } else {
            println!("Request failed with status code: {}", response.status());
        }

        Ok(())
    }
}
