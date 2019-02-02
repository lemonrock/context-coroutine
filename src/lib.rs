// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #context-coroutine
//! 
//! This is a simple set of extensions to the [context](https://github.com/zonyitoo/context-rs) crate to provide stackful coroutines.
//!
//! The intended use case is mostly for read and write (input and output, I/O) with socket file descriptors.
//!
//! The developers are not associated with the authors of [context](https://github.com/zonyitoo/context-rs) but are extremely grateful for the work they've put into to a superb piece of code.


extern crate context;


use ::context::context::*;
use ::context::stack::*;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::ops::Deref;
use ::std::panic::*;
use ::std::ptr::NonNull;
use ::std::ptr::read;
use ::std::thread;


include!("ChildOutcome.rs");
include!("Coroutine.rs");
include!("ParentInstructingChild.rs");
include!("ResumeOnTopFunction.rs");
include!("ResumeOutcome.rs");
include!("SimpleStack.rs");
include!("StackAndTypeSafeTransfer.rs");
include!("StartedStackAndTypeSafeTransfer.rs");
include!("StartOutcome.rs");
include!("TransferableData.rs");
include!("TransferExt.rs");
include!("TypeSafeTransfer.rs");
include!("Yielder.rs");
