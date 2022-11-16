mod cli;
use clap::Parser;
use cli::{process_args, Args};

fn main() {
    let args = Args::parse();
    process_args(args)
}

#[cfg(test)]
mod tests {
    use crate::cli::Args;
    use std::{
        fs,
        path::Path,
        process::{Command, ExitStatus},
    };

    const CACHE: &str = ".test-cache";

    struct TestResult {
        pub status: ExitStatus,
        pub stdout: String,
        pub stderr: String,
    }

    // runs cpar as a bash command to directly debug stderr and stdout
    fn run_cpar(args: Args) -> TestResult {
        let result = Command::new("./target/debug/cpar")
            .arg(args.input_path)
            .arg(args.output_path)
            .output()
            .expect("cpar failed to execute");

        TestResult {
            status: result.status,
            stdout: String::from_utf8_lossy(&result.stdout).to_string(),
            stderr: String::from_utf8_lossy(&result.stderr).to_string()
        }
    }

    fn init_files(files: Vec<&str>) {
        fs::create_dir_all(CACHE).unwrap();
        for filename in files {
            let path = Path::new(CACHE).join(filename);
            fs::write(
                path,
                "// not intended for use.\nconsole.log('Hello World!');",
            )
            .unwrap()
        }
    }

    fn cleanup_files(success: bool) {
        fs::remove_dir_all(CACHE).unwrap();
        if !success {
            panic!()
        }
    }

    #[test]
    fn only_accepts_args_with_variable() {
        let args = Args {
            input_path: "fake-file.js".to_string(),
            output_path: "fake-file.ts".to_string(),
        };

		let result = run_cpar(args);
		println!("{}", &result.stderr);
        if result.stderr.is_empty() {
            panic!()
        }
    }

	// TODO: use real testing library for setup / cleanup
	// TODO: create shorthands for path.join

    // test case for name.ext -> new_name.ext
    #[test]
    fn copies_multiple_names() {
        let files = vec!["temp-1.js", "temp-2.js"];
        let targets = vec!["temp-1.ts", "temp-2.ts"];
        init_files(files);

        let args = Args {
            input_path: String::from(".test-cache/$.js"),
            output_path: String::from(".test-cache/$.ts"),
        };

        println!("{:?}", args);

        let result = run_cpar(args);
		println!("\n\n-- ERROR --\n{}", &result.status);
        if result.status.code() != Some(0) {
            panic!()
        }

        let paths: Vec<String> = fs::read_dir(CACHE)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .map(|buffer| buffer.display().to_string())
            .collect();

		println!("{:?}", paths);

        for target in targets {
            if !paths.contains(&format!(".test-cache/{}", target)) {
                panic!("{}", target)
            }
        }

        cleanup_files(true);
    }
}
