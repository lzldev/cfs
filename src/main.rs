#![feature(unboxed_closures)]
#![feature(fn_traits)]
use std::{env, process::exit};

use seahorse::App;

use crate::commands::{clear, get_value, init, list, remove_value, set_value};

mod actions;
mod commands;
mod config;
mod error;
mod flags;
mod storage;

fn main() -> anyhow::Result<()> {
	let args: Vec<String> = env::args().collect();
	let app = App::new(env!("CARGO_PKG_NAME"))
		.description(env!("CARGO_PKG_DESCRIPTION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.version(env!("CARGO_PKG_VERSION"))
		.usage(format!("{} [commands]", env!("CARGO_PKG_NAME")))
		.command(set_value())
		.command(get_value())
		.command(list())
		.command(init())
		.command(remove_value())
		.command(clear());

	match app.run_with_result(args) {
		Ok(_) => (),
		Err(action_error) => {
			eprintln!("{}", action_error);
			exit(1)
		}
	};

	Ok(())
}
