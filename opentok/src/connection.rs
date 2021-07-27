use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

pub struct Connection {
    ptr: Arc<AtomicPtr<*const ffi::otc_connection>>,
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

impl Drop for Connection {
    fn drop(&mut self) {
        // unsafe { ffi::otc_connection_delete(*self.ptr.lock().unwrap() as *mut ffi::otc_connection) };
    }
}

impl From<*const ffi::otc_connection> for Connection {
    fn from(ptr: *const ffi::otc_connection) -> Connection {
        Connection {
            ptr: Arc::new(AtomicPtr::new(ptr as *mut _)),
        }
    }
}
