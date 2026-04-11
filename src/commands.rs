use seahorse::Command;

use crate::actions::{
	clear_action, get_action, init_action, list_action, list_pop_action, list_push_action,
	remove_action, set_action,
};
use crate::flags::{force_create, ignore_null};

pub fn init() -> Command {
	Command::new("init")
		.description("inits config file")
		.alias("i")
		.usage(format!("{} init", env!("CARGO_PKG_NAME")))
		.action_with_result(init_action)
}

pub fn list() -> Command {
	Command::new("list")
		.description("list all keys and values")
		.alias("l")
		.usage(format!("{} list", env!("CARGO_PKG_NAME")))
		.action_with_result(list_action)
		.flag(force_create())
}

pub fn clear() -> Command {
	Command::new("clear")
		.description("clear your config file")
		.alias("c")
		.usage(format!("{} clear", env!("CARGO_PKG_NAME")))
		.action_with_result(clear_action)
}

pub fn remove_value() -> Command {
	Command::new("remove")
		.description("remove a value")
		.alias("r")
		.usage(format!("{} remove foo", env!("CARGO_PKG_NAME")))
		.action_with_result(remove_action)
}

pub fn get_value() -> Command {
	Command::new("get")
		.description("get a value")
		.alias("g")
		.usage(format!("{} get foo", env!("CARGO_PKG_NAME")))
		.action_with_result(get_action)
		.flag(ignore_null())
		.flag(force_create())
}

pub fn set_value() -> Command {
	Command::new("set")
		.description("set a value")
		.alias("s")
		.usage(format!("{} set foo bar", env!("CARGO_PKG_NAME")))
		.action_with_result(set_action)
		.flag(force_create())
}

pub fn list_push() -> Command {
	Command::new("lpush")
		.description("push a value into a list")
		.alias("lpu")
		.usage(format!("{} lpush foo bar", env!("CARGO_PKG_NAME")))
		.action_with_result(list_push_action)
		.flag(force_create())
}

pub fn list_pop() -> Command {
	Command::new("lpop")
		.description("pop a value from a list")
		.alias("lpo")
		.usage(format!("{} lpop foo bar", env!("CARGO_PKG_NAME")))
		.action_with_result(list_pop_action)
		.flag(force_create())
}
