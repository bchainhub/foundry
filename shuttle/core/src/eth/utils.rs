use corebc_core::utils::{
    rlp,
    rlp::{Encodable, RlpStream},
};
// use foxar_evm::utils::{h176_to_b176, h256_to_u256_be, u256_to_ru256};
// use revm::primitives::{B160, B176, U256 as rU256};

pub fn enveloped<T: Encodable>(id: u8, v: &T, s: &mut RlpStream) {
    let encoded = rlp::encode(v);
    let mut out = vec![0; 1 + encoded.len()];
    out[0] = id;
    out[1..].copy_from_slice(&encoded);
    out.rlp_append(s)
}

// pub fn to_access_list(list: Vec<AccessListItem>) -> Vec<(Address, Vec<U256>)> {
//     list.into_iter()
//         .map(|item| (item.address, item.storage_keys.into_iter().map(h256_to_u256_be).collect()))
//         .collect()
// }

// pub fn to_revm_access_list(list: Vec<AccessListItem>) -> Vec<(B176, Vec<rU256>)> {
//     list.into_iter()
//         .map(|item| {
//             (
//                 h176_to_b176(item.address),
//                 item.storage_keys.into_iter().map(h256_to_u256_be).map(u256_to_ru256).collect(),
//             )
//         })
//         .collect()
// }
