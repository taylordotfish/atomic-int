/*
 * Copyright 2023 taylor.fish <contact@taylor.fish>
 *
 * This file is part of atomic-int.
 *
 * atomic-int is licensed under the Apache License, Version 2.0
 * (the "License"); you may not use atomic-int except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![cfg_attr(not(feature = "libc"), no_std)]
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]
#![deny(unsafe_op_in_unsafe_fn)]

//! atomic-int provides atomics for additional integers, such as C/FFI types
//! like [`c_int`].
//!
//! For integer types that are aliases of primitive integers that have built-in
//! Rust atomics, this crate simply re-exports those atomics. Otherwise, this
//! crate provides a spinlock-based fallback implementation with a compatible
//! API.
//!
//! This crate also provides types that directly correspond with Rust’s
//! standard atomics, like [`AtomicU64`], with the difference that the fallback
//! implementation will similarly be used for any such atomics that are not
//! supported on a given platform. Thus, all atomics provided by this crate are
//! available on all platforms[^1] in some form—either the built-in or fallback
//! implementation.
//!
//! Crate features
//! --------------
//!
//! Types that directly correspond with Rust’s standard atomics like
//! [`AtomicU64`] are available with the feature `primitives` (enabled by
//! default). This includes [`AtomicPtr`], even though it isn’t exactly an
//! integer.
//!
//! Atomic C integer types like [`AtomicCInt`] and [`AtomicCUlong`] are
//! available with the feature `c` (enabled by default). For more granularity,
//! a separate feature exists for each C integer (e.g., `c_int` and `c_ulong`).
//!
//! The spinlock-based fallback implementation can cause deadlocks with signal
//! handlers. To avoid this, enable the feature `signal`, which blocks incoming
//! signals while the lock is held. This feature is Unix-specific.
//!
//! atomic-int can optionally depend on [`libc`]. If this dependency is
//! enabled, atomic-int will use the C integer types from [`libc`] instead of
//! [`core::ffi`]. This should not make a noticeable difference, but it can
//! decrease the minimum required Rust version, as C integer types were added
//! to [`core::ffi`] only in version 1.64. The feature `signal` always enables
//! `libc`.
//!
//! This crate is `no_std` when `libc` is not enabled.
//!
//! [^1]: As long as the platform supports [`AtomicBool`], which is required
//!       for the fallback implementation.
//!
//! [`libc`]: https://docs.rs/libc/0.2
//! [`c_int`]: ffi::c_int
//! [`AtomicBool`]: atomic::AtomicBool

#[allow(unused_imports)]
use core::sync::atomic;

#[allow(unused_imports)]
#[cfg(not(feature = "libc"))]
use core::ffi;

#[allow(unused_imports)]
#[cfg(feature = "libc")]
use libc as ffi;

mod detail {
    pub trait HasAtomic {
        type Atomic;
    }
}

use detail::HasAtomic;

macro_rules! with_primitive_atomics {
    ($macro:path) => {
        $macro!(AtomicI8, i8, "8");
        $macro!(AtomicU8, u8, "8");
        $macro!(AtomicI16, i16, "16");
        $macro!(AtomicU16, u16, "16");
        $macro!(AtomicI32, i32, "32");
        $macro!(AtomicU32, u32, "32");
        $macro!(AtomicI64, i64, "64");
        $macro!(AtomicU64, u64, "64");
        $macro!(AtomicI128, i128, "128");
        $macro!(AtomicU128, u128, "128");
        $macro!(AtomicIsize, isize, "ptr");
        $macro!(AtomicUsize, usize, "ptr");
    };
}

macro_rules! impl_has_atomic {
    ($atomic:ident, $int:ident, $bits:literal) => {
        #[cfg(target_has_atomic = $bits)]
        impl HasAtomic for $int {
            type Atomic = atomic::$atomic;
        }
    };
}

with_primitive_atomics!(impl_has_atomic);

#[allow(unused_macros)]
macro_rules! define_primitive_atomic {
    ($atomic:ident$(<$generic:ident>)?, $type:ty, $bits:literal) => {
        #[cfg(all(not(doc), target_has_atomic = $bits))]
        pub type $atomic$(<$generic>)? = atomic::$atomic$(<$generic>)?;

        #[cfg(any(doc, not(target_has_atomic = $bits)))]
        #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "primitives")))]
        /// An atomic
        #[doc = concat!("[`", stringify!($type), "`].")]
        ///
        /// This is either an alias to the type in [`core::sync::atomic`], or,
        /// if not available, a spinlock-based fallback type.
        ///
        /// [`*mut T`]: pointer
        pub type $atomic$(<$generic>)? = fallback::$atomic$(<$generic>)?;
    };
}

#[cfg(feature = "primitives")]
with_primitive_atomics!(define_primitive_atomic);

#[cfg(feature = "primitives")]
define_primitive_atomic!(AtomicPtr<T>, *mut T, "ptr");

#[cfg(feature = "primitives")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "primitives")))]
/// An atomic [`bool`].
///
/// This type alias is provided for completeness, but it always points to the
/// real [`AtomicBool`][real] in [`core::sync::atomic`], as even the fallback
/// atomic implementation in this crate requires [`AtomicBool`][real].
///
/// [real]: atomic::AtomicBool
pub type AtomicBool = atomic::AtomicBool;

macro_rules! with_c_atomics {
    ($macro:path) => {
        #[cfg(feature = "c_char")]
        $macro!(AtomicCChar, c_char, "c_char", has_c_char_atomic);
        #[cfg(feature = "c_schar")]
        $macro!(AtomicCSchar, c_schar, "c_schar", has_c_schar_atomic);
        #[cfg(feature = "c_uchar")]
        $macro!(AtomicCUchar, c_uchar, "c_uchar", has_c_uchar_atomic);
        #[cfg(feature = "c_short")]
        $macro!(AtomicCShort, c_short, "c_short", has_c_short_atomic);
        #[cfg(feature = "c_ushort")]
        $macro!(AtomicCUshort, c_ushort, "c_ushort", has_c_ushort_atomic);
        #[cfg(feature = "c_int")]
        $macro!(AtomicCInt, c_int, "c_int", has_c_int_atomic);
        #[cfg(feature = "c_uint")]
        $macro!(AtomicCUint, c_uint, "c_uint", has_c_uint_atomic);
        #[cfg(feature = "c_long")]
        $macro!(AtomicCLong, c_long, "c_long", has_c_long_atomic);
        #[cfg(feature = "c_ulong")]
        $macro!(AtomicCUlong, c_ulong, "c_ulong", has_c_ulong_atomic);
        #[cfg(feature = "c_longlong")]
        $macro!(
            AtomicCLonglong,
            c_longlong,
            "c_longlong",
            has_c_longlong_atomic
        );
        #[cfg(feature = "c_ulonglong")]
        $macro!(
            AtomicCUlonglong,
            c_ulonglong,
            "c_ulonglong",
            has_c_ulonglong_atomic
        );
    };
}

#[allow(unused_macros)]
macro_rules! define_c_atomic {
    ($atomic:ident, $int:ident, $feature:literal, $cfg:ident) => {
        #[cfg(all(not(doc), $cfg))]
        pub type $atomic = <ffi::$int as HasAtomic>::Atomic;

        #[cfg(any(doc, not($cfg)))]
        #[cfg_attr(feature = "doc_cfg", doc(cfg(feature = $feature)))]
        /// An atomic
        #[doc = concat!("[`", stringify!($int), "`][1].")]
        ///
        /// This is either an alias to the appropriate atomic integer type in
        /// [`core::sync::atomic`], or a spinlock-based fallback type.
        #[doc = concat!("\n\n[1]: ffi::", stringify!($int))]
        pub type $atomic = fallback::$atomic;
    };
}

with_c_atomics!(define_c_atomic);

mod fallback;

#[rustfmt::skip]
#[cfg(doc)]
#[cfg_attr(feature = "doc_cfg", doc(cfg(doc)))]
/// An example fallback implementation of an atomic integer.
///
/// When no built-in atomic for a certain integer type is available, its type
/// alias in this crate points to a type like this, except with methods that
/// take and return that integer type, rather than [`i32`].
///
/// This type internally uses spinlocks, which can cause deadlocks with signal
/// handlers. To avoid this, enable the feature `signal`, which blocks incoming
/// signals while the spinlock is held.
///
/// The API of this type is designed to be compatible with the atomic integer
/// types in [`core::sync::atomic`].
///
/// This type is exposed only in the documentation for illustrative purposes.
pub use fallback::AtomicFallback;

#[rustfmt::skip]
#[cfg(doc)]
#[cfg_attr(feature = "doc_cfg", doc(cfg(doc)))]
/// An example fallback implementation of an atomic pointer.
///
/// [`AtomicPtr`] points to a type like this when no built-in atomic pointer is
/// available.
///
/// This type is the pointer version of [`AtomicFallback`]; see its
/// documentation for more details. Like [`AtomicFallback`], this type is
/// exposed only in the documentation for illustrative purposes.
pub use fallback::AtomicFallbackPtr;
