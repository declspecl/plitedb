pub mod engine;
pub mod error;
pub mod query;

#[cfg(feature = "ffi")]
const VERSION: &[u8; 6] = b"0.1.0\0";

#[cfg(feature = "ffi")]
#[no_mangle]
extern "C" fn plitedb_version() -> *const libc::c_uchar {
    return VERSION.as_ptr();
}

#[cfg(feature = "ffi")]
#[no_mangle]
extern "C" fn convert_error() -> libc::intptr_t {
    use error::{IntoFfiErrorCode, PliteDbError};

    let err = PliteDbError::LexerError(query::lexer::error::LexerError::UnexpectedCharacter('a'));

    return err.to_error_code();
}

