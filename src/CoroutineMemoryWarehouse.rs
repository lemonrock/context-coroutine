// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// This design does not use a guard page (via `mprotect(PROT_NONE)`) to separate the heap and stack.
///
/// Guard pages need to be at least 1Mb to be effective (see discussion in <https://www.openwall.com/lists/oss-security/2017/06/19/1>), which is probably too large for coroutines if using a design that ensures all virtual memory is physically backed.
///
/// Instead separate heap and stack memory maps are used, which isn't ideal, and does allow one coroutine to trample on another's memory.
#[derive(Debug)]
pub struct CoroutineMemoryWarehouse<HeapSize, StackSize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	global_allocator: &'static GTACSA,
	coroutine_stack_memory_queue: ReferenceCountedLargeRingQueue<CoroutineStackMemory<StackSize>>,
	coroutine_heap_memory_source: ReferenceCountedLargeRingQueue<CoroutineHeapMemory<HeapSize>>,
}

impl<HeapSize, StackSize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> CoroutineMemoryWarehouse<HeapSize, StackSize, GTACSA>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(global_allocator: &'static GTACSA, ideal_maximum_number_of_coroutines: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		Ok
		(
			Self
			{
				global_allocator,
				coroutine_stack_memory_queue: global_allocator.callback_with_thread_local_allocator(|| ReferenceCountedLargeRingQueue::new_exact_fit(ideal_maximum_number_of_coroutines, defaults))?,
				coroutine_heap_memory_source: global_allocator.callback_with_thread_local_allocator(|| ReferenceCountedLargeRingQueue::new_exact_fit(ideal_maximum_number_of_coroutines, defaults))?,
			}
		)
	}

	#[inline(always)]
	pub(crate) fn allocate_coroutine_memory(&self, lifetime_hint: LifetimeHint, heap_memory_block_size_hint: NonZeroUsize) -> Result<CoroutineMemory<HeapSize, StackSize, GTACSA>, AllocErr>
	{
		let stack = self.coroutine_stack_memory_queue.obtain(|| AllocErr)?;
		let heap = self.coroutine_heap_memory_source.obtain(|| AllocErr)?;
		let coroutine_local_allocator = GTACSA::CoroutineLocalAllocator::new_local_allocator(heap, lifetime_hint, heap_memory_block_size_hint);
		
		Ok
		(
			CoroutineMemory
			{
				global_allocator: self.global_allocator,
				stack,
				inactive_coroutine_local_allocator: Some(coroutine_local_allocator),
				inactive_current_allocator_in_use: CurrentAllocatorInUse::CoroutineLocal,
			}
		)
	}
}
