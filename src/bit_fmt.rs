use crate::store::BitStore;
use core::fmt::{self, Write};

pub(crate) struct BinaryDisplay<'a, S: BitStore>(pub(crate) &'a S);

impl<'a, S: BitStore> fmt::Binary for BinaryDisplay<'a, S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let bits = self.0;
		f.write_str("0b")?;
		for bit in (0..S::BITS).rev() {
			if (bit + 1) % 4 == 0 {
				f.write_char('_')?;
			}

			// SAFETY: `bit` is in range.
			let value = unsafe { bits.get(bit) };
			if value {
				f.write_char('1')?;
			} else {
				f.write_char('0')?;
			}
		}

		Ok(())
	}
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
	use super::*;
	use crate::store::{BitStoreConst, BitStoreMut};
	use alloc::format;

	struct DisplayWrapper<S: BitStore>(S);

	impl<S: BitStore> fmt::Display for DisplayWrapper<S> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			fmt::Binary::fmt(&BinaryDisplay(&self.0), f)
		}
	}

	#[test]
	fn test_fmt() {
		assert_eq!(
			format!("{}", DisplayWrapper(0b_0000_0000u8)),
			"0b_0000_0000"
		);

		assert_eq!(
			format!("{}", DisplayWrapper(<u8 as BitStoreConst>::FULL)),
			"0b_1111_1111"
		);

		let mut bits = [0u8; 2];
		unsafe { BitStoreMut::set(&mut bits, 2) };
		unsafe { BitStoreMut::set(&mut bits, 12) };
		assert_eq!(
			format!("{}", DisplayWrapper(bits)),
			"0b_0001_0000_0000_0100"
		);
	}
}
