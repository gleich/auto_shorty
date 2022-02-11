use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
pub struct Social {
	pub name: String,
	pub description: String,
	pub url: String,
}

pub fn fetch_socials(client: &Client) -> Result<Vec<Social>, anyhow::Error> {
	// Making request
	let response = client
		.post("https://graphql.mattglei.ch")
		.json(&json!({"query": "query { socials { accounts { name, url, description } } }"}))
		.send()
		.with_context(|| "Failed to send request")?;
	anyhow::ensure!(
		response.status() == StatusCode::OK,
		"Response didn't have status code of 200"
	);

	// Parsing response
	let accounts: Value = serde_json::from_str(
		&response
			.text()
			.with_context(|| "Failed to get output of request")?,
	)
	.with_context(|| "Failed to parse response")?;

	// Collecting vector of Social
	let mut socials: Vec<Social> = Vec::new();
	for account in accounts["data"]["socials"]["accounts"]
		.as_array()
		.unwrap()
		.iter()
	{
		socials.push(
			serde_json::from_value(account.to_owned())
				.with_context(|| "Failed to parse a specific social account")?,
		);
	}

	Ok(socials)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fetch_socials() -> Result<(), anyhow::Error> {
		let client = Client::new();
		let result = fetch_socials(&client)?;
		assert!(!result.is_empty());
		Ok(())
	}
}
