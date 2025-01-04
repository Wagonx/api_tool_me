use reqwest;
use std::error::Error;
use dialoguer::{Input, MultiSelect, Select, theme::ColorfulTheme};
use serde_json::{Value, from_str};
use urlencoding::encode;
use dotenv::dotenv;
use std::env;

fn format_json_simple(text: &str) -> String {
    match from_str::<Value>(text) {
        Ok(json) => {
            serde_json::to_string_pretty(&json).unwrap_or(text.to_string())
        },
        Err(_) => text.to_string()
    }
}

fn print_separator() {
    println!("\n========================================\n");
}

fn load_env_config() -> Result<(String, String), Box<dyn Error>> {
    dotenv().ok();

    let host = env::var("API_HOST")
        .expect("API_HOST must be set in .env file");
    let base_url = env::var("API_URL")
        .expect("API_URL must be set in .env file");

    Ok((host, base_url))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    let (host, base_url) = load_env_config()?;

    println!("API REQUEST TOOL");
    println!("===============");

    // Get authorization token
    let auth_token: String = Input::new()
        .with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your authorization token")
        .interact_text()?;

    print_separator();

    // Platform selection
    let platforms = vec!["windows", "mac", "linux"];
    println!("Select platforms (Space to select, Enter to confirm):");
    let selected_platforms = MultiSelect::new()
        .with_theme(&ColorfulTheme::default())
        .items(&platforms)
        .interact()?;

    if selected_platforms.is_empty() {
        println!("\nError: You must select at least one platform");
        std::process::exit(1);
    }

    print_separator();

    // Config type selection
    let config_types = vec!["None", "computer", "user"];
    let config_type_index = Select::new()
        .with_theme(&ColorfulTheme::default())
        .with_prompt("Select config type")
        .items(&config_types)
        .default(0)
        .interact()?;

    print_separator();

    // Search package (optional)
    let search_package: String = Input::new()
        .with_theme(&ColorfulTheme::default())
        .with_prompt("Enter search package (press Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    // Build the URL
    let mut url = base_url.clone();
    let mut query_params = Vec::new();

    // Add selected platforms
    for &idx in &selected_platforms {
        query_params.push(format!("platform%5B%5D={}", encode(platforms[idx])));
    }

    // Add config type if selected
    if config_type_index > 0 {
        query_params.push(format!("configType={}", encode(config_types[config_type_index])));
    }

    // Add search package if provided
    if !search_package.is_empty() {
        query_params.push(format!("searchPackage={}", encode(&search_package)));
    }

    // Combine URL with query parameters
    if !query_params.is_empty() {
        url = format!("{}?{}", url, query_params.join("&"));
    }

    print_separator();

    // Show request summary
    println!("REQUEST SUMMARY");
    println!("--------------");
    println!("Method: GET");
    println!("URL: {}", url);
    println!("\nHeaders:");
    println!("Authorization: {}", auth_token);
    println!("Accept: */*");
    println!("Host: {}", host);
    println!("\nSelected Platforms: {:?}", selected_platforms.iter().map(|&idx| platforms[idx]).collect::<Vec<_>>());
    if config_type_index > 0 {
        println!("Config Type: {}", config_types[config_type_index]);
    }
    if !search_package.is_empty() {
        println!("Search Package: {}", search_package);
    }

    print_separator();

    // Confirm before sending
    let proceed = dialoguer::Confirm::new()
        .with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to send this request?")
        .interact()?;

    if !proceed {
        println!("Request cancelled");
        return Ok(());
    }

    println!("Sending GET request...\n");

    // Create client with SSL configuration
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // Make the request with headers
    let response = client
        .get(&url)
        .header("Authorization", &auth_token)
        .header("Accept", "*/*")
        .header("Host", &host)
        .send()
        .await?;

    // Print response details
    println!("RESPONSE DETAILS");
    println!("---------------");
    println!("Status: {}", response.status());

    let text = response.text().await?;
    println!("\nResponse Body:");
    println!("{}", format_json_simple(&text));

    println!("\nPress Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}