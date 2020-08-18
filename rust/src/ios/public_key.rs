use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
use crate::panic::{handle_exception, handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use cardano_serialization_lib::crypto::PublicKey;
use std::convert::TryFrom;

#[no_mangle]
pub unsafe extern "C" fn public_key_hash(
  public_key: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    public_key
      .typed_ref::<PublicKey>()
      .map(|public_key| public_key.hash())
  })
    .map(|keyhash| keyhash.rptr())
    .response(result, error)
}