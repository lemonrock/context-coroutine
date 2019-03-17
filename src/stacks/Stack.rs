// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Organisation of the stack in x86 (and nearly all other modern CPUs).
///
/// * The bottom (origin) of the stack is a *high* address.
/// * The top of the stack is a *low* address.
/// * The stack grows downwards.
/// * Thus pushing onto the stack *subtracts* from the top address, making it lower (smaller).
/// * Thus popping from the stack *adds* to the top address, making it higher (larger).
/// * [Eli Bendersky explains this well](https://eli.thegreenplace.net/2011/02/04/where-the-top-of-the-stack-is-on-x86/).
///
/// A diagram:-
/// ```
/// eg 0x1006  +---+ Top (origin): High Address
///            | S |
///            | T |
///            | A |
///            | C |
///            | K |
/// eg 0x1000  +---+ Bottom: Low Address
///
/// Pushing a 2 byte value, X, grows the stack thus:-
///
/// eg 0x1006  +---+ Top (origin): High Address
///            | S |
///            | T |
///            | A |
///            | C |
///            | K |
/// eg 0x1000  |···| Former Bottom
///            | X |
/// eg  0x998  +---+ Bottom: Low Address
///
/// Stacks can have a 'guard' page below the Bottom which can be mprotected'd as PROT_NONE; any reads or write will cause a SIGSEGV.
///
/// eg 0x1006  +---+ Top (origin): High Address
///            | S |
///            | T |
///            | A | Regular Pages (mprotect: PROT_READ + PROT_WRITE)
///            | C |
///            | K |
/// eg 0x1000  +---+ Bottom: Low Address
///            |   |
///            |   | Guard Page (mprotect: PROT_NONE)
///            |   |
///            +---+ Bottom of Guard Page
///
/// A 'guard' page is 4,096 bytes on x86-64.
/// ```
pub trait Stack
{
	/// Top.
	///
	/// This ***must*** be page aligned.
	#[inline(always)]
	fn top(&self) -> StackPointer;

	/// Size, excluding any guard page at the bottom.
	#[inline(always)]
	fn size(&self) -> usize;
}
