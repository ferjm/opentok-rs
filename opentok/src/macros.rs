macro_rules! ffi_callback_proxy {
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty) => {
        unsafe extern "C" fn $fn_name(_: $target_type, data: *mut c_void) {
            let target = data as *mut $target_rust_type;
            let _ = (*target).$fn_name();
        }
    };
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $arg1_type:ty) => {
        unsafe extern "C" fn $fn_name(_: $target_type, data: *mut c_void, arg1: $arg1_type) {
            let target = data as *mut $target_rust_type;
            let _ = (*target).$fn_name(arg1);
        }
    };
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $arg1_type:ty, $arg2_type:ty) => {
        unsafe extern "C" fn $fn_name(
            _: $target_type,
            data: *mut c_void,
            arg1: $arg1_type,
            arg2: $arg2_type,
        ) {
            let target = data as *mut $target_rust_type;
            let _ = (*target).$fn_name(arg1, arg2);
        }
    };
}

macro_rules! ffi_callback_proxy_with_return {
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $return_type:ty) => {
        unsafe extern "C" fn $fn_name(_: $target_type, data: *mut c_void) -> $return_type {
            let target = data as *mut $target_rust_type;
            let result: OtcBool = (*target).$fn_name().into();
            result.0
        }
    };
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $arg1_type:ty, $return_type:ty) => {
        unsafe extern "C" fn $fn_name(
            _: $target_type,
            data: *mut c_void,
            arg1: $arg1_type,
        ) -> $return_type {
            let target = data as *mut $target_rust_type;
            let result: OtcBool = (*target).$fn_name(arg1).into();
            result.0
        }
    };
}

macro_rules! ffi_callback {
    ($fn_name:ident) => {
        extern "C" fn $fn_name(session: *mut ffi::otc_session, _user_data: *mut c_void) {
            SESSIONS
                .lock()
                .unwrap()
                .get(&(session as usize))
                .unwrap()
                .$fn_name();
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty) => {
        unsafe extern "C" fn $fn_name(
            session: *mut ffi::otc_session,
            _user_data: *mut c_void,
            $arg1: $ty1,
        ) {
            SESSIONS
                .lock()
                .unwrap()
                .get(&(session as usize))
                .unwrap()
                .$fn_name($arg1.into());
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty, $arg2:ident, $ty2:ty) => {
        unsafe extern "C" fn $fn_name(
            session: *mut ffi::otc_session,
            _user_data: *mut c_void,
            $arg1: $ty1,
            $arg2: $ty2,
        ) {
            SESSIONS
                .lock()
                .unwrap()
                .get(&(session as usize))
                .unwrap()
                .$fn_name($arg1.into(), $arg2.into());
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty, $arg2:ident, $ty2:ty, $arg3:ident, $ty3:ty) => {
        unsafe extern "C" fn $fn_name(
            session: *mut ffi::otc_session,
            _user_data: *mut c_void,
            $arg1: $ty1,
            $arg2: $ty2,
            $arg3: $ty3,
        ) {
            SESSIONS
                .lock()
                .unwrap()
                .get(&(session as usize))
                .unwrap()
                .$fn_name($arg1.into(), $arg2.into(), $arg3.into());
        }
    };
}

macro_rules! callback {
    ($fn_name:ident) => {
        pub fn $fn_name(&self) {
            if let Some(ref callback) = self.$fn_name {
                callback();
            }
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty) => {
        pub fn $fn_name(&self, $arg1: $ty1) {
            if let Some(ref callback) = self.$fn_name {
                callback($arg1);
            }
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty, $arg2:ident, $ty2:ty) => {
        pub fn $fn_name(&self, $arg1: $ty1, $arg2: $ty2) {
            if let Some(ref callback) = self.$fn_name {
                callback($arg1, $arg2);
            }
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty, $arg2:ident, $ty2:ty, $arg3:ident, $ty3:ty) => {
        pub fn $fn_name(&self, $arg1: $ty1, $arg2: $ty2, $arg3: $ty3) {
            if let Some(ref callback) = self.$fn_name {
                callback($arg1, $arg2, $arg3);
            }
        }
    };
}

macro_rules! callback_setter {
    ($fn_name:ident) => {
        pub fn $fn_name<F: Fn() + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $ty1:ty) => {
        pub fn $fn_name<F: Fn($ty1) + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $ty1:ty, $ty2:ty) => {
        pub fn $fn_name<F: Fn($ty1, $ty2) + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $ty1:ty, $ty2:ty, $ty3:ty) => {
        pub fn $fn_name<F: Fn($ty1, $ty2, $ty3) + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
}

macro_rules! callback_call {
    ($fn_name:ident) => {
        pub fn $fn_name(&self) {
            self.callbacks.$fn_name();
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty) => {
        pub fn $fn_name(&self, $arg1: $ty1) {
            self.callbacks.$fn_name($arg1);
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty, $arg2:ident, $ty2:ty) => {
        pub fn $fn_name(&self, $arg1: $ty1, $arg2: $ty2) {
            self.callbacks.$fn_name($arg1, $arg2);
        }
    };
    ($fn_name:ident, $arg1:ident, $ty1:ty, $arg2:ident, $ty2:ty, $arg3:ident, $ty3:ty) => {
        pub fn $fn_name(&self, $arg1: $ty1, $arg2: $ty2, $arg3: $ty3) {
            self.callbacks.$fn_name($arg1, $arg2, $arg3);
        }
    };
}

macro_rules! string_getter {
    ($(#[$attr:meta])* => ($method:ident, $ffi:ident)) => {
        pub fn $method(&self) -> String {
            let property = unsafe { ffi::$ffi(self.ptr) };
            let property: &CStr = unsafe { CStr::from_ptr(property) };
            property.to_str().unwrap().to_owned()
        }
    };
}
