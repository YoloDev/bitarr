#![cfg_attr(not(feature = "std"), no_std)]

//! # BitSet
//!
//! A compact data structure for storing bits.
//!
//! The `BitSet` struct is a low-level data structure that stores a sequence
//! of bits and provides methods for accessing and manipulating those bits.
//!
//! It supports operations such as setting and clearing individual bits, and computing
//! the union and intersection of two bit sets.
//!
//! # Examples
//!
//! ```
//! # use bitarr::BitSet;
//! let mut bs = BitSet::from(0u8);
//!
//! bs.set(3);
//! bs.set(7);
//!
//! assert_eq!(bs.get(3), Some(true));
//! assert_eq!(bs.get(7), Some(true));
//! assert_eq!(bs.get(2), Some(false));
//! ```

#![cfg(feature = "alloc")]
extern crate alloc;

mod bit_fmt;
pub mod iter;
pub mod store;

use core::fmt;
use core::ops;
use store::BitStoreMut;
use store::{BitStore, BitStoreConst, DefaultIsEmpty};

/// A compact data structure for storing bits
///
/// # Examples
///
/// ```
/// # use bitarr::BitSet;
/// let mut bs = BitSet::from(0u8);
///
/// bs.set(3);
/// bs.set(7);
///
/// assert_eq!(bs.get(3), Some(true));
/// assert_eq!(bs.get(7), Some(true));
/// assert_eq!(bs.get(2), Some(false));
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSet<S: BitStore = usize> {
	bits: S,
}

impl<S: BitStore + DefaultIsEmpty> Default for BitSet<S> {
	#[inline]
	fn default() -> Self {
		Self { bits: S::default() }
	}
}

impl<S: BitStore> fmt::Debug for BitSet<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(self, f)
	}
}

impl<S: BitStore> fmt::Binary for BitSet<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&bit_fmt::BinaryDisplay(&self.bits), f)
	}
}

impl<S: BitStoreConst> BitSet<S> {
	/// Creates a new `BitSet` with all bits set to 0.
	/// This is equivalent to [BitSet::empty].
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let bs = BitSet::<u8>::new();
	/// ```
	#[inline]
	pub const fn new() -> Self {
		Self::empty()
	}

	/// Creates a new `BitSet` with all bits set to 0.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let bs = BitSet::<u8>::empty();
	/// ```
	#[inline]
	pub const fn empty() -> Self {
		Self { bits: S::EMPTY }
	}

	/// Creates a new `BitSet` with all bits set to 1.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let bs = BitSet::<u8>::full();
	/// ```
	#[inline]
	pub const fn full() -> Self {
		Self { bits: S::FULL }
	}
}

impl<S: BitStore> BitSet<S> {
	/// Gets the value of the bit at the specified index.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// bs.set(3);
	/// assert_eq!(bs.get(3), Some(true));
	/// assert_eq!(bs[3], true);
	/// ```
	#[inline]
	pub fn get(&self, index: u32) -> Option<bool> {
		if index >= S::BITS {
			None
		} else {
			// SAFETY: The index is in bounds
			Some(unsafe { self.bits.get(index) })
		}
	}

	/// Gets the value of the bit at the specified index
	/// without checking that the index is in bounds.
	///
	/// # Safety
	/// Using an index that is out of bounds may lead to
	/// undefined behavior.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// bs.set(3);
	/// assert_eq!(unsafe { bs.get_unchecked(3) }, true);
	/// ```
	#[inline]
	pub unsafe fn get_unchecked(&self, index: u32) -> bool {
		self.bits.get(index)
	}

	/// Returns the number of bits in the `BitSet`.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let bs = BitSet::from(0u8);
	/// assert_eq!(bs.len(), 8);
	/// let bs = BitSet::from(0u16);
	/// assert_eq!(bs.len(), 16);
	/// let bs = BitSet::from([0u16; 2]);
	/// assert_eq!(bs.len(), 32);
	/// ```
	#[inline]
	pub const fn len(&self) -> u32 {
		S::BITS
	}

	/// Returns `true` if the `BitSet` is empty, i.e., all bits are unset.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let bs = BitSet::from(0u16);
	/// assert!(bs.is_empty());
	/// ```
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.bits.is_empty()
	}

	/// Returns `true` if the `BitSet` is full, i.e., all bits are set.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let bs = BitSet::from(!0u16);
	/// assert!(bs.is_full());
	/// ```
	#[inline]
	pub fn is_full(&self) -> bool {
		self.bits.is_full()
	}

	/// Returns `true` if the `BitSet` contains any set bits.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u16);
	/// assert!(!bs.any());
	/// bs.set(3);
	/// assert!(bs.any());
	/// ```
	#[inline]
	pub fn any(&self) -> bool {
		!self.bits.is_empty()
	}

	/// Returns a borrowed iterator over the bits in the `BitSet`.
	#[inline]
	pub const fn iter(&self) -> iter::Bits<&S> {
		iter::Bits::new(&self.bits)
	}

	/// Returns an iterator over the indices of the set bits in the `BitSet`.
	pub fn ones(&self) -> impl Iterator<Item = u32> + DoubleEndedIterator + '_ {
		self
			.iter()
			.enumerate()
			.filter_map(|(i, b)| b.then_some(i as u32))
	}

	/// Returns an iterator over the indices of the set bits in the `BitSet`.
	pub fn into_ones(self) -> impl Iterator<Item = u32> + DoubleEndedIterator {
		self
			.into_iter()
			.enumerate()
			.filter_map(|(i, b)| b.then_some(i as u32))
	}
}

impl<S: BitStore> IntoIterator for BitSet<S> {
	type Item = bool;
	type IntoIter = iter::Bits<S>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		iter::Bits::new(self.bits)
	}
}

impl<S: BitStoreMut> BitSet<S> {
	/// Sets the bit at the specified index, and returns
	/// original value.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// assert_eq!(bs.set(3), Some(false));
	/// assert_eq!(bs.get(3), Some(true));
	/// ```
	#[inline]
	pub fn set(&mut self, index: u32) -> Option<bool> {
		if index >= S::BITS {
			None
		} else {
			// SAFETY: The index is in bounds
			unsafe {
				let old = self.bits.get(index);
				self.bits.set(index);
				Some(old)
			}
		}
	}

	/// Sets the bit at the specified index without checking
	/// that the index is in bounds, and returns original
	/// value.
	///
	/// # Safety
	/// Using an index that is out of bounds may lead to
	/// undefined behavior.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// assert_eq!(unsafe { bs.set_unchecked(3) }, false);
	/// assert_eq!(bs.get(3), Some(true));
	/// ```
	#[inline]
	pub unsafe fn set_unchecked(&mut self, index: u32) -> bool {
		let old = self.bits.get(index);
		self.bits.set(index);
		old
	}

	/// Unsets the bit at the specified index, and returns
	/// original value.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// bs.negate();
	/// assert_eq!(bs.unset(3), Some(true));
	/// assert_eq!(bs.get(3), Some(false));
	/// ```
	#[inline]
	pub fn unset(&mut self, index: u32) -> Option<bool> {
		if index >= S::BITS {
			None
		} else {
			// SAFETY: The index is in bounds
			unsafe {
				let old = self.bits.get(index);
				self.bits.unset(index);
				Some(old)
			}
		}
	}

	/// Unsets the bit at the specified index without checking
	/// that the index is in bounds, and returns original
	/// value.
	///
	/// # Safety
	/// Using an index that is out of bounds may lead to
	/// undefined behavior.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// bs.negate();
	/// assert_eq!(unsafe { bs.unset_unchecked(3) }, true);
	/// assert_eq!(bs.get(3), Some(false));
	/// ```
	#[inline]
	pub unsafe fn unset_unchecked(&mut self, index: u32) -> bool {
		let old = self.bits.get(index);
		self.bits.unset(index);
		old
	}

	/// Changes the bit at the specified index to `value`, and returns
	/// original value.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// assert_eq!(bs.change(3, true), Some(false));
	/// assert_eq!(bs.get(3), Some(true));
	/// assert_eq!(bs.change(3, false), Some(true));
	/// assert_eq!(bs.get(3), Some(false));
	/// ```
	#[inline]
	pub fn change(&mut self, index: u32, value: bool) -> Option<bool> {
		if value {
			self.set(index)
		} else {
			self.unset(index)
		}
	}

	/// Changes the bit at the specified index to `value` without checking
	/// that the index is in bounds, and returns original value.
	///
	/// # Safety
	/// Using an index that is out of bounds may lead to
	/// undefined behavior.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs = BitSet::from(0u8);
	/// assert_eq!(unsafe { bs.change_unchecked(3, true) }, false);
	/// assert_eq!(bs.get(3), Some(true));
	/// assert_eq!(unsafe { bs.change_unchecked(3, false) }, true);
	/// assert_eq!(bs.get(3), Some(false));
	/// ```
	#[inline]
	pub unsafe fn change_unchecked(&mut self, index: u32, value: bool) -> bool {
		if value {
			self.set_unchecked(index)
		} else {
			self.unset_unchecked(index)
		}
	}

	/// Performs the union of two `BitSet`s, modifying `self` in place.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0o16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// bs1.union_with(&bs2);
	///
	/// assert_eq!(bs1.get(3), Some(true));
	/// assert_eq!(bs1.get(7), Some(true));
	/// assert_eq!(bs1.get(9), Some(true));
	/// ```
	#[inline]
	pub fn union_with(&mut self, other: &Self) {
		self.bits.union_with(&other.bits);
	}

	/// Performs the intersection of two `BitSet`s, modifying `self` in place.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// bs1.intersect_with(&bs2);
	///
	/// assert_eq!(bs1.get(3), Some(false));
	/// assert_eq!(bs1.get(7), Some(true));
	/// assert_eq!(bs1.get(9), Some(false));
	/// ```
	#[inline]
	pub fn intersect_with(&mut self, other: &Self) {
		self.bits.intersect_with(&other.bits);
	}

	/// Performs the difference of two `BitSet`s, modifying `self` in place.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// bs1.difference_with(&bs2);
	///
	/// assert_eq!(bs1.get(3), Some(true));
	/// assert_eq!(bs1.get(7), Some(false));
	/// assert_eq!(bs1.get(9), Some(false));
	/// ```
	#[inline]
	pub fn difference_with(&mut self, other: &Self) {
		self.bits.difference_with(&other.bits);
	}

	/// Performs the symmetric difference of two `BitSet`s, modifying `self` in place.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// bs1.symmetric_difference_with(&bs2);
	///
	/// assert_eq!(bs1.get(3), Some(true));
	/// assert_eq!(bs1.get(7), Some(false));
	/// assert_eq!(bs1.get(9), Some(true));
	/// ```
	#[inline]
	pub fn symmetric_difference_with(&mut self, other: &Self) {
		self.bits.symmetric_difference_with(&other.bits);
	}

	/// Performs the negation of all the bits, modifying `self` in place.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	/// bs1.negate();
	///
	/// assert_eq!(bs1.get(3), Some(false));
	/// assert_eq!(bs1.get(7), Some(false));
	/// assert_eq!(bs1.get(9), Some(true));
	/// ```
	#[inline]
	pub fn negate(&mut self) {
		self.bits.negate();
	}
}

impl<S: BitStore> From<S> for BitSet<S> {
	#[inline]
	fn from(bits: S) -> Self {
		Self { bits }
	}
}

impl<S: BitStore> ops::Index<u32> for BitSet<S> {
	type Output = bool;

	#[inline]
	fn index(&self, index: u32) -> &Self::Output {
		if index >= S::BITS {
			panic!(
				"index out of bounds: the len is {} but the index is {}",
				S::BITS,
				index
			);
		}

		// SAFETY: the index is in bounds
		match unsafe { self.bits.get(index) } {
			true => &true,
			false => &false,
		}
	}
}

impl<S: BitStoreMut + Clone> BitSet<S> {
	/// Performs the union of two `BitSet`s.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0o16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// let bs3 = bs1.union(&bs2);
	///
	/// assert_eq!(bs3.get(3), Some(true));
	/// assert_eq!(bs3.get(7), Some(true));
	/// assert_eq!(bs3.get(9), Some(true));
	/// ```
	#[inline]
	pub fn union(&self, other: &Self) -> Self {
		let mut bits = self.bits.clone();
		bits.union_with(&other.bits);
		Self { bits }
	}

	/// Performs the intersection of two `BitSet`s.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// let bs3 = bs1.intersection(&bs2);
	///
	/// assert_eq!(bs3.get(3), Some(false));
	/// assert_eq!(bs3.get(7), Some(true));
	/// assert_eq!(bs3.get(9), Some(false));
	/// ```
	#[inline]
	pub fn intersection(&self, other: &Self) -> Self {
		let mut bits = self.bits.clone();
		bits.intersect_with(&other.bits);
		Self { bits }
	}

	/// Performs the difference of two `BitSet`s.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// let bs3 = bs1.difference(&bs2);
	///
	/// assert_eq!(bs3.get(3), Some(true));
	/// assert_eq!(bs3.get(7), Some(false));
	/// assert_eq!(bs3.get(9), Some(false));
	/// ```
	#[inline]
	pub fn difference(&self, other: &Self) -> Self {
		let mut bits = self.bits.clone();
		bits.difference_with(&other.bits);
		Self { bits }
	}

	/// Performs the symmetric difference of two `BitSet`s.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	/// let mut bs2 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	/// bs2.set(9);
	///
	/// let bs3 = bs1.symmetric_difference(&bs2);
	///
	/// assert_eq!(bs3.get(3), Some(true));
	/// assert_eq!(bs3.get(7), Some(false));
	/// assert_eq!(bs3.get(9), Some(true));
	/// ```
	#[inline]
	pub fn symmetric_difference(&self, other: &Self) -> Self {
		let mut bits = self.bits.clone();
		bits.symmetric_difference_with(&other.bits);
		Self { bits }
	}

	/// Performs the negation of all the bits.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u16);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	/// let bs2 = bs1.negation();
	///
	/// assert_eq!(bs2.get(3), Some(false));
	/// assert_eq!(bs2.get(7), Some(false));
	/// assert_eq!(bs2.get(9), Some(true));
	#[inline]
	pub fn negation(&self) -> Self {
		let mut bits = self.bits.clone();
		bits.negate();
		Self { bits }
	}

	/// Returns `true` if `self` is a subset of `other`,
	/// i.e., every bit in `self` is set in `other`.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u8);
	/// let mut bs2 = BitSet::from(0u8);
	///
	/// bs1.set(7);
	///
	/// bs2.set(3);
	/// bs2.set(7);
	///
	/// assert!(bs1.is_subset(&bs2));
	/// ```
	#[inline]
	pub fn is_subset(&self, other: &Self) -> bool {
		self.difference(other).is_empty()
	}

	/// Returns `true` if `self` is a superset of `other`,
	/// i.e., every bit in `other` is set in `self`.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u8);
	/// let mut bs2 = BitSet::from(0u8);
	///
	/// bs1.set(3);
	/// bs1.set(7);
	///
	/// bs2.set(7);
	///
	/// assert!(bs1.is_superset(&bs2));
	/// ```
	#[inline]
	pub fn is_superset(&self, other: &Self) -> bool {
		other.is_subset(self)
	}

	/// Returns `true` if `self` and `other` have no common bits set,
	/// i.e., their intersection is empty.
	///
	/// # Examples
	///
	/// ```
	/// # use bitarr::BitSet;
	/// let mut bs1 = BitSet::from(0u8);
	/// let mut bs2 = BitSet::from(0u8);
	///
	/// bs1.set(3);
	///
	/// bs2.set(7);
	///
	/// assert!(bs1.is_disjoint(&bs2));
	/// ```
	#[inline]
	pub fn is_disjoint(&self, other: &Self) -> bool {
		self.intersection(other).is_empty()
	}
}

impl<S: BitStoreMut + Clone> ops::Neg for BitSet<S> {
	type Output = Self;

	#[inline]
	fn neg(self) -> Self::Output {
		self.negation()
	}
}

impl<S: BitStoreMut + Clone> ops::BitAnd for BitSet<S> {
	type Output = Self;

	#[inline]
	fn bitand(self, rhs: Self) -> Self::Output {
		self.intersection(&rhs)
	}
}

impl<S: BitStoreMut> ops::BitAndAssign for BitSet<S> {
	#[inline]
	fn bitand_assign(&mut self, rhs: Self) {
		self.intersect_with(&rhs);
	}
}

impl<S: BitStoreMut + Clone> ops::BitOr for BitSet<S> {
	type Output = Self;

	#[inline]
	fn bitor(self, rhs: Self) -> Self::Output {
		self.union(&rhs)
	}
}

impl<S: BitStoreMut> ops::BitOrAssign for BitSet<S> {
	#[inline]
	fn bitor_assign(&mut self, rhs: Self) {
		self.union_with(&rhs);
	}
}

impl<S: BitStoreMut + Clone> ops::BitXor for BitSet<S> {
	type Output = Self;

	#[inline]
	fn bitxor(self, rhs: Self) -> Self::Output {
		self.symmetric_difference(&rhs)
	}
}

impl<S: BitStoreMut> ops::BitXorAssign for BitSet<S> {
	#[inline]
	fn bitxor_assign(&mut self, rhs: Self) {
		self.symmetric_difference_with(&rhs);
	}
}

impl<S: BitStoreMut + Clone> ops::Sub for BitSet<S> {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		self.difference(&rhs)
	}
}

impl<S: BitStoreMut> ops::SubAssign for BitSet<S> {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.difference_with(&rhs);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn union() {
		let mut bs1 = BitSet::from(0u16);
		let mut bs2 = BitSet::from(0u16);

		bs1.set(3);
		bs1.set(7);

		bs2.set(7);
		bs2.set(9);

		bs1.union_with(&bs2);

		assert!(bs1[3]);
		assert!(bs1[7]);
		assert!(bs1[9]);
	}

	#[test]
	fn intersection() {
		let mut bs1 = BitSet::from(0u16);
		let mut bs2 = BitSet::from(0u16);

		bs1.set(3);
		bs1.set(7);

		bs2.set(7);
		bs2.set(9);

		bs1.intersect_with(&bs2);

		assert!(!bs1[3]);
		assert!(bs1[7]);
		assert!(!bs1[9]);
	}
}
