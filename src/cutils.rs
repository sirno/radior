use std::ffi::CStr;

macro_rules! cstr {
    ($s:expr) => {{
        std::ffi::CString::new($s)
            .expect("CString::new failed.")
            .as_ptr()
    }};
}

pub unsafe fn readcstr(s: *const ::std::os::raw::c_char) -> String {
    if s.is_null() {
        return "".to_string();
    }
    return CStr::from_ptr(s).to_string_lossy().into_owned();
}

pub unsafe fn readcstrf(s: *const ::std::os::raw::c_char) -> f32 {
    if s.is_null() {
        return 0.0;
    }
    return CStr::from_ptr(s)
        .to_string_lossy()
        .into_owned()
        .parse()
        .unwrap();
}
