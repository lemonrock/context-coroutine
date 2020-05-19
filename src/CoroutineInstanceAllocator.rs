// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Allocator of coroutine instances.
pub struct CoroutineInstanceAllocator<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized>
{
	allocator: LargeRingQueue<CoroutineInstance<HeapSize, StackSize, GTACSA, C, CoroutineInformation>>,
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized> CoroutineInstanceAllocator<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	/// New.
	#[inline(always)]
	pub fn new(ideal_maximum_number_of_coroutines: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		Ok
		(
			Self
			{
				allocator: LargeRingQueue::new(ideal_maximum_number_of_coroutines, defaults, 0, false)?,
			}
		)
	}
	
	/// New coroutine instance.
	#[inline(always)]
	fn new_coroutine_instance(&mut self, coroutine_information: CoroutineInformation) -> Result<CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>, AllocErr>
	{
		let base_pointer: NonNull<CoroutineInstance<HeapSize, StackSize, GTACSA, C, CoroutineInformation>> = self.allocator.virtual_address().into();
		
		self.allocator.obtain_and_map
		(
			|coroutine_instance|
			{
				let generation = CoroutineInstance::constructor(coroutine_instance, coroutine_information);
				CoroutineInstancePointer(TaggedRelativePointerToData::new(generation.0, coroutine_instance, base_pointer))
			},
			|| AllocErr
		)
	}
	
	/// Free (and kill if necessary) coroutine instance.
	#[inline(always)]
	fn free_coroutine_instance(&mut self, coroutine_instance_pointer: CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>)
	{
		if let Some(non_null_coroutine_instance) = coroutine_instance_pointer.pointer(self)
		{
			CoroutineInstance::free(non_null_coroutine_instance);
			self.allocator.relinquish(non_null_coroutine_instance)
		}
	}
}
