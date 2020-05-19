// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Uses a tagged pointer scheme to a maximum of 2^32 different instances of the same struct.
#[repr(transparent)]
struct TaggedRelativePointerToData<T: Sized>(u64, PhantomData<T>);

impl<T: Sized> Debug for TaggedRelativePointerToData<T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "TaggedRelativePointerToData {{ tag: {}, relative_index: {} }} )", self.tag(), self.relative_index_value())
	}
}

impl<T: Sized> Clone for TaggedRelativePointerToData<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0, PhantomData)
	}
}

impl<T: Sized> Copy for TaggedRelativePointerToData<T>
{
}

impl<T: Sized> TaggedRelativePointerToData<T>
{
	const BitsInAByte: u64 = 8;
	
	const TagBitShift: u64 = (size_of::<u32>() as u64) * Self::BitsInAByte;
	
	const RelativeIndexBitMask: u64 = (1 << Self::TagBitShift) - 1;
	
	const SizeOfT: u64 = size_of::<T>() as u64;
	
	#[inline(always)]
	fn new(tag: u32, pointer: NonNull<T>, base_pointer: NonNull<T>) -> Self
	{
		let tag = (tag as u64) << Self::TagBitShift;
		Self(tag | (Self::relative_index(pointer, base_pointer) as u64), PhantomData)
	}
	
	#[inline(always)]
	fn into_absolute_pointer_from(self, mapped_memory_containing_pointer: &MappedMemory) -> NonNull<T>
	{
		self.into_absolute_pointer(mapped_memory_containing_pointer.virtual_address().into())
	}
	
	#[inline(always)]
	fn into_absolute_pointer(self, base_pointer: NonNull<T>) -> NonNull<T>
	{
		let relative_index = self.relative_index_value();
		let relative_pointer = relative_index * Self::SizeOfT;
		unsafe { NonNull::new_unchecked(base_pointer.as_ptr().add(relative_pointer as usize)) }
	}
	
	#[inline(always)]
	fn relative_index_value(self) -> u64
	{
		self.0 & Self::RelativeIndexBitMask
	}
	
	#[inline(always)]
	fn tag(self) -> u32
	{
		(self.0 >> Self::TagBitShift) as u32
	}
	
	#[inline(always)]
	fn relative_index(larger: NonNull<T>, smaller: NonNull<T>) -> u64
	{
		cfn_debug_assert!(larger >= smaller);
		
		let difference = ((larger.as_ptr() as usize) - (smaller.as_ptr() as usize)) as u64;
		let relative_index = difference / Self::SizeOfT;
		cfn_debug_assert!(relative_index <= u32::MAX as u64);
		
		relative_index
	}
	
	#[inline(always)]
	fn handle(self) -> u64
	{
		self.0
	}
}
