// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A pointer to a CoroutineInstance.
pub struct CoroutineInstancePointer<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized>(TaggedRelativePointerToData<CoroutineInstance<HeapSize, StackSize, GTACSA, C, CoroutineInformation>>);

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized> Debug for CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "CoroutineInstancePointer({:?})", self.0)
	}
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized> Clone for CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0)
	}
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized> Copy for CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized> CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	/// Only returns `Some()` if the generation matches.
	///
	/// Generations are used to manage memory that is recycled but to which something still maintains a `CoroutineInstancePointer`.
	///
	/// This can happen if using `CoroutineInstancePointer` with epoll or io_uring as user data (a user token).
	#[inline(always)]
	fn pointer(self, allocator: &CoroutineInstanceAllocator<HeapSize, StackSize, GTACSA, C, CoroutineInformation>) -> Option<NonNull<CoroutineInstance<HeapSize, StackSize, GTACSA, C, CoroutineInformation>>>
	{
		let absolute_pointer = self.into_absolute_pointer(allocator);
		
		let current_generation = (unsafe { absolute_pointer.as_ref() }).generation;
		
		if current_generation == self.was_generation()
		{
			Some(absolute_pointer)
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn as_coroutine_instance_handle(self) -> CoroutineInstanceHandle
	{
		self.0.handle()
	}
	
	#[inline(always)]
	unsafe fn as_mut_unchecked<'a>(self, allocator: &'a CoroutineInstanceAllocator<HeapSize, StackSize, GTACSA, C, CoroutineInformation>) -> &'a mut CoroutineInstance<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
	{
		&mut * self.into_absolute_pointer(allocator).as_ptr()
	}
	
	/// Only returns `Some()` if the generation matches.
	#[inline(always)]
	fn into_absolute_pointer(self, allocator: &CoroutineInstanceAllocator<HeapSize, StackSize, GTACSA, C, CoroutineInformation>) -> NonNull<CoroutineInstance<HeapSize, StackSize, GTACSA, C, CoroutineInformation>>
	{
		self.0.into_absolute_pointer_from(&allocator.allocator)
	}
	
	#[inline(always)]
	fn was_generation(self) -> CoroutineGenerationCounter
	{
		CoroutineGenerationCounter(self.0.tag())
	}
}
