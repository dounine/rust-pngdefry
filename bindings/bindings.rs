/* automatically generated by rust-bindgen 0.69.4 */

extern "C" {
    pub fn restore_png(
        filePath: *const ::std::os::raw::c_char,
        outputPath: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn is_iphone_png(
        filePath: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
