// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A 4-bit value.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UserBits(u8);

impl TryFrom<u8> for UserBits
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(four_bits: u8) -> Result<Self, Self::Error>
	{
		if likely!((four_bits | 0x0F) == four_bits)
		{
			Ok(Self(four_bits))
		}
		else
		{
			Err(())
		}
	}
}

impl Into<u8> for UserBits
{
	#[inline(always)]
	fn into(self) -> u8
	{
		self.0
	}
}

impl UserBits
{
	const Zero: Self = Self(0);
}
