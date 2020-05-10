// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


use super::*;


use linux_support::file_descriptors::CreationError;
use linux_support::memory::PageSize;
use linux_support::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use linux_support::memory::mapping::*;
use linux_support::resource_limits::ResourceName;
use std::cmp::min;
use std::num::NonZeroU64;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;


include!("ProtectedStack.rs");
include!("Stack.rs");
include!("StackPointer.rs");
