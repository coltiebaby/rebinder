#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// import it with use crate::remixer;
pub mod remixer {
    // macro just gets the content of the file and adds it to this one.
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use super::remixer;

    #[test]
    pub fn test_remixer_multi_two() {
        let value = unsafe { remixer::multi_two(2) };
        assert_eq!(value, 4);
    }

    #[test]
    pub fn test_remixer_get_words() {
        let mut buf: *mut std::ffi::c_char = std::ptr::null_mut();

        let option: Vec<u8> = remixer::WORD_FOX.to_vec();
        let option = unsafe { std::ffi::CString::from_vec_with_nul(option).unwrap() };

        unsafe {
            remixer::get_words(option.as_ptr(), &mut buf);
        }

        let value = unsafe { std::ffi::CStr::from_ptr(buf).to_str().unwrap_or_default() };

        assert_eq!(
            value,
            "The quick brown fox jumps over the lazy dog.".to_string()
        );
    }
}
