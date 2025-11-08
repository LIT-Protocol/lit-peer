pub use pkpnft_metadata::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod pkpnft_metadata {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/PKPNFTMetadata.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_resolver"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_env"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("enum ContractResolver.Env"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bytesToHex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bytesToHex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("buffer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contractResolver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contractResolver"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ContractResolver",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("env"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("env"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeProfileForPkp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeProfileForPkp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeUrlForPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeUrlForPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProfileForPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setProfileForPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imgUrl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setUrlForPKP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUrlForPKP"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("url"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ethAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PKPNFTMETADATA_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x1C}8\x03\x80a\x1C}\x839\x81\x01`@\x81\x90R`,\x91`vV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U\x83\x92\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x02\x81\x11\x15`kW`k`\xBDV[\x02\x17\x90UPPP`\xD3V[`\0\x80`@\x83\x85\x03\x12\x15`\x88W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\x9EW`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x03\x81\x10`\xB2W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a\x1B\x9B\x80a\0\xE2`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80cE\x1D\x89\xFA\x14a\0}W\x80cP\xD1{^\x14a\0\xA6W\x80cQ\x9A!\x8E\x14a\0\xD1W\x80c\x85^\xEC\"\x14a\0\xE6W\x80c\x90\0\xFE\xE1\x14a\0\xF9W\x80c\x95\x04b\xEE\x14a\x01\x0CW\x80c\x9D\xCA\x002\x14a\x01\x1FW\x80c\xB6:vw\x14a\x01@W[`\0\x80\xFD[a\0\x90a\0\x8B6`\x04a\x0F\xB0V[a\x01SV[`@Qa\0\x9D\x91\x90a\x10\x10V[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xB9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x9DV[a\0\xE4a\0\xDF6`\x04a\x10CV[a\x03\x0CV[\0[a\0\xE4a\0\xF46`\x04a\x10\\V[a\x04VV[a\0\xE4a\x01\x076`\x04a\x10\\V[a\x05\x89V[a\0\x90a\x01\x1A6`\x04a\x10\xCEV[a\x06\xB7V[`\0Ta\x013\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\0\x9D\x91\x90a\x11JV[a\0\xE4a\x01N6`\x04a\x10CV[a\x06\xF3V[```\0\x82Q`\x02a\x01e\x91\x90a\x11nV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01|Wa\x01|a\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x01\xA6W` \x82\x01\x81\x806\x837\x01\x90P[P`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x90\x91P`\0[\x84Q\x81\x10\x15a\x02\xE2W\x81\x82Q\x86\x83\x81Q\x81\x10a\x01\xF2Wa\x01\xF2a\x11\x85V[\x01` \x01Qa\x02\x04\x91\x90`\xF8\x1Ca\x11\xB1V[\x81Q\x81\x10a\x02\x14Wa\x02\x14a\x11\x85V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x83a\x02/\x83`\x02a\x11nV[\x81Q\x81\x10a\x02?Wa\x02?a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81\x82Q\x86\x83\x81Q\x81\x10a\x02kWa\x02ka\x11\x85V[\x01` \x01Qa\x02}\x91\x90`\xF8\x1Ca\x11\xC5V[\x81Q\x81\x10a\x02\x8DWa\x02\x8Da\x11\x85V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x83a\x02\xA8\x83`\x02a\x11nV[a\x02\xB3\x90`\x01a\x11\xD9V[\x81Q\x81\x10a\x02\xC3Wa\x02\xC3a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x01\xD4V[P\x81`@Q` \x01a\x02\xF4\x91\x90a\x12\x08V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x03^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x82\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x03\xB1\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF2\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x84\x81R`\x01\x90\x91R\x91\x90\x91 \x90a\x04R\x90\x82a\x13vV[PPV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCC\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x04\xFB\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05<\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`\0\x82\x81R`\x01` R`@\x90 a\x05\x84\x82\x82a\x13vV[PPPV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFF\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x06.\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06o\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`\0\x82\x81R`\x02` R`@\x90 a\x05\x84\x82\x82a\x13vV[```\0a\x06\xC6\x85\x85\x85a\x080V[\x90P\x80`@Q` \x01a\x06\xD9\x91\x90a\x144V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP[\x93\x92PPPV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07i\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x07\x98\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xD9\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x84\x81R`\x02\x90\x91R\x91\x90\x91 \x90a\x04R\x90\x82a\x13vV[```\0`@Q\x80a\x04\x80\x01`@R\x80a\x04V\x81R` \x01a\x16\xD0a\x04V\x919\x90P`\0a\x08]\x85a\x01SV[\x90P`\0a\x08j\x85a\n\x84V[\x90P`\0a\x08w\x88a\n\xA0V[`\0\x89\x81R`\x01` R`@\x81 \x80T\x92\x93P\x90\x91a\x08\x95\x90a\x12\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xC1\x90a\x12\xEEV[\x80\x15a\t\x0EW\x80`\x1F\x10a\x08\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x0EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0`\x02`\0\x8B\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\t5\x90a\x12\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\ta\x90a\x12\xEEV[\x80\x15a\t\xAEW\x80`\x1F\x10a\t\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x81Q`\0\x14\x80\x15a\t\xC6WP\x80Q\x15\x15[\x15a\t\xF2W\x82`@Q` \x01a\t\xDC\x91\x90a\x14yV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa\nFV[\x81Q\x15\x80\x15\x90a\n\x01WP\x80Q\x15[\x15a\n\rWP\x84a\nFV[\x81Q\x15\x80\x15a\n\x1BWP\x80Q\x15[\x15a\nFW\x82`@Q` \x01a\n1\x91\x90a\x14yV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x85\x90P[a\nv\x82\x82\x87\x87\x87`@Q` \x01a\nb\x95\x94\x93\x92\x91\x90a\x14\xAAV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0B2V[\x9A\x99PPPPPPPPPPV[``a\n\x9A`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14a\x0C\x91V[\x92\x91PPV[```\0a\n\xAD\x83a\x0E,V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCCWa\n\xCCa\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\n\xF6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B\0WP\x93\x92PPPV[``\x81Q`\0\x03a\x0BQWPP`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[`\0`@Q\x80``\x01`@R\x80`@\x81R` \x01a\x1B&`@\x919\x90P`\0`\x03\x84Q`\x02a\x0B\x80\x91\x90a\x11\xD9V[a\x0B\x8A\x91\x90a\x11\xB1V[a\x0B\x95\x90`\x04a\x11nV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xACWa\x0B\xACa\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xD6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x01\x82\x01` \x82\x01\x85\x86Q\x87\x01` \x81\x01\x80Q`\0\x82R[\x82\x84\x10\x15a\x0CLW`\x03\x84\x01\x93P\x83Q`?\x81`\x12\x1C\x16\x87\x01Q\x86S`\x01\x86\x01\x95P`?\x81`\x0C\x1C\x16\x87\x01Q\x86S`\x01\x86\x01\x95P`?\x81`\x06\x1C\x16\x87\x01Q\x86S`\x01\x86\x01\x95P`?\x81\x16\x87\x01Q\x86SP`\x01\x85\x01\x94Pa\x0B\xF1V[\x90RPP\x85Q`\x03\x90\x06`\x01\x81\x14a\x0CkW`\x02\x81\x14a\x0C~Wa\x0C\x86V[`=`\x01\x83\x03S`=`\x02\x83\x03Sa\x0C\x86V[`=`\x01\x83\x03S[P\x91\x95\x94PPPPPV[```\0a\x0C\xA0\x83`\x02a\x11nV[a\x0C\xAB\x90`\x02a\x11\xD9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xC2Wa\x0C\xC2a\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0C\xECW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\r\x07Wa\r\x07a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\r6Wa\r6a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\rZ\x84`\x02a\x11nV[a\re\x90`\x01a\x11\xD9V[\x90P[`\x01\x81\x11\x15a\r\xDDWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\r\x99Wa\r\x99a\x11\x85V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\r\xAFWa\r\xAFa\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\r\xD6\x81a\x16\xB8V[\x90Pa\rhV[P\x83\x15a\x06\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x04\"V[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x0EkWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[i\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x10a\x0E\x95Wi\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x0E\xB3Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x0E\xCBWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x0E\xDFWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x0E\xF1W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\n\x9AW`\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x0F2Wa\x0F2a\x0F\x02V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x0F`Wa\x0F`a\x0F\x02V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x0FxW`\0\x80\xFD[\x83\x83` \x83\x017`\0` \x85\x83\x01\x01RP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\xA1W`\0\x80\xFD[a\x06\xEC\x83\x835` \x85\x01a\x0F\x18V[`\0` \x82\x84\x03\x12\x15a\x0F\xC2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xD8W`\0\x80\xFD[a\x0F\xE4\x84\x82\x85\x01a\x0F\x90V[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\x10\x07W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\xEFV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x10/\x81`@\x85\x01` \x87\x01a\x0F\xECV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10UW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10oW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x8CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x10\x9DW`\0\x80\xFD[a\x10\xAC\x85\x825` \x84\x01a\x0F\x18V[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xCBW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xE3W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\0W`\0\x80\xFD[a\x11\x0C\x86\x82\x87\x01a\x0F\x90V[\x92PP`@\x84\x015a\x11\x1D\x81a\x10\xB6V[\x80\x91PP\x92P\x92P\x92V[`\x03\x81\x10a\x11FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\n\x9A\x82\x84a\x11(V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\x9AWa\n\x9Aa\x11XV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x11\xC0Wa\x11\xC0a\x11\x9BV[P\x04\x90V[`\0\x82a\x11\xD4Wa\x11\xD4a\x11\x9BV[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\n\x9AWa\n\x9Aa\x11XV[`\0\x81Qa\x11\xFE\x81\x85` \x86\x01a\x0F\xECV[\x92\x90\x92\x01\x92\x91PPV[a\x06\x0F`\xF3\x1B\x81R`\0\x82Qa\x12%\x81`\x02\x85\x01` \x87\x01a\x0F\xECV[\x91\x90\x91\x01`\x02\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12DW`\0\x80\xFD[PQ\x91\x90PV[\x82\x81R`@\x81\x01a\x06\xEC` \x83\x01\x84a\x11(V[`\0` \x82\x84\x03\x12\x15a\x12qW`\0\x80\xFD[\x81Qa\x06\xEC\x81a\x10\xB6V[` \x80\x82R`L\x90\x82\x01R\x7FPKPHelper: only the Domain Walle`@\x82\x01R\x7Ft registry is allowed to mint do``\x82\x01Rkmain wallets`\xA0\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\x02W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\"WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x84W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x13OWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x13oW`\0\x81U`\x01\x01a\x13[V[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x8FWa\x13\x8Fa\x0F\x02V[a\x13\xA3\x81a\x13\x9D\x84Ta\x12\xEEV[\x84a\x13(V[` `\x1F\x82\x11`\x01\x81\x14a\x13\xD7W`\0\x83\x15a\x13\xBFWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x13oV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x14\x07W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x13\xE7V[P\x84\x82\x10\x15a\x14%W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7Fdata:application/json;base64,\0\0\0\x81R`\0\x82Qa\x14l\x81`\x1D\x85\x01` \x87\x01a\x0F\xECV[\x91\x90\x91\x01`\x1D\x01\x92\x91PPV[hLit PKP #`\xB8\x1B\x81R`\0\x82Qa\x14\x9D\x81`\t\x85\x01` \x87\x01a\x0F\xECV[\x91\x90\x91\x01`\t\x01\x92\x91PPV[h=\x9170\xB6\xB2\x91\x1D\x11`\xB9\x1B\x81R\x85Q`\0\x90a\x14\xCF\x81`\t\x85\x01` \x8B\x01a\x0F\xECV[\x7F\", \"description\": \"This NFT enti`\t\x91\x84\x01\x91\x82\x01R\x7Ftles the holder to use a Lit Pro`)\x82\x01R\x7Ftocol PKP, and to grant access t`I\x82\x01R\x7Fo other users and Lit Actions to`i\x82\x01R\x7F use this PKP\",\"image_data\": \"\0\0`\x89\x82\x01R\x86Qa\x15\xA4\x81`\xA7\x84\x01` \x8B\x01a\x0F\xECV[`\t\x81\x83\x01\x01\x91PP\x7F\",\"attributes\": [{\"trait_type\": `\x9E\x82\x01Rw\x11(:\xB164\xB1\x90%\xB2\xBC\x91\x16\x10\x11;0\xB6:\xB2\x91\x1D\x10\x11`A\x1B`\xBE\x82\x01Ra\x16\xACa\x16\x9Ca\x16\x96a\x16[a\x16Ua\x16\x10`\xD6\x87\x01\x8Ca\x11\xECV[\x7F\"}, {\"trait_type\": \"ETH Wallet A\x81Rr2292\xB9\xB9\x91\x16\x10\x11;0\xB6:\xB2\x91\x1D\x10\x11`i\x1B` \x82\x01R`3\x01\x90V[\x89a\x11\xECV[\x7F\"}, {\"trait_type\": \"Token ID\", \"\x81Rh;0\xB6:\xB2\x91\x1D\x10\x11`\xB9\x1B` \x82\x01R`)\x01\x90V[\x86a\x11\xECV[c\"}]}`\xE0\x1B\x81R`\x04\x01\x90V[\x98\x97PPPPPPPPV[`\0\x81a\x16\xC7Wa\x16\xC7a\x11XV[P`\0\x19\x01\x90V\xFE<svg xmlns='http://www.w3.org/2000/svg' width='1080' height='1080' fill='none' xmlns:v='https://vecta.io/nano'><path d='M363.076 392.227s-.977 18.524-36.874 78.947c-41.576 70.018-45.481 151.978-3.017 220.4 89.521 144.245 332.481 141.52 422.556.089 34.832-54.707 44.816-117.479 32.924-181.248 0 0-28.819-133.144-127.237-217.099 1.553 1.308 5.369 19.122 6.101 26.722 2.241 23.354.045 47.838-7.787 70.062-5.746 16.33-13.711 30.467-27.178 41.368 0-3.811-.954-10.635-.976-12.918-.644-46.508-18.659-89.582-48.011-125.743-25.647-31.552-60.812-53.089-97.84-68.932.931 3.191 2.662 16.419 2.906 19.033 1.908 21.958 2.263 52.713-.621 74.649s-7.832 33.878-14.554 54.441c-10.184 31.175-24.05 54.285-41.621 82.004-3.24 5.096-12.913 19.078-18.082 26.146 0 0-8.897-56.191-40.667-87.921h-.022z' fill='#000'/><path d='M562.5 27.28l410.279 236.874c13.923 8.039 22.5 22.895 22.5 38.971v473.75c0 16.076-8.577 30.932-22.5 38.971L562.5 1052.72c-13.923 8.04-31.077 8.04-45 0L107.221 815.846c-13.923-8.039-22.5-22.895-22.5-38.971v-473.75a45 45 0 0 1 22.5-38.971L517.5 27.28a45 45 0 0 1 45 0z' stroke='#000' stroke-width='24.75'/></svg>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\xA2dipfsX\"\x12 \x8F\x13\x99\xB0\x85\xBF\xFD\xE0ex\xF1\xDD\x04\xAD6!~\xEDG\xFB\xA0mu\xA3\x1F\\\xE9\xE6qQx\x82dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static PKPNFTMETADATA_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80cE\x1D\x89\xFA\x14a\0}W\x80cP\xD1{^\x14a\0\xA6W\x80cQ\x9A!\x8E\x14a\0\xD1W\x80c\x85^\xEC\"\x14a\0\xE6W\x80c\x90\0\xFE\xE1\x14a\0\xF9W\x80c\x95\x04b\xEE\x14a\x01\x0CW\x80c\x9D\xCA\x002\x14a\x01\x1FW\x80c\xB6:vw\x14a\x01@W[`\0\x80\xFD[a\0\x90a\0\x8B6`\x04a\x0F\xB0V[a\x01SV[`@Qa\0\x9D\x91\x90a\x10\x10V[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\xB9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x9DV[a\0\xE4a\0\xDF6`\x04a\x10CV[a\x03\x0CV[\0[a\0\xE4a\0\xF46`\x04a\x10\\V[a\x04VV[a\0\xE4a\x01\x076`\x04a\x10\\V[a\x05\x89V[a\0\x90a\x01\x1A6`\x04a\x10\xCEV[a\x06\xB7V[`\0Ta\x013\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\0\x9D\x91\x90a\x11JV[a\0\xE4a\x01N6`\x04a\x10CV[a\x06\xF3V[```\0\x82Q`\x02a\x01e\x91\x90a\x11nV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01|Wa\x01|a\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x01\xA6W` \x82\x01\x81\x806\x837\x01\x90P[P`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x90\x91P`\0[\x84Q\x81\x10\x15a\x02\xE2W\x81\x82Q\x86\x83\x81Q\x81\x10a\x01\xF2Wa\x01\xF2a\x11\x85V[\x01` \x01Qa\x02\x04\x91\x90`\xF8\x1Ca\x11\xB1V[\x81Q\x81\x10a\x02\x14Wa\x02\x14a\x11\x85V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x83a\x02/\x83`\x02a\x11nV[\x81Q\x81\x10a\x02?Wa\x02?a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81\x82Q\x86\x83\x81Q\x81\x10a\x02kWa\x02ka\x11\x85V[\x01` \x01Qa\x02}\x91\x90`\xF8\x1Ca\x11\xC5V[\x81Q\x81\x10a\x02\x8DWa\x02\x8Da\x11\x85V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x83a\x02\xA8\x83`\x02a\x11nV[a\x02\xB3\x90`\x01a\x11\xD9V[\x81Q\x81\x10a\x02\xC3Wa\x02\xC3a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x01\xD4V[P\x81`@Q` \x01a\x02\xF4\x91\x90a\x12\x08V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x03^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x82\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x03\xB1\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF2\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x84\x81R`\x01\x90\x91R\x91\x90\x91 \x90a\x04R\x90\x82a\x13vV[PPV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCC\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x04\xFB\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05<\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`\0\x82\x81R`\x01` R`@\x90 a\x05\x84\x82\x82a\x13vV[PPPV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFF\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x06.\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06o\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`\0\x82\x81R`\x02` R`@\x90 a\x05\x84\x82\x82a\x13vV[```\0a\x06\xC6\x85\x85\x85a\x080V[\x90P\x80`@Q` \x01a\x06\xD9\x91\x90a\x144V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP[\x93\x92PPPV[`\0T`@\x80Qc\tw\xA8\x07`\xE4\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x97z\x80p\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07i\x91\x90a\x122V[`\0T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x07\x98\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x12KV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xD9\x91\x90a\x12_V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\"\x90a\x12|V[`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x84\x81R`\x02\x90\x91R\x91\x90\x91 \x90a\x04R\x90\x82a\x13vV[```\0`@Q\x80a\x04\x80\x01`@R\x80a\x04V\x81R` \x01a\x16\xD0a\x04V\x919\x90P`\0a\x08]\x85a\x01SV[\x90P`\0a\x08j\x85a\n\x84V[\x90P`\0a\x08w\x88a\n\xA0V[`\0\x89\x81R`\x01` R`@\x81 \x80T\x92\x93P\x90\x91a\x08\x95\x90a\x12\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xC1\x90a\x12\xEEV[\x80\x15a\t\x0EW\x80`\x1F\x10a\x08\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x0EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P`\0`\x02`\0\x8B\x81R` \x01\x90\x81R` \x01`\0 \x80Ta\t5\x90a\x12\xEEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\ta\x90a\x12\xEEV[\x80\x15a\t\xAEW\x80`\x1F\x10a\t\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x81Q`\0\x14\x80\x15a\t\xC6WP\x80Q\x15\x15[\x15a\t\xF2W\x82`@Q` \x01a\t\xDC\x91\x90a\x14yV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91Pa\nFV[\x81Q\x15\x80\x15\x90a\n\x01WP\x80Q\x15[\x15a\n\rWP\x84a\nFV[\x81Q\x15\x80\x15a\n\x1BWP\x80Q\x15[\x15a\nFW\x82`@Q` \x01a\n1\x91\x90a\x14yV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x85\x90P[a\nv\x82\x82\x87\x87\x87`@Q` \x01a\nb\x95\x94\x93\x92\x91\x90a\x14\xAAV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0B2V[\x9A\x99PPPPPPPPPPV[``a\n\x9A`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14a\x0C\x91V[\x92\x91PPV[```\0a\n\xAD\x83a\x0E,V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCCWa\n\xCCa\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\n\xF6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B\0WP\x93\x92PPPV[``\x81Q`\0\x03a\x0BQWPP`@\x80Q` \x81\x01\x90\x91R`\0\x81R\x90V[`\0`@Q\x80``\x01`@R\x80`@\x81R` \x01a\x1B&`@\x919\x90P`\0`\x03\x84Q`\x02a\x0B\x80\x91\x90a\x11\xD9V[a\x0B\x8A\x91\x90a\x11\xB1V[a\x0B\x95\x90`\x04a\x11nV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xACWa\x0B\xACa\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xD6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x01\x82\x01` \x82\x01\x85\x86Q\x87\x01` \x81\x01\x80Q`\0\x82R[\x82\x84\x10\x15a\x0CLW`\x03\x84\x01\x93P\x83Q`?\x81`\x12\x1C\x16\x87\x01Q\x86S`\x01\x86\x01\x95P`?\x81`\x0C\x1C\x16\x87\x01Q\x86S`\x01\x86\x01\x95P`?\x81`\x06\x1C\x16\x87\x01Q\x86S`\x01\x86\x01\x95P`?\x81\x16\x87\x01Q\x86SP`\x01\x85\x01\x94Pa\x0B\xF1V[\x90RPP\x85Q`\x03\x90\x06`\x01\x81\x14a\x0CkW`\x02\x81\x14a\x0C~Wa\x0C\x86V[`=`\x01\x83\x03S`=`\x02\x83\x03Sa\x0C\x86V[`=`\x01\x83\x03S[P\x91\x95\x94PPPPPV[```\0a\x0C\xA0\x83`\x02a\x11nV[a\x0C\xAB\x90`\x02a\x11\xD9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xC2Wa\x0C\xC2a\x0F\x02V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0C\xECW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\r\x07Wa\r\x07a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\r6Wa\r6a\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\rZ\x84`\x02a\x11nV[a\re\x90`\x01a\x11\xD9V[\x90P[`\x01\x81\x11\x15a\r\xDDWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\r\x99Wa\r\x99a\x11\x85V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\r\xAFWa\r\xAFa\x11\x85V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\r\xD6\x81a\x16\xB8V[\x90Pa\rhV[P\x83\x15a\x06\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x04\"V[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x0EkWr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[i\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x10a\x0E\x95Wi\x04\xEE-mA[\x85\xAC\xEF\x81` \x1B\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x0E\xB3Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x0E\xCBWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x0E\xDFWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x0E\xF1W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\n\x9AW`\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x0F2Wa\x0F2a\x0F\x02V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x0F`Wa\x0F`a\x0F\x02V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x0FxW`\0\x80\xFD[\x83\x83` \x83\x017`\0` \x85\x83\x01\x01RP\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\xA1W`\0\x80\xFD[a\x06\xEC\x83\x835` \x85\x01a\x0F\x18V[`\0` \x82\x84\x03\x12\x15a\x0F\xC2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xD8W`\0\x80\xFD[a\x0F\xE4\x84\x82\x85\x01a\x0F\x90V[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\x10\x07W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\xEFV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x10/\x81`@\x85\x01` \x87\x01a\x0F\xECV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10UW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10oW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x8CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x10\x9DW`\0\x80\xFD[a\x10\xAC\x85\x825` \x84\x01a\x0F\x18V[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xCBW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xE3W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\0W`\0\x80\xFD[a\x11\x0C\x86\x82\x87\x01a\x0F\x90V[\x92PP`@\x84\x015a\x11\x1D\x81a\x10\xB6V[\x80\x91PP\x92P\x92P\x92V[`\x03\x81\x10a\x11FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\n\x9A\x82\x84a\x11(V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\x9AWa\n\x9Aa\x11XV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x11\xC0Wa\x11\xC0a\x11\x9BV[P\x04\x90V[`\0\x82a\x11\xD4Wa\x11\xD4a\x11\x9BV[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\n\x9AWa\n\x9Aa\x11XV[`\0\x81Qa\x11\xFE\x81\x85` \x86\x01a\x0F\xECV[\x92\x90\x92\x01\x92\x91PPV[a\x06\x0F`\xF3\x1B\x81R`\0\x82Qa\x12%\x81`\x02\x85\x01` \x87\x01a\x0F\xECV[\x91\x90\x91\x01`\x02\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12DW`\0\x80\xFD[PQ\x91\x90PV[\x82\x81R`@\x81\x01a\x06\xEC` \x83\x01\x84a\x11(V[`\0` \x82\x84\x03\x12\x15a\x12qW`\0\x80\xFD[\x81Qa\x06\xEC\x81a\x10\xB6V[` \x80\x82R`L\x90\x82\x01R\x7FPKPHelper: only the Domain Walle`@\x82\x01R\x7Ft registry is allowed to mint do``\x82\x01Rkmain wallets`\xA0\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\x02W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\"WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x05\x84W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x13OWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x13oW`\0\x81U`\x01\x01a\x13[V[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x8FWa\x13\x8Fa\x0F\x02V[a\x13\xA3\x81a\x13\x9D\x84Ta\x12\xEEV[\x84a\x13(V[` `\x1F\x82\x11`\x01\x81\x14a\x13\xD7W`\0\x83\x15a\x13\xBFWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x13oV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x14\x07W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x13\xE7V[P\x84\x82\x10\x15a\x14%W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x7Fdata:application/json;base64,\0\0\0\x81R`\0\x82Qa\x14l\x81`\x1D\x85\x01` \x87\x01a\x0F\xECV[\x91\x90\x91\x01`\x1D\x01\x92\x91PPV[hLit PKP #`\xB8\x1B\x81R`\0\x82Qa\x14\x9D\x81`\t\x85\x01` \x87\x01a\x0F\xECV[\x91\x90\x91\x01`\t\x01\x92\x91PPV[h=\x9170\xB6\xB2\x91\x1D\x11`\xB9\x1B\x81R\x85Q`\0\x90a\x14\xCF\x81`\t\x85\x01` \x8B\x01a\x0F\xECV[\x7F\", \"description\": \"This NFT enti`\t\x91\x84\x01\x91\x82\x01R\x7Ftles the holder to use a Lit Pro`)\x82\x01R\x7Ftocol PKP, and to grant access t`I\x82\x01R\x7Fo other users and Lit Actions to`i\x82\x01R\x7F use this PKP\",\"image_data\": \"\0\0`\x89\x82\x01R\x86Qa\x15\xA4\x81`\xA7\x84\x01` \x8B\x01a\x0F\xECV[`\t\x81\x83\x01\x01\x91PP\x7F\",\"attributes\": [{\"trait_type\": `\x9E\x82\x01Rw\x11(:\xB164\xB1\x90%\xB2\xBC\x91\x16\x10\x11;0\xB6:\xB2\x91\x1D\x10\x11`A\x1B`\xBE\x82\x01Ra\x16\xACa\x16\x9Ca\x16\x96a\x16[a\x16Ua\x16\x10`\xD6\x87\x01\x8Ca\x11\xECV[\x7F\"}, {\"trait_type\": \"ETH Wallet A\x81Rr2292\xB9\xB9\x91\x16\x10\x11;0\xB6:\xB2\x91\x1D\x10\x11`i\x1B` \x82\x01R`3\x01\x90V[\x89a\x11\xECV[\x7F\"}, {\"trait_type\": \"Token ID\", \"\x81Rh;0\xB6:\xB2\x91\x1D\x10\x11`\xB9\x1B` \x82\x01R`)\x01\x90V[\x86a\x11\xECV[c\"}]}`\xE0\x1B\x81R`\x04\x01\x90V[\x98\x97PPPPPPPPV[`\0\x81a\x16\xC7Wa\x16\xC7a\x11XV[P`\0\x19\x01\x90V\xFE<svg xmlns='http://www.w3.org/2000/svg' width='1080' height='1080' fill='none' xmlns:v='https://vecta.io/nano'><path d='M363.076 392.227s-.977 18.524-36.874 78.947c-41.576 70.018-45.481 151.978-3.017 220.4 89.521 144.245 332.481 141.52 422.556.089 34.832-54.707 44.816-117.479 32.924-181.248 0 0-28.819-133.144-127.237-217.099 1.553 1.308 5.369 19.122 6.101 26.722 2.241 23.354.045 47.838-7.787 70.062-5.746 16.33-13.711 30.467-27.178 41.368 0-3.811-.954-10.635-.976-12.918-.644-46.508-18.659-89.582-48.011-125.743-25.647-31.552-60.812-53.089-97.84-68.932.931 3.191 2.662 16.419 2.906 19.033 1.908 21.958 2.263 52.713-.621 74.649s-7.832 33.878-14.554 54.441c-10.184 31.175-24.05 54.285-41.621 82.004-3.24 5.096-12.913 19.078-18.082 26.146 0 0-8.897-56.191-40.667-87.921h-.022z' fill='#000'/><path d='M562.5 27.28l410.279 236.874c13.923 8.039 22.5 22.895 22.5 38.971v473.75c0 16.076-8.577 30.932-22.5 38.971L562.5 1052.72c-13.923 8.04-31.077 8.04-45 0L107.221 815.846c-13.923-8.039-22.5-22.895-22.5-38.971v-473.75a45 45 0 0 1 22.5-38.971L517.5 27.28a45 45 0 0 1 45 0z' stroke='#000' stroke-width='24.75'/></svg>ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\xA2dipfsX\"\x12 \x8F\x13\x99\xB0\x85\xBF\xFD\xE0ex\xF1\xDD\x04\xAD6!~\xEDG\xFB\xA0mu\xA3\x1F\\\xE9\xE6qQx\x82dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static PKPNFTMETADATA_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PKPNFTMetadata<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PKPNFTMetadata<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PKPNFTMetadata<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PKPNFTMetadata<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PKPNFTMetadata<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PKPNFTMetadata))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PKPNFTMetadata<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PKPNFTMETADATA_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                PKPNFTMETADATA_ABI.clone(),
                PKPNFTMETADATA_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `bytesToHex` (0x451d89fa) function
        pub fn bytes_to_hex(
            &self,
            buffer: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([69, 29, 137, 250], buffer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractResolver` (0x50d17b5e) function
        pub fn contract_resolver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 209, 123, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `env` (0x9dca0032) function
        pub fn env(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([157, 202, 0, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeProfileForPkp` (0xb63a7677) function
        pub fn remove_profile_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 58, 118, 119], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeUrlForPKP` (0x519a218e) function
        pub fn remove_url_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 154, 33, 142], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProfileForPKP` (0x9000fee1) function
        pub fn set_profile_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
            img_url: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 0, 254, 225], (token_id, img_url))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUrlForPKP` (0x855eec22) function
        pub fn set_url_for_pkp(
            &self,
            token_id: ::ethers::core::types::U256,
            url: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 94, 236, 34], (token_id, url))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0x950462ee) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
            pub_key: ::ethers::core::types::Bytes,
            eth_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 4, 98, 238], (token_id, pub_key, eth_address))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PKPNFTMetadata<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `bytesToHex` function with signature `bytesToHex(bytes)` and selector `0x451d89fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bytesToHex", abi = "bytesToHex(bytes)")]
    pub struct BytesToHexCall {
        pub buffer: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `contractResolver` function with signature `contractResolver()` and selector `0x50d17b5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "contractResolver", abi = "contractResolver()")]
    pub struct ContractResolverCall;
    ///Container type for all input parameters for the `env` function with signature `env()` and selector `0x9dca0032`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "env", abi = "env()")]
    pub struct EnvCall;
    ///Container type for all input parameters for the `removeProfileForPkp` function with signature `removeProfileForPkp(uint256)` and selector `0xb63a7677`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeProfileForPkp", abi = "removeProfileForPkp(uint256)")]
    pub struct RemoveProfileForPkpCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeUrlForPKP` function with signature `removeUrlForPKP(uint256)` and selector `0x519a218e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeUrlForPKP", abi = "removeUrlForPKP(uint256)")]
    pub struct RemoveUrlForPKPCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setProfileForPKP` function with signature `setProfileForPKP(uint256,string)` and selector `0x9000fee1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setProfileForPKP", abi = "setProfileForPKP(uint256,string)")]
    pub struct SetProfileForPKPCall {
        pub token_id: ::ethers::core::types::U256,
        pub img_url: ::std::string::String,
    }
    ///Container type for all input parameters for the `setUrlForPKP` function with signature `setUrlForPKP(uint256,string)` and selector `0x855eec22`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setUrlForPKP", abi = "setUrlForPKP(uint256,string)")]
    pub struct SetUrlForPKPCall {
        pub token_id: ::ethers::core::types::U256,
        pub url: ::std::string::String,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256,bytes,address)` and selector `0x950462ee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256,bytes,address)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
        pub pub_key: ::ethers::core::types::Bytes,
        pub eth_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PKPNFTMetadataCalls {
        BytesToHex(BytesToHexCall),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        RemoveProfileForPkp(RemoveProfileForPkpCall),
        RemoveUrlForPKP(RemoveUrlForPKPCall),
        SetProfileForPKP(SetProfileForPKPCall),
        SetUrlForPKP(SetUrlForPKPCall),
        TokenURI(TokenURICall),
    }
    impl ::ethers::core::abi::AbiDecode for PKPNFTMetadataCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BytesToHexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BytesToHex(decoded));
            }
            if let Ok(decoded) = <ContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractResolver(decoded));
            }
            if let Ok(decoded) = <EnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Env(decoded));
            }
            if let Ok(decoded) = <RemoveProfileForPkpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveProfileForPkp(decoded));
            }
            if let Ok(decoded) = <RemoveUrlForPKPCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveUrlForPKP(decoded));
            }
            if let Ok(decoded) = <SetProfileForPKPCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProfileForPKP(decoded));
            }
            if let Ok(decoded) = <SetUrlForPKPCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUrlForPKP(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PKPNFTMetadataCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BytesToHex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Env(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveProfileForPkp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveUrlForPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProfileForPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUrlForPKP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PKPNFTMetadataCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BytesToHex(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveProfileForPkp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveUrlForPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProfileForPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUrlForPKP(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BytesToHexCall> for PKPNFTMetadataCalls {
        fn from(value: BytesToHexCall) -> Self {
            Self::BytesToHex(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for PKPNFTMetadataCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for PKPNFTMetadataCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<RemoveProfileForPkpCall> for PKPNFTMetadataCalls {
        fn from(value: RemoveProfileForPkpCall) -> Self {
            Self::RemoveProfileForPkp(value)
        }
    }
    impl ::core::convert::From<RemoveUrlForPKPCall> for PKPNFTMetadataCalls {
        fn from(value: RemoveUrlForPKPCall) -> Self {
            Self::RemoveUrlForPKP(value)
        }
    }
    impl ::core::convert::From<SetProfileForPKPCall> for PKPNFTMetadataCalls {
        fn from(value: SetProfileForPKPCall) -> Self {
            Self::SetProfileForPKP(value)
        }
    }
    impl ::core::convert::From<SetUrlForPKPCall> for PKPNFTMetadataCalls {
        fn from(value: SetUrlForPKPCall) -> Self {
            Self::SetUrlForPKP(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for PKPNFTMetadataCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    ///Container type for all return fields from the `bytesToHex` function with signature `bytesToHex(bytes)` and selector `0x451d89fa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BytesToHexReturn(pub ::std::string::String);
    ///Container type for all return fields from the `contractResolver` function with signature `contractResolver()` and selector `0x50d17b5e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ContractResolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `env` function with signature `env()` and selector `0x9dca0032`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvReturn(pub u8);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256,bytes,address)` and selector `0x950462ee`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenURIReturn(pub ::std::string::String);
}
