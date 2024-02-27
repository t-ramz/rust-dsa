pub fn binary_search<T: PartialEq + PartialOrd>(arr: &[T], target: T, offset: usize) -> Option<usize> {
	let mut index: Option<usize> = None;
	let arr_len = arr.len();

	let midpoint = arr_len / 2;
	let comp_val = &arr[midpoint];

	if arr_len == 2 { // short circuit
		if *comp_val == target { index = Some(midpoint + offset) }
		else if arr[midpoint - 1] == target { index = Some(midpoint + offset - 1) }
		return index
	}
	else if *comp_val > target {
		index = binary_search(&arr[0..midpoint + 1], target, offset + 0);
	}
	else {
		index = binary_search(&arr[midpoint..arr_len], target, offset + midpoint);
	}

	index
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn search_sorted_list() {
		let my_sorted_vec = vec![1, 3, 5, 9, 15, 22, 32];
		let result = binary_search(&my_sorted_vec, 9, 0);
		assert_eq!(result.unwrap(), 3);
	}
}
