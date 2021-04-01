use std::env;

use anyhow::{bail, Context};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::json;
use tracing::info;

use crate::api::Social;

pub fn update_social_links(client: &Client, socials: Vec<Social>) -> anyhow::Result<()> {
	let shorty_token = env::var("SHORTY_TOKEN")?;
	for social in socials.iter() {
		// Making request
		let response = client
			.patch(format!(
				"https://links.mattglei.ch/api/link/{}",
				social.name
			))
			.json(&json!({"url": social.url, "description": social.description}))
			.bearer_auth(&shorty_token)
			.send()
			.context(format!("Failed to update link /{}", social.name))?;

		// Checking response status code
		let status = response.status();
		if status != StatusCode::OK {
			bail!(
				"Failed to update link of {} with status code of {}",
				social.name,
				status
			)
		}

		info!("Update link for /{}", social.name)
	}

	Ok(())
}
