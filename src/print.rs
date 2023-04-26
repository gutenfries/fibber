use crate::Config;

/// prints a given fib sequence to stdout, given a pointer
/// to the global config object, and a reference to sequence to output
///
/// all parsing is O(n) runtime complexity
///
/// ## Args
/// - `seq`: the sequence to print
/// - `config` a pointer to the config object
#[allow(clippy::needless_range_loop)]
pub fn print_fib_seq(seq: Vec<u128>, config: &Config) {
	let mut output: String = String::new();

	match (config.numbering, config.one_line) {
		(true, true) => {
			for i in 0..seq.len() {
				output.push_str(&format!("{}:{} ", i + 1, seq[i]))
			}
		},
		(true, false) => {
			for i in 0..seq.len() {
				output.push_str(&format!("{}:{} \n", i + 1, seq[i]))
			}
		},
		(false, true) => {
			for i in 0..seq.len() {
				output.push_str(&format!("{} ", seq[i]))
			}
		},
		(false, false) => {
			for i in 0..seq.len() {
				output.push_str(&format!("{} \n", seq[i]))
			}
		},
	}

	print!("{}", output);
}

/// prints the nth value of a fib sequence to the stdout
///
/// performs in O(1)
///
/// ## args:
///
/// - `n`: the value to outut
/// - `pos`: the position of the value in the fibbinacci sequence
/// - `config` a pointer to the config object
pub fn print_fib_n(n: u128, pos: u128, config: &Config) {
	match config.numbering {
		true => {
			print!("{}:{}", n, pos)
		},
		false => {
			print!("{}", n)
		},
	}
}
