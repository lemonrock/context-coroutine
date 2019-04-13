// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


struct CoroutineInstance<GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine>
{
	type_safe_transfer: TypeSafeTransfer<ChildOutcome<C::Yields, C::Complete>, ParentInstructingChild<C::ResumeArguments>>,
	coroutine_memory: CoroutineMemory<GTACSA>,
	child_coroutine_is_active: bool,
}

impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine> Drop for CoroutineInstance<GTACSA, C>
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

impl<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine> CoroutineInstance<GTACSA, C>
{
	#[inline(always)]
	pub(crate) fn new(coroutine_memory: CoroutineMemory<GTACSA>) -> Self
	{
		Self
		{
			type_safe_transfer: TypeSafeTransfer::new(&coroutine_memory, C::context_entry_point_function_pointer),
			coroutine_memory,
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
		self.coroutine_memory.pre_transfer_control_to_coroutine()
	}

	#[inline(always)]
	fn post_transfer_control_to_coroutine(&mut self)
	{
		self.coroutine_memory.post_transfer_control_to_coroutine()
	}

	#[inline(always)]
	fn start_process_child_outcome(mut self, child_outcome: ChildOutcome<C::Yields, C::Complete>) -> StartOutcome<GTACSA, C>
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
