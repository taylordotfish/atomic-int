use core::mem::MaybeUninit;
use core::ptr;

pub struct SignalGuard(libc::sigset_t);

impl SignalGuard {
    pub fn new() -> Self {
        let mut new_set = MaybeUninit::uninit();
        let mut old_set = MaybeUninit::uninit();
        unsafe {
            libc::sigfillset(new_set.as_mut_ptr());
            libc::pthread_sigmask(
                libc::SIG_SETMASK,
                new_set.as_ptr(),
                old_set.as_mut_ptr(),
            );
        }
        // SAFETY: `pthread_sigmask` initializes `old_set`.
        Self(unsafe { old_set.assume_init() })
    }
}

impl Drop for SignalGuard {
    fn drop(&mut self) {
        unsafe {
            libc::pthread_sigmask(
                libc::SIG_SETMASK,
                &self.0 as _,
                ptr::null_mut(),
            );
        }
    }
}
