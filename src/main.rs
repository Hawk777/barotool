#![forbid(unsafe_code)]
#![warn(
	// Turn on extra language lints.
	future_incompatible,
	missing_abi,
	nonstandard_style,
	rust_2018_idioms,
	// Disabled due to <https://github.com/rust-lang/rust/issues/69952>.
	// single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unused,
	unused_crate_dependencies,
	unused_import_braces,
	unused_lifetimes,
	unused_qualifications,

	// Turn on extra Rustdoc lints.
	rustdoc::all,

	// Turn on extra Clippy lints.
	clippy::cargo,
	clippy::pedantic,
)]

use clap::{Arg, Command};
use std::collections::HashSet;

mod save;
mod submarine;

fn make_clap_command() -> Command<'static> {
	Command::new("barotool")
		.author(clap::crate_authors!())
		.about("Manipulates Barotrauma save files and submarines.")
		.version(clap::crate_version!())
		.infer_subcommands(true)
		.subcommand_required(true)
		.arg_required_else_help(true)
		.arg(Arg::new("verbose")
			.short('v')
			.long("verbose")
			.help("Displays more information while running")
			.global(true))
		.subcommand(Command::new("clear-containers")
			.about("Removes all items form inside all containers in a submarine.")
			.arg(Arg::new("submarine")
				.help("The .sub file to modify")
				.required(true)
				.allow_invalid_utf8(true)))
		.subcommand(Command::new("ident-submarine")
			.about("Parses and re-saves a submarine file, not modifying it, verifying that the data structures are complete.")
			.arg(Arg::new("submarine")
				.help("The .sub file to rewrite")
				.required(true)
				.allow_invalid_utf8(true)))
		.subcommand(Command::new("list-save")
			.about("Lists the files contained within a .save file.")
			.arg(Arg::new("save")
				.help("The .save file to read")
				.required(true)
				.allow_invalid_utf8(true)))
		.subcommand(Command::new("pack-save")
			.about("Creates a .save file, packing it with other files.")
			.arg(Arg::new("save")
				.help("The .save file to create")
				.required(true)
				.allow_invalid_utf8(true))
			.arg(Arg::new("members")
				.help("The file(s) to pack into the archive.")
				.required(true)
				.multiple_values(true)))
		.subcommand(Command::new("show-containers")
			.about("Shows a summary of all containers in a submarine without modifying anything.")
			.arg(Arg::new("submarine")
				.help("The .sub file to read")
				.required(true)
				.allow_invalid_utf8(true)))
		.subcommand(Command::new("unpack-save")
			.about("Extracts files from a .save file.")
			.arg(Arg::new("save")
				.help("The .save file to read")
				.required(true)
				.allow_invalid_utf8(true))
			.arg(Arg::new("members")
				.help("The file(s) to extract from the archive (omit to extract all members).")
				.multiple_values(true)))
}

fn main() -> std::io::Result<()> {
	let matches = make_clap_command().get_matches();
	let verbose = matches.is_present("verbose");
	if let Some(matches) = matches.subcommand_matches("clear-containers") {
		let filename = matches.value_of_os("submarine").unwrap();
		submarine::clear_containers(filename, verbose)?;
	}
	if let Some(matches) = matches.subcommand_matches("ident-submarine") {
		let filename = matches.value_of_os("submarine").unwrap();
		submarine::ident(filename)?;
	}
	if let Some(matches) = matches.subcommand_matches("list-save") {
		let filename = matches.value_of_os("save").unwrap();
		save::list(filename)?;
	}
	if let Some(matches) = matches.subcommand_matches("show-containers") {
		let filename = matches.value_of_os("submarine").unwrap();
		submarine::list_containers(filename, verbose)?;
	}
	if let Some(matches) = matches.subcommand_matches("pack-save") {
		let filename = matches.value_of_os("save").unwrap();
		let members = matches.values_of("members").unwrap().collect::<Vec<&str>>();
		save::pack(filename, &members)?;
	}
	if let Some(matches) = matches.subcommand_matches("unpack-save") {
		let filename = matches.value_of_os("save").unwrap();
		let mut members = matches
			.values_of("members")
			.map_or(HashSet::<&str>::new(), Iterator::collect::<HashSet<&str>>);
		save::unpack(filename, &mut members)?;
		if !members.is_empty() {
			eprintln!("Some members were not found:");
			for i in members {
				eprintln!("{}", i);
			}
		}
	}
	Ok(())
}

#[test]
fn test_make_clap_command() {
	make_clap_command().debug_assert()
}
