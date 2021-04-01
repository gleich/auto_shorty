use reqwest::blocking::Client;
use shorty::update_social_links;

mod api;
mod shorty;
fn main() {
	let client = Client::new();
	let socials = api::fetch_socials(&client).expect("Failed to get social media accounts");
	update_social_links(&client, socials).expect("Failed to update social links");
}
