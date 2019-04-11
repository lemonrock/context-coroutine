// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


struct CoroutineInstance<S: Stack, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine>
{
	type_safe_transfer: TypeSafeTransfer<ChildOutcome<C::Yields, C::Complete>, ParentInstructingChild<C::ResumeArguments>>,
	#[allow(dead_code)] stack: S,
	global_allocator: &'static GTACSA,
	inactive_coroutine_local_allocator: Option<GTACSA::CoroutineLocalAllocator>,
	inactive_current_allocator_in_use: CurrentAllocatorInUse,
	child_coroutine_is_active: bool,
}

impl<S: Stack, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine> Drop for CoroutineInstance<S, GTACSA, C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.child_coroutine_is_active
		{
			use self::ChildOutcome::*;

			match self.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Kill)
			{
				WouldLikeToResume(_) => panic!("A killed coroutine MUST NOT return `WouldLikeToResume`"),

				Complete(Err(panic_information)) => resume_unwind(panic_information),

				Complete(Ok(_)) => (),
			}
		}
	}
}

impl<S: Stack, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine> CoroutineInstance<S, GTACSA, C>
{
	#[inline(always)]
	pub(crate) fn new(stack: S, global_allocator: &'static GTACSA, coroutine_local_allocator: GTACSA::CoroutineLocalAllocator) -> Self
	{
		Self
		{
			type_safe_transfer: TypeSafeTransfer::new(&stack, C::context_entry_point_function_pointer),
			stack,
			global_allocator,
			inactive_coroutine_local_allocator: Some(coroutine_local_allocator),
			inactive_current_allocator_in_use: CurrentAllocatorInUse::Global,
			child_coroutine_is_active: false,
		}
	}

	#[inline(always)]
	pub(crate) fn start(mut self, start_arguments: C::StartArguments) -> StartOutcome<S, GTACSA, C>
	{
		self.pre_transfer_control_to_coroutine();
		let child_outcome = self.type_safe_transfer.resume_drop_safe_unsafe_typing(start_arguments);
		self.post_transfer_control_to_coroutine();

		self.start_process_child_outcome(child_outcome)
	}

	#[inline(always)]
	pub(crate) fn resume_drop_safe(&mut self, arguments: C::ResumeArguments) -> ResumeOutcome<C>
	{
		self.pre_transfer_control_to_coroutine();
		let child_outcome = self.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Resume(arguments));
		self.post_transfer_control_to_coroutine();

		self.resume_drop_safe_process_child_outcome(child_outcome)
	}

	#[inline(always)]
	fn pre_transfer_control_to_coroutine(&mut self)
	{
		self.inactive_coroutine_local_allocator = self.global_allocator.replace_coroutine_local_allocator(self.inactive_coroutine_local_allocator.take());
		self.inactive_current_allocator_in_use = self.global_allocator.save_current_allocator_in_use();
		self.global_allocator.restore_current_allocator_in_use(CurrentAllocatorInUse::CoroutineLocal);
	}

	#[inline(always)]
	fn post_transfer_control_to_coroutine(&mut self)
	{
		self.inactive_coroutine_local_allocator = self.global_allocator.replace_coroutine_local_allocator(self.inactive_coroutine_local_allocator.take());
		self.global_allocator.restore_current_allocator_in_use(self.inactive_current_allocator_in_use);
	}

	#[inline(always)]
	fn start_process_child_outcome(mut self, child_outcome: ChildOutcome<C::Yields, C::Complete>) -> StartOutcome<S, GTACSA, C>
	{
		use self::ChildOutcome::*;

		match child_outcome
		{
			WouldLikeToResume(yields) =>
			{
				self.child_coroutine_is_active = true;

				StartOutcome::WouldLikeToResume(yields, StartedCoroutineInstance { owns: self })
			},

			Complete(Ok(complete)) => StartOutcome::Complete(complete),

			Complete(Err(panic_information)) => resume_unwind(panic_information),
		}
	}

	#[inline(always)]
	fn resume_drop_safe_process_child_outcome(&mut self, child_outcome: ChildOutcome<C::Yields, C::Complete>) -> ResumeOutcome<C>
	{
		use self::ChildOutcome::*;

		match child_outcome
		{
			WouldLikeToResume(yields) => ResumeOutcome::WouldLikeToResume(yields),

			Complete(thread_result) =>
			{
				self.child_coroutine_is_active = false;

				match thread_result
				{
					Ok(complete) => ResumeOutcome::Complete(complete),

					Err(panic_information) => resume_unwind(panic_information),
				}
			}
		}
	}
}
