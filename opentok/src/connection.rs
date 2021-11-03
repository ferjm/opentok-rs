use std::sync::atomic::{AtomicPtr, Ordering};

pub struct Connection {
    ptr: AtomicPtr<*const ffi::otc_connection>,
}

impl Connection {
    /*string_getter!(
        /// Returns the unique identifier for this connection.
        => (get_id, otc_connection_get_id)
    );

    string_getter!(
        /// Returns the session ID associated with this connection.
        => (get_session_id, otc_connection_get_session_id)
    );*/

    /// Returns the timestamp corresponding with the creation of the OpenTok
    /// session.
    pub fn get_creation_time(&self) -> i64 {
        unsafe {
            ffi::otc_connection_get_creation_time(self.ptr.load(Ordering::Relaxed) as *const _)
        }
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        (self.ptr.load(Ordering::Relaxed) as *const ffi::otc_connection).into()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Relaxed);

        if ptr.is_null() {
            return;
        }

        unsafe {
            ffi::otc_connection_delete(ptr as *mut _);
        }

        self.ptr.store(std::ptr::null_mut(), Ordering::Relaxed);
    }
}

impl From<*const ffi::otc_connection> for Connection {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(ptr: *const ffi::otc_connection) -> Connection {
        let ptr = unsafe { ffi::otc_connection_copy(ptr) };
        Connection {
            ptr: AtomicPtr::new(ptr as *mut _),
        }
    }
}
