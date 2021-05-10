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
        extern "C" fn $fn_name(
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
        extern "C" fn $fn_name(
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
        extern "C" fn $fn_name(
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
