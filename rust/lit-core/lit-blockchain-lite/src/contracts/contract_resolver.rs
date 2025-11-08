pub use contract_resolver::*;
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
pub mod contract_resolver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ContractResolver.json",
        );
    };
    #[allow(deprecated)]
        fn __abi_functions() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::Function>> {
        
        std::collections::BTreeMap::from(
        [
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
                    ::std::borrow::ToOwned::to_owned("ALLOWLIST_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ALLOWLIST_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("BACKUP_RECOVERY_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BACKUP_RECOVERY_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_WALLET_REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DOMAIN_WALLET_REGISTRY",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("FORWARDER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FORWARDER_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("HD_KEY_DERIVER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HD_KEY_DERIVER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("HOST_COMMANDS_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HOST_COMMANDS_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("LEDGER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LEDGER_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("LIT_TOKEN_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LIT_TOKEN_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("MULTI_SENDER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MULTI_SENDER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PAYMENT_DELEGATION_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PAYMENT_DELEGATION_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_HELPER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_HELPER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_HELPER_V2_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_HELPER_V2_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_NFT_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PKP_NFT_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("PKP_NFT_METADATA_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_NFT_METADATA_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PKP_PERMISSIONS_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PKP_PERMISSIONS_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PRICE_FEED_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRICE_FEED_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PUB_KEY_ROUTER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PUB_KEY_ROUTER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("RATE_LIMIT_NFT_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RATE_LIMIT_NFT_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("RELEASE_REGISTER_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELEASE_REGISTER_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_BALANCES_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STAKING_BALANCES_CONTRACT",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("STAKING_CONTRACT"),
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
                    ::std::borrow::ToOwned::to_owned("addAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("addAllowedEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAllowedEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("removeAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adminBeingRemoved"),
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
                    ::std::borrow::ToOwned::to_owned("removeAllowedEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAllowedEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("setContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ContractResolver.Env",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
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
                (
                    ::std::borrow::ToOwned::to_owned("typeAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("typeAddresses"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]
        )
        }
        
        #[allow(deprecated)]
        fn __abi_events() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::Event>> {
        
        std::collections::BTreeMap::from(
        [
                (
                    ::std::borrow::ToOwned::to_owned("AllowedEnvAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllowedEnvAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedEnvRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllowedEnvRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                (
                    ::std::borrow::ToOwned::to_owned("SetContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]
        )
        }
        
        #[allow(deprecated)]
        fn __abi_errors() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::AbiError>> {
        
        std::collections::BTreeMap::from(
        [
                (
                    ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]
        )
        }
        
        #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        
        
        
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("env"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("enum ContractResolver.Env"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from(__abi_functions()),
            events: ::core::convert::From::from(__abi_events()),
            errors: ::core::convert::From::from(__abi_errors()),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONTRACTRESOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x14g8\x03\x80a\x14g\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xE0V[a\0G`\0\x80Q` a\x14G\x839\x81Q\x91R3a\0\xE9V[a\0_`\0\x80Q` a\x14G\x839\x81Q\x91R\x80a\0\xF7V[`\x01\x80`\0\x83`\x02\x81\x11\x15a\0vWa\0va\x02\x08V[`\x02\x81\x11\x15a\0\x87Wa\0\x87a\x02\x08V[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\0\xDB\x91\x90a\x02\x1EV[`@Q\x80\x91\x03\x90\xA1Pa\x02FV[a\0\xF3\x82\x82a\x01BV[PPV[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\0\xF3W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x01\x9C3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0` \x82\x84\x03\x12\x15a\x01\xF2W`\0\x80\xFD[\x81Q`\x03\x81\x10a\x02\x01W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x02@WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[a\x11\xF2\x80a\x02U`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xBCW`\x005`\xE0\x1C\x80c|\xAD\xF6\x9F\x11a\0\xF5W\x80c|\xAD\xF6\x9F\x14a\x04\x17W\x80c}J\x03\xBD\x14a\x04>W\x80c}\x9D(\x80\x14a\x04eW\x80c\x7F\x90 \x9F\x14a\x04\x8CW\x80c\x85\xCB\x11\x91\x14a\x04\xB3W\x80c\x8C\x156\xDF\x14a\x04\xDAW\x80c\x8D\xEB8\x93\x14a\x05\x01W\x80c\x8E\x8D\xFD\x16\x14a\x05\x14W\x80c\x90r\xF88\x14a\x05'W\x80c\x91\xD1HT\x14a\x05NW\x80c\x97z\x80p\x14a\x05aW\x80c\xA2\x17\xFD\xDF\x14a\x05\x88W\x80c\xAD\x1C\x8A\x86\x14a\x05\x90W\x80c\xCD\xDC\xAC\xE5\x14a\x05\xB7W\x80c\xD5Gt\x1F\x14a\x05\xDEW\x80c\xDA\x19\xDD\xFB\x14a\x05\xF1W\x80c\xDF8\x06\x93\x14a\x06\x18W\x80c\xF8\xAE\x93\xB4\x14a\x06?W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC1W\x80c\x11\xEE\x8F\xF7\x14a\x01\xE9W\x80c\x16\xF7k\xBF\x14a\x02\x1EW\x80c\x17\x85\xF5<\x14a\x02EW\x80c!\x9C&j\x14a\x02ZW\x80c$\x8A\x9C\xA3\x14a\x02\x81W\x80c&h\xF3\x05\x14a\x02\x94W\x80c,\x0B\x8B\xF7\x14a\x02\xBBW\x80c.H\x85\xE8\x14a\x02\xE2W\x80c//\xF1]\x14a\x03\tW\x80c6V\x8A\xBE\x14a\x03\x1CW\x80c>\xBFy\x85\x14a\x03/W\x80cB\x16\xE7:\x14a\x03{W\x80cQ\xAD\n\x80\x14a\x03\xA2W\x80cZ\xF2\x7Fy\x14a\x03\xB5W\x80cpH\x02u\x14a\x03\xDCW\x80ct\xBC\x819\x14a\x03\xEFW\x80cu\xB28\xFC\x14a\x04\x02W[`\0\x80\xFD[a\x01\xD4a\x01\xCF6`\x04a\x0E\xB1V[a\x06fV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x10\x7FX\xA0\x04N\x0E\xCD\x81\x02^9\x8B\xF1\x81Pu\xD1#L\xBA\xC3t\x96\x14\xB0\xB3:@L.\xE2\xBA\xBF\x81V[`@Q\x90\x81R` \x01a\x01\xE0V[a\x02\x10\x7F\xF1OC\x1D\xAD\xC8.}\xBC^7\x9Fq#NW5\xC9\x18~C'\xA7\xC6\xAC\x01MU\xD1\xB7rz\x81V[a\x02Xa\x02S6`\x04a\x0E\xF7V[a\x06\x9DV[\0[a\x02\x10\x7FO\xD3\xE0Hz\x03\x82\xFB\x02|w\xB1\xAELV6r\xC9\xFB0\xA7Hy\x85_\x0C\x86\xC3v\xCF\x96\xEA\x81V[a\x02\x10a\x02\x8F6`\x04a\x0F\x12V[a\x07NV[a\x02\x10\x7F\xB1\xF7\x98\x13\xBCv0\xA5*\xE9H\xBC\x99x\x13\x97\xE4\t\xD0\xDD5!\x95;\xF7\xD8\xD7\xA2\xDBaG\xF7\x81V[a\x02\x10\x7F\xB7\xB4\xFD\xE9\x94M<\x13\xE9\xA7\x885C\x1C3\xA5\x08M\x90\xA7\xF0\xC7=\xEFv\xD7\x88c\x15\xFE\x87\xB0\x81V[a\x02\x10\x7F\xB91\xB2q\x9A\xEB*e\xA5\x03_\xA0\xA1\x90\xBF\xDCL\x86\"\xCE\x8C\xBF\xF7\xA3\xD1\xABBS\x1F\xB1\xA9\x18\x81V[a\x02Xa\x03\x176`\x04a\x0F+V[a\x07cV[a\x02Xa\x03*6`\x04a\x0F+V[a\x07\x84V[a\x03ca\x03=6`\x04a\x0FfV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE0V[a\x02\x10\x7FLA\xAEEK\xEBk\xBB\xE9\xBEP\xAC\xCC\x95z;\x156\xE4\x8B\x83Z\x86\x91\x9A\xF9\x81\xB5$M\xB7U\x81V[a\x02Xa\x03\xB06`\x04a\x0F\x89V[a\x07\xFEV[a\x02\x10\x7F\xA2\xC772\xDEez\xD0\xF3n\r\xDB\xB2q\x0FK\x13\xE8\xDD\xE4d!8k\xB9-\x1E\x17\x9D\xAEMM\x81V[a\x02Xa\x03\xEA6`\x04a\x0E\xF7V[a\t\x82V[a\x02Xa\x03\xFD6`\x04a\x0F\xC5V[a\t\xB2V[a\x02\x10`\0\x80Q` a\x11\x9D\x839\x81Q\x91R\x81V[a\x02\x10\x7Ft\x84]\xE3|\xFA\xBD5v3!KG\xFA\x91\xCC\xD1\x9B\x05\xB7\xC5\xA0\x8A\xC2,\x18\x7F\x81\x1F\xB6+\xCA\x81V[a\x02\x10\x7F\x9F5\xEF>\x0C&R\xA8\xBB\x87G\xD9/@\x7F\xCD9\xA7v\x8D\xAC\xC7\xF1e\x81\xC7\xA7\x1F\x10>Ub\x81V[a\x02\x10\x7F\xC2o\xAE\xDA\xEE\xDA/\xB9Jf\xD7\x86\xAA\x89\xC4\xA1\x8B\xB7\x90\xFA\0\x9D\x9D\xA9JT\x1D\x92\x18\\\xA9\x16\x81V[a\x02\x10\x7F\xC6gO\x98\xBA5\xC0\x1C\x13\x0E\x08\x19]\xD2lpF`7G:\x06\x8CZ\xAAG\nx=\x99\xC1l\x81V[a\x02\x10\x7F\xAEy\xA95sp\x12\xD0f\xE7\x1802i.R\x1F\xFE\x1A\xDE+\xED\xA2g\xE2>\x02\xB1\xD6\xE9\x11\x87\x81V[a\x02\x10\x7F\xAA\x06\xD1\x08\xDB\xD7\xBF\x97k\x16\xB7\xBFZ\xDB)\xD2\xD0\xEF,8\\\xA8\xB9\xD83\xCC\x80/3\x94-r\x81V[a\x02Xa\x05\x0F6`\x04a\x0F\xC5V[a\nnV[a\x03ca\x05\"6`\x04a\x0FfV[a\x0B\x12V[a\x02\x10\x7FT\x95<#\x06\x8B\x8F\xC4\xC0sc\x01\xB5\x0F\x10\x02}kF\x93'\xDE\x1F\xD4(A\xA5\x07+\x1B\xCE\xBE\x81V[a\x01\xD4a\x05\\6`\x04a\x0F+V[a\x0BhV[a\x02\x10\x7F'\xD7d\xEA*J8eCK\xBFJ9\x11\x10\x14\x96D\xBE1D\x8F4y\xFD\x15\xB4C\x88uWe\x81V[a\x02\x10`\0\x81V[a\x02\x10\x7F:h\xDB\xFD\x8B\xBBd\x01\\B\xBC\x13\x1C8\x8D\xEAye\xE2\x8C\x10\x04\xD0\x9B9\xF5\x95\0\xC3\xA7c\xEC\x81V[a\x02\x10\x7F\x0F'\xB9\xE4k\x89\xC5\xC7B\xE2\x80\x94\xDC\xEF\xE5\xE9F\xC3\xB9\x8F\x0F\xBE\xD8}\x9F\xCF[\x10\xBA\x96\x84\xEC\x81V[a\x02Xa\x05\xEC6`\x04a\x0F+V[a\x0B\x91V[a\x02\x10\x7F\x08\t\t\xC1\x8C\x95\x8C\xE5\xA2\xD3d\x81ix$\xE4w1\x93#\xD01T\xCE\xBA;x\xF2\x8Aa\x88{\x81V[a\x02\x10\x7F\xB4\xBF\x99\x9Bh\xD8\x08]\xBB\xF7\xA0\xEC/Z-f\x08s\x93[\xDF\x1E\xD0\x8E\xB4!\xACm\xCB\xC0\x03b\x81V[a\x02\x10\x7F\xDD[\x9B\x8A^\x8E\x01\xF2\x96.\xD7\xE9\x83\xD5\x8F\xE3.\x1Ff\xAA\x88\xDDz\xB3\x07p\xFA\x9Bw\xDArC\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\x97WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0\x80Q` a\x11\x9D\x839\x81Q\x91Ra\x06\xB5\x81a\x0B\xADV[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x072W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FCannot remove self as admin.  Ha`D\x82\x01Rv;2\x90:42\x9072\xBB\x900\xB26\xB4\xB7\x1027\x904\xBA\x17`I\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07J`\0\x80Q` a\x11\x9D\x839\x81Q\x91R\x83a\x0B\xBAV[PPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x07l\x82a\x07NV[a\x07u\x81a\x0B\xADV[a\x07\x7F\x83\x83a\x0C\x1FV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x07\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x07)V[a\x07J\x82\x82a\x0B\xBAV[a\x08\x16`\0\x80Q` a\x11\x9D\x839\x81Q\x91R3a\x0BhV[a\x083W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x83`\x02\x81\x11\x15a\x08IWa\x08Ia\x0F\xE0V[`\x02\x81\x11\x15a\x08ZWa\x08Za\x0F\xE0V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x08\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FThe provided Env is not valid fo`D\x82\x01Rn\x1C\x88\x1D\x1A\x1A\\\xC8\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x8A\x1B`d\x82\x01R`\x84\x01a\x07)V[\x80`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x84`\x02\x81\x11\x15a\x08\xFFWa\x08\xFFa\x0F\xE0V[`\x02\x81\x11\x15a\t\x10Wa\t\x10a\x0F\xE0V[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F3\xF0\x14\x89\x0F\x10\x92)\xBB\xCF\x8D\xD4r\x04\xC1S\xA2\xC0\xFF\x1CW*a\xDE\"\r\x103e0\xF5=\x83\x83\x83`@Qa\tu\x93\x92\x91\x90a\x10\x18V[`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x80Q` a\x11\x9D\x839\x81Q\x91Ra\t\x9A\x81a\x0B\xADV[a\x07J`\0\x80Q` a\x11\x9D\x839\x81Q\x91R\x83a\x0C\x1FV[a\t\xCA`\0\x80Q` a\x11\x9D\x839\x81Q\x91R3a\x0BhV[a\t\xE7W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80`\0\x83`\x02\x81\x11\x15a\t\xFEWa\t\xFEa\x0F\xE0V[`\x02\x81\x11\x15a\n\x0FWa\n\x0Fa\x0F\xE0V[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\nc\x91\x90a\x10EV[`@Q\x80\x91\x03\x90\xA1PV[a\n\x86`\0\x80Q` a\x11\x9D\x839\x81Q\x91R3a\x0BhV[a\n\xA3W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x82`\x02\x81\x11\x15a\n\xB9Wa\n\xB9a\x0F\xE0V[`\x02\x81\x11\x15a\n\xCAWa\n\xCAa\x0F\xE0V[\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\xFF\x19\x16\x90UQ\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x90a\nc\x90\x83\x90a\x10EV[`\0\x82\x81R`\x02` \x81\x90R`@\x82 \x90\x82\x90\x84\x90\x81\x11\x15a\x0B6Wa\x0B6a\x0F\xE0V[`\x02\x81\x11\x15a\x0BGWa\x0BGa\x0F\xE0V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x0B\x9A\x82a\x07NV[a\x0B\xA3\x81a\x0B\xADV[a\x07\x7F\x83\x83a\x0B\xBAV[a\x0B\xB7\x813a\x0C\xA3V[PV[a\x0B\xC4\x82\x82a\x0BhV[\x15a\x07JW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x0C)\x82\x82a\x0BhV[a\x07JW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0C_3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x0C\xAD\x82\x82a\x0BhV[a\x07JWa\x0C\xBA\x81a\x0C\xFCV[a\x0C\xC5\x83` a\r\x0EV[`@Q` \x01a\x0C\xD6\x92\x91\x90a\x10wV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x07)\x91`\x04\x01a\x10\xE6V[``a\x06\x97`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\r\x1D\x83`\x02a\x11/V[a\r(\x90`\x02a\x11FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r@Wa\r@a\x11YV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\rjW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\r\x85Wa\r\x85a\x11oV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\r\xB4Wa\r\xB4a\x11oV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\r\xD8\x84`\x02a\x11/V[a\r\xE3\x90`\x01a\x11FV[\x90P[`\x01\x81\x11\x15a\x0E[Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0E\x17Wa\x0E\x17a\x11oV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0E-Wa\x0E-a\x11oV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0ET\x81a\x11\x85V[\x90Pa\r\xE6V[P\x83\x15a\x0E\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x07)V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0E\xC3W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\xAAW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xF2W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\tW`\0\x80\xFD[a\x0E\xAA\x82a\x0E\xDBV[`\0` \x82\x84\x03\x12\x15a\x0F$W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F>W`\0\x80\xFD[\x825\x91Pa\x0FN` \x84\x01a\x0E\xDBV[\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a\x0E\xF2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0FyW`\0\x80\xFD[\x825\x91Pa\x0FN` \x84\x01a\x0FWV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x9EW`\0\x80\xFD[\x835\x92Pa\x0F\xAE` \x85\x01a\x0FWV[\x91Pa\x0F\xBC`@\x85\x01a\x0E\xDBV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD7W`\0\x80\xFD[a\x0E\xAA\x82a\x0FWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x10\x14WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x83\x81R``\x81\x01a\x10,` \x83\x01\x85a\x0F\xF6V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x92\x91PPV[` \x81\x01a\x06\x97\x82\x84a\x0F\xF6V[`\0[\x83\x81\x10\x15a\x10nW\x81\x81\x01Q\x83\x82\x01R` \x01a\x10VV[PP`\0\x91\x01RV[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa\x10\xA9\x81`\x17\x85\x01` \x88\x01a\x10SV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x10\xDA\x81`(\x84\x01` \x88\x01a\x10SV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x11\x05\x81`@\x85\x01` \x87\x01a\x10SV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x97Wa\x06\x97a\x11\x19V[\x80\x82\x01\x80\x82\x11\x15a\x06\x97Wa\x06\x97a\x11\x19V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x11\x94Wa\x11\x94a\x11\x19V[P`\0\x19\x01\x90V\xFE\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\xA2dipfsX\"\x12 \xBC\xFBZ\xA3%\x1D\xDF3\xA8sl\x96\x89\xBD\x99\xCE\xA1\xDF[\xAA\x0F\xAC8|Ui\x13rX%\xC9rdsolcC\0\x08\x1C\x003\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB";
    /// The bytecode of the contract.
    pub static CONTRACTRESOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xBCW`\x005`\xE0\x1C\x80c|\xAD\xF6\x9F\x11a\0\xF5W\x80c|\xAD\xF6\x9F\x14a\x04\x17W\x80c}J\x03\xBD\x14a\x04>W\x80c}\x9D(\x80\x14a\x04eW\x80c\x7F\x90 \x9F\x14a\x04\x8CW\x80c\x85\xCB\x11\x91\x14a\x04\xB3W\x80c\x8C\x156\xDF\x14a\x04\xDAW\x80c\x8D\xEB8\x93\x14a\x05\x01W\x80c\x8E\x8D\xFD\x16\x14a\x05\x14W\x80c\x90r\xF88\x14a\x05'W\x80c\x91\xD1HT\x14a\x05NW\x80c\x97z\x80p\x14a\x05aW\x80c\xA2\x17\xFD\xDF\x14a\x05\x88W\x80c\xAD\x1C\x8A\x86\x14a\x05\x90W\x80c\xCD\xDC\xAC\xE5\x14a\x05\xB7W\x80c\xD5Gt\x1F\x14a\x05\xDEW\x80c\xDA\x19\xDD\xFB\x14a\x05\xF1W\x80c\xDF8\x06\x93\x14a\x06\x18W\x80c\xF8\xAE\x93\xB4\x14a\x06?W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xC1W\x80c\x11\xEE\x8F\xF7\x14a\x01\xE9W\x80c\x16\xF7k\xBF\x14a\x02\x1EW\x80c\x17\x85\xF5<\x14a\x02EW\x80c!\x9C&j\x14a\x02ZW\x80c$\x8A\x9C\xA3\x14a\x02\x81W\x80c&h\xF3\x05\x14a\x02\x94W\x80c,\x0B\x8B\xF7\x14a\x02\xBBW\x80c.H\x85\xE8\x14a\x02\xE2W\x80c//\xF1]\x14a\x03\tW\x80c6V\x8A\xBE\x14a\x03\x1CW\x80c>\xBFy\x85\x14a\x03/W\x80cB\x16\xE7:\x14a\x03{W\x80cQ\xAD\n\x80\x14a\x03\xA2W\x80cZ\xF2\x7Fy\x14a\x03\xB5W\x80cpH\x02u\x14a\x03\xDCW\x80ct\xBC\x819\x14a\x03\xEFW\x80cu\xB28\xFC\x14a\x04\x02W[`\0\x80\xFD[a\x01\xD4a\x01\xCF6`\x04a\x0E\xB1V[a\x06fV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x10\x7FX\xA0\x04N\x0E\xCD\x81\x02^9\x8B\xF1\x81Pu\xD1#L\xBA\xC3t\x96\x14\xB0\xB3:@L.\xE2\xBA\xBF\x81V[`@Q\x90\x81R` \x01a\x01\xE0V[a\x02\x10\x7F\xF1OC\x1D\xAD\xC8.}\xBC^7\x9Fq#NW5\xC9\x18~C'\xA7\xC6\xAC\x01MU\xD1\xB7rz\x81V[a\x02Xa\x02S6`\x04a\x0E\xF7V[a\x06\x9DV[\0[a\x02\x10\x7FO\xD3\xE0Hz\x03\x82\xFB\x02|w\xB1\xAELV6r\xC9\xFB0\xA7Hy\x85_\x0C\x86\xC3v\xCF\x96\xEA\x81V[a\x02\x10a\x02\x8F6`\x04a\x0F\x12V[a\x07NV[a\x02\x10\x7F\xB1\xF7\x98\x13\xBCv0\xA5*\xE9H\xBC\x99x\x13\x97\xE4\t\xD0\xDD5!\x95;\xF7\xD8\xD7\xA2\xDBaG\xF7\x81V[a\x02\x10\x7F\xB7\xB4\xFD\xE9\x94M<\x13\xE9\xA7\x885C\x1C3\xA5\x08M\x90\xA7\xF0\xC7=\xEFv\xD7\x88c\x15\xFE\x87\xB0\x81V[a\x02\x10\x7F\xB91\xB2q\x9A\xEB*e\xA5\x03_\xA0\xA1\x90\xBF\xDCL\x86\"\xCE\x8C\xBF\xF7\xA3\xD1\xABBS\x1F\xB1\xA9\x18\x81V[a\x02Xa\x03\x176`\x04a\x0F+V[a\x07cV[a\x02Xa\x03*6`\x04a\x0F+V[a\x07\x84V[a\x03ca\x03=6`\x04a\x0FfV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE0V[a\x02\x10\x7FLA\xAEEK\xEBk\xBB\xE9\xBEP\xAC\xCC\x95z;\x156\xE4\x8B\x83Z\x86\x91\x9A\xF9\x81\xB5$M\xB7U\x81V[a\x02Xa\x03\xB06`\x04a\x0F\x89V[a\x07\xFEV[a\x02\x10\x7F\xA2\xC772\xDEez\xD0\xF3n\r\xDB\xB2q\x0FK\x13\xE8\xDD\xE4d!8k\xB9-\x1E\x17\x9D\xAEMM\x81V[a\x02Xa\x03\xEA6`\x04a\x0E\xF7V[a\t\x82V[a\x02Xa\x03\xFD6`\x04a\x0F\xC5V[a\t\xB2V[a\x02\x10`\0\x80Q` a\x11\x9D\x839\x81Q\x91R\x81V[a\x02\x10\x7Ft\x84]\xE3|\xFA\xBD5v3!KG\xFA\x91\xCC\xD1\x9B\x05\xB7\xC5\xA0\x8A\xC2,\x18\x7F\x81\x1F\xB6+\xCA\x81V[a\x02\x10\x7F\x9F5\xEF>\x0C&R\xA8\xBB\x87G\xD9/@\x7F\xCD9\xA7v\x8D\xAC\xC7\xF1e\x81\xC7\xA7\x1F\x10>Ub\x81V[a\x02\x10\x7F\xC2o\xAE\xDA\xEE\xDA/\xB9Jf\xD7\x86\xAA\x89\xC4\xA1\x8B\xB7\x90\xFA\0\x9D\x9D\xA9JT\x1D\x92\x18\\\xA9\x16\x81V[a\x02\x10\x7F\xC6gO\x98\xBA5\xC0\x1C\x13\x0E\x08\x19]\xD2lpF`7G:\x06\x8CZ\xAAG\nx=\x99\xC1l\x81V[a\x02\x10\x7F\xAEy\xA95sp\x12\xD0f\xE7\x1802i.R\x1F\xFE\x1A\xDE+\xED\xA2g\xE2>\x02\xB1\xD6\xE9\x11\x87\x81V[a\x02\x10\x7F\xAA\x06\xD1\x08\xDB\xD7\xBF\x97k\x16\xB7\xBFZ\xDB)\xD2\xD0\xEF,8\\\xA8\xB9\xD83\xCC\x80/3\x94-r\x81V[a\x02Xa\x05\x0F6`\x04a\x0F\xC5V[a\nnV[a\x03ca\x05\"6`\x04a\x0FfV[a\x0B\x12V[a\x02\x10\x7FT\x95<#\x06\x8B\x8F\xC4\xC0sc\x01\xB5\x0F\x10\x02}kF\x93'\xDE\x1F\xD4(A\xA5\x07+\x1B\xCE\xBE\x81V[a\x01\xD4a\x05\\6`\x04a\x0F+V[a\x0BhV[a\x02\x10\x7F'\xD7d\xEA*J8eCK\xBFJ9\x11\x10\x14\x96D\xBE1D\x8F4y\xFD\x15\xB4C\x88uWe\x81V[a\x02\x10`\0\x81V[a\x02\x10\x7F:h\xDB\xFD\x8B\xBBd\x01\\B\xBC\x13\x1C8\x8D\xEAye\xE2\x8C\x10\x04\xD0\x9B9\xF5\x95\0\xC3\xA7c\xEC\x81V[a\x02\x10\x7F\x0F'\xB9\xE4k\x89\xC5\xC7B\xE2\x80\x94\xDC\xEF\xE5\xE9F\xC3\xB9\x8F\x0F\xBE\xD8}\x9F\xCF[\x10\xBA\x96\x84\xEC\x81V[a\x02Xa\x05\xEC6`\x04a\x0F+V[a\x0B\x91V[a\x02\x10\x7F\x08\t\t\xC1\x8C\x95\x8C\xE5\xA2\xD3d\x81ix$\xE4w1\x93#\xD01T\xCE\xBA;x\xF2\x8Aa\x88{\x81V[a\x02\x10\x7F\xB4\xBF\x99\x9Bh\xD8\x08]\xBB\xF7\xA0\xEC/Z-f\x08s\x93[\xDF\x1E\xD0\x8E\xB4!\xACm\xCB\xC0\x03b\x81V[a\x02\x10\x7F\xDD[\x9B\x8A^\x8E\x01\xF2\x96.\xD7\xE9\x83\xD5\x8F\xE3.\x1Ff\xAA\x88\xDDz\xB3\x07p\xFA\x9Bw\xDArC\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\x97WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\0\x80Q` a\x11\x9D\x839\x81Q\x91Ra\x06\xB5\x81a\x0B\xADV[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x072W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FCannot remove self as admin.  Ha`D\x82\x01Rv;2\x90:42\x9072\xBB\x900\xB26\xB4\xB7\x1027\x904\xBA\x17`I\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x07J`\0\x80Q` a\x11\x9D\x839\x81Q\x91R\x83a\x0B\xBAV[PPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x07l\x82a\x07NV[a\x07u\x81a\x0B\xADV[a\x07\x7F\x83\x83a\x0C\x1FV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x07\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x07)V[a\x07J\x82\x82a\x0B\xBAV[a\x08\x16`\0\x80Q` a\x11\x9D\x839\x81Q\x91R3a\x0BhV[a\x083W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x83`\x02\x81\x11\x15a\x08IWa\x08Ia\x0F\xE0V[`\x02\x81\x11\x15a\x08ZWa\x08Za\x0F\xE0V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x08\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FThe provided Env is not valid fo`D\x82\x01Rn\x1C\x88\x1D\x1A\x1A\\\xC8\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x8A\x1B`d\x82\x01R`\x84\x01a\x07)V[\x80`\x02`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x84`\x02\x81\x11\x15a\x08\xFFWa\x08\xFFa\x0F\xE0V[`\x02\x81\x11\x15a\t\x10Wa\t\x10a\x0F\xE0V[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F3\xF0\x14\x89\x0F\x10\x92)\xBB\xCF\x8D\xD4r\x04\xC1S\xA2\xC0\xFF\x1CW*a\xDE\"\r\x103e0\xF5=\x83\x83\x83`@Qa\tu\x93\x92\x91\x90a\x10\x18V[`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x80Q` a\x11\x9D\x839\x81Q\x91Ra\t\x9A\x81a\x0B\xADV[a\x07J`\0\x80Q` a\x11\x9D\x839\x81Q\x91R\x83a\x0C\x1FV[a\t\xCA`\0\x80Q` a\x11\x9D\x839\x81Q\x91R3a\x0BhV[a\t\xE7W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x80`\0\x83`\x02\x81\x11\x15a\t\xFEWa\t\xFEa\x0F\xE0V[`\x02\x81\x11\x15a\n\x0FWa\n\x0Fa\x0F\xE0V[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\nc\x91\x90a\x10EV[`@Q\x80\x91\x03\x90\xA1PV[a\n\x86`\0\x80Q` a\x11\x9D\x839\x81Q\x91R3a\x0BhV[a\n\xA3W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0\x82`\x02\x81\x11\x15a\n\xB9Wa\n\xB9a\x0F\xE0V[`\x02\x81\x11\x15a\n\xCAWa\n\xCAa\x0F\xE0V[\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\xFF\x19\x16\x90UQ\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x90a\nc\x90\x83\x90a\x10EV[`\0\x82\x81R`\x02` \x81\x90R`@\x82 \x90\x82\x90\x84\x90\x81\x11\x15a\x0B6Wa\x0B6a\x0F\xE0V[`\x02\x81\x11\x15a\x0BGWa\x0BGa\x0F\xE0V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x0B\x9A\x82a\x07NV[a\x0B\xA3\x81a\x0B\xADV[a\x07\x7F\x83\x83a\x0B\xBAV[a\x0B\xB7\x813a\x0C\xA3V[PV[a\x0B\xC4\x82\x82a\x0BhV[\x15a\x07JW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x0C)\x82\x82a\x0BhV[a\x07JW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0C_3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x0C\xAD\x82\x82a\x0BhV[a\x07JWa\x0C\xBA\x81a\x0C\xFCV[a\x0C\xC5\x83` a\r\x0EV[`@Q` \x01a\x0C\xD6\x92\x91\x90a\x10wV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x07)\x91`\x04\x01a\x10\xE6V[``a\x06\x97`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\r\x1D\x83`\x02a\x11/V[a\r(\x90`\x02a\x11FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r@Wa\r@a\x11YV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\rjW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\r\x85Wa\r\x85a\x11oV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\r\xB4Wa\r\xB4a\x11oV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\r\xD8\x84`\x02a\x11/V[a\r\xE3\x90`\x01a\x11FV[\x90P[`\x01\x81\x11\x15a\x0E[Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0E\x17Wa\x0E\x17a\x11oV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0E-Wa\x0E-a\x11oV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0ET\x81a\x11\x85V[\x90Pa\r\xE6V[P\x83\x15a\x0E\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x07)V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0E\xC3W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\xAAW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xF2W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\tW`\0\x80\xFD[a\x0E\xAA\x82a\x0E\xDBV[`\0` \x82\x84\x03\x12\x15a\x0F$W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F>W`\0\x80\xFD[\x825\x91Pa\x0FN` \x84\x01a\x0E\xDBV[\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a\x0E\xF2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0FyW`\0\x80\xFD[\x825\x91Pa\x0FN` \x84\x01a\x0FWV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x9EW`\0\x80\xFD[\x835\x92Pa\x0F\xAE` \x85\x01a\x0FWV[\x91Pa\x0F\xBC`@\x85\x01a\x0E\xDBV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\xD7W`\0\x80\xFD[a\x0E\xAA\x82a\x0FWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x10\x14WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x83\x81R``\x81\x01a\x10,` \x83\x01\x85a\x0F\xF6V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x92\x91PPV[` \x81\x01a\x06\x97\x82\x84a\x0F\xF6V[`\0[\x83\x81\x10\x15a\x10nW\x81\x81\x01Q\x83\x82\x01R` \x01a\x10VV[PP`\0\x91\x01RV[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa\x10\xA9\x81`\x17\x85\x01` \x88\x01a\x10SV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x10\xDA\x81`(\x84\x01` \x88\x01a\x10SV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x11\x05\x81`@\x85\x01` \x87\x01a\x10SV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x97Wa\x06\x97a\x11\x19V[\x80\x82\x01\x80\x82\x11\x15a\x06\x97Wa\x06\x97a\x11\x19V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x11\x94Wa\x11\x94a\x11\x19V[P`\0\x19\x01\x90V\xFE\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\xA2dipfsX\"\x12 \xBC\xFBZ\xA3%\x1D\xDF3\xA8sl\x96\x89\xBD\x99\xCE\xA1\xDF[\xAA\x0F\xAC8|Ui\x13rX%\xC9rdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static CONTRACTRESOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ContractResolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ContractResolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ContractResolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ContractResolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ContractResolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ContractResolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ContractResolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONTRACTRESOLVER_ABI.clone(),
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
                CONTRACTRESOLVER_ABI.clone(),
                CONTRACTRESOLVER_BYTECODE.clone().into(),
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
        ///Calls the contract's `ALLOWLIST_CONTRACT` (0x7cadf69f) function
        pub fn allowlist_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([124, 173, 246, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BACKUP_RECOVERY_CONTRACT` (0x7d4a03bd) function
        pub fn backup_recovery_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([125, 74, 3, 189], ())
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
        ///Calls the contract's `DOMAIN_WALLET_REGISTRY` (0x4216e73a) function
        pub fn domain_wallet_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([66, 22, 231, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FORWARDER_CONTRACT` (0x7d9d2880) function
        pub fn forwarder_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([125, 157, 40, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HD_KEY_DERIVER_CONTRACT` (0x85cb1191) function
        pub fn hd_key_deriver_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([133, 203, 17, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HOST_COMMANDS_CONTRACT` (0x5af27f79) function
        pub fn host_commands_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([90, 242, 127, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LEDGER_CONTRACT` (0xcddcace5) function
        pub fn ledger_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([205, 220, 172, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIT_TOKEN_CONTRACT` (0xdf380693) function
        pub fn lit_token_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([223, 56, 6, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MULTI_SENDER_CONTRACT` (0xf8ae93b4) function
        pub fn multi_sender_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([248, 174, 147, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PAYMENT_DELEGATION_CONTRACT` (0x7f90209f) function
        pub fn payment_delegation_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([127, 144, 32, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_HELPER_CONTRACT` (0x977a8070) function
        pub fn pkp_helper_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 122, 128, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_HELPER_V2_CONTRACT` (0x11ee8ff7) function
        pub fn pkp_helper_v2_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([17, 238, 143, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_NFT_CONTRACT` (0x2c0b8bf7) function
        pub fn pkp_nft_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([44, 11, 139, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_NFT_METADATA_CONTRACT` (0x16f76bbf) function
        pub fn pkp_nft_metadata_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([22, 247, 107, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PKP_PERMISSIONS_CONTRACT` (0x9072f838) function
        pub fn pkp_permissions_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([144, 114, 248, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PRICE_FEED_CONTRACT` (0x219c266a) function
        pub fn price_feed_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([33, 156, 38, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PUB_KEY_ROUTER_CONTRACT` (0x2668f305) function
        pub fn pub_key_router_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([38, 104, 243, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RATE_LIMIT_NFT_CONTRACT` (0x2e4885e8) function
        pub fn rate_limit_nft_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([46, 72, 133, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELEASE_REGISTER_CONTRACT` (0xad1c8a86) function
        pub fn release_register_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([173, 28, 138, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_BALANCES_CONTRACT` (0x8c1536df) function
        pub fn staking_balances_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([140, 21, 54, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_CONTRACT` (0xda19ddfb) function
        pub fn staking_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([218, 25, 221, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAdmin` (0x70480275) function
        pub fn add_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 72, 2, 117], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAllowedEnv` (0x74bc8139) function
        pub fn add_allowed_env(
            &self,
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 188, 129, 57], env)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getContract` (0x8e8dfd16) function
        pub fn get_contract(
            &self,
            typ: [u8; 32],
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([142, 141, 253, 22], (typ, env))
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
        ///Calls the contract's `removeAdmin` (0x1785f53c) function
        pub fn remove_admin(
            &self,
            admin_being_removed: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 133, 245, 60], admin_being_removed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAllowedEnv` (0x8deb3893) function
        pub fn remove_allowed_env(
            &self,
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 235, 56, 147], env)
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
        ///Calls the contract's `setContract` (0x51ad0a80) function
        pub fn set_contract(
            &self,
            typ: [u8; 32],
            env: u8,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 173, 10, 128], (typ, env, addr))
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
        ///Calls the contract's `typeAddresses` (0x3ebf7985) function
        pub fn type_addresses(
            &self,
            p0: [u8; 32],
            p1: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([62, 191, 121, 133], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AllowedEnvAdded` event
        pub fn allowed_env_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedEnvAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedEnvRemoved` event
        pub fn allowed_env_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedEnvRemovedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `SetContract` event
        pub fn set_contract_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetContractFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractResolverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ContractResolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AdminRoleRequired` with signature `AdminRoleRequired()` and selector `0xc890f84a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AdminRoleRequired", abi = "AdminRoleRequired()")]
    pub struct AdminRoleRequired;
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
    #[ethevent(name = "AllowedEnvAdded", abi = "AllowedEnvAdded(uint8)")]
    pub struct AllowedEnvAddedFilter {
        pub env: u8,
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
    #[ethevent(name = "AllowedEnvRemoved", abi = "AllowedEnvRemoved(uint8)")]
    pub struct AllowedEnvRemovedFilter {
        pub env: u8,
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
    #[ethevent(name = "SetContract", abi = "SetContract(bytes32,uint8,address)")]
    pub struct SetContractFilter {
        pub typ: [u8; 32],
        pub env: u8,
        pub addr: ::ethers::core::types::Address,
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
    pub enum ContractResolverEvents {
        AllowedEnvAddedFilter(AllowedEnvAddedFilter),
        AllowedEnvRemovedFilter(AllowedEnvRemovedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SetContractFilter(SetContractFilter),
    }
    impl ::ethers::contract::EthLogDecode for ContractResolverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllowedEnvAddedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::AllowedEnvAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowedEnvRemovedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::AllowedEnvRemovedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ContractResolverEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SetContractFilter::decode_log(log) {
                return Ok(ContractResolverEvents::SetContractFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ContractResolverEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowedEnvAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedEnvRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowedEnvAddedFilter> for ContractResolverEvents {
        fn from(value: AllowedEnvAddedFilter) -> Self {
            Self::AllowedEnvAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedEnvRemovedFilter> for ContractResolverEvents {
        fn from(value: AllowedEnvRemovedFilter) -> Self {
            Self::AllowedEnvRemovedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ContractResolverEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ContractResolverEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ContractResolverEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<SetContractFilter> for ContractResolverEvents {
        fn from(value: SetContractFilter) -> Self {
            Self::SetContractFilter(value)
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
    ///Container type for all input parameters for the `ALLOWLIST_CONTRACT` function with signature `ALLOWLIST_CONTRACT()` and selector `0x7cadf69f`
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
    #[ethcall(name = "ALLOWLIST_CONTRACT", abi = "ALLOWLIST_CONTRACT()")]
    pub struct AllowlistContractCall;
    ///Container type for all input parameters for the `BACKUP_RECOVERY_CONTRACT` function with signature `BACKUP_RECOVERY_CONTRACT()` and selector `0x7d4a03bd`
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
    #[ethcall(name = "BACKUP_RECOVERY_CONTRACT", abi = "BACKUP_RECOVERY_CONTRACT()")]
    pub struct BackupRecoveryContractCall;
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
    ///Container type for all input parameters for the `DOMAIN_WALLET_REGISTRY` function with signature `DOMAIN_WALLET_REGISTRY()` and selector `0x4216e73a`
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
    #[ethcall(name = "DOMAIN_WALLET_REGISTRY", abi = "DOMAIN_WALLET_REGISTRY()")]
    pub struct DomainWalletRegistryCall;
    ///Container type for all input parameters for the `FORWARDER_CONTRACT` function with signature `FORWARDER_CONTRACT()` and selector `0x7d9d2880`
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
    #[ethcall(name = "FORWARDER_CONTRACT", abi = "FORWARDER_CONTRACT()")]
    pub struct ForwarderContractCall;
    ///Container type for all input parameters for the `HD_KEY_DERIVER_CONTRACT` function with signature `HD_KEY_DERIVER_CONTRACT()` and selector `0x85cb1191`
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
    #[ethcall(name = "HD_KEY_DERIVER_CONTRACT", abi = "HD_KEY_DERIVER_CONTRACT()")]
    pub struct HdKeyDeriverContractCall;
    ///Container type for all input parameters for the `HOST_COMMANDS_CONTRACT` function with signature `HOST_COMMANDS_CONTRACT()` and selector `0x5af27f79`
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
    #[ethcall(name = "HOST_COMMANDS_CONTRACT", abi = "HOST_COMMANDS_CONTRACT()")]
    pub struct HostCommandsContractCall;
    ///Container type for all input parameters for the `LEDGER_CONTRACT` function with signature `LEDGER_CONTRACT()` and selector `0xcddcace5`
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
    #[ethcall(name = "LEDGER_CONTRACT", abi = "LEDGER_CONTRACT()")]
    pub struct LedgerContractCall;
    ///Container type for all input parameters for the `LIT_TOKEN_CONTRACT` function with signature `LIT_TOKEN_CONTRACT()` and selector `0xdf380693`
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
    #[ethcall(name = "LIT_TOKEN_CONTRACT", abi = "LIT_TOKEN_CONTRACT()")]
    pub struct LitTokenContractCall;
    ///Container type for all input parameters for the `MULTI_SENDER_CONTRACT` function with signature `MULTI_SENDER_CONTRACT()` and selector `0xf8ae93b4`
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
    #[ethcall(name = "MULTI_SENDER_CONTRACT", abi = "MULTI_SENDER_CONTRACT()")]
    pub struct MultiSenderContractCall;
    ///Container type for all input parameters for the `PAYMENT_DELEGATION_CONTRACT` function with signature `PAYMENT_DELEGATION_CONTRACT()` and selector `0x7f90209f`
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
        name = "PAYMENT_DELEGATION_CONTRACT",
        abi = "PAYMENT_DELEGATION_CONTRACT()"
    )]
    pub struct PaymentDelegationContractCall;
    ///Container type for all input parameters for the `PKP_HELPER_CONTRACT` function with signature `PKP_HELPER_CONTRACT()` and selector `0x977a8070`
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
    #[ethcall(name = "PKP_HELPER_CONTRACT", abi = "PKP_HELPER_CONTRACT()")]
    pub struct PkpHelperContractCall;
    ///Container type for all input parameters for the `PKP_HELPER_V2_CONTRACT` function with signature `PKP_HELPER_V2_CONTRACT()` and selector `0x11ee8ff7`
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
    #[ethcall(name = "PKP_HELPER_V2_CONTRACT", abi = "PKP_HELPER_V2_CONTRACT()")]
    pub struct PkpHelperV2ContractCall;
    ///Container type for all input parameters for the `PKP_NFT_CONTRACT` function with signature `PKP_NFT_CONTRACT()` and selector `0x2c0b8bf7`
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
    #[ethcall(name = "PKP_NFT_CONTRACT", abi = "PKP_NFT_CONTRACT()")]
    pub struct PkpNftContractCall;
    ///Container type for all input parameters for the `PKP_NFT_METADATA_CONTRACT` function with signature `PKP_NFT_METADATA_CONTRACT()` and selector `0x16f76bbf`
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
    #[ethcall(name = "PKP_NFT_METADATA_CONTRACT", abi = "PKP_NFT_METADATA_CONTRACT()")]
    pub struct PkpNftMetadataContractCall;
    ///Container type for all input parameters for the `PKP_PERMISSIONS_CONTRACT` function with signature `PKP_PERMISSIONS_CONTRACT()` and selector `0x9072f838`
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
    #[ethcall(name = "PKP_PERMISSIONS_CONTRACT", abi = "PKP_PERMISSIONS_CONTRACT()")]
    pub struct PkpPermissionsContractCall;
    ///Container type for all input parameters for the `PRICE_FEED_CONTRACT` function with signature `PRICE_FEED_CONTRACT()` and selector `0x219c266a`
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
    #[ethcall(name = "PRICE_FEED_CONTRACT", abi = "PRICE_FEED_CONTRACT()")]
    pub struct PriceFeedContractCall;
    ///Container type for all input parameters for the `PUB_KEY_ROUTER_CONTRACT` function with signature `PUB_KEY_ROUTER_CONTRACT()` and selector `0x2668f305`
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
    #[ethcall(name = "PUB_KEY_ROUTER_CONTRACT", abi = "PUB_KEY_ROUTER_CONTRACT()")]
    pub struct PubKeyRouterContractCall;
    ///Container type for all input parameters for the `RATE_LIMIT_NFT_CONTRACT` function with signature `RATE_LIMIT_NFT_CONTRACT()` and selector `0x2e4885e8`
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
    #[ethcall(name = "RATE_LIMIT_NFT_CONTRACT", abi = "RATE_LIMIT_NFT_CONTRACT()")]
    pub struct RateLimitNftContractCall;
    ///Container type for all input parameters for the `RELEASE_REGISTER_CONTRACT` function with signature `RELEASE_REGISTER_CONTRACT()` and selector `0xad1c8a86`
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
    #[ethcall(name = "RELEASE_REGISTER_CONTRACT", abi = "RELEASE_REGISTER_CONTRACT()")]
    pub struct ReleaseRegisterContractCall;
    ///Container type for all input parameters for the `STAKING_BALANCES_CONTRACT` function with signature `STAKING_BALANCES_CONTRACT()` and selector `0x8c1536df`
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
    #[ethcall(name = "STAKING_BALANCES_CONTRACT", abi = "STAKING_BALANCES_CONTRACT()")]
    pub struct StakingBalancesContractCall;
    ///Container type for all input parameters for the `STAKING_CONTRACT` function with signature `STAKING_CONTRACT()` and selector `0xda19ddfb`
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
    #[ethcall(name = "STAKING_CONTRACT", abi = "STAKING_CONTRACT()")]
    pub struct StakingContractCall;
    ///Container type for all input parameters for the `addAdmin` function with signature `addAdmin(address)` and selector `0x70480275`
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
    #[ethcall(name = "addAdmin", abi = "addAdmin(address)")]
    pub struct AddAdminCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addAllowedEnv` function with signature `addAllowedEnv(uint8)` and selector `0x74bc8139`
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
    #[ethcall(name = "addAllowedEnv", abi = "addAllowedEnv(uint8)")]
    pub struct AddAllowedEnvCall {
        pub env: u8,
    }
    ///Container type for all input parameters for the `getContract` function with signature `getContract(bytes32,uint8)` and selector `0x8e8dfd16`
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
    #[ethcall(name = "getContract", abi = "getContract(bytes32,uint8)")]
    pub struct GetContractCall {
        pub typ: [u8; 32],
        pub env: u8,
    }
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
    ///Container type for all input parameters for the `removeAdmin` function with signature `removeAdmin(address)` and selector `0x1785f53c`
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
    #[ethcall(name = "removeAdmin", abi = "removeAdmin(address)")]
    pub struct RemoveAdminCall {
        pub admin_being_removed: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeAllowedEnv` function with signature `removeAllowedEnv(uint8)` and selector `0x8deb3893`
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
    #[ethcall(name = "removeAllowedEnv", abi = "removeAllowedEnv(uint8)")]
    pub struct RemoveAllowedEnvCall {
        pub env: u8,
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
    ///Container type for all input parameters for the `setContract` function with signature `setContract(bytes32,uint8,address)` and selector `0x51ad0a80`
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
    #[ethcall(name = "setContract", abi = "setContract(bytes32,uint8,address)")]
    pub struct SetContractCall {
        pub typ: [u8; 32],
        pub env: u8,
        pub addr: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `typeAddresses` function with signature `typeAddresses(bytes32,uint8)` and selector `0x3ebf7985`
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
    #[ethcall(name = "typeAddresses", abi = "typeAddresses(bytes32,uint8)")]
    pub struct TypeAddressesCall(pub [u8; 32], pub u8);
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
    pub enum ContractResolverCalls {
        AdminRole(AdminRoleCall),
        AllowlistContract(AllowlistContractCall),
        BackupRecoveryContract(BackupRecoveryContractCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        DomainWalletRegistry(DomainWalletRegistryCall),
        ForwarderContract(ForwarderContractCall),
        HdKeyDeriverContract(HdKeyDeriverContractCall),
        HostCommandsContract(HostCommandsContractCall),
        LedgerContract(LedgerContractCall),
        LitTokenContract(LitTokenContractCall),
        MultiSenderContract(MultiSenderContractCall),
        PaymentDelegationContract(PaymentDelegationContractCall),
        PkpHelperContract(PkpHelperContractCall),
        PkpHelperV2Contract(PkpHelperV2ContractCall),
        PkpNftContract(PkpNftContractCall),
        PkpNftMetadataContract(PkpNftMetadataContractCall),
        PkpPermissionsContract(PkpPermissionsContractCall),
        PriceFeedContract(PriceFeedContractCall),
        PubKeyRouterContract(PubKeyRouterContractCall),
        RateLimitNftContract(RateLimitNftContractCall),
        ReleaseRegisterContract(ReleaseRegisterContractCall),
        StakingBalancesContract(StakingBalancesContractCall),
        StakingContract(StakingContractCall),
        AddAdmin(AddAdminCall),
        AddAllowedEnv(AddAllowedEnvCall),
        GetContract(GetContractCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RemoveAdmin(RemoveAdminCall),
        RemoveAllowedEnv(RemoveAllowedEnvCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetContract(SetContractCall),
        SupportsInterface(SupportsInterfaceCall),
        TypeAddresses(TypeAddressesCall),
    }
    impl ::ethers::core::abi::AbiDecode for ContractResolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded) = <AllowlistContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowlistContract(decoded));
            }
            if let Ok(decoded) = <BackupRecoveryContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BackupRecoveryContract(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <DomainWalletRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainWalletRegistry(decoded));
            }
            if let Ok(decoded) = <ForwarderContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ForwarderContract(decoded));
            }
            if let Ok(decoded) = <HdKeyDeriverContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKeyDeriverContract(decoded));
            }
            if let Ok(decoded) = <HostCommandsContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HostCommandsContract(decoded));
            }
            if let Ok(decoded) = <LedgerContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LedgerContract(decoded));
            }
            if let Ok(decoded) = <LitTokenContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LitTokenContract(decoded));
            }
            if let Ok(decoded) = <MultiSenderContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MultiSenderContract(decoded));
            }
            if let Ok(decoded) = <PaymentDelegationContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PaymentDelegationContract(decoded));
            }
            if let Ok(decoded) = <PkpHelperContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpHelperContract(decoded));
            }
            if let Ok(decoded) = <PkpHelperV2ContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpHelperV2Contract(decoded));
            }
            if let Ok(decoded) = <PkpNftContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpNftContract(decoded));
            }
            if let Ok(decoded) = <PkpNftMetadataContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpNftMetadataContract(decoded));
            }
            if let Ok(decoded) = <PkpPermissionsContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PkpPermissionsContract(decoded));
            }
            if let Ok(decoded) = <PriceFeedContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PriceFeedContract(decoded));
            }
            if let Ok(decoded) = <PubKeyRouterContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubKeyRouterContract(decoded));
            }
            if let Ok(decoded) = <RateLimitNftContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RateLimitNftContract(decoded));
            }
            if let Ok(decoded) = <ReleaseRegisterContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseRegisterContract(decoded));
            }
            if let Ok(decoded) = <StakingBalancesContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingBalancesContract(decoded));
            }
            if let Ok(decoded) = <StakingContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingContract(decoded));
            }
            if let Ok(decoded) = <AddAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAdmin(decoded));
            }
            if let Ok(decoded) = <AddAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAllowedEnv(decoded));
            }
            if let Ok(decoded) = <GetContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetContract(decoded));
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
            if let Ok(decoded) = <RemoveAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAdmin(decoded));
            }
            if let Ok(decoded) = <RemoveAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAllowedEnv(decoded));
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
            if let Ok(decoded) = <SetContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetContract(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TypeAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TypeAddresses(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ContractResolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowlistContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BackupRecoveryContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainWalletRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForwarderContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HdKeyDeriverContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HostCommandsContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LedgerContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LitTokenContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiSenderContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PaymentDelegationContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpHelperContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpHelperV2Contract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpNftContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpNftMetadataContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PkpPermissionsContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceFeedContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubKeyRouterContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RateLimitNftContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseRegisterContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingBalancesContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TypeAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ContractResolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowlistContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::BackupRecoveryContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainWalletRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ForwarderContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::HdKeyDeriverContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HostCommandsContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LedgerContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::LitTokenContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiSenderContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PaymentDelegationContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PkpHelperContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::PkpHelperV2Contract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PkpNftContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::PkpNftMetadataContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PkpPermissionsContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PriceFeedContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubKeyRouterContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RateLimitNftContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReleaseRegisterContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingBalancesContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TypeAddresses(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminRoleCall> for ContractResolverCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<AllowlistContractCall> for ContractResolverCalls {
        fn from(value: AllowlistContractCall) -> Self {
            Self::AllowlistContract(value)
        }
    }
    impl ::core::convert::From<BackupRecoveryContractCall> for ContractResolverCalls {
        fn from(value: BackupRecoveryContractCall) -> Self {
            Self::BackupRecoveryContract(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ContractResolverCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<DomainWalletRegistryCall> for ContractResolverCalls {
        fn from(value: DomainWalletRegistryCall) -> Self {
            Self::DomainWalletRegistry(value)
        }
    }
    impl ::core::convert::From<ForwarderContractCall> for ContractResolverCalls {
        fn from(value: ForwarderContractCall) -> Self {
            Self::ForwarderContract(value)
        }
    }
    impl ::core::convert::From<HdKeyDeriverContractCall> for ContractResolverCalls {
        fn from(value: HdKeyDeriverContractCall) -> Self {
            Self::HdKeyDeriverContract(value)
        }
    }
    impl ::core::convert::From<HostCommandsContractCall> for ContractResolverCalls {
        fn from(value: HostCommandsContractCall) -> Self {
            Self::HostCommandsContract(value)
        }
    }
    impl ::core::convert::From<LedgerContractCall> for ContractResolverCalls {
        fn from(value: LedgerContractCall) -> Self {
            Self::LedgerContract(value)
        }
    }
    impl ::core::convert::From<LitTokenContractCall> for ContractResolverCalls {
        fn from(value: LitTokenContractCall) -> Self {
            Self::LitTokenContract(value)
        }
    }
    impl ::core::convert::From<MultiSenderContractCall> for ContractResolverCalls {
        fn from(value: MultiSenderContractCall) -> Self {
            Self::MultiSenderContract(value)
        }
    }
    impl ::core::convert::From<PaymentDelegationContractCall> for ContractResolverCalls {
        fn from(value: PaymentDelegationContractCall) -> Self {
            Self::PaymentDelegationContract(value)
        }
    }
    impl ::core::convert::From<PkpHelperContractCall> for ContractResolverCalls {
        fn from(value: PkpHelperContractCall) -> Self {
            Self::PkpHelperContract(value)
        }
    }
    impl ::core::convert::From<PkpHelperV2ContractCall> for ContractResolverCalls {
        fn from(value: PkpHelperV2ContractCall) -> Self {
            Self::PkpHelperV2Contract(value)
        }
    }
    impl ::core::convert::From<PkpNftContractCall> for ContractResolverCalls {
        fn from(value: PkpNftContractCall) -> Self {
            Self::PkpNftContract(value)
        }
    }
    impl ::core::convert::From<PkpNftMetadataContractCall> for ContractResolverCalls {
        fn from(value: PkpNftMetadataContractCall) -> Self {
            Self::PkpNftMetadataContract(value)
        }
    }
    impl ::core::convert::From<PkpPermissionsContractCall> for ContractResolverCalls {
        fn from(value: PkpPermissionsContractCall) -> Self {
            Self::PkpPermissionsContract(value)
        }
    }
    impl ::core::convert::From<PriceFeedContractCall> for ContractResolverCalls {
        fn from(value: PriceFeedContractCall) -> Self {
            Self::PriceFeedContract(value)
        }
    }
    impl ::core::convert::From<PubKeyRouterContractCall> for ContractResolverCalls {
        fn from(value: PubKeyRouterContractCall) -> Self {
            Self::PubKeyRouterContract(value)
        }
    }
    impl ::core::convert::From<RateLimitNftContractCall> for ContractResolverCalls {
        fn from(value: RateLimitNftContractCall) -> Self {
            Self::RateLimitNftContract(value)
        }
    }
    impl ::core::convert::From<ReleaseRegisterContractCall> for ContractResolverCalls {
        fn from(value: ReleaseRegisterContractCall) -> Self {
            Self::ReleaseRegisterContract(value)
        }
    }
    impl ::core::convert::From<StakingBalancesContractCall> for ContractResolverCalls {
        fn from(value: StakingBalancesContractCall) -> Self {
            Self::StakingBalancesContract(value)
        }
    }
    impl ::core::convert::From<StakingContractCall> for ContractResolverCalls {
        fn from(value: StakingContractCall) -> Self {
            Self::StakingContract(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for ContractResolverCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddAllowedEnvCall> for ContractResolverCalls {
        fn from(value: AddAllowedEnvCall) -> Self {
            Self::AddAllowedEnv(value)
        }
    }
    impl ::core::convert::From<GetContractCall> for ContractResolverCalls {
        fn from(value: GetContractCall) -> Self {
            Self::GetContract(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ContractResolverCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ContractResolverCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ContractResolverCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RemoveAdminCall> for ContractResolverCalls {
        fn from(value: RemoveAdminCall) -> Self {
            Self::RemoveAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedEnvCall> for ContractResolverCalls {
        fn from(value: RemoveAllowedEnvCall) -> Self {
            Self::RemoveAllowedEnv(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ContractResolverCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ContractResolverCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetContractCall> for ContractResolverCalls {
        fn from(value: SetContractCall) -> Self {
            Self::SetContract(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ContractResolverCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TypeAddressesCall> for ContractResolverCalls {
        fn from(value: TypeAddressesCall) -> Self {
            Self::TypeAddresses(value)
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
    ///Container type for all return fields from the `ALLOWLIST_CONTRACT` function with signature `ALLOWLIST_CONTRACT()` and selector `0x7cadf69f`
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
    pub struct AllowlistContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `BACKUP_RECOVERY_CONTRACT` function with signature `BACKUP_RECOVERY_CONTRACT()` and selector `0x7d4a03bd`
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
    pub struct BackupRecoveryContractReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `DOMAIN_WALLET_REGISTRY` function with signature `DOMAIN_WALLET_REGISTRY()` and selector `0x4216e73a`
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
    pub struct DomainWalletRegistryReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FORWARDER_CONTRACT` function with signature `FORWARDER_CONTRACT()` and selector `0x7d9d2880`
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
    pub struct ForwarderContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HD_KEY_DERIVER_CONTRACT` function with signature `HD_KEY_DERIVER_CONTRACT()` and selector `0x85cb1191`
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
    pub struct HdKeyDeriverContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HOST_COMMANDS_CONTRACT` function with signature `HOST_COMMANDS_CONTRACT()` and selector `0x5af27f79`
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
    pub struct HostCommandsContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `LEDGER_CONTRACT` function with signature `LEDGER_CONTRACT()` and selector `0xcddcace5`
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
    pub struct LedgerContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `LIT_TOKEN_CONTRACT` function with signature `LIT_TOKEN_CONTRACT()` and selector `0xdf380693`
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
    pub struct LitTokenContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MULTI_SENDER_CONTRACT` function with signature `MULTI_SENDER_CONTRACT()` and selector `0xf8ae93b4`
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
    pub struct MultiSenderContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PAYMENT_DELEGATION_CONTRACT` function with signature `PAYMENT_DELEGATION_CONTRACT()` and selector `0x7f90209f`
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
    pub struct PaymentDelegationContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_HELPER_CONTRACT` function with signature `PKP_HELPER_CONTRACT()` and selector `0x977a8070`
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
    pub struct PkpHelperContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_HELPER_V2_CONTRACT` function with signature `PKP_HELPER_V2_CONTRACT()` and selector `0x11ee8ff7`
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
    pub struct PkpHelperV2ContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_NFT_CONTRACT` function with signature `PKP_NFT_CONTRACT()` and selector `0x2c0b8bf7`
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
    pub struct PkpNftContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_NFT_METADATA_CONTRACT` function with signature `PKP_NFT_METADATA_CONTRACT()` and selector `0x16f76bbf`
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
    pub struct PkpNftMetadataContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PKP_PERMISSIONS_CONTRACT` function with signature `PKP_PERMISSIONS_CONTRACT()` and selector `0x9072f838`
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
    pub struct PkpPermissionsContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PRICE_FEED_CONTRACT` function with signature `PRICE_FEED_CONTRACT()` and selector `0x219c266a`
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
    pub struct PriceFeedContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PUB_KEY_ROUTER_CONTRACT` function with signature `PUB_KEY_ROUTER_CONTRACT()` and selector `0x2668f305`
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
    pub struct PubKeyRouterContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `RATE_LIMIT_NFT_CONTRACT` function with signature `RATE_LIMIT_NFT_CONTRACT()` and selector `0x2e4885e8`
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
    pub struct RateLimitNftContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `RELEASE_REGISTER_CONTRACT` function with signature `RELEASE_REGISTER_CONTRACT()` and selector `0xad1c8a86`
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
    pub struct ReleaseRegisterContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_BALANCES_CONTRACT` function with signature `STAKING_BALANCES_CONTRACT()` and selector `0x8c1536df`
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
    pub struct StakingBalancesContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_CONTRACT` function with signature `STAKING_CONTRACT()` and selector `0xda19ddfb`
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
    pub struct StakingContractReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getContract` function with signature `getContract(bytes32,uint8)` and selector `0x8e8dfd16`
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
    pub struct GetContractReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `typeAddresses` function with signature `typeAddresses(bytes32,uint8)` and selector `0x3ebf7985`
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
    pub struct TypeAddressesReturn(pub ::ethers::core::types::Address);
}
