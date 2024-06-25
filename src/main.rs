use std::str::FromStr;

use cdk::{
    nuts::{BlindSignature, BlindSignatureDleq, Id, PublicKey, SecretKey},
    Amount,
};

fn main() {
    let mint_pubkey =
        PublicKey::from_hex("03fd64b1fc294f6643ead5f44175e4620666ccb91497cfa508096b557d92dc7a74")
            .unwrap();

    let b =
        PublicKey::from_hex("03035272e854c1effc5d9855e909a4aeb1a901273af003e69b5b499acdabd4a2ca")
            .unwrap();

    let c =
        PublicKey::from_hex("031e820bd09398890836f3f54ae1f3b6d4b82e260bc2a104bad6e9a0b05538c7ba")
            .unwrap();

    let keyset_id = Id::from_str("00178484f5e74df9").unwrap();

    let dleq = BlindSignatureDleq {
        e: SecretKey::from_hex("4492de5a782433fe89a91aeff7115547336a50700230e1371e65fd9a884b161d")
            .unwrap(),
        s: SecretKey::from_hex("59fee37c55a0c9fbe62b656acd1ba64b925b9862db74e27bf3de33f2d0d72074")
            .unwrap(),
    };

    let blind_sig = BlindSignature {
        amount: Amount::from(1),
        keyset_id,
        c,
        dleq: Some(dleq),
    };

    let verified = blind_sig.verify_dleq(mint_pubkey, b).unwrap();

    println!("veirfied: {:?}", verified);
}
