const VERSION: &[u8; 6] = b"0.1.0\0";

#[no_mangle]
extern "C" fn plitedb_version() -> *const libc::c_uchar {
    return VERSION.as_ptr();
}
