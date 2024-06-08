mod press_keys;
mod run_command;
mod switch_profile;

use openaction::*;

use simplelog::{CombinedLogger, ConfigBuilder, LevelFilter, WriteLogger};

struct GlobalEventHandler {}
impl openaction::GlobalEventHandler for GlobalEventHandler {}

struct ActionEventHandler {}
impl openaction::ActionEventHandler for ActionEventHandler {
	async fn key_down(
		&self,
		event: KeyEvent,
		_outbound: &mut openaction::OutboundEventManager,
	) -> EventHandlerResult {
		match &event.action[..] {
			"com.amansprojects.starterpack.runcommand" => run_command::key_down(event),
			"com.amansprojects.starterpack.inputsimulation" => press_keys::key_down(event),
			_ => Ok(()),
		}
	}

	async fn key_up(
		&self,
		event: KeyEvent,
		_outbound: &mut openaction::OutboundEventManager,
	) -> EventHandlerResult {
		match &event.action[..] {
			"com.amansprojects.starterpack.runcommand" => run_command::key_up(event),
			"com.amansprojects.starterpack.inputsimulation" => press_keys::key_up(event),
			"com.amansprojects.starterpack.switchprofile" => switch_profile::key_up(event),
			_ => Ok(()),
		}
	}
}

#[tokio::main]
async fn main() {
	CombinedLogger::init(vec![WriteLogger::new(
		LevelFilter::Debug,
		ConfigBuilder::new().add_filter_ignore_str("enigo").build(),
		std::fs::File::create("plugin.log").unwrap(),
	)])
	.unwrap();

	if let Err(error) = init_plugin(GlobalEventHandler {}, ActionEventHandler {}).await {
		log::error!("Failed to initialise plugin: {}", error);
	}
}
