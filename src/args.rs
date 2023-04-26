use clap::Parser;

use crate::constants;

/// The main Cli object, containing all the command line arguments
/// and options
#[derive(Debug, Parser)]
#[command(name = constants::APPNAME.as_str())]
#[command(author = constants::AUTHOR.as_str())]
#[command(about = constants::DESCRIPTION.as_str())]
#[command(version = constants::LONG_VERSION.as_str())]
pub struct Cli {
	/// Explicitly set the log level to verbose
	#[clap(short = 'v', long, default_value_t = false)]
	pub verbose: bool,

	/// Explicitly set the log level to quiet (overrides verbose)
	#[clap(short = 'q', long, default_value_t = false)]
	pub quiet: bool,

	/// the length of the sequence to output, i.e.:
	/// 0, 1, 1, 2, 3, 5 would be the result of -c 6
	#[clap(short = 'c', long, required = true)]
	pub count: u128,

	/// Print on one line
	#[clap(short = '1', long, required = false, default_value_t = false)]
	pub one_line: bool,

	/// Preface each number in the sequence with it's position within the sequence, i.e:
	/// 1:0, 2:1, 3:1, 4:2, 5:3, 6:5
	#[clap(long, required = false, default_value_t = false)]
	pub numbering: bool,

	/// Print only the last number of the sequence for the given count
	#[clap(long, required = false, default_value_t = false)]
	pub last_only: bool,
}
