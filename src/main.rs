mod cli;
use clap::Parser;
use cli::{process_args, Args};

fn main() {
    let args = Args::parse();
    process_args(args)
}

#[cfg(test)]
mod tests {
    use std::{process::{Command, Output}, io::Error, result::Result, fs};
    use crate::cli::Args;

	// runs cpar as a bash command to directly debug stderr and stdout
    fn run_cpar(args: Args) -> Result<Output, Error>  {
        Command::new("cpar")
			.arg(args.input_path)
			.arg(args.output_path)
			.output()
    }

	fn create_test_files(files: Vec<&str>) {
		//
		for filename in files
	}

    #[test]
    fn only_accepts_args_with_variable() {
        let args = Args {
            input_path: "fake-file.js".to_string(),
            output_path: "fake-file.ts".to_string(),
        };

		if run_cpar(args).is_ok() { panic!() }
    }

	// test case for name.ext -> new_name.ext
	fn copies_multiple_names() {
		// creates test directories
		fs::create_dir(".test-cache/").unwrap();
		fs::write(".test-cache/temp-1.js", r#"// not intended for use"#).unwrap();
		fs::write(".test-cache/temp-2.js", r#"// not intended for use"#).unwrap();

		let args = Args {
            input_path: "fake-file.js".to_string(),
            output_path: "fake-file.ts".to_string(),
        };

		if run_cpar(args).is_ok() { panic!() }
	}
}
