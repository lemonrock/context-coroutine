// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Data that can be transfered to a continuation.
pub trait TransferableData
{
	/// Into usize; panics on failure.
	fn into_usize(self) -> usize;

	/// From usize; panics on failure.
	fn from_usize(value: usize) -> Self;
}

macro_rules! as_transferable_data
{
	($name: tt) =>
	{
		impl TransferableData for $name
		{
			#[inline(always)]
			fn into_usize(self) -> usize
			{
				self as usize
			}

			#[inline(always)]
			fn from_usize(value: usize) -> Self
			{
				value as $name
			}
		}
	}
}

as_transferable_data!(u8);
as_transferable_data!(u16);
as_transferable_data!(u32);
#[cfg(target_pointer_width = "64")] as_transferable_data!(u64);
as_transferable_data!(usize);

as_transferable_data!(i8);
as_transferable_data!(i16);
as_transferable_data!(i32);
#[cfg(target_pointer_width = "64")] as_transferable_data!(i64);
as_transferable_data!(isize);

as_transferable_data!(f32);
#[cfg(target_pointer_width = "64")] as_transferable_data!(f64);

impl TransferableData for ()
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		0
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		debug_assert!(value == 0, "value was not zero");

		()
	}
}

impl TransferableData for bool
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value != 0
	}
}

impl<T> TransferableData for *const T
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as *const T
	}
}

impl<T> TransferableData for *mut T
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		value as *mut T
	}
}

impl<T> TransferableData for NonNull<T>
{
	#[inline(always)]
	fn into_usize(self) -> usize
	{
		self.as_ptr() as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		let pointer = value as *mut T;
		debug_assert!(!pointer.is_null(), "pointer value is null");
		unsafe { NonNull::new_unchecked(pointer) }
	}
}
