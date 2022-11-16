use clap::{error::ErrorKind, CommandFactory, Parser};
use glob::glob;
use std::{fs, path::Path};

/// Copies and mass-renames files using the variable character ($)
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
	name = "cpar", 
	author = "Falchion Studios",
	version = VERSION, 
	about = "Copies and mass-renames files using the variable character ($)", 
	long_about = None
)]
pub struct Args {
    /// The files to be copied and renamed.
	#[arg(required = true)]
    pub input_path: String,

	#[arg(required = true)]
    /// The path for the renamed files
    pub output_path: String,
}

pub fn process_args(args: Args) {
    // get the list of paths
    if !args.output_path.contains('$') {
        Args::command()
            .error(
                ErrorKind::InvalidValue,
                format!(
				"cpar only works on paths with $. $ was not found in output_path path {}", 
				args.output_path
			),
            )
            .exit();
    }

    
	if !args.input_path.contains('$') {
		Args::command()
			.error(
				ErrorKind::InvalidValue,
				format!(
				"cpar only works on paths with $. $ was not found in input path {}", 
				args.input_path
			),
			)
			.exit();
	}

	if Path::new(&args.input_path).is_dir() {
		Args::command()
			.error(ErrorKind::InvalidValue, "cpar does not accept directories.")
			.exit();
	}
	
	// gets text surrounding variable in args.input_path
	let text_around_variable = args.input_path.split_once('$').unwrap_or(("", ""));
	println!("text around: {:?}", text_around_variable);

	let mut unpacked_files: Vec<String> = vec![];
	let glob = glob(&args.input_path.replace('$', "*"))
		.expect("failed to process input glob pattern");
	
	for entry in glob {
		match entry {
			Ok(path) => unpacked_files.push(path.display().to_string()),
			Err(e) => Args::command()
				.error(ErrorKind::InvalidValue, format!("{:?}", e))
				.exit()
		}
	}

	

	// TODO: handle output file name collision
	// TODO: have case for when input files do not actually exist
	for unpacked_file in unpacked_files {
		// extract variable value from individual args.input_path
		let variable_value = unpacked_file
			.replace(text_around_variable.0, "")
			.replace(text_around_variable.1, "");
		
		// create output path
		let output_path = args.output_path.replace('$', variable_value.trim());

		// debug
		println!("{} -> {}", unpacked_file, output_path);
		let result = fs::copy(unpacked_file, output_path);
		match result {
			Ok(_) => (),
			Err(err) => Args::command()
				.error(ErrorKind::ValueValidation, err)
				.exit(),
		}
	}
}
