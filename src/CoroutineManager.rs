// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Manages a particular type of coroutine.
pub struct CoroutineManager<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized>
{
	global_allocator: &'static GTACSA,
	coroutine_instance_allocator: CoroutineInstanceAllocator<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
}

impl<HeapSize: MemorySize, StackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>, C: Coroutine, CoroutineInformation: Sized> CoroutineManager<HeapSize, StackSize, GTACSA, C, CoroutineInformation>
{
	/// New instance.
	#[inline(always)]
	pub fn new(global_allocator: &'static GTACSA, ideal_maximum_number_of_coroutines: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		Ok
		(
			Self
			{
				global_allocator,
				coroutine_instance_allocator: CoroutineInstanceAllocator::new(ideal_maximum_number_of_coroutines, defaults)?,
			}
		)
	}
	
	/// Starts the coroutine; execution will transfer to the coroutine.
	///
	/// Execution does not start (returns `Err(AllocErr)`) if there is not memory available to start the coroutine.
	///
	/// Ownership of `start_arguments` will also transfer.
	///
	/// Returns the data transferred to us after the start and a guard object (`StartOutcome<C>`) to resume the coroutine again or the final result.
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn start_coroutine(&mut self, coroutine_information: CoroutineInformation, start_arguments: C::StartArguments) -> Result<StartOutcome<C>, AllocErr>
	{
		let coroutine_instance_pointer = self.coroutine_instance_allocator.new_coroutine_instance(coroutine_information)?;
		Ok(CoroutineInstance::start(coroutine_instance_pointer, &mut self.coroutine_instance_allocator, self.global_allocator, start_arguments))
	}
	
	/// Ownership of `resume_arguments` will also transfer.
	///
	/// Returns the data transferred to us after the resume and a guard object (`ResumeOutcome<C>`) to resume the coroutine again or the final result.
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn resume_coroutine(&mut self, coroutine_instance_pointer: CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>, resume_arguments: C::ResumeArguments) -> ResumeOutcome<C>
	{
		CoroutineInstance::resume(coroutine_instance_pointer, &mut self.coroutine_instance_allocator, self.global_allocator, resume_arguments)
	}
	
	/// Cancels (kills) an active, but not running, coroutine awaiting its resumption and frees memory.
	#[inline(always)]
	pub fn cancel_coroutine(&mut self, coroutine_instance_pointer: CoroutineInstancePointer<HeapSize, StackSize, GTACSA, C, CoroutineInformation>)
	{
		self.coroutine_instance_allocator.free_coroutine_instance(coroutine_instance_pointer)
	}
}
