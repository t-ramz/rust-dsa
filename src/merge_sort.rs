pub fn merge_sort<T: PartialOrd>(arr_to_sort: &mut Vec<T>) -> Vec<T> {

}

#[cfg(test)]
mod tests {
	use crate::merge_sort;
	use super::*;

	#[test]
	fn it_works() {
		let mut arr_to_sort = vec![23, 10, 11, 42, 7, 14, 92, 64, 32, 71, 9];

		let sorted_array = merge_sort(&mut arr_to_sort);

		assert_eq!(sorted_array, vec![7, 9, 10, 11, 14, 23, 32, 42, 64, 71, 92]);
	}
}