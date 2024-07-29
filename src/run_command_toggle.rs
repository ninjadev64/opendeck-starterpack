use std::fs::File;
use std::process::{Command, Stdio};

use openaction::*;

pub fn key_down(event: KeyEvent) -> EventHandlerResult {
	let setting = if event.payload.state == 0 {
		"s1_down"
	} else {
		"s2_down"
	};
	if let Some(value) = event.payload.settings.as_object().unwrap().get(setting) {
		let file = File::create("command.log")?;
		Command::new("sh")
			.arg("-c")
			.arg(value.as_str().unwrap())
			.stdout(Stdio::from(file.try_clone()?))
			.stderr(Stdio::from(file))
			.spawn()?;
	}

	Ok(())
}

pub fn key_up(event: KeyEvent) -> EventHandlerResult {
	let setting = if event.payload.state == 0 {
		"s1_up"
	} else {
		"s2_up"
	};
	if let Some(value) = event.payload.settings.as_object().unwrap().get(setting) {
		let file = File::create("command.log")?;
		Command::new("sh")
			.arg("-c")
			.arg(value.as_str().unwrap())
			.stdout(Stdio::from(file.try_clone()?))
			.stderr(Stdio::from(file))
			.spawn()?;
	}

	Ok(())
}
