// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Uses a tagged pointer scheme of 64 bits packed into an `u64` giving a total of 2^24 (16 million) possible coroutines.
///
/// The actual layout might change
///
/// ```bo
/// ┌──────────────┬──────────┬───────────┬────────────┬────────┐
/// │      63      │ 62 …  52 │ 51  …  48 │ 47   …  24 │ 23 … 0 │
/// ├──────────────┼──────────┼───────────┼────────────┼────────┤
/// │ Is Coroutine │ Reserved │ User Bits │ Generation │ Index  │
/// └──────────────┴──────────┴───────────┴────────────┴────────┘
/// ```
/// * `Is Coroutine`: if set (`1`), then bits `62 … 0` have meaning as defined above.
/// * `Reserved`: bits reserved for future use or expansion
/// * `User Bits`: Currently, 4 bits available for coroutine user state notification; always zero when a coroutine is passed this data.
/// * `Generation`: A counter used to avoid the ABA problem when re-using memory for coroutines where the `Index` may have previously been used, eg in user data passed to epoll or io_uring.
/// * `Index`: A relative index to a coroutine in memory.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct TaggedRelativePointerToData<T: Sized>(CoroutineInstanceHandle, PhantomData<T>);

impl<T: Sized> Debug for TaggedRelativePointerToData<T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "TaggedRelativePointerToData({:?})", self.0)
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
	#[inline(always)]
	fn new(is_coroutine: bool, user_bits: UserBits, generation: CoroutineGenerationCounter, pointer: NonNull<T>, base_pointer: NonNull<T>) -> Self
	{
		Self
		(
			CoroutineInstanceHandle::new::<T>(is_coroutine, user_bits, generation, pointer, base_pointer),
			PhantomData,
		)
	}
	
	#[inline(always)]
	unsafe fn from_handle(coroutine_instance_handle: CoroutineInstanceHandle) -> Self
	{
		Self(coroutine_instance_handle, PhantomData)
	}
	
	#[inline(always)]
	fn into_absolute_pointer_from(self, mapped_memory_containing_pointer: &MappedMemory) -> NonNull<T>
	{
		self.0.into_absolute_pointer::<T>(mapped_memory_containing_pointer.virtual_address().into())
	}
	
	#[inline(always)]
	fn handle(self) -> CoroutineInstanceHandle
	{
		self.0
	}
}
