// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


use super::*;


use ::libc::c_int;
use ::libc::getrlimit;
use ::libc::MAP_ANON;
use ::libc::MAP_FAILED;
#[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd", target_os = "linux", target_os = "openbsd"))] use ::libc::MAP_STACK;
use ::libc::MAP_NORESERVE;
use ::libc::MAP_PRIVATE;
use ::libc::mmap;
use ::libc::mprotect;
use ::libc::munmap;
use ::libc::off_t;
use ::libc::PROT_NONE;
use ::libc::PROT_READ;
use ::libc::PROT_WRITE;
use ::libc::RLIMIT_STACK;
use ::libc::RLIM_INFINITY;
use ::libc::rlim_t;
use ::libc_extra::unix::unistd::getpagesize;
use ::std::cmp::min;
use ::std::io;
use ::std::ptr::null_mut;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering;


include!("ProtectedStack.rs");
include!("Stack.rs");
include!("StackPointer.rs");
