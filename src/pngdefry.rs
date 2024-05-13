use std::path::Path;
use crate::error::PngdefryError;

include!("../bindings/bindings.rs");

pub fn convert<T>(file_path: T, output_path: T) -> Result<(), PngdefryError>
    where T: AsRef<str>
{
    let mut error_mut: [std::os::raw::c_char; 1024] = [0; 1024];
    let file_path = std::ffi::CString::new(file_path.as_ref()).map_err(|e| PngdefryError::Msg(e.to_string()))?;
    let output_path = std::ffi::CString::new(output_path.as_ref()).map_err(|e| PngdefryError::Msg(e.to_string()))?;
    unsafe {
        restore_png(file_path.as_ptr(), output_path.as_ptr(), error_mut.as_mut_ptr());
        let error_msg = std::ffi::CStr::from_ptr(error_mut.as_ptr())
            .to_string_lossy()
            .to_string();
        if error_msg.is_empty() {
            return Ok(());
        }
        Err(PngdefryError::Msg(error_msg))
    }
}

pub fn iphone_png<T: AsRef<Path> + ?Sized>(file_path: &T) -> Result<bool, PngdefryError> {
    let path = file_path.as_ref().to_string_lossy();
    let file_path = std::ffi::CString::new(path.to_string()).map_err(|e| PngdefryError::Msg(e.to_string()))?;
    unsafe {
        let mut error_mut: [std::os::raw::c_char; 1024] = [0; 1024];
        let result = is_iphone_png(file_path.as_ptr(), error_mut.as_mut_ptr());
        let error_msg = std::ffi::CStr::from_ptr(error_mut.as_ptr())
            .to_string_lossy()
            .to_string();
        if result == 1 {
            return Ok(true);
        } else if error_msg.is_empty() {
            return Ok(false);
        }
        Err(PngdefryError::Msg(error_msg))
    }
}