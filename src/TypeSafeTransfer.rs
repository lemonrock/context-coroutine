// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Wraps the type of the data being transferred.
#[derive(Debug)]
pub(crate) struct TypeSafeTransfer<Receive: Sized, Send: Sized>
{
	transfer: Transfer,
	marker: PhantomData<(Receive, Send)>,
}

impl<Receive: Sized, Send: Sized> TypeSafeTransfer<Receive, Send>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new<S: Sized + Deref<Target=Stack>>(stack: S, context_function: ContextFn) -> (S, Self)
	{
		let this = Self::wrap(Transfer::new(unsafe { Context::new(&stack, context_function) }, 0));
		(stack, this)
	}

	/// Wraps a transfer, eg from first call to `context_function`.
	#[inline(always)]
	pub(crate) fn wrap(transfer: Transfer) -> Self
	{
		Self
		{
			transfer,
			marker: PhantomData,
		}
	}

	/// Resumes with modification in-place; data transferred can implement drop.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// Uses `take()` so that ownership is transferred to the stack that is extant when `resume_drop_safe()` returns.
	#[inline(always)]
	pub(crate) fn resume_drop_safe(&mut self, data_to_transfer: Send) -> Receive
	{
		self.resume_drop_safe_unsafe_typing::<Send>(data_to_transfer)
	}

	/// Resumes on top with modification in-place.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// Uses `take()` so that ownership is transferred to the stack that is extant when `resume_on_top_drop_safe()` returns.
	///
	/// It is unlikely you need to use this function.
	#[inline(always)]
	pub(crate) fn resume_on_top_drop_safe(&mut self, data_to_transfer: Send, resume_on_top_function: ResumeOnTopFunction) -> Receive
	{
		self.resume_on_top_drop_safe_unsafe_typing::<Send>(data_to_transfer, resume_on_top_function)
	}

	/// Resumes with modification in-place; data transferred can implement drop.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// Uses `take()` so that ownership is transferred to the stack that is extant when `resume_drop_safe()` returns.
	#[inline(always)]
	pub(crate) fn resume_drop_safe_unsafe_typing<T>(&mut self, data_to_transfer: T) -> Receive
	{
		let mut data_to_transfer_drop_safe = Some(data_to_transfer);
		let pointer_out = Self::option_to_pointer::<T>(&mut data_to_transfer_drop_safe);

		self.transfer.resume::<NonNull<Option<T>>>(pointer_out);

		self.take_data()
	}

	/// Resumes on top with modification in-place.
	///
	/// Returns the data transferred to us after the resume.
	///
	/// Uses `take()` so that ownership is transferred to the stack that is extant when `resume_on_top_drop_safe()` returns.
	///
	/// It is unlikely you need to use this function.
	#[inline(always)]
	pub(crate) fn resume_on_top_drop_safe_unsafe_typing<T>(&mut self, data_to_transfer: T, resume_on_top_function: ResumeOnTopFunction) -> Receive
	{
		let mut data_to_transfer_drop_safe = Some(data_to_transfer);
		let pointer_out = Self::option_to_pointer::<T>(&mut data_to_transfer_drop_safe);

		self.transfer.resume_on_top::<NonNull<Option<T>>>(pointer_out, resume_on_top_function);

		self.take_data()
	}

	#[inline(always)]
	pub(crate) fn start_child_arguments<T>(&self) -> T
	{
		self.take_data_unsafe_typing::<T>()
	}

	#[inline(always)]
	fn take_data(&self) -> Receive
	{
		self.take_data_unsafe_typing::<Receive>()
	}

	#[inline(always)]
	fn take_data_unsafe_typing<UnsafeT>(&self) -> UnsafeT
	{
		let mut pointer_in = self.transfer.transferred_data::<NonNull<Option<UnsafeT>>>();
		let data_from_transfer_drop_safe = unsafe { pointer_in.as_mut() };
		data_from_transfer_drop_safe.take().expect("take_data can only be called once per resumption")
	}

	#[inline(always)]
	fn option_to_pointer<T>(data_to_transfer_drop_safe: &mut Option<T>) -> NonNull<Option<T>>
	{
		unsafe { NonNull::new_unchecked(data_to_transfer_drop_safe as *mut Option<T>) }
	}
}
