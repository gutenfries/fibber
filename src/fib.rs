/// Calculates the given fib sequence for the given range, `n`
///
/// runtime complexity: O(n)
///
/// ## Args:
///
/// - `n`: the range of the sequence to calculate
///
/// ## Returns:
///
/// - `Vec<u128>` the calculated sequence
pub fn fibonacci(n: u128) -> Vec<u128> {
	let mut sequence: Vec<u128> = Vec::with_capacity(n as usize);

	let mut prev = 0;
	let mut next = 1;

	for _ in 0..n {
		// current element in the sequence
		let current = prev + next;

		// push the current number to the vector

		// NOTE: we push the number to the vec BEFORE the operations below as
		// to include the 0th element (0)
		sequence.push(current);

		// now we are moving on, so the previous number is now the current number
		prev = next;
		// the next number is also now the current number
		next = current;
	}

	sequence
}

/// calculates the nth fibonacci number
///
/// runtime complexity: O(n)
///
/// ## Args:
///
/// - `n`: the nth number of the sequence to return
///
/// ## Returns:
/// ` `u128`: the nth term of the sequence
pub fn n_fibonacci(n: u128) -> u128 {
	let mut prev = 0;
	let mut next = 1;

	for _ in 0..n {
		// current element in the sequence
		let current = prev + next;

		// now we are moving on, so the previous number is now the current number
		prev = next;
		// the next number is also now the current number
		next = current;
	}

	// if the loop has iterated through the values 0-n, then `next` is the nth element.
	next
}
