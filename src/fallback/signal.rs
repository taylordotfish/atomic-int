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

//! All functions in this module must be async-signal-safe.

use core::mem::MaybeUninit;
use core::ptr;

fn stderr(msg: &[u8]) {
    unsafe {
        libc::write(libc::STDERR_FILENO, msg.as_ptr().cast(), msg.len());
    }
}

pub struct SignalGuard(libc::sigset_t);

impl SignalGuard {
    pub fn new() -> Self {
        let mut new_set = MaybeUninit::uninit();
        let mut old_set = MaybeUninit::uninit();
        unsafe {
            if libc::sigfillset(new_set.as_mut_ptr()) != 0 {
                stderr(b"[new] sigfillset() failed\n");
                libc::abort();
            }
            if libc::pthread_sigmask(
                libc::SIG_SETMASK,
                new_set.as_ptr(),
                old_set.as_mut_ptr(),
            ) != 0
            {
                stderr(b"[new] pthread_sigmask() failed\n");
                libc::abort();
            }
        }
        // SAFETY: `pthread_sigmask` initializes `old_set`.
        Self(unsafe { old_set.assume_init() })
    }
}

impl Drop for SignalGuard {
    fn drop(&mut self) {
        unsafe {
            if libc::pthread_sigmask(
                libc::SIG_SETMASK,
                &self.0 as _,
                ptr::null_mut(),
            ) != 0
            {
                stderr(b"[drop] pthread_sigmask() failed\n");
                libc::abort();
            }
        }
    }
}
