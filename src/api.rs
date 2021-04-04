use anyhow::{bail, Context};
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

pub fn fetch_socials(client: &Client) -> anyhow::Result<Vec<Social>> {
	// Making request
	let response = client
		.post("https://gql.api.mattglei.ch")
		.json(&json!({"query": "query { socials { accounts { name, url, description } } }"}))
		.send()
		.context("Failed to send request")?;
	let status = response.status();
	if status != StatusCode::OK {
		bail!("Status code of {} not equal to 200 (ok)", status);
	}

	// Parsing response
	let response_body = response.text().context("Failed to get output of request")?;
	let accounts: Value =
		serde_json::from_str(&response_body).context("Failed to parse response")?;

	// Collecting vector of Social
	let mut socials: Vec<Social> = Vec::new();
	for account in accounts["data"]["socials"]["accounts"]
		.as_array()
		.unwrap()
		.iter()
	{
		socials.push(
			serde_json::from_value(account.to_owned())
				.context("Failed to parse a specific social account")?,
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
