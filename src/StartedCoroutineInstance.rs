// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Holds a stack and a type-safe transfer of a started coroutine; suitable for the ultimate owner of a coroutine.
///
/// On drop the the closure is killed and the stack is then relinquished.
///
/// To create an instance of this struct, use `Coroutine::start_coroutine()`.
pub struct StartedCoroutineInstance<S: Stack, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine>
{
	owns: CoroutineInstance<S, GTACSA, C>,
}

impl<S: Stack, GTACSA: GlobalThreadAndCoroutineSwitchableAllocator, C: Coroutine> StartedCoroutineInstance<S, GTACSA, C>
{
	/// Resumes.
	///
	/// Returns the data transferred to us after the resume (`WouldLikeToResume`) or the final result (`Complete`).
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	pub fn resume(&mut self, arguments: C::ResumeArguments) -> ResumeOutcome<C>
	{
		self.owns.resume_drop_safe(arguments)
	}
}
