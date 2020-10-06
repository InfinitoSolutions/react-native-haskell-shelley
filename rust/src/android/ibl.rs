
use cardano::hdwallet::{self, XPrv, XPRV_SIZE};
use std::{ffi, slice, ptr};
use std::os::raw::{c_char};
use cardano::bip::bip39::{self, Mnemonics, MnemonicString, dictionary};
use std::println;

extern crate jni;
  use super::*;
  use self::jni::JNIEnv;
  use self::jni::objects::{JClass, JString};
  use self::jni::sys::{jint, jstring };


#[no_mangle]
fn create_rootkey( mnemonics: *const c_char
                 , password:  *const c_char )
-> *mut c_char
{
    let mnemonics     = unsafe {ffi::CStr::from_ptr(mnemonics)};
    let mnemonics_str = mnemonics.to_str().unwrap();
    let mnemonics     = MnemonicString::new(&dictionary::ENGLISH, mnemonics_str.to_string()).unwrap();

    let password      = unsafe {ffi::CStr::from_ptr(password)};
    let password_str  = password.to_str().unwrap();
    let password      = password_str.as_bytes();
    let seed = bip39::Seed::from_mnemonic_string(&mnemonics, &password);
    let xprv = XPrv::generate_from_bip39(&seed);

    ffi::CString::new(xprv.to_string()).unwrap().into_raw()
}
#[allow(non_snake_case)]
#[no_mangle]
    pub unsafe extern "C" fn Java_io_emurgo_rnhaskellshelley_Native_createRootKey(
    env: JNIEnv, _: JClass, mnemonics: JString, password: JString
  ) -> jstring {
      let rootkey = create_rootkey(env.get_string(mnemonics).expect("invalid pattern string").as_ptr(), env.get_string(password).expect("invalid pattern string").as_ptr());
      let rootkey_ptr = ffi::CString::from_raw(rootkey);
      let output = env.new_string(rootkey_ptr.to_str().unwrap()).expect("Couldn't create java string!");
      output.into_inner()
  }