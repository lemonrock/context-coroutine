// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// A coroutine instance.
///
/// All other fields except for "Initialized once in `initializer()`" must be safe to drop ***REGARDLESS*** of whether they have been initialized.
#[derive(Debug)]
struct CoroutineInstance<CoroutineHeapSize: MemorySize, CoroutineStackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized>
{
	// Never initialized.
	heap: CoroutineHeapMemory<CoroutineHeapSize>,
	stack: CoroutineStackMemory<CoroutineStackSize>,
	
	// Initialized once in `initializer()`.
	// Updated on `free()`.
	generation: CoroutineGenerationCounter,
	
	// Updated on `free()`.
	// Initialized on allocation in `constructor()`.
	child_coroutine_is_active: bool,
	inactive_coroutine_local_allocator: Option<GTACSA::CoroutineLocalAllocator>,
	
	// Initialized on allocation in `constructor()`.
	inactive_current_allocator_in_use: CurrentAllocatorInUse,
	type_safe_transfer: TypeSafeTransfer<ChildOutcome<C::Yields, C::Complete>, ParentInstructingChild<C::ResumeArguments>>,
	coroutine_information: CoroutineInformation,
}

impl<CoroutineHeapSize: MemorySize, CoroutineStackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized> Drop for CoroutineInstance<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		Self::free(unsafe { NonNull::new_unchecked(self) })
	}
}

impl<CoroutineHeapSize: MemorySize, CoroutineStackSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized> LargeRingQueueElement for CoroutineInstance<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>
{
	const Initialization: LargeRingQueueInitialization<Self> = LargeRingQueueInitialization::CreateFullUsingInitializer(Self::initializer);
	
	const ElementsAllocatedFromQueueDropWhenQueueIsDropped: bool = true;
	
	const ElementsLeftOnQueueDropWhenQueueIsDropped: bool = false;
}

macro_rules! initialize_field
{
	($non_null_coroutine_instance: expr, $field: ident, $field_value: expr) =>
	{
		{
			let this = $non_null_coroutine_instance.as_mut();
			Self::initialize_field(&mut this.$field, $field_value)
		}
	}
}

macro_rules! get_field
{
	($non_null_coroutine_instance: expr, $field: ident) =>
	{
		unsafe
		{
			let this = $non_null_coroutine_instance.as_ref();
			
			&(this.$field)
		}
	}
}

impl<CoroutineHeapSize: MemorySize, CoroutineStackSize: MemorySize, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, C: Coroutine, CoroutineInformation: Sized> CoroutineInstance<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>
{
	#[inline(always)]
	unsafe fn initialize_field<Field: Sized>(field: &mut Field, value_to_initialize_field_with: Field)
	{
		write(field, value_to_initialize_field_with)
	}
	
	#[inline(always)]
	unsafe fn initializer(_index: u64, mut non_null_coroutine_instance: NonNull<CoroutineInstance<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>>)
	{
		initialize_field!(non_null_coroutine_instance, generation, CoroutineGenerationCounter::default())
	}
	
	#[inline(always)]
	fn constructor(mut non_null_coroutine_instance: NonNull<Self>, coroutine_information: CoroutineInformation) -> CoroutineGenerationCounter
	{
		let heap: &CoroutineHeapMemory<CoroutineHeapSize> = get_field!(non_null_coroutine_instance, heap);
		let stack: &CoroutineStackMemory<CoroutineStackSize> = get_field!(non_null_coroutine_instance, stack);
		let generation: CoroutineGenerationCounter = *get_field!(non_null_coroutine_instance, generation);
		
		let coroutine_local_allocator = Some(GTACSA::CoroutineLocalAllocator::new_local_allocator(heap.into_memory_source(), C::LifetimeHint, C::HeapMemoryAllocatorBlockSizeHint));
		
		let type_safe_transfer =
		{
			let stack_bottom =
			{
				let size = size_of::<CoroutineStackMemory<CoroutineStackSize>>();
				unsafe { (stack as *const CoroutineStackMemory<CoroutineStackSize> as *const u8).add(size) }
			};
			TypeSafeTransfer::new(stack_bottom, C::context_entry_point_function_pointer)
		};
		
		unsafe
		{
			initialize_field!(non_null_coroutine_instance, inactive_coroutine_local_allocator, coroutine_local_allocator);
			initialize_field!(non_null_coroutine_instance, inactive_current_allocator_in_use, CurrentAllocatorInUse::CoroutineLocal);
			initialize_field!(non_null_coroutine_instance, type_safe_transfer, type_safe_transfer);
			initialize_field!(non_null_coroutine_instance, coroutine_information, coroutine_information);
		}
		generation
	}

	#[inline(always)]
	fn start(coroutine_instance_pointer: CoroutineInstancePointer<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>, coroutine_instance_allocator: &mut CoroutineInstanceAllocator<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>, global_allocator: &'static GTACSA, start_arguments: C::StartArguments) -> StartOutcome<C::Yields, C::Complete>
	{
		let coroutine_instance_handle = coroutine_instance_pointer.as_coroutine_instance_handle();
		
		let this = unsafe { coroutine_instance_pointer.as_mut_unchecked(coroutine_instance_allocator) };
		this.pre_transfer_control_to_coroutine(global_allocator);
		let child_outcome = this.type_safe_transfer.resume_drop_safe_unsafe_typing((coroutine_instance_handle, start_arguments));
		this.post_transfer_control_to_coroutine(global_allocator);
		
		use self::ChildOutcome::*;
		
		match child_outcome
		{
			WouldLikeToResume(yields) =>
			{
				this.child_coroutine_is_active = true;
				
				StartOutcome::WouldLikeToResume(yields)
			}
			
			Complete(thread_result) =>
			{
				this.child_coroutine_is_active = false;
				
				coroutine_instance_allocator.free_coroutine_instance(coroutine_instance_pointer);
				match thread_result
				{
					Ok(complete) => StartOutcome::Complete(complete),
					
					Err(panic_information) => resume_unwind(panic_information),
				}
			}
		}
	}

	#[inline(always)]
	fn resume(coroutine_instance_pointer: CoroutineInstancePointer<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>, coroutine_instance_allocator: &mut CoroutineInstanceAllocator<CoroutineHeapSize, CoroutineStackSize, GTACSA, C, CoroutineInformation>, global_allocator: &'static GTACSA, resume_arguments: C::ResumeArguments) -> ResumeOutcome<C::Yields, C::Complete>
	{
		let this = unsafe { coroutine_instance_pointer.as_mut_unchecked(coroutine_instance_allocator) };
		this.pre_transfer_control_to_coroutine(global_allocator);
		let child_outcome = this.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Resume(resume_arguments));
		this.post_transfer_control_to_coroutine(global_allocator);

		use self::ChildOutcome::*;

		match child_outcome
		{
			WouldLikeToResume(yields) =>
			{
				this.child_coroutine_is_active = true;
				
				ResumeOutcome::WouldLikeToResume(yields)
			},

			Complete(thread_result) =>
			{
				this.child_coroutine_is_active = false;
				
				coroutine_instance_allocator.free_coroutine_instance(coroutine_instance_pointer);
				match thread_result
				{
					Ok(complete) => ResumeOutcome::Complete(complete),

					Err(panic_information) => resume_unwind(panic_information),
				}
			}
		}
	}
	
	#[inline(always)]
	fn free(mut coroutine_instance: NonNull<Self>)
	{
		let this = unsafe { coroutine_instance.as_mut() };
		
		this.generation.increment();
		
		if this.child_coroutine_is_active
		{
			use self::ChildOutcome::*;
			
			match this.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Kill)
			{
				WouldLikeToResume(_) => panic!("A killed coroutine MUST NOT return `WouldLikeToResume`"),
				
				Complete(Err(panic_information)) => resume_unwind(panic_information),
				
				Complete(Ok(_)) => (),
			}
		}
		
		// Force `drop()` of the allocator (if required).
		// LocalAllocators shouldn't rely on `impl Drop` though.
		this.inactive_coroutine_local_allocator = None;
	}
	
	#[inline(always)]
	fn pre_transfer_control_to_coroutine(&mut self, global_allocator: &'static GTACSA)
	{
		self.inactive_coroutine_local_allocator = global_allocator.replace_coroutine_local_allocator(self.read_inactive_coroutine_local_allocator());
		self.inactive_current_allocator_in_use = global_allocator.replace_current_allocator_in_use(self.inactive_current_allocator_in_use);
	}
	
	#[inline(always)]
	fn post_transfer_control_to_coroutine(&mut self, global_allocator: &'static GTACSA)
	{
		self.inactive_current_allocator_in_use = global_allocator.replace_current_allocator_in_use(self.inactive_current_allocator_in_use);
		self.inactive_coroutine_local_allocator = global_allocator.replace_coroutine_local_allocator(self.read_inactive_coroutine_local_allocator());
	}
	
	/// Borrow checker hack to avoid the need to use `self.inactive_coroutine_local_allocator.take()`, which also writes-back to memory.
	#[inline(always)]
	fn read_inactive_coroutine_local_allocator(&self) -> Option<GTACSA::CoroutineLocalAllocator>
	{
		unsafe { read(&self.inactive_coroutine_local_allocator) }
	}
}
