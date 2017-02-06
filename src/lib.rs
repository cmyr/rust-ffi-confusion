extern crate libc;

use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use libc::size_t;


#[repr(C)]
pub struct HasVec {
    members: Vec<*mut c_char>,
    len: size_t,
}

impl HasVec {
    fn new<'a>(items: Vec<&'a str>) -> HasVec {
        HasVec {
            members: items.iter()
                .map(|&i| CString::new(i).unwrap().into_raw())
                .collect::<Vec<_>>(), 
            len: items.len() as size_t,
        }
    }
}
 
#[no_mangle]
pub extern fn test_vec() -> *mut HasVec {
    let strings =  vec!["these", "are", "some", "strings"];
//.iter()
        //.map(|s| s.to_string())
        //.collect::<Vec<String>>();

    let has_vec = HasVec::new(strings);
	Box::into_raw(Box::new(has_vec))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_test_vec() {
        let x = test_vec();
        assert!(true);
        
    }
}
//fn main() {
    //let test  = test_vec();
    //for t in test {
        //println!("{}", t);
    //}
    ////let input = 4;
    ////let output = unsafe { double_input(input) };
    ////println!("{} * 2 = {}", input, output);
//}
