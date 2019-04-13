// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


#[derive(Debug)]
struct CoroutineMemory<GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator>
{
	global_allocator: &'static GTACSA,
	stack_bottom_which_is_higher_memory_address: MemoryAddress,
	inactive_coroutine_local_allocator: Option<GTACSA::CoroutineLocalAllocator>,
	inactive_current_allocator_in_use: CurrentAllocatorInUse,
}

impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator> Stack for CoroutineMemory<GTACSA>
{
	#[inline(always)]
	fn bottom(&self) -> StackPointer
	{
		self.stack_bottom_which_is_higher_memory_address.as_ptr() as StackPointer
	}
}

impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator> CoroutineMemory<GTACSA>
{
	#[inline(always)]
	fn pre_transfer_control_to_coroutine(&mut self)
	{
		self.inactive_coroutine_local_allocator = self.global_allocator.replace_coroutine_local_allocator(self.read_inactive_coroutine_local_allocator());
		self.inactive_current_allocator_in_use = self.global_allocator.replace_current_allocator_in_use(self.inactive_current_allocator_in_use);
	}

	#[inline(always)]
	fn post_transfer_control_to_coroutine(&mut self)
	{
		self.inactive_current_allocator_in_use = self.global_allocator.replace_current_allocator_in_use(self.inactive_current_allocator_in_use);
		self.inactive_coroutine_local_allocator = self.global_allocator.replace_coroutine_local_allocator(self.read_inactive_coroutine_local_allocator());
	}

	/// Borrow checker hack to avoid the need to use `self.inactive_coroutine_local_allocator.take()`, which also writes-back to memory.
	#[inline(always)]
	fn read_inactive_coroutine_local_allocator(&self) -> Option<GTACSA::CoroutineLocalAllocator>
	{
		unsafe { read(&self.inactive_coroutine_local_allocator) }
	}
}
