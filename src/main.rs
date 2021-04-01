mod api;
fn main() {
	let socials = api::fetch_socials().expect("Failed to get social media accounts");
	println!("{:?}", socials);
}
