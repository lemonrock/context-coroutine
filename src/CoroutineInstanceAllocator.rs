// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Allocator of coroutine instances.
struct CoroutineInstanceAllocator<CoroutineHeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized>(LargeRingQueue<CoroutineInstance<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>>);

impl<CoroutineHeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized> CoroutineInstanceAllocator<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	#[inline(always)]
	fn new(ideal_maximum_number_of_coroutines: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		LargeRingQueue::new(ideal_maximum_number_of_coroutines, defaults, 0, false).map(Self)
	}
	
	#[inline(always)]
	fn new_coroutine_instance(&mut self, our_coroutine_manager_index: CoroutineManagerIndex, coroutine_information: CoroutineInformation) -> Result<CoroutineInstancePointer<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>, AllocErr>
	{
		let base_pointer: NonNull<CoroutineInstance<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>> = self.0.virtual_address().into();
		
		self.0.obtain_and_map
		(
			|coroutine_instance|
			{
				let generation = CoroutineInstance::constructor(coroutine_instance, coroutine_information);
				CoroutineInstancePointer(TaggedRelativePointerToData::new(true, our_coroutine_manager_index, UserBits::Zero, generation, coroutine_instance, base_pointer))
			},
			|| AllocErr
		)
	}
	
	#[inline(always)]
	fn free_coroutine_instance(&mut self, coroutine_instance_pointer: CoroutineInstancePointer<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>)
	{
		if let Some(non_null_coroutine_instance) = coroutine_instance_pointer.pointer(self)
		{
			CoroutineInstance::free(non_null_coroutine_instance);
			self.0.relinquish(non_null_coroutine_instance)
		}
	}
	
	#[inline(always)]
	fn mapped_memory(&self) -> &MappedMemory
	{
		&self.0
	}
}
