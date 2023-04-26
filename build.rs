//! this code is compile time run and snapshotted,
//! it is used in this implimentation to capture the
//! git hash at compile time

/// Entry point for the build script
pub fn main() {
	/// Super small git wrapper function
	///
	/// ## Args
	///
	/// * `args` - A vector of arguments to pass to git
	///
	/// ## Returns
	///
	/// * `String` - The output of the git command
	///
	/// ## Panics
	///
	/// * If the git command fails
	/// * If the output of the git command is not valid utf8
	/// * If git is not installed on the compiler's system
	fn git<T>(args: Vec<T>) -> String
	where T: AsRef<std::ffi::OsStr> {
		let output = std::process::Command::new("git").args(args).output().unwrap();
		String::from_utf8(output.stdout).unwrap()
	}

	// set the git commit hash as an environment variable available to the compiler
	println!(
		"cargo:rustc-env=GIT_COMMIT_HASH={}",
		&git(vec!["rev-parse", "--short", "HEAD"])
	);
}
