use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
use crate::panic::{handle_exception, handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use cardano_serialization_lib::crypto::Bip32PublicKey;
use std::convert::TryFrom;

#[no_mangle]
pub unsafe extern "C" fn bip_32_public_key_to_raw_key(
  bip_32_public_key: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    bip_32_public_key
      .typed_ref::<Bip32PublicKey>()
      .map(|bip_32_public_key| bip_32_public_key.to_raw_key())
  })
  .map(|public_key| public_key.rptr())
  .response(result, error)
}