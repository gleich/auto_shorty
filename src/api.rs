use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Social {
	pub name: String,
	pub description: String,
	pub url: String,
}

pub fn fetch_socials() -> Result<Vec<Social>, anyhow::Error> {
	let mut query = HashMap::new();
	query.insert(
		"query",
		"query { socials { accounts { name, url, description } } }",
	);

	// Making request
	let client = Client::new();
	let response = client
		.post("https://gql.api.mattglei.ch")
		.json(&query)
		.send()
		.context("Failed to send request")?;
	if response.status() != StatusCode::OK {
		bail!("Status code of {} not equal to 200 (ok)", response.status());
	}

	// Parsing response
	let response_body = response.text().context("Failed to get output of request")?;
	let accounts: Value =
		serde_json::from_str(response_body.as_str()).context("Failed to parse response")?;

	// Collecting vector of Social
	let mut socials: Vec<Social> = Vec::new();
	for account in accounts["data"]["socials"]["accounts"]
		.as_array()
		.unwrap()
		.iter()
	{
		socials.push(Social {
			name: account["name"].to_string().replace("\"", ""),
			description: account["description"].to_string().replace("\"", ""),
			url: account["url"].to_string().replace("\"", ""),
		})
	}

	Ok(socials)
}
