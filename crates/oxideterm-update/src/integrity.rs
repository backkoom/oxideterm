// Copyright (C) 2026 AnalyseDeCircuit
// SPDX-License-Identifier: GPL-3.0-only

use base64::Engine as _;
use minisign_verify::{PublicKey, Signature};

use crate::NativeUpdateError;

// 本分支自己的 minisign 公钥（keyid B013FA9AD0649563）。
// 上游那个（36E19D6992B57EBB）是作者的公钥，我们没有对应私钥，签不出能过验的包。
// 换公钥后，老版本验不了新包——必须手动装一次带这个公钥的版本。
pub const OXIDETERM_UPDATER_PUBKEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEIwMTNGQTlBRDA2NDk1NjMKUldSamxXVFFtdm9Uc0xtNWVROWNDMkxDZHNtNmIwemxqenNFZG8rdEhWM3pNNE14cThSc3JQalMK";

pub fn verify_minisign_signature(
    data: &[u8],
    release_signature: &str,
) -> Result<(), NativeUpdateError> {
    let pub_key_decoded = base64_to_string(OXIDETERM_UPDATER_PUBKEY)?;
    let public_key = PublicKey::decode(&pub_key_decoded).map_err(|error| {
        NativeUpdateError::Integrity(format!("decode public key failed: {error}"))
    })?;
    let signature_decoded = base64_to_string(release_signature)?;
    let signature = Signature::decode(&signature_decoded).map_err(|error| {
        NativeUpdateError::Integrity(format!("decode release signature failed: {error}"))
    })?;
    public_key.verify(data, &signature, true).map_err(|error| {
        NativeUpdateError::Integrity(format!("signature verification failed: {error}"))
    })?;
    Ok(())
}

fn base64_to_string(value: &str) -> Result<String, NativeUpdateError> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|error| NativeUpdateError::Integrity(format!("base64 decode failed: {error}")))?;
    std::str::from_utf8(&decoded)
        .map(str::to_string)
        .map_err(|_| NativeUpdateError::Integrity("invalid utf8 in signature".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 用本分支密钥对（keyid B013FA9AD0649563）签的固定向量。
    /// 换公钥时必须同步换这组向量，否则线上更新会全部验签失败——这个测试就是拦这个的。
    const SELFTEST_DATA: &[u8] = b"OxideTerm updater key self-test";
    const SELFTEST_SIGNATURE_B64: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IHNlbGZ0ZXN0ClJVUmpsV1RRbXZvVHNGQnk4QjAvZE52dVdvRUtLMmRkcUNwTnR3R3FQSXpodmpIbkJjdUljcXROSWs3WlM3R3A5UnJQdU9jUUhKQUo2SEFsMEVQMVJpaG9La1NrNGQ0MVp3UT0KdHJ1c3RlZCBjb21tZW50OiBzZWxmdGVzdApUekpJUFRWQmlPOENRUUVFMW1ieS9ibzJ3QlFSR3pJVys1VGlzV1YybFVxaVFFUDlPaVREZ2lqanI2a1pmUU4reFVIc3hFSXN3bUp3TnIweDNUUnlEdz09Cg==";

    #[test]
    fn public_key_pairs_with_the_release_signing_key() {
        verify_minisign_signature(SELFTEST_DATA, SELFTEST_SIGNATURE_B64).expect(
            "公钥与签名不匹配：OXIDETERM_UPDATER_PUBKEY 必须和发布用的私钥成对，\
             否则所有自动更新都会在验签这步失败",
        );
    }

    #[test]
    fn rejects_a_tampered_payload() {
        assert!(
            verify_minisign_signature(b"tampered payload", SELFTEST_SIGNATURE_B64).is_err(),
            "被改过的包必须验签失败"
        );
    }
}
