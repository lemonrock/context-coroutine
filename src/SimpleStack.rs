// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// A simple stack.
#[derive(Debug)]
pub struct SimpleStack;

impl Deref for SimpleStack
{
	type Target = Stack;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		// TODO: pre-allocate and check for allocation failures!
//		let coroutine_stack_size: usize = xxxx;
//		let coroutine_stack = ProtectedFixedSizeStack::new(coroutine_stack_size);
		unimplemented!();
	}
}
