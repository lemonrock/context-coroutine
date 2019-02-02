// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Holds a stack and a type-safe transfer of a started coroutine; suitable for the ultimate owner of a coroutine.
///
/// On drop the the closure is killed and the stack is then relinquished.
pub struct StartedStackAndTypeSafeTransfer<S: Sized + Deref<Target=Stack>, C: Coroutine>
{
	owns: StackAndTypeSafeTransfer<S, C>,
}

impl<S: Sized + Deref<Target=Stack>, C: Coroutine> Debug for StartedStackAndTypeSafeTransfer<S, C>
where S: Debug, C::ResumeArguments: Debug, C::Yields: Debug, C::Complete: Debug
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "StartedStackAndTypeSafeTransfer {{ owns: {:?} }}", self.owns)
	}
}

impl<S: Sized + Deref<Target=Stack>, C: Coroutine> StartedStackAndTypeSafeTransfer<S, C>
{
	#[inline(always)]
	fn own(mut owns: StackAndTypeSafeTransfer<S, C>) -> Self
	{
		owns.child_coroutine_is_active = true;

		Self
		{
			owns
		}
	}

	/// Resumes.
	///
	/// Returns the data transferred to us after the resume (`Left`) or the final result (`Right`).
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn resume(&mut self, arguments: C::ResumeArguments) -> ResumeOutcome<C>
	{
		let child_outcome = self.owns.type_safe_transfer.resume_drop_safe(ParentInstructingChild::Resume(arguments));
		self.process_child_outcome(child_outcome)
	}

	/// Resumes on top.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn resume_on_top_drop_safe(&mut self, arguments: C::ResumeArguments, resume_on_top_function: ResumeOnTopFunction) -> ResumeOutcome<C>
	{
		let child_outcome = self.owns.type_safe_transfer.resume_on_top_drop_safe(ParentInstructingChild::Resume(arguments), resume_on_top_function);
		self.process_child_outcome(child_outcome)
	}

	#[inline(always)]
	fn process_child_outcome(&mut self, child_outcome: ChildOutcome<C::Yields, C::Complete>) -> ResumeOutcome<C>
	{
		use self::ChildOutcome::*;

		match child_outcome
		{
			WouldLikeToResume(yields) => ResumeOutcome::WouldLikeToResume(yields),

			Complete(thread_result) =>
			{
				self.owns.child_coroutine_is_active = false;

				match thread_result
				{
					Ok(complete) => ResumeOutcome::Complete(complete),

					Err(panic_information) => resume_unwind(panic_information),
				}
			}
		}
	}
}
