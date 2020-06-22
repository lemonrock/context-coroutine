// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Manages a particular type of coroutine.
#[derive(Debug)]
pub struct CoroutineManager<CoroutineHeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized>
{
	global_allocator: &'static GTACSA,
	coroutine_instance_allocator: CoroutineInstanceAllocator<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>,
	index: CoroutineManagerIndex,
}

impl<CoroutineHeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized> CoroutineManager<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	/// New instance.
	///
	/// `index` is a zero-based value used when looking up coroutines when they are encoded in tokens or user data for use with epoll or io_uring.
	#[inline(always)]
	pub fn new(index: CoroutineManagerIndex, global_allocator: &'static GTACSA, ideal_maximum_number_of_coroutines: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		Ok
		(
			Self
			{
				global_allocator,
				coroutine_instance_allocator: CoroutineInstanceAllocator::new(ideal_maximum_number_of_coroutines, defaults)?,
				index,
			}
		)
	}
	
	/// Starts the coroutine; execution will transfer to the coroutine.
	///
	/// Execution does not start (returns `Err(AllocErr)`) if there is not memory available to start the coroutine.
	///
	/// Ownership of `start_arguments` will also transfer.
	///
	/// Returns the data transferred to us after the start and a guard object (`StartOutcome<C::Yields, C::Complete>`) to resume the coroutine again or the final result.
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn start_coroutine(&mut self, coroutine_information: CoroutineInformation, start_arguments: C::StartArguments) -> Result<StartOutcome<C::Yields, C::Complete>, AllocErr>
	{
		let coroutine_instance_pointer = self.coroutine_instance_allocator.new_coroutine_instance(self.index, coroutine_information)?;
		Ok(CoroutineInstance::start(coroutine_instance_pointer, &mut self.coroutine_instance_allocator, self.global_allocator, start_arguments))
	}
	
	/// Ownership of `resume_arguments` will also transfer.
	///
	/// Returns the data transferred to us after the resume and a guard object (`ResumeOutcome<C::Yields, C::Complete>`) to resume the coroutine again or the final result.
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn resume_coroutine(&mut self, coroutine_instance_pointer: CoroutineInstancePointer<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>, resume_arguments: C::ResumeArguments) -> ResumeOutcome<C::Yields, C::Complete>
	{
		CoroutineInstance::resume(coroutine_instance_pointer, &mut self.coroutine_instance_allocator, self.global_allocator, resume_arguments)
	}
	
	/// Cancels (kills) an active, but not running, coroutine awaiting its resumption and frees memory.
	#[inline(always)]
	pub fn cancel_coroutine(&mut self, coroutine_instance_pointer: CoroutineInstancePointer<CoroutineHeapSize, StackSize, GTACSA, C, CoroutineInformation>)
	{
		self.coroutine_instance_allocator.free_coroutine_instance(coroutine_instance_pointer)
	}
	
	#[cfg(debug_assertions)]
	#[doc(hidden)]
	#[inline(always)]
	pub fn has_index(&self, index: CoroutineManagerIndex) -> bool
	{
		self.index == index
	}
}
