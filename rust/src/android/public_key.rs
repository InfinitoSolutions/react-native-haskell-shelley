use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::ToString;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jlong, jobject};
use jni::JNIEnv;
use cardano_serialization_lib::crypto::{PublicKey};
use std::convert::TryFrom;


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnhaskellshelley_Native_publicKeyHash(
  env: JNIEnv, _: JObject, public_key: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let public_key = public_key.rptr(&env)?;
    public_key
      .typed_ref::<PublicKey>()
      .map(|public_key| public_key.hash())
      .and_then(|keyhash| keyhash.rptr().jptr(&env))
  })
  .jresult(&env)
}
