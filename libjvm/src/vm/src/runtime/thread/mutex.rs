use std::cell::UnsafeCell;
use std::mem;
use std::mem::MaybeUninit;

/// # Safety
pub unsafe fn raw(m: &ReentrantMutex) -> *mut libc::pthread_mutex_t {
    m.inner.get()
}

pub struct ReentrantMutex {
    inner: UnsafeCell<libc::pthread_mutex_t>,
}

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
    /// # Safety
    /// todo: This function should really be documented
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {
            inner: MaybeUninit::uninit().assume_init(),
        }
    }

    /// # Safety
    /// todo: This function should really be documented
    pub unsafe fn init(&mut self) {
        let mut attr = MaybeUninit::<libc::pthread_mutexattr_t>::uninit();
        let mut ptr_attr = attr.as_mut_ptr();
        let result = libc::pthread_mutexattr_init(ptr_attr as *mut _);
        debug_assert_eq!(result, 0);
        let result =
            libc::pthread_mutexattr_settype(ptr_attr as *mut _, libc::PTHREAD_MUTEX_RECURSIVE);
        debug_assert_eq!(result, 0);
        let result = libc::pthread_mutex_init(self.inner.get(), ptr_attr as *const _);
        debug_assert_eq!(result, 0);
        let result = libc::pthread_mutexattr_destroy(ptr_attr as *mut _);
        debug_assert_eq!(result, 0);
    }

    /// # Safety
    /// todo: This function should really be documented
    pub unsafe fn lock(&self) {
        let result = libc::pthread_mutex_lock(self.inner.get());
        debug_assert_eq!(result, 0);
    }

    /// # Safety
    /// todo: This function should really be documented
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        libc::pthread_mutex_trylock(self.inner.get()) == 0
    }

    /// # Safety
    /// todo: This function should really be documented
    pub unsafe fn unlock(&self) {
        let result = libc::pthread_mutex_unlock(self.inner.get());
        debug_assert_eq!(result, 0);
    }

    /// # Safety
    /// todo: This function should really be documented
    pub unsafe fn destroy(&self) {
        let result = libc::pthread_mutex_destroy(self.inner.get());
        debug_assert_eq!(result, 0);
    }
}
