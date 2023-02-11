//! Substrate Node Template CLI library.
#![warn(missing_docs)]
mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;
mod streams;
fn main() -> Result<(), sc_cli::Error> {
	command::run()
}
