macro_rules! ffi_callback {
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty) => {
        extern "C" fn $fn_name(target: $target_type, _: *mut c_void) {
            INSTANCES
                .lock()
                .unwrap()
                .get(&(target as usize))
                .unwrap()
                .$fn_name();
        }
    };
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $arg1_ty1:ty) => {
        unsafe extern "C" fn $fn_name(target: $target_type, _: *mut c_void, arg1: $arg1_ty1) {
            INSTANCES
                .lock()
                .unwrap()
                .get(&(target as usize))
                .unwrap()
                .$fn_name(arg1.into());
        }
    };
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $arg1_ty:ty, $arg2_ty:ty) => {
        unsafe extern "C" fn $fn_name(
            target: $target_type,
            _: *mut c_void,
            arg1: $arg1_ty,
            arg2: $arg2_ty,
        ) {
            INSTANCES
                .lock()
                .unwrap()
                .get(&(target as usize))
                .unwrap()
                .$fn_name(arg1.into(), arg2.into());
        }
    };
    ($fn_name:ident, $target_type:ty, $target_rust_type:ty, $arg1_ty:ty, $arg2_ty:ty, $arg3_ty:ty) => {
        unsafe extern "C" fn $fn_name(
            target: $target_type,
            _: *mut c_void,
            arg1: $arg1_ty,
            arg2: $arg2_ty,
            arg3: $arg3_ty,
        ) {
            INSTANCES
                .lock()
                .unwrap()
                .get(&(target as usize))
                .unwrap()
                .$fn_name(arg1.into(), arg2.into(), arg3.into());
        }
    };
}

macro_rules! ffi_callback_with_return_user_data {
    ($fn_name:ident, $target_type:ty, $return_type:ty) => {
        unsafe extern "C" fn $fn_name(_: $target_type, user_data: *mut c_void) -> $return_type {
            let result: OtcBool = INSTANCES
                .lock()
                .unwrap()
                .get(&(user_data as usize))
                .unwrap()
                .$fn_name()
                .into();
            result.0
        }
    };
    ($fn_name:ident, $target_type:ty, $arg1_type:ty, $return_type:ty) => {
        unsafe extern "C" fn $fn_name(
            _: $target_type,
            user_data: *mut c_void,
            arg1: $arg1_type,
        ) -> $return_type {
            let result: OtcBool = INSTANCES
                .lock()
                .unwrap()
                .get(&(user_data as usize))
                .unwrap()
                .$fn_name(arg1)
                .into();
            result.0
        }
    };
}

macro_rules! callback {
    ($fn_name:ident, $target:ty) => {
        pub fn $fn_name(&self, target: $target) {
            if let Some(ref callback) = self.$fn_name {
                callback(target);
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty) => {
        pub fn $fn_name(&self, target: $target, arg1: $ty1) {
            if let Some(ref callback) = self.$fn_name {
                callback(target, arg1);
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty) => {
        pub fn $fn_name(&self, target: $target, arg1: $ty1, arg2: $ty2) {
            if let Some(ref callback) = self.$fn_name {
                callback(target, arg1, arg2);
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ty3:ty) => {
        pub fn $fn_name(&self, target: $target, arg1: $ty1, arg2: $ty2, arg3: $ty3) {
            if let Some(ref callback) = self.$fn_name {
                callback(target, arg1, arg2, arg3);
            }
        }
    };
}

macro_rules! callback_with_return {
    ($fn_name:ident, $target:ty, $ret:ty) => {
        pub fn $fn_name(&self, target: $target) -> $ret {
            if let Some(ref callback) = self.$fn_name {
                return callback(target);
            }
            Ok(())
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ret:ty) => {
        pub fn $fn_name(&self, target: $target, arg1: $ty1) -> $ret {
            if let Some(ref callback) = self.$fn_name {
                return callback(target, arg1);
            }
            Ok(())
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ret:ty) => {
        pub fn $fn_name(&self, target: $target, arg1: $ty1, arg2: $ty2) -> $ret {
            if let Some(ref callback) = self.$fn_name {
                return callback(target, arg1, arg2);
            }
            Ok(())
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ty3:ty, $ret:ty) => {
        pub fn $fn_name(&self, target: $target, arg1: $ty1, arg2: $ty2, arg3: $ty3) -> $ret {
            if let Some(ref callback) = self.$fn_name {
                return callback(target, arg1, arg2, arg3);
            }
            Ok(())
        }
    };
}

macro_rules! callback_setter {
    ($fn_name:ident, $target:ty) => {
        pub fn $fn_name<F: Fn($target) + Send + Sync + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1) + Send + Sync + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1, $ty2) + 'static + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ty3:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1, $ty2, $ty3) + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ty3:ty, $t4:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1, $ty2, $ty3, $t4) + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
}

macro_rules! callback_setter_with_return {
    ($fn_name:ident, $target:ty, $ret:ty) => {
        pub fn $fn_name<F: Fn($target) -> $ret + Send + Sync + 'static>(self, callback: F) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ret:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1) -> $ret + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ret:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1, $ty2) -> $ret + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ty3:ty, $ret:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1, $ty2, $ty3) -> $ret + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
    ($fn_name:ident, $target:ty, $ty1:ty, $ty2:ty, $ty3:ty, $t4:ty, $ret:ty) => {
        pub fn $fn_name<F: Fn($target, $ty1, $ty2, $ty3, $t4) -> $ret + Send + Sync + 'static>(
            self,
            callback: F,
        ) -> Self {
            Self {
                $fn_name: Some(Box::new(callback)),
                ..self
            }
        }
    };
}

macro_rules! callback_call {
    ($fn_name:ident) => {
        fn $fn_name(&self) {
            self.callbacks.lock().unwrap().$fn_name(self);
        }
    };
    ($fn_name:ident, $ty1:ty) => {
        fn $fn_name(&self, arg1: $ty1) {
            self.callbacks.lock().unwrap().$fn_name(self, arg1.into());
        }
    };
    ($fn_name:ident, $ty1:ty, $ty2:ty) => {
        fn $fn_name(&self, arg1: $ty1, arg2: $ty2) {
            self.callbacks
                .lock()
                .unwrap()
                .$fn_name(self, arg1.into(), arg2.into());
        }
    };
    ($fn_name:ident, $ty1:ty, $ty2:ty, $ty3:ty) => {
        fn $fn_name(&self, arg1: $ty1, arg2: $ty2, arg3: $ty3) {
            self.callbacks
                .lock()
                .unwrap()
                .$fn_name(self, arg1.into(), arg2.into(), arg3.into());
        }
    };
}

macro_rules! callback_call_with_return {
    ($fn_name:ident, $ret:ty) => {
        fn $fn_name(&self) -> $ret {
            self.callbacks.lock().unwrap().$fn_name(self)
        }
    };
    ($fn_name:ident, $ty1:ty, $ret:ty) => {
        fn $fn_name(&self, arg1: $ty1) -> $ret {
            self.callbacks.lock().unwrap().$fn_name(self, arg1.into())
        }
    };
    ($fn_name:ident, $ty1:ty, $ty2:ty, $ret:ty) => {
        fn $fn_name(&self, arg1: $ty1, arg2: $ty2) -> $ret {
            self.callbacks
                .lock()
                .unwrap()
                .$fn_name(self, arg1.into(), arg2.into())
        }
    };
    ($fn_name:ident, $ty1:ty, $ty2:ty, $ty3:ty, $ret:ty) => {
        fn $fn_name(&self, arg1: $ty1, arg2: $ty2, arg3: $ty3) -> $ret {
            self.callbacks
                .lock()
                .unwrap()
                .$fn_name(self, arg1.into(), arg2.into(), arg3.into())
        }
    };
}

macro_rules! string_getter {
    ($(#[$attr:meta])* => ($method:ident, $ffi:ident)) => {
        pub fn $method(&self) -> String {
            let property = unsafe { ffi::$ffi(self.ptr.load(Ordering::Relaxed) as *const _) };
            let property: &CStr = unsafe { CStr::from_ptr(property) };
            property.to_str().unwrap().to_owned()
        }
    };
}
