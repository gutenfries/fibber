use clap::ValueEnum;

/// Log verbosity level
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Verbosity {
	/// Sets the log level to no output
	Quiet,
	/// Sets the log level to normal output (default)
	Normal,
	/// Sets the log level to verbose output
	Verbose,
}

impl Default for Verbosity {
	/// Default verbosity level
	fn default() -> Self {
		Self::Normal
	}
}
