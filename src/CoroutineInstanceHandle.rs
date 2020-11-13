// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A `CoroutineInstanceHandle` is an untyped `CoroutineInstancePointer` suitable for a coroutine to know and pass in user data (tokens) to epoll or io_uring.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoroutineInstanceHandle(u64);

impl CoroutineInstanceHandle
{
	// Using the very topmost bit allows us to use regular pointers if the bit is clear.
	const IsCoroutineBitCount: u64 = 1;
	const IsCoroutineBitShift: u64 = 63;
	const IsCoroutineBitMask: u64 = Self::bit_mask(Self::IsCoroutineBitCount, Self::IsCoroutineBitShift);
	
	#[allow(dead_code)] const ReservedBitCount: u64 = 3;
	#[allow(dead_code)] const ReservedBitShift: u64 = Self::CoroutineManagerIndexBitCount + Self::CoroutineManagerIndexBitShift;
	#[allow(dead_code)] const ReservedBitMask: u64 = Self::bit_mask(Self::ReservedBitCount, Self::ReservedBitShift);
	
	const CoroutineManagerIndexBitCount: u64 = 8;
	const CoroutineManagerIndexBitShift: u64 = Self::UserBitsBitCount + Self::UserBitsBitShift;
	const CoroutineManagerIndexBitMask: u64 = Self::bit_mask(Self::ReservedBitCount, Self::ReservedBitShift);
	
	const UserBitsBitCount: u64 = 4;
	const UserBitsBitShift: u64 = Self::GenerationBitCount + Self::GenerationBitShift;
	const UserBitsBitMask: u64 = Self::bit_mask(Self::UserBitsBitCount, Self::UserBitsBitShift);
	
	const GenerationBitCount: u64 = 24;
	const GenerationBitShift: u64 = Self::IndexBitCount + Self::IndexBitShift;
	const GenerationBitMask: u64 = Self::bit_mask(Self::GenerationBitCount, Self::GenerationBitShift);
	
	const IndexBitCount: u64 = 24;
	const IndexBitShift: u64 = 0;
	const IndexBitMask: u64 = Self::bit_mask(Self::IndexBitCount, Self::IndexBitShift);
	
	/// Wrap user data from epoll or io_uring.
	#[inline(always)]
	pub const fn wrap(user_data: u64) -> Self
	{
		Self(user_data)
	}
	
	/// Unwrap to user data for epoll or io_uring.
	#[inline(always)]
	pub const fn unwrap(self) -> u64
	{
		self.0
	}
	
	#[inline(always)]
	fn new<T: Sized>(is_coroutine: bool, coroutine_manager_index: CoroutineManagerIndex, user_bits: UserBits, generation: CoroutineGenerationCounter, pointer: NonNull<T>, base_pointer: NonNull<T>) -> Self
	{
		let is_coroutine_unshifted = is_coroutine as u64;
		let coroutine_manager_index = coroutine_manager_index.0 as u64;
		let user_bits_unshifted = user_bits.0 as u64;
		let generation_unshifted = generation.0 as u64;
		let index_unshifted = Self::calculate_index::<T>(pointer, base_pointer);
		
		Self
		(
			is_coroutine_unshifted << Self::IsCoroutineBitCount
			| coroutine_manager_index << Self::CoroutineManagerIndexBitShift
			| user_bits_unshifted << Self::UserBitsBitShift
			| generation_unshifted << Self::GenerationBitShift
			| index_unshifted << Self::IndexBitShift
		)
	}
	
	/// Is not for a coroutine?
	#[inline(always)]
	pub const fn is_not_for_a_coroutine(user_data: u64) -> bool
	{
		user_data & Self::IsCoroutineBitMask == 0
	}
	
	/// Is for a coroutine?
	#[inline(always)]
	pub const fn is_coroutine(self) -> bool
	{
		self.0 & Self::IsCoroutineBitMask != 0
	}
	
	/// Coroutine manager index.
	#[inline(always)]
	pub const fn coroutine_manager_index(self) -> CoroutineManagerIndex
	{
		CoroutineManagerIndex(((self.0 & Self::CoroutineManagerIndexBitMask) >> Self::CoroutineManagerIndexBitShift) as u8)
	}
	
	/// User bits.
	#[inline(always)]
	pub const fn user_bits(self) -> UserBits
	{
		UserBits(((self.0 & Self::UserBitsBitMask) >> Self::UserBitsBitShift) as u8)
	}
	
	/// User bits.
	#[inline(always)]
	pub fn set_user_bits(self, user_bits: UserBits) -> Self
	{
		let user_bits_unshifted = user_bits.0 as u64;
		let user_bits_shifted = user_bits_unshifted << Self::UserBitsBitShift;
		Self((self.0 & !Self::UserBitsBitMask) | user_bits_shifted)
	}
	
	/// Coroutine generation counter.
	#[inline(always)]
	const fn generation(self) -> CoroutineGenerationCounter
	{
		CoroutineGenerationCounter(((self.0 & Self::GenerationBitMask) >> Self::GenerationBitShift) as u32)
	}
	
	#[inline(always)]
	fn into_absolute_pointer<T: Sized>(self, base_pointer: NonNull<T>) -> NonNull<T>
	{
		let base_pointer = base_pointer.as_ptr() as usize;
		let relative_pointer = self.relative_pointer::<T>();
		
		new_non_null((base_pointer + relative_pointer) as *mut T)
	}
	
	#[inline(always)]
	fn relative_pointer<T: Sized>(self) -> usize
	{
		let index = (self.0 & Self::IndexBitMask) >> Self::IndexBitShift;
		(index as usize) * size_of::<T>()
	}
	
	#[inline(always)]
	fn calculate_index<T: Sized>(larger: NonNull<T>, smaller: NonNull<T>) -> u64
	{
		cfn_debug_assert!(larger >= smaller);
		
		let difference = ((larger.as_ptr() as usize) - (smaller.as_ptr() as usize)) as u64;
		let index = difference / (size_of::<T>() as u64);
		cfn_debug_assert!(index <= u32::MAX as u64);
		
		index
	}
	
	#[inline(always)]
	const fn bit_mask(bit_count: u64, bit_shift: u64) -> u64
	{
		Self::unshifted_bit_mask(bit_count) << bit_shift
	}
	
	#[inline(always)]
	const fn unshifted_bit_mask(bit_count: u64) -> u64
	{
		(1 << bit_count) - 1
	}
}
