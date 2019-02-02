// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Extension trait to Transfer.
pub trait TransferExt: Sized
{
	/// Create a new instance with initial data.
	fn new<TD: TransferableData, S: Sized + Deref<Target=Stack>>(stack: S, context_function: ContextFn, initial_data_to_transfer: TD) -> (S, Self);

	/// Get data.
	fn transferred_data<TD: TransferableData>(&self) -> TD;

	/// Resume.
	fn resume<TD: TransferableData>(&mut self, data_to_transfer: TD);

	/// Resume on top.
	fn resume_on_top<TD: TransferableData>(&mut self, data_to_transfer: TD, resume_on_top_function: ResumeOnTopFunction);

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn context(&mut self) -> Context;
}

impl TransferExt for Transfer
{
	#[inline(always)]
	fn new<TD: TransferableData, S: Sized + Deref<Target=Stack>>(stack: S, context_function: ContextFn, initial_data_to_transfer: TD) -> (S, Self)
	{
		let transfer = Transfer::new(unsafe { Context::new(stack.deref(), context_function) }, initial_data_to_transfer.into_usize());
		(stack, transfer)
	}

	#[inline(always)]
	fn transferred_data<TD: TransferableData>(&self) -> TD
	{
		TD::from_usize(self.data)
	}

	/// Resume.
	#[inline(always)]
	fn resume<TD: TransferableData>(&mut self, data_to_transfer: TD)
	{
		*self = unsafe { self.context().resume(data_to_transfer.into_usize()) };
	}

	/// Resume on top.
	#[inline(always)]
	fn resume_on_top<TD: TransferableData>(&mut self, data_to_transfer: TD, resume_on_top_function: ResumeOnTopFunction)
	{
		*self = unsafe { self.context().resume_ontop(data_to_transfer.into_usize(), resume_on_top_function) };
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn context(&mut self) -> Context
	{
		// This is horrible, but:-
		//
		// * `Context` is nether Clone nor Copy;
		// * `Context.resume()` and `Context.resume_ontop()` take Context by value, not by reference, and we can't move it or derefence it as we only have a mutable reference to it.
		//
		// However, we happen to know the contents of Context are just `&'static c_void`.
		read(&mut self.context)
	}
}
