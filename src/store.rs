//! Traits for types that can be used to store bits.

/// A trait for types that can be used to store bits.
pub trait BitStore {
	/// The number of bits that can be stored in this type.
	const BITS: u32;

	/// Returns the bit at the given index.
	///
	/// # Safety
	/// The index must be in range 0..[Self::BITS].
	unsafe fn get(&self, index: u32) -> bool;

	/// Returns the number of bits set to 1.
	fn count_ones(&self) -> u32;

	/// Returns the number of trailing bits set to 0.
	fn trailing_zeros(&self) -> u32;

	/// Returns the number of trailing bits set to 1.
	fn trailing_ones(&self) -> u32;

	/// Returns the number of leading bits set to 0.
	fn leading_zeros(&self) -> u32;

	/// Returns the number of leading bits set to 1.
	fn leading_ones(&self) -> u32;

	/// Returns `true` if this bitset is empty, i.e., all bits are unset.
	#[inline]
	fn is_empty(&self) -> bool {
		self.count_ones() == 0
	}

	/// Returns `true` if this bitset is full, i.e., all bits are set.
	#[inline]
	fn is_full(&self) -> bool {
		self.count_ones() == Self::BITS
	}
}

/// A trait for types that can be used to store bits and can be modified.
pub trait BitStoreMut: BitStore {
	/// Sets the bit at the given index to 1.
	///
	/// # Safety
	/// The index must be in range 0..[BitStore::BITS].
	unsafe fn set(&mut self, index: u32);

	/// Sets the bit at the given index to 0.
	///
	/// # Safety
	/// The index must be in range 0..[BitStore::BITS].
	unsafe fn unset(&mut self, index: u32);

	/// Unions this bitset with another, modifying `self` in place.
	fn union_with(&mut self, other: &Self);

	/// Intersects this bitset with another, modifying `self` in place.
	fn intersect_with(&mut self, other: &Self);

	/// Subtracts another bitset from this one, modifying `self` in place.
	fn difference_with(&mut self, other: &Self);

	/// Subtracts this bitset from another one, modifying `self` in place.
	fn symmetric_difference_with(&mut self, other: &Self);

	/// Negates this bitset, modifying `self` in place.
	fn negate(&mut self);
}

/// A trait for types that can be used to store bits and have constants for
/// empty and full bitsets.
pub trait BitStoreConst: BitStore + Sized {
	const EMPTY: Self;
	const FULL: Self;
}

/// A trait for types that have all bits set to 0 when they are created.
///
/// # Safety
/// This marker trait requires that all bits are 0 when [Default::default]
/// is used - or more specifically that `BitStore::is_empty(&Default::default())`
/// is `true`.
pub unsafe trait DefaultIsEmpty: Default {}

// SAFETY: all of these types have all storage-bits set to 0 when they are created.
unsafe impl<T: DefaultIsEmpty, const N: usize> DefaultIsEmpty for [T; N] where [T; N]: Default {}

#[cfg(feature = "alloc")]
unsafe impl<T: DefaultIsEmpty> DefaultIsEmpty for alloc::boxed::Box<T> {}

macro_rules! impl_bitstore_uint {
	($ty:ty) => {
		unsafe impl DefaultIsEmpty for $ty {}

		impl BitStoreConst for $ty {
			const EMPTY: Self = 0;
			const FULL: Self = !0;
		}

		impl BitStore for $ty {
			const BITS: u32 = core::mem::size_of::<$ty>() as u32 * 8;

			#[inline]
			unsafe fn get(&self, index: u32) -> bool {
				#[cfg(debug_assertions)]
				if index >= Self::BITS {
					panic!(
						"index out of bounds: the len is {} but the index is {}",
						Self::BITS,
						index
					);
				}

				(1 << index) & *self != 0
			}

			#[inline]
			fn count_ones(&self) -> u32 {
				<$ty>::count_ones(*self)
			}

			#[inline]
			fn trailing_zeros(&self) -> u32 {
				<$ty>::trailing_zeros(*self)
			}

			#[inline]
			fn trailing_ones(&self) -> u32 {
				<$ty>::trailing_ones(*self)
			}

			#[inline]
			fn leading_zeros(&self) -> u32 {
				<$ty>::leading_zeros(*self)
			}

			#[inline]
			fn leading_ones(&self) -> u32 {
				<$ty>::leading_ones(*self)
			}

			#[inline]
			fn is_empty(&self) -> bool {
				*self == 0
			}

			#[inline]
			fn is_full(&self) -> bool {
				*self == !0
			}
		}

		impl BitStoreMut for $ty {
			#[inline]
			unsafe fn set(&mut self, index: u32) {
				#[cfg(debug_assertions)]
				if index >= Self::BITS {
					panic!(
						"index out of bounds: the len is {} but the index is {}",
						Self::BITS,
						index
					);
				}

				*self |= 1 << index;
			}

			#[inline]
			unsafe fn unset(&mut self, index: u32) {
				#[cfg(debug_assertions)]
				if index >= Self::BITS {
					panic!(
						"index out of bounds: the len is {} but the index is {}",
						Self::BITS,
						index
					);
				}

				*self &= !(1 << index);
			}

			#[inline]
			fn union_with(&mut self, other: &Self) {
				*self |= *other
			}

			#[inline]
			fn intersect_with(&mut self, other: &Self) {
				*self &= *other
			}

			#[inline]
			fn difference_with(&mut self, other: &Self) {
				*self &= !*other
			}

			#[inline]
			fn symmetric_difference_with(&mut self, other: &Self) {
				*self ^= *other
			}

			#[inline]
			fn negate(&mut self) {
				*self = !*self
			}
		}
	};
}

impl_bitstore_uint!(u8);
impl_bitstore_uint!(u16);
impl_bitstore_uint!(u32);
impl_bitstore_uint!(u64);
impl_bitstore_uint!(u128);
impl_bitstore_uint!(usize);

impl<T: BitStoreConst, const N: usize> BitStoreConst for [T; N] {
	const EMPTY: Self = [T::EMPTY; N];
	const FULL: Self = [T::FULL; N];
}

impl<T: BitStore, const N: usize> BitStore for [T; N] {
	const BITS: u32 = N as u32 * T::BITS;

	#[inline]
	unsafe fn get(&self, index: u32) -> bool {
		let (i, j) = (index / T::BITS, index % T::BITS);
		self[i as usize].get(j)
	}

	#[inline]
	fn count_ones(&self) -> u32 {
		self.iter().map(|x| x.count_ones()).sum()
	}

	#[inline]
	fn trailing_zeros(&self) -> u32 {
		let mut result = 0u32;
		for trailing in self.iter().map(BitStore::trailing_zeros) {
			result += trailing;

			if trailing != T::BITS {
				break;
			}
		}

		result
	}

	#[inline]
	fn trailing_ones(&self) -> u32 {
		let mut result = 0u32;
		for trailing in self.iter().map(BitStore::trailing_ones) {
			result += trailing;

			if trailing != T::BITS {
				break;
			}
		}

		result
	}

	#[inline]
	fn leading_zeros(&self) -> u32 {
		let mut result = 0u32;
		for leading in self.iter().rev().map(BitStore::leading_zeros) {
			result += leading;

			if leading != T::BITS {
				break;
			}
		}

		result
	}

	#[inline]
	fn leading_ones(&self) -> u32 {
		let mut result = 0u32;
		for leading in self.iter().rev().map(BitStore::leading_ones) {
			result += leading;

			if leading != T::BITS {
				break;
			}
		}

		result
	}
}

impl<T: BitStoreMut, const N: usize> BitStoreMut for [T; N] {
	#[inline]
	unsafe fn set(&mut self, index: u32) {
		let (i, j) = (index / T::BITS, index % T::BITS);
		self[i as usize].set(j);
	}

	#[inline]
	unsafe fn unset(&mut self, index: u32) {
		let (i, j) = (index / T::BITS, index % T::BITS);
		self[i as usize].unset(j);
	}

	#[inline]
	fn union_with(&mut self, other: &Self) {
		self
			.iter_mut()
			.zip(other.iter())
			.for_each(|(x, y)| x.union_with(y));
	}

	#[inline]
	fn intersect_with(&mut self, other: &Self) {
		self
			.iter_mut()
			.zip(other.iter())
			.for_each(|(x, y)| x.intersect_with(y))
	}

	#[inline]
	fn difference_with(&mut self, other: &Self) {
		self
			.iter_mut()
			.zip(other.iter())
			.for_each(|(x, y)| x.difference_with(y))
	}

	#[inline]
	fn symmetric_difference_with(&mut self, other: &Self) {
		self
			.iter_mut()
			.zip(other.iter())
			.for_each(|(x, y)| x.symmetric_difference_with(y))
	}

	#[inline]
	fn negate(&mut self) {
		self.iter_mut().for_each(BitStoreMut::negate)
	}
}

macro_rules! impl_bitstore_ptr {
	(const) => {
		#[inline]
		unsafe fn get(&self, index: u32) -> bool {
			BitStore::get(&**self, index)
		}

		#[inline]
		fn count_ones(&self) -> u32 {
			BitStore::count_ones(&**self)
		}

		#[inline]
		fn trailing_zeros(&self) -> u32 {
			BitStore::trailing_zeros(&**self)
		}

		#[inline]
		fn trailing_ones(&self) -> u32 {
			BitStore::trailing_ones(&**self)
		}

		#[inline]
		fn leading_zeros(&self) -> u32 {
			BitStore::leading_zeros(&**self)
		}

		#[inline]
		fn leading_ones(&self) -> u32 {
			BitStore::leading_ones(&**self)
		}
	};

	(mut) => {
		#[inline]
		unsafe fn set(&mut self, index: u32) {
			BitStoreMut::set(&mut **self, index)
		}

		#[inline]
		unsafe fn unset(&mut self, index: u32) {
			BitStoreMut::unset(&mut **self, index)
		}

		#[inline]
		fn union_with(&mut self, other: &Self) {
			BitStoreMut::union_with(&mut **self, other)
		}

		#[inline]
		fn intersect_with(&mut self, other: &Self) {
			BitStoreMut::intersect_with(&mut **self, other)
		}

		#[inline]
		fn difference_with(&mut self, other: &Self) {
			BitStoreMut::difference_with(&mut **self, other)
		}

		#[inline]
		fn symmetric_difference_with(&mut self, other: &Self) {
			BitStoreMut::symmetric_difference_with(&mut **self, other)
		}

		#[inline]
		fn negate(&mut self) {
			BitStoreMut::negate(&mut **self)
		}
	};
}

impl<'a, T: BitStore> BitStore for &'a T {
	const BITS: u32 = <T as BitStore>::BITS;
	impl_bitstore_ptr!(const);
}

impl<'a, T: BitStore> BitStore for &'a mut T {
	const BITS: u32 = <T as BitStore>::BITS;
	impl_bitstore_ptr!(const);
}

impl<'a, T: BitStoreMut> BitStoreMut for &'a mut T {
	impl_bitstore_ptr!(mut);
}

#[cfg(feature = "alloc")]
impl<T: BitStore> BitStore for alloc::boxed::Box<T> {
	const BITS: u32 = <T as BitStore>::BITS;
	impl_bitstore_ptr!(const);
}

#[cfg(feature = "alloc")]
impl<T: BitStoreMut> BitStoreMut for alloc::boxed::Box<T> {
	impl_bitstore_ptr!(mut);
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! test_bitstore {
		(@tests $ty:ty) => {
			#[test]
			fn empty_is_empty() {
				assert!(BitStore::is_empty(&<$ty as BitStoreConst>::EMPTY));
			}

			#[test]
			fn full_is_full() {
				assert!(BitStore::is_full(&<$ty as BitStoreConst>::FULL));
			}

			#[test]
			fn bits_is_size_of() {
				assert_eq!(
					<$ty as BitStore>::BITS,
					core::mem::size_of::<$ty>() as u32 * 8
				);
			}

			#[test]
			fn zero_is_all_false() {
				let x = <$ty as BitStoreConst>::EMPTY;
				for i in 0..<$ty as BitStore>::BITS {
					assert_eq!(unsafe { BitStore::get(&x, i) }, false);
				}
			}

			#[test]
			fn any_individual_index_can_be_set() {
				for i in 0..<$ty as BitStore>::BITS {
					let mut x = <$ty as BitStoreConst>::EMPTY;
					unsafe { x.set(i) };
					assert_eq!(unsafe { BitStore::get(&x, i) }, true);
					assert_eq!(BitStore::count_ones(&x), 1, "count_ones()");
					assert_eq!(BitStore::trailing_zeros(&x), i, "trailing_zeros()");
					assert_eq!(BitStore::leading_zeros(&x), <$ty as BitStore>::BITS - 1 - i, "leading_zeros()");
				}
			}

			#[test]
			fn trailing_ones() {
				let mut x = <$ty as BitStoreConst>::EMPTY;
				for i in (0..<$ty as BitStore>::BITS).rev() {
					unsafe { x.set(i) };
					assert_eq!(unsafe { BitStore::get(&x, i) }, true);
					assert_eq!(BitStore::count_ones(&x), <$ty as BitStore>::BITS - i, "count_ones()");
					// assert_eq!(BitStore::leading_ones(&x), <$ty as BitStore>::BITS - 1, "leading_ones()");
					// assert_eq!(BitStore::leading_zeros(&x), i, "leading_zeros()");
				}
			}

			#[test]
			fn negate() {
				let mut x = <$ty as BitStoreConst>::EMPTY;
				BitStoreMut::negate(&mut x);
				assert!(BitStore::is_full(&x));

				BitStoreMut::negate(&mut x);
				assert!(BitStore::is_empty(&x));

				unsafe { x.set(4) };
				BitStoreMut::negate(&mut x);
				assert_eq!(unsafe { BitStore::get(&x, 4) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 3) }, true);
			}

			#[test]
			fn union() {
				// 1 | 1 = 1
				let mut x = <$ty as BitStoreConst>::FULL;
				let y = <$ty as BitStoreConst>::FULL;
				BitStoreMut::union_with(&mut x, &y);
				assert!(BitStore::is_full(&x));

				// 0 | 1 = 1
				let mut x = <$ty as BitStoreConst>::EMPTY;
				let y = <$ty as BitStoreConst>::FULL;
				BitStoreMut::union_with(&mut x, &y);
				assert!(BitStore::is_full(&x));

				// 1 | 0 = 1
				let mut x = <$ty as BitStoreConst>::FULL;
				let y = <$ty as BitStoreConst>::EMPTY;
				BitStoreMut::union_with(&mut x, &y);
				assert!(BitStore::is_full(&x));

				// 0 | 0 = 0
				let mut x = <$ty as BitStoreConst>::EMPTY;
				let y = <$ty as BitStoreConst>::EMPTY;
				BitStoreMut::union_with(&mut x, &y);
				assert!(BitStore::is_empty(&x));

				let mut x = <$ty as BitStoreConst>::EMPTY;
				let mut y = <$ty as BitStoreConst>::EMPTY;
				unsafe { x.set(3) };
				unsafe { y.set(4) };
				BitStoreMut::union_with(&mut x, &y);
				assert_eq!(unsafe { BitStore::get(&x, 2) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 3) }, true);
				assert_eq!(unsafe { BitStore::get(&x, 4) }, true);
				assert_eq!(unsafe { BitStore::get(&x, 5) }, false);
			}

			#[test]
			fn intersection() {
				// 1 & 1 = 1
				let mut x = <$ty as BitStoreConst>::FULL;
				let y = <$ty as BitStoreConst>::FULL;
				BitStoreMut::intersect_with(&mut x, &y);
				assert!(BitStore::is_full(&x));

				// 0 & 1 = 0
				let mut x = <$ty as BitStoreConst>::EMPTY;
				let y = <$ty as BitStoreConst>::FULL;
				BitStoreMut::intersect_with(&mut x, &y);
				assert!(BitStore::is_empty(&x));

				// 1 & 0 = 1
				let mut x = <$ty as BitStoreConst>::FULL;
				let y = <$ty as BitStoreConst>::EMPTY;
				BitStoreMut::intersect_with(&mut x, &y);
				assert!(BitStore::is_empty(&x));

				// 0 | 0 = 0
				let mut x = <$ty as BitStoreConst>::EMPTY;
				let y = <$ty as BitStoreConst>::EMPTY;
				BitStoreMut::intersect_with(&mut x, &y);
				assert!(BitStore::is_empty(&x));

				let mut x = <$ty as BitStoreConst>::EMPTY;
				let mut y = <$ty as BitStoreConst>::EMPTY;
				unsafe { x.set(3) };
				unsafe { y.set(4) };
				BitStoreMut::intersect_with(&mut x, &y);
				assert_eq!(unsafe { BitStore::get(&x, 2) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 3) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 4) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 5) }, false);

				let mut x = <$ty as BitStoreConst>::EMPTY;
				let mut y = <$ty as BitStoreConst>::EMPTY;
				unsafe { x.set(3) };
				unsafe { y.set(3) };
				BitStoreMut::intersect_with(&mut x, &y);
				assert_eq!(unsafe { BitStore::get(&x, 2) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 3) }, true);
				assert_eq!(unsafe { BitStore::get(&x, 4) }, false);
				assert_eq!(unsafe { BitStore::get(&x, 5) }, false);
			}
		};

		($ty:ty, $mod:ident) => {
			mod $mod {
				use super::*;

				test_bitstore!(@tests $ty);
				mod x2 {
					use super::*;
					test_bitstore!(@tests [$ty; 2]);
				}

				mod x10 {
					use super::*;
					test_bitstore!(@tests [$ty; 10]);
				}
			}
		};
	}

	test_bitstore!(u8, u8_bitstore);
	test_bitstore!(u16, u16_bitstore);
	test_bitstore!(u32, u32_bitstore);
	test_bitstore!(u64, u64_bitstore);
	test_bitstore!(u128, u128_bitstore);
	test_bitstore!(usize, usize_bitstore);
}
