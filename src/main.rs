use std::thread;
use std::time::Duration;

use reqwest::blocking::Client;
use shorty::update_social_links;
use tracing::info;
use tracing_subscriber;

mod api;
mod shorty;
fn main() {
	tracing_subscriber::fmt::init();
	let client = Client::new();
	info!("Created client");
	let sleep_time = Duration::from_secs(60);

	loop {
		let socials = api::fetch_socials(&client).expect("Failed to get social media accounts");
		info!("Got social data from API");

		update_social_links(&client, socials).expect("Failed to update social links");
		info!("Updated social links");

		thread::sleep(sleep_time);
	}
}
