// This file is part of context-coroutines. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutines/master/COPYRIGHT. No part of context-coroutines, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutines. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutines/master/COPYRIGHT.


/// Outcome of a coroutine's start.
pub enum StartOutcome<S: Sized + Deref<Target=Stack>, C: Coroutine>
{
	/// Coroutine has returned an intermediate result and would to resume.
	WouldLikeToResume(C::Yields, StartedStackAndTypeSafeTransfer<S, C>),

	/// Coroutine has completed.
	Complete(C::Complete),
}
