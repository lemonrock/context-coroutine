// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// This design does not use a guard page (via `mprotect(PROT_NONE)`) to separate the heap and stack.
///
/// Guard pages need to be at least 1Mb to be effective (see discussion in <https://www.openwall.com/lists/oss-security/2017/06/19/1>), which is probably too large for coroutines if using a design that ensures all virtual memory is physically backed.
#[derive(Debug)]
pub struct CoroutineMemorySource<GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator, CoroutineLocalAllocatorConstructor: Fn(RcMemorySource<ArenaMemorySource<MemoryMapSource>>, NonZeroUsize) -> Result<GTACSA::CoroutineLocalAllocator, AllocErr>>
{
	global_allocator: &'static GTACSA,
	coroutine_local_allocator_constructor: CoroutineLocalAllocatorConstructor,
	arena: RcMemorySource<ArenaMemorySource<MemoryMapSource>>,
	heap_size: NonZeroUsize,
	stack_size: NonZeroUsize,
	block_size: NonZeroUsize,
}

impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator, CoroutineLocalAllocatorConstructor: Fn(RcMemorySource<ArenaMemorySource<MemoryMapSource>>, NonZeroUsize) -> Result<GTACSA::CoroutineLocalAllocator, AllocErr>> CoroutineMemorySource<GTACSA, CoroutineLocalAllocatorConstructor>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(global_allocator: &'static GTACSA, coroutine_local_allocator_constructor: CoroutineLocalAllocatorConstructor, memory_map_source: MemoryMapSource, memory_source_size: NonZeroUsize, heap_size: NonZeroUsize, stack_size: NonZeroUsize) -> Result<Self, AllocErr>
	{
		/// On x86-64, the stack needs to be 16 byte aligned with a minimum size of 64 bytes in order to store a `SavedContext`.
		///
		/// Some allocators, such as MultipleBinarySearchTreeAllocator, use a 32-byte alignment - so the heap needs to be 64-byte aligned.
		#[inline(always)]
		const fn align_memory_size(size: NonZeroUsize) -> NonZeroUsize
		{
			const MemoryAlignment: usize = 64;
			non_zero_usize((size.get() + (MemoryAlignment - 1)) / MemoryAlignment)
		}

		let heap_size = align_memory_size(heap_size);
		let stack_size = align_memory_size(stack_size);
		let block_size = heap_size.add_non_zero(stack_size);

		Ok
		(
			Self
			{
				global_allocator,
				coroutine_local_allocator_constructor,
				arena: RcMemorySource::new_thread_local(global_allocator, ArenaMemorySource::new_by_amount(memory_map_source, block_size, memory_source_size, |_block_memory_address, _block_size| {})?),
				heap_size,
				stack_size,
				block_size,
			}
		)
	}

	#[inline(always)]
	fn allocate_coroutine_memory(&self) -> Result<CoroutineMemory<GTACSA>, AllocErr>
	{
		let coroutine_local_allocator = (self.coroutine_local_allocator_constructor)(self.arena.clone(), self.block_size)?;

		let MemoryRange { to: stack_bottom_which_is_higher_memory_address, .. } = coroutine_local_allocator.memory_range();

		Ok
		(
			CoroutineMemory
			{
				global_allocator: self.global_allocator,
				stack_bottom_which_is_higher_memory_address,
				inactive_coroutine_local_allocator: Some(coroutine_local_allocator),
				inactive_current_allocator_in_use: CurrentAllocatorInUse::CoroutineLocal,
			}
		)
	}
}
