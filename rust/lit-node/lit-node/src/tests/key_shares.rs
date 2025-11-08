// Valid key shares taken from a test run of our BLS implementation.
pub(crate) const TEST_BLS_KEY_SHARE: &str = r#"{
    "hex_private_share": "2afa8872bb7a544fc2da7277d4b5d43f0a2d43a66afe27564dfed48211833503",
    "hex_public_key": "83fc126ef56547bb28734a4a5393a873b8c22a9ba2d507036285a506567b2b3a376fc524cd589bb018613e24c51ebbae",
    "curve_type": "BLS",
    "peer_id": "89576ca455130b4c31c8a689fc9e04d0cd4ccfad25f5ad3fed5ab98f1d7a0238",
    "threshold": 3,
    "total_shares": 3,
    "txn_prefix": "EPOCH_DKG_1_4.BLS12381G1.STANDARD_key_0",
    "realm_id": 1,
    "peers": [  "89576ca455130b4c31c8a689fc9e04d0cd4ccfad25f5ad3fed5ab98f1d7a0238",
                "2dc1123a65d4ac9d5ef65369bdee40a58e2258f08e8be43abaad4ca7ca3c6007",
                "70f3eca8de2ce91465de6de55c1bbeeb0f755cac1716031c3b5a6d745f33735f"]
}"#;

pub(crate) const TEST_BLS_KEY_SHARE_COMMITMENT: &str = r#"{
    "dkg_id":"EPOCH_DKG_1_4.BLS12381G1.STANDARD_key_0",
    "commitments": ["83fc126ef56547bb28734a4a5393a873b8c22a9ba2d507036285a506567b2b3a376fc524cd589bb018613e24c51ebbae",
                    "87a3326851f776fd66492c0d567d2dd7d8c4f3c174cd297645e79d9dc61643bbb5971cdaf2e27067f3f2aa48fd4bda54",
                    "b3b4ff5d2e50349cbfe99ddf42b6fef9f92f29d8a135275082905b2eb619afb2770c982512339575eac8f976df294baa"
]
}"#;

// A valid key share taken from a test run of our Cait-Sith implementation.
pub(crate) const TEST_ECDSA_KEY_SHARE: &str = r#"{
    "hex_private_share": "89c78de77ab5d3452ba189f619e39fd299e34b939257ad5a5cda02c70fca0f44",
    "hex_public_key": "0268c27a16f03d19949f0a64d58c71ea32049b754888211cc25827f5449d26bf74",
    "curve_type": "K256",
    "peer_id": "89576ca455130b4c31c8a689fc9e04d0cd4ccfad25f5ad3fed5ab98f1d7a0238",
    "threshold": 3,
    "total_shares": 3,
    "txn_prefix": "EPOCH_DKG_1_4.Secp256k1.STANDARD_key_0",
    "realm_id": 1,
    "peers": [  "89576ca455130b4c31c8a689fc9e04d0cd4ccfad25f5ad3fed5ab98f1d7a0238",
                "2dc1123a65d4ac9d5ef65369bdee40a58e2258f08e8be43abaad4ca7ca3c6007",
                "70f3eca8de2ce91465de6de55c1bbeeb0f755cac1716031c3b5a6d745f33735f"]
}"#;

pub(crate) const TEST_ECDSA_KEY_SHARE_COMMITMENT: &str = r#"{
    "dkg_id": "EPOCH_DKG_1_4.Secp256k1.STANDARD_key_0",
    "commitments": ["0268c27a16f03d19949f0a64d58c71ea32049b754888211cc25827f5449d26bf74",
                    "033c24b1962c1322e8911e298fd4703fec217da3b19ade9725be21951f8085be7d",
                    "0263258bdb87c33c3d2637c7092e8ab3680596dc8bf6e2f5b81ed53cd9931af4f2"]
}"#;

// Test BLS public key and corresponding private key shares with 2/3 threshold
pub(crate) const TEST_BLS_PUB_KEY: &str = "b5ab5cece2d13ecb8363128c8076ad3708d274ba01cc11331aedc6ed5d8b2a477a21f333312918039f6ec146dc97e42c";
pub(crate) const TEST_BLS_PRI_KEY_SHARE_1: &str =
    "18ebaba9f88cd9820ec0b63603539479ad18245da7831007cfaa92a1b390d41a";
pub(crate) const TEST_BLS_PRI_KEY_SHARE_2: &str =
    "016d66abcb7d96b06351f92091dd906f5ebe849f2b428023285eb8c10699a66c";
#[allow(dead_code)]
pub(crate) const TEST_BLS_PRI_KEY_SHARE_3: &str =
    "5ddcc900c80bd126eb1d14132a09646a642288e3af004c3d8112dedf59a278bf";
// Blinders are also scalars, independent from the other keys
pub(crate) const TEST_BLS_BLINDER: &str =
    "52dc3bed022962b5584a760f4b24b44161dc72642965656d60de64965c71a51e";

// Test ECDSA public key and corresponding private key shares with 2/3 threshold
pub(crate) const TEST_ECDSA_PUB_KEY: &str =
    "0215b81f6b17e4a3f09ed9b618f0f5ae96f31770d64a2c3d7522d58a837f6b50f7";
pub(crate) const TEST_ECDSA_PRI_KEY_SHARE_1: &str =
    "259d4c57d02cc2b688ab821c6f6d02f8fc8f37cbde54a2fe15dff89bb0396fc2";
pub(crate) const TEST_ECDSA_PRI_KEY_SHARE_2: &str =
    "9bb3eea41e12043f1092476f8f59a05948cedd961d28b113333006c003979620";
#[allow(dead_code)]
pub(crate) const TEST_ECDSA_PRI_KEY_SHARE_3: &str =
    "11ca90f06bf745c798790cc2af463dbada5fa679acb41eec90adb65786bf7b3d";
// Blinders are also scalars, independent from the other keys
pub(crate) const TEST_ECDSA_BLINDER: &str =
    "0B4A40A99C64439F8E8742FD29582D3ED5C18689147293B7332CF6E2C5F35F16";

// Key shares used in `test_untar_old_backup`
pub(crate) const TEST_OLD_BLS_KEY_SHARE: &str = r#"{
    "hex_private_share": "14b887a1414cd47382b11d3478a0b4f6d7f9890e5c9be0c334cadf0e392a1087",
    "hex_public_key": "a5b236b172c1b2cfb5f0942b73b01bbd9af660771069f4d1b924f3280810016ebff10167fe80d06e2ced53558d2a8c2f",
    "curve_type": "BLS",
    "peer_id": "89576ca455130b4c31c8a689fc9e04d0cd4ccfad25f5ad3fed5ab98f1d7a0238",
    "threshold": 2,
    "total_shares": 3,
    "txn_prefix": "test txn_prefix",
    "realm_id": 1,
    "peers": []
}"#;

// Key shares used in `test_untar_old_backup`
pub(crate) const TEST_OLD_K256_KEY_SHARE: &str = r#"{
    "hex_private_share": "62B2794C091C0F6D8871CC3F77506B07C6CDC872FE98063A5D83C0F9E8599B39",
    "hex_public_key": "03522EC015CA40C781EB60FA9CB98C555390AC0634C3B0366BF682B357E0857D3C",
    "curve_type": "K256",
    "peer_id": "89576ca455130b4c31c8a689fc9e04d0cd4ccfad25f5ad3fed5ab98f1d7a0238",
    "threshold": 2,
    "total_shares": 3,
    "txn_prefix": "test txn_prefix",
    "realm_id": 1,
    "peers": []
}"#;

pub(crate) fn hex_to_bls_dec_key_share(hex: &str, share_index: u8) -> Vec<u8> {
    let mut bytes = hex::decode(hex).unwrap();
    let mut share = vec![share_index];
    bytes.reverse();
    share.extend(bytes);
    share
}

pub(crate) fn hex_to_k256_dec_key_share(hex: &str, share_index: u8) -> Vec<u8> {
    let bytes = hex::decode(hex).unwrap();
    let mut share = vec![share_index];
    share.extend(bytes);
    share
}
