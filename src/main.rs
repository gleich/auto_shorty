use std::thread;
use std::time::Duration;

use reqwest::blocking::Client;
use tracing::info;
use tracing_subscriber;

mod api;
mod shorty;

fn main() {
	tracing_subscriber::fmt::init();
	let client = Client::new();
	info!("Created client");
	let sleep_time = Duration::from_secs(60 * 5);

	loop {
		let socials = api::fetch_socials(&client).expect("Failed to get social media accounts");
		info!("Got social data from API");

		let links = shorty::get_links(&client).expect("Failed to get shorty links");
		info!("Got shorty links");

		shorty::update_social_links(&client, socials, links)
			.expect("Failed to update social links");
		info!("Updated social links");

		thread::sleep(sleep_time);
	}
}
