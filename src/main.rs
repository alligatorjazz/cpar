use clap::{Parser, CommandFactory, error::ErrorKind};
use std::{fs, path::Path, process::{Command}};

/// Copies and mass-renames files using the wildcard character (*)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The files to be copied and renamed.
    #[clap(value_parser)]
    files: Vec<String>,
    /// The destination path for the renamed files
    #[clap(last = true, required = true)]
    destination: String
}


fn process_args(args: Args)  {
    // get the list of paths
    // use last path as destination
	let mut output_files: Vec<String>;
	if !args.destination.contains('*') {
		Args::command().error(
			ErrorKind::InvalidValue, 
			format!(
				"cpar only works on paths with wildcards. wildcard was not found in destination path {}", 
				args.destination
			)
		).exit();
	}

	for filename in &args.files {
		// assert!(filename.contains('*'));
		println!("{}", filename.contains('*'));
		if !filename.contains('*') {
			Args::command().error(
				ErrorKind::InvalidValue, 
				format!(
					"cpar only works on paths with wildcards. wildcard was not found in input path {}", 
					filename
				)
			).exit();
		}

		let path = Path::new(&filename);
		
		if path.is_dir() {
			Args::command().error(ErrorKind::InvalidValue, "cpar does not accept directories.").exit();
		}	
	}

	println!("{:?}", args.files)
    
    // cpar src/ContactForm* src/MembershipForm* ->
    // ContactForm.tsx -> MembershipForm.tsx
    // ContactForm.scss -> MembershipForm.scss
	// or
	// cpar src/*.ts -> src/*.js
	// script.ts -> script.js
    // etc.
}

fn main() {
    let args = Args::parse();
	process_args(args)
    // println!("Hello {:?}! I'm sending you to {:?}", args.files, args.destination)
}
