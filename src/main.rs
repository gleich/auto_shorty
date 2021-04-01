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

	let socials = api::fetch_socials(&client).expect("Failed to get social media accounts");
	info!("Got social data from API");

	update_social_links(&client, socials).expect("Failed to update social links");
	info!("Update social links")
}
