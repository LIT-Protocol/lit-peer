pub use arbitrum_key_deriver::*;
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
pub mod arbitrum_key_deriver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ArbitrumKeyDeriver.json",
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
                    ::std::borrow::ToOwned::to_owned("ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HD_KDF_K256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HD_KDF_K256"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HD_KDF_P256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HD_KDF_P256"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeHDPubKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("computeHDPubKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derivedKeyId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootHDKeys"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPubkeyRouter.RootKey[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setContractResolver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setContractResolver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "contractResolverAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ARBITRUMKEYDERIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x11\xCA8\x03\x80a\x11\xCA\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xA4V[a\0G`\0\x80Q` a\x11\xAA\x839\x81Q\x91R3a\0\xADV[a\0_`\0\x80Q` a\x11\xAA\x839\x81Q\x91R\x80a\0\xBBV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U\x83\x92\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83`\x02\x81\x11\x15a\0\xA1Wa\0\xA1a\x01\xEEV[\x02\x17\x90UPPPa\x02\x04V[a\0\xB7\x82\x82a\x01\x06V[PPV[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\0\xB7W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x01`3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xB7W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xCEW`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x03\x81\x10a\x01\xE3W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a\x0F\x97\x80a\x02\x13`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBAW`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0\xBFW\x80c$\x8A\x9C\xA3\x14a\0\xE7W\x80c//\xF1]\x14a\x01\x08W\x80c6V\x8A\xBE\x14a\x01\x1DW\x80cP\xD1{^\x14a\x010W\x80cu\xB28\xFC\x14a\x01[W\x80c\x91\xD1HT\x14a\x01\x82W\x80c\x9D\xCA\x002\x14a\x01\x95W\x80c\xA2\x17\xFD\xDF\x14a\x01\xB6W\x80c\xA3,+\x99\x14a\x01\xBEW\x80c\xB2N\xD3\x08\x14a\x01\xDFW\x80c\xD5Gt\x1F\x14a\x02\x06W\x80c\xF9]q\xB1\x14a\x02\x19W\x80c\xFE\x89\xC9p\x14a\x02,W[`\0\x80\xFD[a\0\xD2a\0\xCD6`\x04a\t\x8DV[a\x02RV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFAa\0\xF56`\x04a\t\xB7V[a\x02\x89V[`@Q\x90\x81R` \x01a\0\xDEV[a\x01\x1Ba\x01\x166`\x04a\t\xE5V[a\x02\x9EV[\0[a\x01\x1Ba\x01+6`\x04a\t\xE5V[a\x02\xBFV[`\x01Ta\x01C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xDEV[a\0\xFA\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[a\0\xD2a\x01\x906`\x04a\t\xE5V[a\x03BV[`\x01Ta\x01\xA9\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\0\xDE\x91\x90a\n7V[a\0\xFA`\0\x81V[a\x01\xD1a\x01\xCC6`\x04a\n\xDAV[a\x03kV[`@Qa\0\xDE\x92\x91\x90a\x0C\x92V[a\0\xFA\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9\x81V[a\x01\x1Ba\x02\x146`\x04a\t\xE5V[a\x04\xF6V[a\x01\x1Ba\x02'6`\x04a\x0C\xB5V[a\x05\x12V[a\0\xFA~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\x83WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x02\xA7\x82a\x02\x89V[a\x02\xB0\x81a\x05_V[a\x02\xBA\x83\x83a\x05lV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x034W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03>\x82\x82a\x05\xF0V[PPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0```\0a\x03|\x86\x86\x86a\x06UV[\x90P`\0\x81`\0\x81Q\x81\x10a\x03\x93Wa\x03\x93a\x0C\xD2V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16`\0\x03a\x03\xD0WP\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9a\x03\xF2V[P~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd[`\x01T`@QcGF\xFE\x8B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x81\x16\x91c\x8E\x8D\xFD\x16\x91a\x04/\x91\x86\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x0C\xE8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04p\x91\x90a\x0C\xFCV[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xECr3g\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA0\x91\x90a\r\x19V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE5\x91\x90\x81\x01\x90a\r,V[`\x01\x9A\x90\x99P\x97PPPPPPPPV[a\x04\xFF\x82a\x02\x89V[a\x05\x08\x81a\x05_V[a\x02\xBA\x83\x83a\x05\xF0V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x05<\x81a\x05_V[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x05i\x813a\x07\x87V[PV[a\x05v\x82\x82a\x03BV[a\x03>W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x05\xAC3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x05\xFA\x82\x82a\x03BV[\x15a\x03>W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R``\x91\x80[\x85Q\x81\x10\x15a\x06\xEFW\x84\x86\x82\x81Q\x81\x10a\x06\x86Wa\x06\x86a\x0C\xD2V[` \x02` \x01\x01Q` \x01Q\x03a\x06\xE7W\x82\x86\x82\x81Q\x81\x10a\x06\xAAWa\x06\xAAa\x0C\xD2V[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\x06\xC7\x92\x91\x90a\r\xA2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\x06\xE3\x90a\r\xE7V[\x92PP[`\x01\x01a\x06jV[P\x83`\x02\x03a\x07\x01W`\x01\x93Pa\x07\x0EV[\x83`\x03\x03a\x07\x0EW`\0\x93P[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\x0F7`+\x919\x80Q`@Q\x91\x92P`\xF8\x87\x90\x1B\x91`\x01`\xE5\x1B\x91`\xE0\x90\x81\x1B\x91\x90\x86\x90\x1B\x90`\0\x90a\x07e\x90\x86\x90\x86\x90\x8F\x90\x87\x90\x8B\x90\x88\x90\x8F\x90` \x01a\x0E\x0CV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PPPPPPPPP[\x93\x92PPPV[a\x07\x91\x82\x82a\x03BV[a\x03>Wa\x07\x9E\x81a\x07\xE0V[a\x07\xA9\x83` a\x07\xF2V[`@Q` \x01a\x07\xBA\x92\x91\x90a\x0E\x86V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03+\x91`\x04\x01a\r\x19V[``a\x02\x83`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x08\x01\x83`\x02a\x0E\xF5V[a\x08\x0C\x90`\x02a\x0F\x0CV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08#Wa\x08#a\nEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x08MW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x08hWa\x08ha\x0C\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x08\x97Wa\x08\x97a\x0C\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x08\xBB\x84`\x02a\x0E\xF5V[a\x08\xC6\x90`\x01a\x0F\x0CV[\x90P[`\x01\x81\x11\x15a\t>Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x08\xFAWa\x08\xFAa\x0C\xD2V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\t\x10Wa\t\x10a\x0C\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\t7\x81a\x0F\x1FV[\x90Pa\x08\xC9V[P\x83\x15a\x07\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03+V[`\0` \x82\x84\x03\x12\x15a\t\x9FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\x80W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\xC9W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05iW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\t\xF8W`\0\x80\xFD[\x825\x91P` \x83\x015a\n\n\x81a\t\xD0V[\x80\x91PP\x92P\x92\x90PV[`\x03\x81\x10a\n3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x02\x83\x82\x84a\n\x15V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\n}Wa\n}a\nEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\n\xABWa\n\xABa\nEV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\n\xCCWa\n\xCCa\nEV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xEFW`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x0CW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x0B\x1DW`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B6Wa\x0B6a\nEV[\x80`\x05\x1Ba\x0BF` \x82\x01a\n\x83V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x89\x84\x11\x15a\x0BbW`\0\x80\xFD[` \x85\x01\x92P[\x83\x83\x10\x15a\x0C-W\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x87W`\0\x80\xFD[\x85\x01`@\x81\x8C\x03`\x1F\x19\x01\x12\x15a\x0B\x9DW`\0\x80\xFD[a\x0B\xA5a\n[V[` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xBEW`\0\x80\xFD[\x82\x01` \x81\x01\x90`?\x01\x8D\x13a\x0B\xD3W`\0\x80\xFD[\x805a\x0B\xE6a\x0B\xE1\x82a\n\xB3V[a\n\x83V[\x81\x81R\x8E` \x83\x85\x01\x01\x11\x15a\x0B\xFBW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x92\x82\x01\x83\x01R\x83R`@\x93\x90\x93\x015\x82\x84\x01RP\x83R\x92\x83\x01\x92\x91\x90\x91\x01\x90a\x0BiV[\x96\x99\x96\x98PPPP`@\x94\x90\x94\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x0C]W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0CEV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0C~\x81` \x86\x01` \x86\x01a\x0CBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x0C\xAD`@\x83\x01\x84a\x0CfV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\xC7W`\0\x80\xFD[\x815a\x07\x80\x81a\t\xD0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x82\x81R`@\x81\x01a\x07\x80` \x83\x01\x84a\n\x15V[`\0` \x82\x84\x03\x12\x15a\r\x0EW`\0\x80\xFD[\x81Qa\x07\x80\x81a\t\xD0V[` \x81R`\0a\x07\x80` \x83\x01\x84a\x0CfV[`\0` \x82\x84\x03\x12\x15a\r>W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\rTW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\reW`\0\x80\xFD[\x80Qa\rsa\x0B\xE1\x82a\n\xB3V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\r\x88W`\0\x80\xFD[a\r\x99\x82` \x83\x01` \x86\x01a\x0CBV[\x95\x94PPPPPV[`\0\x83Qa\r\xB4\x81\x84` \x88\x01a\x0CBV[\x83Q\x90\x83\x01\x90a\r\xC8\x81\x83` \x88\x01a\x0CBV[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x0E\x03Wa\x0E\x03a\r\xD1V[`\x01\x01\x92\x91PPV[`\x01`\x01`\xF8\x1B\x03\x19\x88\x16\x81R`\x01`\x01`\xE0\x1B\x03\x19\x87\x81\x16`\x01\x83\x01R`\x05\x82\x01\x87\x90R\x85\x16`%\x82\x01R\x83Q`\0\x90a\x0EN\x81`)\x85\x01` \x89\x01a\x0CBV[`\x01`\x01`\xE0\x1B\x03\x19\x85\x16`)\x91\x84\x01\x91\x82\x01R\x83Qa\x0Eu\x81`-\x84\x01` \x88\x01a\x0CBV[\x01`-\x01\x99\x98PPPPPPPPPV[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa\x0E\xB8\x81`\x17\x85\x01` \x88\x01a\x0CBV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x0E\xE9\x81`(\x84\x01` \x88\x01a\x0CBV[\x01`(\x01\x94\x93PPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x83Wa\x02\x83a\r\xD1V[\x80\x82\x01\x80\x82\x11\x15a\x02\x83Wa\x02\x83a\r\xD1V[`\0\x81a\x0F.Wa\x0F.a\r\xD1V[P`\0\x19\x01\x90V\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 7$;\xC4>x\xAC4\xBBE\xBF\xE2CE\x0C(\x1Fa\xD2&\xE1,\xDA\xB0K(\x99\x83*D\xFFwdsolcC\0\x08\x1C\x003\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB";
    /// The bytecode of the contract.
    pub static ARBITRUMKEYDERIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBAW`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0\xBFW\x80c$\x8A\x9C\xA3\x14a\0\xE7W\x80c//\xF1]\x14a\x01\x08W\x80c6V\x8A\xBE\x14a\x01\x1DW\x80cP\xD1{^\x14a\x010W\x80cu\xB28\xFC\x14a\x01[W\x80c\x91\xD1HT\x14a\x01\x82W\x80c\x9D\xCA\x002\x14a\x01\x95W\x80c\xA2\x17\xFD\xDF\x14a\x01\xB6W\x80c\xA3,+\x99\x14a\x01\xBEW\x80c\xB2N\xD3\x08\x14a\x01\xDFW\x80c\xD5Gt\x1F\x14a\x02\x06W\x80c\xF9]q\xB1\x14a\x02\x19W\x80c\xFE\x89\xC9p\x14a\x02,W[`\0\x80\xFD[a\0\xD2a\0\xCD6`\x04a\t\x8DV[a\x02RV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFAa\0\xF56`\x04a\t\xB7V[a\x02\x89V[`@Q\x90\x81R` \x01a\0\xDEV[a\x01\x1Ba\x01\x166`\x04a\t\xE5V[a\x02\x9EV[\0[a\x01\x1Ba\x01+6`\x04a\t\xE5V[a\x02\xBFV[`\x01Ta\x01C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xDEV[a\0\xFA\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\x81V[a\0\xD2a\x01\x906`\x04a\t\xE5V[a\x03BV[`\x01Ta\x01\xA9\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\0\xDE\x91\x90a\n7V[a\0\xFA`\0\x81V[a\x01\xD1a\x01\xCC6`\x04a\n\xDAV[a\x03kV[`@Qa\0\xDE\x92\x91\x90a\x0C\x92V[a\0\xFA\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9\x81V[a\x01\x1Ba\x02\x146`\x04a\t\xE5V[a\x04\xF6V[a\x01\x1Ba\x02'6`\x04a\x0C\xB5V[a\x05\x12V[a\0\xFA~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\x83WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x02\xA7\x82a\x02\x89V[a\x02\xB0\x81a\x05_V[a\x02\xBA\x83\x83a\x05lV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x034W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03>\x82\x82a\x05\xF0V[PPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0```\0a\x03|\x86\x86\x86a\x06UV[\x90P`\0\x81`\0\x81Q\x81\x10a\x03\x93Wa\x03\x93a\x0C\xD2V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16`\0\x03a\x03\xD0WP\x7F\x9A\x91\x86.\xF1T4\xE2e\x8Eh'R\xE7C\xFAIu\xA1\x17\x80}\xF7\xF0\xEA\xCA\xB6n7\xE8\x04\xD9a\x03\xF2V[P~\xC3H\xEF\x80\xE6m\"\xF4D\n\x90\xBF\x96C\xA0<\x82&\r\r\xCC\xA4(l\xF1\x14\xCC\x97\xDB\x0Cd[`\x01T`@QcGF\xFE\x8B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x81\x16\x91c\x8E\x8D\xFD\x16\x91a\x04/\x91\x86\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a\x0C\xE8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04p\x91\x90a\x0C\xFCV[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xECr3g\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA0\x91\x90a\r\x19V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE5\x91\x90\x81\x01\x90a\r,V[`\x01\x9A\x90\x99P\x97PPPPPPPPV[a\x04\xFF\x82a\x02\x89V[a\x05\x08\x81a\x05_V[a\x02\xBA\x83\x83a\x05\xF0V[\x7F\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECBa\x05<\x81a\x05_V[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x05i\x813a\x07\x87V[PV[a\x05v\x82\x82a\x03BV[a\x03>W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x05\xAC3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x05\xFA\x82\x82a\x03BV[\x15a\x03>W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R``\x91\x80[\x85Q\x81\x10\x15a\x06\xEFW\x84\x86\x82\x81Q\x81\x10a\x06\x86Wa\x06\x86a\x0C\xD2V[` \x02` \x01\x01Q` \x01Q\x03a\x06\xE7W\x82\x86\x82\x81Q\x81\x10a\x06\xAAWa\x06\xAAa\x0C\xD2V[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\x06\xC7\x92\x91\x90a\r\xA2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\x06\xE3\x90a\r\xE7V[\x92PP[`\x01\x01a\x06jV[P\x83`\x02\x03a\x07\x01W`\x01\x93Pa\x07\x0EV[\x83`\x03\x03a\x07\x0EW`\0\x93P[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\x0F7`+\x919\x80Q`@Q\x91\x92P`\xF8\x87\x90\x1B\x91`\x01`\xE5\x1B\x91`\xE0\x90\x81\x1B\x91\x90\x86\x90\x1B\x90`\0\x90a\x07e\x90\x86\x90\x86\x90\x8F\x90\x87\x90\x8B\x90\x88\x90\x8F\x90` \x01a\x0E\x0CV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x98PPPPPPPPP[\x93\x92PPPV[a\x07\x91\x82\x82a\x03BV[a\x03>Wa\x07\x9E\x81a\x07\xE0V[a\x07\xA9\x83` a\x07\xF2V[`@Q` \x01a\x07\xBA\x92\x91\x90a\x0E\x86V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03+\x91`\x04\x01a\r\x19V[``a\x02\x83`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x08\x01\x83`\x02a\x0E\xF5V[a\x08\x0C\x90`\x02a\x0F\x0CV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08#Wa\x08#a\nEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x08MW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x08hWa\x08ha\x0C\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x08\x97Wa\x08\x97a\x0C\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x08\xBB\x84`\x02a\x0E\xF5V[a\x08\xC6\x90`\x01a\x0F\x0CV[\x90P[`\x01\x81\x11\x15a\t>Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x08\xFAWa\x08\xFAa\x0C\xD2V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\t\x10Wa\t\x10a\x0C\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\t7\x81a\x0F\x1FV[\x90Pa\x08\xC9V[P\x83\x15a\x07\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03+V[`\0` \x82\x84\x03\x12\x15a\t\x9FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\x80W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\xC9W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05iW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\t\xF8W`\0\x80\xFD[\x825\x91P` \x83\x015a\n\n\x81a\t\xD0V[\x80\x91PP\x92P\x92\x90PV[`\x03\x81\x10a\n3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x02\x83\x82\x84a\n\x15V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\n}Wa\n}a\nEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\n\xABWa\n\xABa\nEV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\n\xCCWa\n\xCCa\nEV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xEFW`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x0CW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x0B\x1DW`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B6Wa\x0B6a\nEV[\x80`\x05\x1Ba\x0BF` \x82\x01a\n\x83V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x89\x84\x11\x15a\x0BbW`\0\x80\xFD[` \x85\x01\x92P[\x83\x83\x10\x15a\x0C-W\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x87W`\0\x80\xFD[\x85\x01`@\x81\x8C\x03`\x1F\x19\x01\x12\x15a\x0B\x9DW`\0\x80\xFD[a\x0B\xA5a\n[V[` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xBEW`\0\x80\xFD[\x82\x01` \x81\x01\x90`?\x01\x8D\x13a\x0B\xD3W`\0\x80\xFD[\x805a\x0B\xE6a\x0B\xE1\x82a\n\xB3V[a\n\x83V[\x81\x81R\x8E` \x83\x85\x01\x01\x11\x15a\x0B\xFBW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x92\x82\x01\x83\x01R\x83R`@\x93\x90\x93\x015\x82\x84\x01RP\x83R\x92\x83\x01\x92\x91\x90\x91\x01\x90a\x0BiV[\x96\x99\x96\x98PPPP`@\x94\x90\x94\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x0C]W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0CEV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0C~\x81` \x86\x01` \x86\x01a\x0CBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x0C\xAD`@\x83\x01\x84a\x0CfV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\xC7W`\0\x80\xFD[\x815a\x07\x80\x81a\t\xD0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x82\x81R`@\x81\x01a\x07\x80` \x83\x01\x84a\n\x15V[`\0` \x82\x84\x03\x12\x15a\r\x0EW`\0\x80\xFD[\x81Qa\x07\x80\x81a\t\xD0V[` \x81R`\0a\x07\x80` \x83\x01\x84a\x0CfV[`\0` \x82\x84\x03\x12\x15a\r>W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\rTW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\reW`\0\x80\xFD[\x80Qa\rsa\x0B\xE1\x82a\n\xB3V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\r\x88W`\0\x80\xFD[a\r\x99\x82` \x83\x01` \x86\x01a\x0CBV[\x95\x94PPPPPV[`\0\x83Qa\r\xB4\x81\x84` \x88\x01a\x0CBV[\x83Q\x90\x83\x01\x90a\r\xC8\x81\x83` \x88\x01a\x0CBV[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x0E\x03Wa\x0E\x03a\r\xD1V[`\x01\x01\x92\x91PPV[`\x01`\x01`\xF8\x1B\x03\x19\x88\x16\x81R`\x01`\x01`\xE0\x1B\x03\x19\x87\x81\x16`\x01\x83\x01R`\x05\x82\x01\x87\x90R\x85\x16`%\x82\x01R\x83Q`\0\x90a\x0EN\x81`)\x85\x01` \x89\x01a\x0CBV[`\x01`\x01`\xE0\x1B\x03\x19\x85\x16`)\x91\x84\x01\x91\x82\x01R\x83Qa\x0Eu\x81`-\x84\x01` \x88\x01a\x0CBV[\x01`-\x01\x99\x98PPPPPPPPPV[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa\x0E\xB8\x81`\x17\x85\x01` \x88\x01a\x0CBV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x0E\xE9\x81`(\x84\x01` \x88\x01a\x0CBV[\x01`(\x01\x94\x93PPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x83Wa\x02\x83a\r\xD1V[\x80\x82\x01\x80\x82\x11\x15a\x02\x83Wa\x02\x83a\r\xD1V[`\0\x81a\x0F.Wa\x0F.a\r\xD1V[P`\0\x19\x01\x90V\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 7$;\xC4>x\xAC4\xBBE\xBF\xE2CE\x0C(\x1Fa\xD2&\xE1,\xDA\xB0K(\x99\x83*D\xFFwdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static ARBITRUMKEYDERIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ArbitrumKeyDeriver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ArbitrumKeyDeriver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ArbitrumKeyDeriver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ArbitrumKeyDeriver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ArbitrumKeyDeriver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ArbitrumKeyDeriver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ArbitrumKeyDeriver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ARBITRUMKEYDERIVER_ABI.clone(),
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
                ARBITRUMKEYDERIVER_ABI.clone(),
                ARBITRUMKEYDERIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ADMIN_ROLE` (0x75b238fc) function
        pub fn admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([117, 178, 56, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HD_KDF_K256` (0xfe89c970) function
        pub fn hd_kdf_k256(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([254, 137, 201, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HD_KDF_P256` (0xb24ed308) function
        pub fn hd_kdf_p256(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([178, 78, 211, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeHDPubKey` (0xa32c2b99) function
        pub fn compute_hd_pub_key(
            &self,
            derived_key_id: [u8; 32],
            root_hd_keys: ::std::vec::Vec<RootKey>,
            key_type: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash(
                    [163, 44, 43, 153],
                    (derived_key_id, root_hd_keys, key_type),
                )
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setContractResolver` (0xf95d71b1) function
        pub fn set_contract_resolver(
            &self,
            contract_resolver_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 93, 113, 177], contract_resolver_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ArbitrumKeyDeriverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ArbitrumKeyDeriver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
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
    pub enum ArbitrumKeyDeriverEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ArbitrumKeyDeriverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ArbitrumKeyDeriverEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ArbitrumKeyDeriverEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ArbitrumKeyDeriverEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ArbitrumKeyDeriverEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ArbitrumKeyDeriverEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ArbitrumKeyDeriverEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ArbitrumKeyDeriverEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ADMIN_ROLE` function with signature `ADMIN_ROLE()` and selector `0x75b238fc`
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
    #[ethcall(name = "ADMIN_ROLE", abi = "ADMIN_ROLE()")]
    pub struct AdminRoleCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `HD_KDF_K256` function with signature `HD_KDF_K256()` and selector `0xfe89c970`
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
    #[ethcall(name = "HD_KDF_K256", abi = "HD_KDF_K256()")]
    pub struct HdKdfK256Call;
    ///Container type for all input parameters for the `HD_KDF_P256` function with signature `HD_KDF_P256()` and selector `0xb24ed308`
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
    #[ethcall(name = "HD_KDF_P256", abi = "HD_KDF_P256()")]
    pub struct HdKdfP256Call;
    ///Container type for all input parameters for the `computeHDPubKey` function with signature `computeHDPubKey(bytes32,(bytes,uint256)[],uint256)` and selector `0xa32c2b99`
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
    #[ethcall(
        name = "computeHDPubKey",
        abi = "computeHDPubKey(bytes32,(bytes,uint256)[],uint256)"
    )]
    pub struct ComputeHDPubKeyCall {
        pub derived_key_id: [u8; 32],
        pub root_hd_keys: ::std::vec::Vec<RootKey>,
        pub key_type: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setContractResolver` function with signature `setContractResolver(address)` and selector `0xf95d71b1`
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
    #[ethcall(name = "setContractResolver", abi = "setContractResolver(address)")]
    pub struct SetContractResolverCall {
        pub contract_resolver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
    pub enum ArbitrumKeyDeriverCalls {
        AdminRole(AdminRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        HdKdfK256(HdKdfK256Call),
        HdKdfP256(HdKdfP256Call),
        ComputeHDPubKey(ComputeHDPubKeyCall),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetContractResolver(SetContractResolverCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ArbitrumKeyDeriverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <HdKdfK256Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKdfK256(decoded));
            }
            if let Ok(decoded) = <HdKdfP256Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKdfP256(decoded));
            }
            if let Ok(decoded) = <ComputeHDPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeHDPubKey(decoded));
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
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SetContractResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetContractResolver(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ArbitrumKeyDeriverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HdKdfK256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HdKdfP256(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeHDPubKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Env(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ArbitrumKeyDeriverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HdKdfK256(element) => ::core::fmt::Display::fmt(element, f),
                Self::HdKdfP256(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHDPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractResolver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<HdKdfK256Call> for ArbitrumKeyDeriverCalls {
        fn from(value: HdKdfK256Call) -> Self {
            Self::HdKdfK256(value)
        }
    }
    impl ::core::convert::From<HdKdfP256Call> for ArbitrumKeyDeriverCalls {
        fn from(value: HdKdfP256Call) -> Self {
            Self::HdKdfP256(value)
        }
    }
    impl ::core::convert::From<ComputeHDPubKeyCall> for ArbitrumKeyDeriverCalls {
        fn from(value: ComputeHDPubKeyCall) -> Self {
            Self::ComputeHDPubKey(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for ArbitrumKeyDeriverCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for ArbitrumKeyDeriverCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ArbitrumKeyDeriverCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ArbitrumKeyDeriverCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetContractResolverCall> for ArbitrumKeyDeriverCalls {
        fn from(value: SetContractResolverCall) -> Self {
            Self::SetContractResolver(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ArbitrumKeyDeriverCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `ADMIN_ROLE` function with signature `ADMIN_ROLE()` and selector `0x75b238fc`
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
    pub struct AdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HD_KDF_K256` function with signature `HD_KDF_K256()` and selector `0xfe89c970`
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
    pub struct HdKdfK256Return(pub [u8; 32]);
    ///Container type for all return fields from the `HD_KDF_P256` function with signature `HD_KDF_P256()` and selector `0xb24ed308`
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
    pub struct HdKdfP256Return(pub [u8; 32]);
    ///Container type for all return fields from the `computeHDPubKey` function with signature `computeHDPubKey(bytes32,(bytes,uint256)[],uint256)` and selector `0xa32c2b99`
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
    pub struct ComputeHDPubKeyReturn(pub bool, pub ::ethers::core::types::Bytes);
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
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///`RootKey(bytes,uint256)`
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
    pub struct RootKey {
        pub pubkey: ::ethers::core::types::Bytes,
        pub key_type: ::ethers::core::types::U256,
    }
}
