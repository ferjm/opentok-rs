use std::ffi::CStr;

pub struct Connection {
    connection_ptr: *const ffi::otc_connection,
}

impl Connection {
    /// Returns the unique identifier for this connection.
    pub fn get_id(&self) -> String {
        let id = unsafe { ffi::otc_connection_get_id(self.connection_ptr) };
        let id: &CStr = unsafe { CStr::from_ptr(id) };
        id.to_str().unwrap().to_owned()
    }

    /// Returns the timestamp corresponding with the creation of the OpenTok
    /// session.
    pub fn get_creation_time(&self) -> i64 {
        unsafe { ffi::otc_connection_get_creation_time(self.connection_ptr) }
    }

    /// Returns the session ID associated with this connection.
    pub fn get_session_id(&self) -> String {
        let id = unsafe { ffi::otc_connection_get_session_id(self.connection_ptr) };
        let id: &CStr = unsafe { CStr::from_ptr(id) };
        id.to_str().unwrap().to_owned()
    }
}

impl From<*const ffi::otc_connection> for Connection {
    fn from(ptr: *const ffi::otc_connection) -> Connection {
        Connection { connection_ptr: ptr }
    }
}