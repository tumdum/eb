/// Clamp a value within a range
///
/// This function is taken from the standard library's `clamp` function which
/// is implemented on `core::cmp::Ord`.  That specific behavior has not yet
/// been stabilized.
///
/// **This code is stable, but should not be used**, as it will go away once
/// `Ord`'s implementation of the `clamp` API has been stabilized.
///
/// # Examples
///
/// ```rust
/// # use eb::clamp;
/// // Values in the range are returned unmodified
/// assert_eq!(clamp(1_u32, 0_u32, 2_u32), 1_u32);
/// ```
///
/// ```rust
/// # use eb::clamp;
/// // Values below the range cause `clamp` to return the lower bound
/// assert_eq!(clamp(0_u32, 1_u32, 2_u32), 1_u32);
/// ```
///
/// ```rust
/// # use eb::clamp;
/// // Values above the range cause `clamp` to return the upper bound
/// assert_eq!(clamp(2_u32, 0_u32, 1_u32), 1_u32);
/// ```
pub fn clamp<T>(value: T, min: T, max: T) -> T
where
	T: PartialOrd,
{
	assert!(min <= max);

	if value < min {
		min
	} else if value > max {
		max
	} else {
		value
	}
}

#[cfg(test)]
mod tests {
	use super::clamp;

	mod i32 {
		use super::clamp;

		#[test]
		fn inside_range() {
			assert_eq!(clamp(1_i32, 0_i32, 2_i32), 1_i32);
		}

		#[test]
		fn below_range() {
			assert_eq!(clamp(-1_i32, 0_i32, 2_i32), 0_i32);
		}

		#[test]
		fn above_range() {
			assert_eq!(clamp(3_i32, 0_i32, 2_i32), 2_i32);
		}

		#[test]
		#[should_panic]
		fn max_gt_min() {
			clamp(1_i32, 2_i32, 0_i32);
		}
	}

	mod u32 {
		use super::clamp;

		#[test]
		fn inside_range() {
			assert_eq!(clamp(1_u32, 0_u32, 2_u32), 1_u32);
		}

		#[test]
		fn below_range() {
			assert_eq!(clamp(0_u32, 1_u32, 2_u32), 1_u32);
		}

		#[test]
		fn above_range() {
			assert_eq!(clamp(3_u32, 0_u32, 2_u32), 2_u32);
		}

		#[test]
		#[should_panic]
		fn max_gt_min() {
			clamp(1_u32, 2_u32, 0_u32);
		}
	}
}
