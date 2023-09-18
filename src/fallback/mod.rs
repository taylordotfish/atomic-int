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

#![allow(unused_macros)]
#[allow(unused_imports)]
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
#[cfg(doc)]
use core::sync::atomic;
use core::sync::atomic::{AtomicBool, Ordering};

#[allow(dead_code)]
#[cfg_attr(not(feature = "signal"), path = "signal_none.rs")]
mod signal;
use signal::SignalGuard;

struct Guard<'a, T> {
    value: &'a mut T,
    lock: &'a AtomicBool,
    order: Ordering,
    _signal: SignalGuard,
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(
            false,
            match self.order {
                Ordering::SeqCst => Ordering::SeqCst,
                _ => Ordering::Release,
            },
        );
    }
}

macro_rules! define_fallback {
    ($atomic:ident$(<$generic:ident>)?, $type:ty, $doc:expr) => {
        pub struct $atomic$(<$generic>)? {
            value: UnsafeCell<$type>,
            lock: AtomicBool,
        }

        impl$(<$generic>)? $atomic$(<$generic>)? {
            /// Creates a new atomic.
            #[doc = concat!("\n\n", $doc, "::new`].")]
            pub const fn new(v: $type) -> Self {
                Self {
                    value: UnsafeCell::new(v),
                    lock: AtomicBool::new(false),
                }
            }

            fn lock(&self, order: Ordering) -> Guard<'_, $type> {
                let success = match order {
                    Ordering::SeqCst => Ordering::SeqCst,
                    _ => Ordering::Acquire,
                };
                let signal = SignalGuard::new();
                while self
                    .lock
                    .compare_exchange_weak(
                        false,
                        true,
                        success,
                        Ordering::Relaxed,
                    )
                    .is_err()
                {
                    while self.lock.load(Ordering::Relaxed) {
                        core::hint::spin_loop();
                    }
                }
                Guard {
                    // SAFETY: This type uses locks to ensure the value won't
                    // be accessed concurrently.
                    value: unsafe { &mut *self.value.get() },
                    lock: &self.lock,
                    order,
                    _signal: signal,
                }
            }

            /// Returns a mutable reference to the underlying value.
            #[doc = concat!("\n\n", $doc, "::get_mut`].")]
            pub fn get_mut(&mut self) -> &mut $type {
                self.value.get_mut()
            }

            /// Consumes the atomic and returns the contained value.
            #[doc = concat!("\n\n", $doc, "::into_inner`].")]
            pub fn into_inner(self) -> $type {
                self.value.into_inner()
            }

            /// Loads a value from the atomic.
            #[doc = concat!("\n\n", $doc, "::load`].")]
            pub fn load(&self, order: Ordering) -> $type {
                *self.lock(order)
            }

            /// Stores a value into the atomic.
            #[doc = concat!("\n\n", $doc, "::store`].")]
            pub fn store(&self, val: $type, order: Ordering) {
                let mut guard = self.lock(order);
                *guard = val;
            }

            /// Stores a value into the atomic, returning the previous
            /// value.
            #[doc = concat!("\n\n", $doc, "::swap`].")]
            pub fn swap(&self, val: $type, order: Ordering) -> $type {
                let mut guard = self.lock(order);
                core::mem::replace(&mut *guard, val)
            }

            /// Stores a value into the atomic if the current value is the same
            /// as the `current` value.
            #[doc = concat!("\n\n", $doc, "::compare_and_swap`].")]
            pub fn compare_and_swap(
                &self,
                current: $type,
                new: $type,
                order: Ordering,
            ) -> $type {
                let mut guard = self.lock(order);
                let prev = *guard;
                if prev == current {
                    *guard = new;
                }
                prev
            }

            /// Stores a value into the atomic if the current value is the same
            /// as the `current` value.
            #[doc = concat!("\n\n", $doc, "::compare_exchange`].")]
            pub fn compare_exchange(
                &self,
                current: $type,
                new: $type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$type, $type> {
                let _ = failure;
                let prev = self.compare_and_swap(current, new, success);
                if prev == current {
                    Ok(prev)
                } else {
                    Err(prev)
                }
            }

            /// Stores a value into the atomic if the current value is the same
            /// as the `current` value.
            #[doc = concat!("\n\n", $doc, "::compare_exchange_weak`].")]
            pub fn compare_exchange_weak(
                &self,
                current: $type,
                new: $type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$type, $type> {
                self.compare_exchange(current, new, success, failure)
            }

            /// Fetches the value, and applies a function to it that returns an
            /// optional new value.
            #[doc = concat!("\n\n", $doc, "::fetch_update`].")]
            pub fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                mut f: F,
            ) -> Result<$type, $type>
            where
                F: FnMut($type) -> Option<$type>,
            {
                let _ = fetch_order;
                let mut guard = self.lock(set_order);
                let prev = *guard;
                if let Some(value) = f(prev) {
                    *guard = value;
                    Ok(prev)
                } else {
                    Err(prev)
                }
            }

            /// Returns a mutable pointer to the underlying value.
            #[doc = concat!("\n\n", $doc, "::as_ptr`].")]
            pub const fn as_ptr(&self) -> *mut $type {
                self.value.get()
            }
        }

        // SAFETY: This type uses locks to ensure concurrent access is sound.
        unsafe impl$(<$generic>)? Sync for $atomic$(<$generic>)? {}
    };
}

macro_rules! define_fallback_int {
    ($atomic:ident, $int:ty, $doc:expr) => {
        define_fallback!($atomic, $int, $doc);

        impl $atomic {
            /// Adds to the current value, returning the previous value.
            #[doc = concat!("\n\n", $doc, "::fetch_add`].")]
            pub fn fetch_add(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard += val;
                prev
            }

            /// Subtracts from the current value, returning the previous value.
            #[doc = concat!("\n\n", $doc, "::fetch_sub`].")]
            pub fn fetch_sub(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard -= val;
                prev
            }

            /// Bitwise “and” with the current value.
            #[doc = concat!("\n\n", $doc, "::fetch_and`].")]
            pub fn fetch_and(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard &= val;
                prev
            }

            /// Bitwise “nand” with the current value.
            #[doc = concat!("\n\n", $doc, "::fetch_nand`].")]
            pub fn fetch_nand(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard = !(prev & val);
                prev
            }

            /// Bitwise “or” with the current value.
            #[doc = concat!("\n\n", $doc, "::fetch_or`].")]
            pub fn fetch_or(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard |= val;
                prev
            }

            /// Bitwise “xor” with the current value.
            #[doc = concat!("\n\n", $doc, "::fetch_xor`].")]
            pub fn fetch_xor(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard ^= val;
                prev
            }

            /// Maximum with the current value.
            #[doc = concat!("\n\n", $doc, "::fetch_max`].")]
            pub fn fetch_max(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard = prev.max(val);
                prev
            }

            /// Minimum with the current value.
            #[doc = concat!("\n\n", $doc, "::fetch_min`].")]
            pub fn fetch_min(&self, val: $int, order: Ordering) -> $int {
                let mut guard = self.lock(order);
                let prev = *guard;
                *guard = prev.min(val);
                prev
            }
        }
    };
}

macro_rules! define_primitive_fallback {
    ($atomic:ident, $int:ident, $bits:literal) => {
        #[cfg(any(doc, not(target_has_atomic = $bits)))]
        define_fallback_int!(
            $atomic,
            $int,
            concat!("See [`atomic::", stringify!($atomic))
        );
    };
}

#[cfg(feature = "primitives")]
with_primitive_atomics!(define_primitive_fallback);

#[cfg(feature = "primitives")]
#[cfg(any(doc, not(target_has_atomic = "ptr")))]
define_fallback!(AtomicPtr<T>, *mut T, "See [`atomic::AtomicPtr");

macro_rules! define_c_fallback {
    ($atomic:ident, $int:ident, $feature:literal, $cfg:ident) => {
        #[cfg(any(doc, not($cfg)))]
        define_fallback_int!(
            $atomic,
            super::ffi::$int,
            "See, e.g., [`atomic::AtomicI32"
        );
    };
}

with_c_atomics!(define_c_fallback);

#[cfg(doc)]
define_fallback_int!(AtomicFallback, i32, "See, e.g., [`atomic::AtomicI32");

#[cfg(doc)]
define_fallback!(AtomicFallbackPtr<T>, *mut T, "See [`atomic::AtomicPtr");
