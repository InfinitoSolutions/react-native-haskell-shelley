extern crate cardano_serialization_lib;
extern crate wasm_bindgen;

mod js_result;
mod panic;
mod ptr;
mod ptr_impl;
mod utils;
// declare other modules here
// mod transaction;
//pub use ibl::*;

pub use ptr::*;

#[cfg(target_os = "android")]
extern crate jni;
#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use self::android::*;


#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use self::ios::*;


#[cfg(test)]
mod ios;
mod tests {
    use super::*;
    use std::str;
    //use std::print;
    use std::ffi::CString;
    use std::{ffi, slice, ptr};
    use cardano::bip::bip39::{self, Mnemonics, MnemonicString, dictionary};
    use cardano::hdwallet::{self, XPrv, XPRV_SIZE};
    use std::io::{self, Write};
    use std::os::raw::c_char;
    #[test]
    fn test_add() {
      
     
      
      /* let password      = unsafe {ffi::CStr::from_ptr(p.as_ptr() as *const i8)};
      let password_str  = password.to_str().unwrap();
      let password      = password_str.as_bytes(); */
      //let seed = bip39::Seed::from_mnemonic_string(&mnemonics, &password);
      //let xprv = XPrv::generate_from_bip39(&seed);

    
      
    //println!("mnemonics_str {:?}",ffi::CString::new(xprv.to_string()).unwrap().into_raw());
    //let mnemonics     = MnemonicString::new(&dictionary::ENGLISH, mnemonics_str.to_string()).unwrap();

      //let seed = bip39::Seed::from_mnemonic_string(&mnemonics, &password);

      //println!("truchq {:?}", password);
       let rootkey = ios::ibl::create_rootkey("observe sadness bulb kid embark more goddess onion unknown acid vessel brief".as_ptr() as *const c_char, "infinitowallet".as_ptr() as  *const c_char );
       unsafe {
        
        println!("{:?}", CString::from_raw(rootkey));
        
      }
        
        //assert_eq!("mnemonics_int", "5");
    }
}