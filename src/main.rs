use structopt::StructOpt;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ResponseJson {
	id: i32,
	#[serde(rename = "type")]
	t: String,
	setup: String,
	punchline: String
}

fn main() {
	let cli = Cli::from_args();
	if cli.programming_joke {
		let joke = get_joke::<Vec<ResponseJson>>(
			"https://official-joke-api.appspot.com/jokes/programming/random"
		);
		println!("{}", joke[0].setup);
		if !cli.no_delay {
			sleep(std::time::Duration::from_millis(2500));
		}
		println!("{}", joke[0].punchline);
	} else {
		let joke = get_joke::<ResponseJson>(
			"https://official-joke-api.appspot.com/random_joke"
		);
		println!("{}", joke.setup);
		if !cli.no_delay {
			sleep(std::time::Duration::from_millis(2500));
		}
		println!("{}", joke.punchline);
	}
}

fn get_joke<T>(joke_link: &str) -> T where T: serde::de::DeserializeOwned {
	reqwest::blocking::get(joke_link)
			.expect("Could not send request")
			.json::<T>()
			.expect("Could parse JSON")
}

fn sleep(t: std::time::Duration) {
	std::thread::sleep(t)
}

#[derive(StructOpt)]
#[structopt(name = "Joke CLI", usage = "Run for a random joke")]
struct Cli {
	/// Gets a random programming joke instead of random joke
	#[structopt(short, long)]
	programming_joke: bool,

	#[structopt(short, long)]
	no_delay: bool
}
