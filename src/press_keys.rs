use openaction::*;

use enigo::{Enigo, DSL, Settings};

pub fn key_down(event: KeyEvent) -> EventHandlerResult {
	if let Some(value) = event.payload.settings.as_object().unwrap().get("down") {
		let mut enigo = Enigo::new(&Settings::default())?;
		enigo.key_sequence_parse(value.as_str().unwrap());
	}

	Ok(())
}

pub fn key_up(event: KeyEvent) -> EventHandlerResult {
	if let Some(value) = event.payload.settings.as_object().unwrap().get("up") {
		let mut enigo = Enigo::new(&Settings::default())?;
		enigo.key_sequence_parse(value.as_str().unwrap());
	}

	Ok(())
}
