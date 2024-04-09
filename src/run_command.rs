use openaction::*;

pub fn key_down(event: KeyEvent) -> EventHandlerResult {
	if let Some(value) = event.payload.settings.as_object().unwrap().get("down") {
		std::process::Command::new("sh").arg("-c").arg(value.as_str().unwrap()).spawn()?;
	}

	Ok(())
}

pub fn key_up(event: KeyEvent) -> EventHandlerResult {
	if let Some(value) = event.payload.settings.as_object().unwrap().get("up") {
		std::process::Command::new("sh").arg("-c").arg(value.as_str().unwrap()).spawn()?;
	}

	Ok(())
}
