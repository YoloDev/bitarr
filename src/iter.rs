use crate::store::BitStore;
use core::ops;

#[derive(Debug, Clone)]
pub struct Bits<S: BitStore> {
	bits: S,
	range: ops::Range<u32>,
}

impl<S: BitStore> From<S> for Bits<S> {
	#[inline]
	fn from(bits: S) -> Self {
		Self::new(bits)
	}
}

impl<S: BitStore> Bits<S> {
	#[inline]
	pub const fn new(bits: S) -> Self {
		Self {
			bits,
			range: 0..S::BITS,
		}
	}

	#[inline]
	pub const fn with_range(bits: S, range: ops::Range<u32>) -> Self {
		if range.end > S::BITS {
			panic!("Range end is out of bounds");
		}

		Self { bits, range }
	}

	/// # Safety
	/// Range parameter must be in bounds for the bit store.
	#[inline]
	pub const unsafe fn with_range_unchecked(bits: S, range: ops::Range<u32>) -> Self {
		Self { bits, range }
	}
}

impl<S: BitStore> Iterator for Bits<S> {
	type Item = bool;

	fn next(&mut self) -> Option<Self::Item> {
		self.range.next().map(|i| {
			// SAFETY: `range` is in bounds.
			unsafe { self.bits.get(i) }
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.range.size_hint()
	}
}

impl<S: BitStore> DoubleEndedIterator for Bits<S> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.range.next_back().map(|i| {
			// SAFETY: `range` is in bounds.
			unsafe { self.bits.get(i) }
		})
	}
}

impl<S: BitStore> ExactSizeIterator for Bits<S> {
	#[inline]
	fn len(&self) -> usize {
		self.range.len()
	}
}

impl<S: BitStore> core::iter::FusedIterator for Bits<S> {}
