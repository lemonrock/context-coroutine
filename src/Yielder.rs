// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// A simple structure to make it easy to 'yield' from a coroutine.
#[derive(Debug)]
pub struct Yielder<'yielder, ResumeArguments: 'yielder, Yields: 'yielder, Complete: 'yielder>
{
	type_safe_transfer: &'yielder mut TypeSafeTransfer<ParentInstructingChild<ResumeArguments>, ChildOutcome<Yields, Complete>>
}

impl<'yielder, ResumeArguments: 'yielder, Yields: 'yielder, Complete: 'yielder> Yielder<'yielder, ResumeArguments, Yields, Complete>
{
	#[inline(always)]
	fn new(type_safe_transfer: &'yielder mut TypeSafeTransfer<ParentInstructingChild<ResumeArguments>, ChildOutcome<Yields, Complete>>) -> Self
	{
		Self
		{
			type_safe_transfer
		}
	}

	/// Yields.
	///
	/// Returns either `Ok(resume_arguments)` or `Err(kill_error)`.
	#[inline(always)]
	pub fn yields<E>(&mut self, yields: Yields, kill_error: E) -> Result<ResumeArguments, E>
	{
		use self::ParentInstructingChild::*;

		match self.type_safe_transfer.resume_drop_safe(ChildOutcome::WouldLikeToResume(yields))
		{
			Resume(resume_arguments) => Ok(resume_arguments),

			Kill => Err(kill_error),
		}
	}
}
