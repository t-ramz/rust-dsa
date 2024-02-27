fn merge<T: PartialOrd>(dest_arr: &mut Vec<T>, left_sub: &Vec<T>, right_sub: &Vec<T>, start: usize, end: usize) -> ()
where T: Clone {
	let mut left_idx: usize = 0;
	let mut right_idx: usize = 0;

	let dest_size = end - start + 1;

	let left_size = left_sub.len();
	let right_size = right_sub.len();

	// we want to sort every element, but there is no need to use the index from for loop
	for _ in 0..dest_size {
		let left_in_bounds = left_size > left_idx;
		let right_in_bounds = right_size > right_idx;
		if left_in_bounds && right_in_bounds && &left_sub[left_idx] < &right_sub[right_idx] {
			let dest_val = left_sub.get(left_idx).unwrap().clone();
			dest_arr.push(dest_val);
			left_idx += 1;
		}
		else if right_in_bounds {
			let dest_val = right_sub.get(right_idx).unwrap().clone();
			dest_arr.push(dest_val);
			right_idx += 1;
		}
		else {
			let dest_val = left_sub.get(left_idx).unwrap().clone();
			dest_arr.push(dest_val);
			left_idx += 1;
		}
	}
}

pub fn merge_sort<T: PartialOrd>(arr_to_sort: &Vec<T>, start: usize, end: usize) -> Vec<T>
where T: Clone {
	let mut sorted_array: Vec<T> = Vec::with_capacity(end + 1 - start); // NOTE: this leads to massive space complexity
	if start < end {
		let partition_split = ((end - start) / 2) + start; // NOTE: floor taken
		// split left
		let left_sub = merge_sort(&arr_to_sort, start, partition_split);
		// split right
		let right_sub = merge_sort(&arr_to_sort, partition_split + 1, end);

		merge(&mut sorted_array, &left_sub, &right_sub, start, end);
	}
	else if start == end { // down to single index
		let dest_value = arr_to_sort.get(end).unwrap().clone();
		sorted_array.push(dest_value)
	}

	sorted_array
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sort_ints() {
		let arr_to_sort = vec![23, 10, 11, 42, 7, 14, 92, 11, 64, 32, 71, 9];
		let arr_end_idx = arr_to_sort.len() - 1;
		let sorted_array = merge_sort(&arr_to_sort, 0, arr_end_idx);

		assert_eq!(sorted_array, vec![7, 9, 10, 11, 11, 14, 23, 32, 42, 64, 71, 92]);
	}

	#[test]
	fn lorem_string() {
		let mut my_arr: Vec<char> = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sit amet mattis vulputate enim. Quis auctor elit sed vulputate mi sit amet mauris. Suspendisse ultrices gravida dictum fusce ut placerat orci nulla. Dui accumsan sit amet nulla. Eu nisl nunc mi ipsum faucibus vitae aliquet nec ullamcorper. Erat pellentesque adipiscing commodo elit at imperdiet dui accumsan. Integer malesuada nunc vel risus commodo. Nisl pretium fusce id velit ut tortor pretium viverra. Vulputate odio ut enim blandit. Dui vivamus arcu felis bibendum ut tristique et egestas.".chars().collect();
		let mut test_arr = my_arr.clone();
		test_arr.sort();
		let end  = my_arr.len() - 1;
		let sorted_array = merge_sort(&my_arr, 0, end);
		assert_eq!(sorted_array, test_arr)
	}
}