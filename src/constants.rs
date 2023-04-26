use once_cell::sync::Lazy;

/// Application description
pub static DESCRIPTION: Lazy<String> = Lazy::new(|| -> String {
	format!(
		"\x1b[1;4m{} {}:\x1b[0m {}\n",
		APPNAME.as_str(),
		VERSION.to_owned(),
		env!("CARGO_PKG_DESCRIPTION").to_owned(),
	)
});

/// Application name
pub static APPNAME: Lazy<String> = Lazy::new(|| -> String {
	// capitalize the first letter of the application name
	let mut appname = env!("CARGO_PKG_NAME").to_owned();
	appname.replace_range(..1, &appname[..1].to_uppercase());
	appname
});

/// Application author
pub static AUTHOR: Lazy<String> = Lazy::new(|| -> String { env!("CARGO_PKG_AUTHORS").to_owned() });

fn cargo_ver() -> &'static str {
	env!("CARGO_PKG_VERSION")
}

/// returns the git commit hash at compile time
fn git_commit_hash() -> &'static str {
	env!("GIT_COMMIT_HASH")
}

/// Verbose version string
/// ```text
/// v<version>+<commit hash>
/// ```
pub static LONG_VERSION: Lazy<String> = Lazy::new(|| -> String { format!("v{}+{}", cargo_ver(), git_commit_hash()) });

/// Short version string
/// ```text
/// v<version>
/// ```
pub static VERSION: Lazy<String> = Lazy::new(|| -> String { format!("v{}", cargo_ver().to_owned()) });
