use std::fs::File;
use std::process::{Command, Stdio};

use openaction::*;

fn run_command(value: &str) -> EventHandlerResult {
	#[cfg(unix)]
	let command = if std::env::var("container").is_ok() {
		"flatpak-spawn"
	} else {
		"sh"
	};
	#[cfg(unix)]
	let extra_args = if std::env::var("container").is_ok() {
		vec!["--host", "sh", "-c"]
	} else {
		vec!["-c"]
	};

	#[cfg(windows)]
	let command = "cmd";
	#[cfg(windows)]
	let extra_args = ["/C"];

	let file = File::create("command.log")?;
	Command::new(command)
		.args(extra_args)
		.arg(value)
		.stdout(Stdio::from(file.try_clone()?))
		.stderr(Stdio::from(file))
		.spawn()?;

	Ok(())
}

pub fn key_down(event: KeyEvent) -> EventHandlerResult {
	if let Some(value) = event.payload.settings.as_object().unwrap().get("down") {
		run_command(value.as_str().unwrap())
	} else {
		Ok(())
	}
}

pub fn key_up(event: KeyEvent) -> EventHandlerResult {
	if let Some(value) = event.payload.settings.as_object().unwrap().get("up") {
		run_command(value.as_str().unwrap())
	} else {
		Ok(())
	}
}
