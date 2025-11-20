pub use pkp_helper::*;
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
pub mod pkp_helper {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/PKPHelper.json",
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
                    ::std::borrow::ToOwned::to_owned(
                        "claimAndMintNextAndAddAuthMethods",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "claimAndMintNextAndAddAuthMethods",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claimMaterial"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibPKPNFTStorage.ClaimMaterial",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authMethodData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PKPHelper.AuthMethodData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "claimAndMintNextAndAddAuthMethodsWithTypes",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "claimAndMintNextAndAddAuthMethodsWithTypes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claimMaterial"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibPKPNFTStorage.ClaimMaterial",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authMethodData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PKPHelper.AuthMethodData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "claimAndMintNextAndAddAuthMethodsWithTypesV2",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "claimAndMintNextAndAddAuthMethodsWithTypesV2",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claimMaterial"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LibPKPNFTStorage.ClaimMaterialV2",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authMethodData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PKPHelper.AuthMethodData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("getDomainWalletRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDomainWalletRegistry",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getPKPNftMetdataAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPKPNftMetdataAddress",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getPkpNftAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPkpNftAddress"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getPkpPermissionsAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPkpPermissionsAddress",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getStakingAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStakingAddress"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("mintNextAndAddAuthMethods"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "mintNextAndAddAuthMethods",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keySetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodTypes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodIds",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodPubkeys",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodScopes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "addPkpEthAddressAsPermittedAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sendPkpToItself"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "mintNextAndAddAuthMethodsWithTypes",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "mintNextAndAddAuthMethodsWithTypes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct PKPHelper.MintNextAndAddAuthMethodsWithTypesParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "mintNextAndAddDomainWalletMetadata",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "mintNextAndAddDomainWalletMetadata",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keySetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodTypes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodIds",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodPubkeys",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "permittedAuthMethodScopes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nftMetadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "addPkpEthAddressAsPermittedAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sendPkpToItself"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("removePkpMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePkpMetadata"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setContractResolver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setContractResolver",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newResolverAddress",
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
                    ::std::borrow::ToOwned::to_owned("setPkpMetadata"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPkpMetadata"),
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
                                    name: ::std::borrow::ToOwned::to_owned("nftMetadata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ContractResolverAddressSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractResolverAddressSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newResolverAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PKPHELPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa:f8\x03\x80a:f\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xD5V[a\083a\0\x85V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x83U\x83\x92\x91`\x01`\x01`\xA8\x1B\x03\x19\x16\x17`\x01`\xA0\x1B\x83\x83\x81\x11\x15a\0yWa\0ya\x01\x1FV[\x02\x17\x90UPPPa\x015V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x80`@\x83\x85\x03\x12\x15a\0\xE8W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xFFW`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x03\x81\x10a\x01\x14W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a9\"\x80a\x01D`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01<W`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x01AW\x80c\x0E\x9E\xD6\x8B\x14a\x01vW\x80c\x13\xAFA\x1B\x14a\x01\x98W\x80c\x15\x0Bz\x02\x14a\x01\xB9W\x80c /rO\x14a\x01\xF2W\x80c$\x8A\x9C\xA3\x14a\x02\x05W\x80c+U5Q\x14a\x02%W\x80c//\xF1]\x14a\x02GW\x80c2vU\x8C\x14a\x02gW\x80c6V\x8A\xBE\x14a\x02|W\x80cPC\x02l\x14a\x02\x9CW\x80cP\xD1{^\x14a\x02\xB1W\x80cqP\x18\xA6\x14a\x02\xD1W\x80cs\xCCA\x11\x14a\x02\xE6W\x80cw\x8F\xE5r\x14a\x02\xFBW\x80cx..\xA5\x14a\x03\x0EW\x80c\x8D\xA5\xCB[\x14a\x03.W\x80c\x91\xD1HT\x14a\x03CW\x80c\x91\xEEO\xD5\x14a\x03cW\x80c\x9D\xCA\x002\x14a\x03vW\x80c\xA2\x17\xFD\xDF\x14a\x03\xA4W\x80c\xCA\xEA\xD0\xC7\x14a\x03\xB9W\x80c\xD5Gt\x1F\x14a\x03\xCEW\x80c\xDB\x0B\xF93\x14a\x03\xEEW\x80c\xE4\xF1\x1D\xF6\x14a\x04\x01W\x80c\xF2\xFD\xE3\x8B\x14a\x04\x14W\x80c\xF9]q\xB1\x14a\x044W[`\0\x80\xFD[4\x80\x15a\x01MW`\0\x80\xFD[Pa\x01aa\x01\\6`\x04a%\x88V[a\x04TV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x82W`\0\x80\xFD[Pa\x01\x8Ba\x04\x8BV[`@Qa\x01m\x91\x90a%\xB2V[a\x01\xABa\x01\xA66`\x04a+tV[a\x05vV[`@Q\x90\x81R` \x01a\x01mV[4\x80\x15a\x01\xC5W`\0\x80\xFD[Pa\x01\xD9a\x01\xD46`\x04a,\"V[a\x05\xF3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01mV[a\x01\xABa\x02\x006`\x04a+tV[a\x06\x97V[4\x80\x15a\x02\x11W`\0\x80\xFD[Pa\x01\xABa\x02 6`\x04a,\xC1V[a\x06\xAAV[4\x80\x15a\x021W`\0\x80\xFD[Pa\x02Ea\x02@6`\x04a,\xC1V[a\x06\xC0V[\0[4\x80\x15a\x02SW`\0\x80\xFD[Pa\x02Ea\x02b6`\x04a,\xDAV[a\x08\x9BV[4\x80\x15a\x02sW`\0\x80\xFD[Pa\x01\x8Ba\x08\xBCV[4\x80\x15a\x02\x88W`\0\x80\xFD[Pa\x02Ea\x02\x976`\x04a,\xDAV[a\t\x0EV[4\x80\x15a\x02\xA8W`\0\x80\xFD[Pa\x01\x8Ba\t\x8CV[4\x80\x15a\x02\xBDW`\0\x80\xFD[P`\x02Ta\x01\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xDDW`\0\x80\xFD[Pa\x02Ea\t\xDEV[4\x80\x15a\x02\xF2W`\0\x80\xFD[Pa\x01\x8Ba\t\xF2V[a\x01\xABa\x03\t6`\x04a-*V[a\nDV[4\x80\x15a\x03\x1AW`\0\x80\xFD[Pa\x02Ea\x03)6`\x04a/\xA8V[a\x10VV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x01\x8Ba\x12yV[4\x80\x15a\x03OW`\0\x80\xFD[Pa\x01aa\x03^6`\x04a,\xDAV[a\x12\x88V[a\x01\xABa\x03q6`\x04a/\xE4V[a\x12\xB3V[4\x80\x15a\x03\x82W`\0\x80\xFD[P`\x02Ta\x03\x97\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x01m\x91\x90a0\xE0V[4\x80\x15a\x03\xB0W`\0\x80\xFD[Pa\x01\xAB`\0\x81V[4\x80\x15a\x03\xC5W`\0\x80\xFD[Pa\x01\x8Ba\x19UV[4\x80\x15a\x03\xDAW`\0\x80\xFD[Pa\x02Ea\x03\xE96`\x04a,\xDAV[a\x19\xA7V[a\x01\xABa\x03\xFC6`\x04a0\xEEV[a\x19\xC3V[a\x01\xABa\x04\x0F6`\x04a2+V[a\x1F\xCDV[4\x80\x15a\x04 W`\0\x80\xFD[Pa\x02Ea\x04/6`\x04a3=V[a! V[4\x80\x15a\x04@W`\0\x80\xFD[Pa\x02Ea\x04O6`\x04a3=V[a!\x99V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x85WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\x02T`@\x80Qc\xDA\x19\xDD\xFB`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\xDA\x19\xDD\xFB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x01\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x050\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05q\x91\x90a3\x87V[\x90P\x90V[`\0\x80`@Q\x80`\xA0\x01`@R\x80\x85`\0\x01Q\x81R` \x01`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01knaga-keyset1`\xA0\x1B\x81RP\x81R` \x01\x85` \x01Q\x81R` \x01\x85`@\x01Q\x81R` \x01a\x05\xD4a\x04\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x05\xEB\x81\x84a\x12\xB3V[\x94\x93PPPPV[`\0a\x05\xFDa\x19UV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FPKPHelper: only accepts transfer`D\x82\x01Ry\x1C\xC8\x19\x9C\x9B\xDBH\x1D\x1A\x19H\x14\x12\xD4\x13\x91\x95\x08\x18\xDB\xDB\x9D\x1C\x98X\xDD`2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[Pc\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[`\0a\x06\xA3\x83\x83a\x05vV[\x93\x92PPPV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x01T\x90V[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x076\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x07e\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA6\x91\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a3\xA4V[`\0a\x07\xE0a\t\x8CV[`@Qc\xB6:vw`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB6:vw\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x089W=`\0\x80>=`\0\xFD[PP`@Qc(\xCD\x10\xC7`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92PcQ\x9A!\x8E\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x7FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x93W=`\0\x80>=`\0\xFD[PPPPPPV[a\x08\xA4\x82a\x06\xAAV[a\x08\xAD\x81a!\xF7V[a\x08\xB7\x83\x83a\"\x01V[PPPV[`\x02T`@\x80Qc\x12\x0E_\x07`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x90r\xF88\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\t~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x06|V[a\t\x88\x82\x82a\"lV[PPV[`\x02T`@\x80Qc\x16\xF7k\xBF`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x16\xF7k\xBF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[a\t\xE6a\"\xD3V[a\t\xF0`\0a#2V[V[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[`\0\x80a\nOa\x19UV[\x83Q` \x85\x01Q`@Qc?\xF8\x06\x97`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92c\x7F\xF0\r.\x924\x92a\n\x84\x92`\x04\x01a4tV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\n\xA2W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC7\x91\x90a3ZV[\x90P\x82``\x01QQ\x83`@\x01QQ\x14a\n\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\x8DV[\x82`\xA0\x01QQ\x83`\x80\x01QQ\x14a\x0B\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\xE3V[\x82`\xE0\x01QQ\x83`\xC0\x01QQ\x14a\x0BDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a58V[\x82a\x01\0\x01QQ\x83`\xC0\x01QQ\x14a\x0BnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\x81V[\x82a\x01 \x01QQ\x83`\xC0\x01QQ\x14a\x0B\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\xCCV[`@\x83\x01QQ\x15a\x0CdW`\0[\x83`@\x01QQ\x81\x10\x15a\x0CbWa\x0B\xBBa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x8AC\x15x\x83\x86`@\x01Q\x84\x81Q\x81\x10a\x0B\xE0Wa\x0B\xE0a6\x17V[` \x02` \x01\x01Q\x87``\x01Q\x85\x81Q\x81\x10a\x0B\xFEWa\x0B\xFEa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C$\x93\x92\x91\x90a6iV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C>W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0CRW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x0B\xA6\x90PV[P[`\x80\x83\x01QQ\x15a\r0W`\0[\x83`\x80\x01QQ\x81\x10\x15a\r.Wa\x0C\x87a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x86`\x80\x01Q\x84\x81Q\x81\x10a\x0C\xACWa\x0C\xACa6\x17V[` \x02` \x01\x01Q\x87`\xA0\x01Q\x85\x81Q\x81\x10a\x0C\xCAWa\x0C\xCAa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xF0\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\x1EW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x0Cr\x90PV[P[`\xC0\x83\x01QQ\x15a\x0ERW`\0[\x83`\xC0\x01QQ\x81\x10\x15a\x0EPWa\rSa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x88`\xC0\x01Q\x86\x81Q\x81\x10a\r\x83Wa\r\x83a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x88`\xE0\x01Q\x86\x81Q\x81\x10a\r\xA6Wa\r\xA6a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x88a\x01\0\x01Q\x86\x81Q\x81\x10a\r\xCAWa\r\xCAa6\x17V[` \x02` \x01\x01Q\x81RP\x87a\x01 \x01Q\x85\x81Q\x81\x10a\r\xECWa\r\xECa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x12\x93\x92\x91\x90a6\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E@W=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\r>\x90PV[P[`\0a\x0E\\a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCA\x91\x90a3\x87V[\x90P\x83a\x01@\x01Q\x15a\x0FlWa\x0E\xDFa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x83`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x1AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F9\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FSW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[PPPP[\x83a\x01`\x01Q\x15a\x0F\xE5Wa\x0F\x7Fa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xAE\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xC8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xDCW=`\0\x80>=`\0\xFD[PPPPa\x10OV[a\x0F\xEDa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x1C\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x106W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10JW=`\0\x80>=`\0\xFD[PPPP[P\x92\x91PPV[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x10\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCC\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x10\xFB\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11<\x91\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a3\xA4V[`\0a\x11va\t\x8CV[\x82Q\x90\x91P\x15a\x08\xB7W\x80`\x01`\x01`\xA0\x1B\x03\x16c\x85^\xEC\"\x84\x84`\0\x81Q\x81\x10a\x11\xA3Wa\x11\xA3a6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xC8\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF6W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x90\0\xFE\xE1\x84\x84`\x01\x81Q\x81\x10a\x12\x1DWa\x12\x1Da6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12B\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12pW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0\x91\x82R`\x01` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[\x80Q\x82Q`\0\x91\x14a\x13-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FPKPHelper: Claim key type must m`D\x82\x01R\x7Fatch Auth Method data key type\0\0`d\x82\x01R`\x84\x01a\x06|V[`\x01`\0a\x139a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cq\xAA\x9A\xCF4\x84\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Q\x8B``\x01Q\x8C`\x80\x01Q`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x83\x96\x95\x94\x93\x92\x91\x90a7SV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x13\xA1W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xC6\x91\x90a3ZV[\x90P\x83`@\x01QQ\x84` \x01QQ\x14a\x13\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\x8DV[\x83`\x80\x01QQ\x84``\x01QQ\x14a\x14\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\xE3V[\x83`\xC0\x01QQ\x84`\xA0\x01QQ\x14a\x14CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a58V[\x83`\xE0\x01QQ\x84`\xA0\x01QQ\x14a\x14lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\x81V[\x83a\x01\0\x01QQ\x84`\xA0\x01QQ\x14a\x14\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\xCCV[` \x84\x01QQ\x15a\x15bW`\0[\x84` \x01QQ\x81\x10\x15a\x15`Wa\x14\xB9a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x8AC\x15x\x83\x87` \x01Q\x84\x81Q\x81\x10a\x14\xDEWa\x14\xDEa6\x17V[` \x02` \x01\x01Q\x88`@\x01Q\x85\x81Q\x81\x10a\x14\xFCWa\x14\xFCa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\"\x93\x92\x91\x90a6iV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15PW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x14\xA4\x90PV[P[``\x84\x01QQ\x15a\x16.W`\0[\x84``\x01QQ\x81\x10\x15a\x16,Wa\x15\x85a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x87``\x01Q\x84\x81Q\x81\x10a\x15\xAAWa\x15\xAAa6\x17V[` \x02` \x01\x01Q\x88`\x80\x01Q\x85\x81Q\x81\x10a\x15\xC8Wa\x15\xC8a6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xEE\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x08W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x1CW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x15p\x90PV[P[`\xA0\x84\x01QQ\x15a\x17OW`\0[\x84`\xA0\x01QQ\x81\x10\x15a\x17MWa\x16Qa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x89`\xA0\x01Q\x86\x81Q\x81\x10a\x16\x81Wa\x16\x81a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x89`\xC0\x01Q\x86\x81Q\x81\x10a\x16\xA4Wa\x16\xA4a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x89`\xE0\x01Q\x86\x81Q\x81\x10a\x16\xC7Wa\x16\xC7a6\x17V[` \x02` \x01\x01Q\x81RP\x88a\x01\0\x01Q\x85\x81Q\x81\x10a\x16\xE9Wa\x16\xE9a6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x0F\x93\x92\x91\x90a6\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17)W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17=W=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x16<\x90PV[P[`\0a\x17Ya\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x86\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC7\x91\x90a3\x87V[\x90P\x84a\x01 \x01Q\x15a\x18iWa\x17\xDCa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x83`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x17W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x186\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18PW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18dW=`\0\x80>=`\0\xFD[PPPP[\x84a\x01@\x01Q\x15a\x18\xE2Wa\x18|a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xAB\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xD9W=`\0\x80>=`\0\xFD[PPPPa\x19LV[a\x18\xEAa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x19\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x193W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19GW=`\0\x80>=`\0\xFD[PPPP[P\x94\x93PPPPV[`\x02T`@\x80Qc,\x0B\x8B\xF7`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c,\x0B\x8B\xF7\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[a\x19\xB0\x82a\x06\xAAV[a\x19\xB9\x81a!\xF7V[a\x08\xB7\x83\x83a\"lV[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A9\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x1Ah\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA9\x91\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a3\xA4V[`\0a\x1A\xE3a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xF0\r.4\x8D\x8D`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x11\x92\x91\x90a4tV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x1B/W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BT\x91\x90a3ZV[\x90P\x87Q\x89Q\x14a\x1BwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a58V[\x86Q\x89Q\x14a\x1B\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\x81V[\x85Q\x89Q\x14a\x1B\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\xCCV[\x88Q\x15a\x1C\xC1W`\0[\x89Q\x81\x10\x15a\x1C\xBFWa\x1B\xD4a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x8E\x86\x81Q\x81\x10a\x1C\0Wa\x1C\0a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x8D\x86\x81Q\x81\x10a\x1C\x1FWa\x1C\x1Fa6\x17V[` \x02` \x01\x01Q\x81R` \x01\x8C\x86\x81Q\x81\x10a\x1C>Wa\x1C>a6\x17V[` \x02` \x01\x01Q\x81RP\x8A\x85\x81Q\x81\x10a\x1C[Wa\x1C[a6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x81\x93\x92\x91\x90a6\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xAFW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x1B\xC3\x90PV[P[`\0a\x1C\xCBa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xF8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D9\x91\x90a3\x87V[\x90P\x84\x15a\x1D\xD6Wa\x1DIa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x83`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xA3\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xBDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D\xD1W=`\0\x80>=`\0\xFD[PPPP[\x83\x15a\x1EJWa\x1D\xE4a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x13\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1EAW=`\0\x80>=`\0\xFD[PPPPa\x1E\xB4V[a\x1ERa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x81\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\xAFW=`\0\x80>=`\0\xFD[PPPP[\x85Q\x15a\x1F\xBEWa\x1E\xC3a\t\x8CV[`\x01`\x01`\xA0\x1B\x03\x16c\x85^\xEC\"\x83\x88`\0\x81Q\x81\x10a\x1E\xE5Wa\x1E\xE5a6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\n\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F8W=`\0\x80>=`\0\xFD[PPPPa\x1FDa\t\x8CV[`\x01`\x01`\xA0\x1B\x03\x16c\x90\0\xFE\xE1\x83\x88`\x01\x81Q\x81\x10a\x1FfWa\x1Ffa6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\x8B\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xB9W=`\0\x80>=`\0\xFD[PPPP[P\x9A\x99PPPPPPPPPPV[`\0\x80`@Q\x80a\x01\x80\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x81\x11\x15a \x01Wa \x01a%\xC6V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a 4W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x1FW\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a oW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a ZW\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xA0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xDBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xC6W\x90P[P\x81R` \x01\x89\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x15\x15\x81R` \x01\x84\x15\x15\x81RP\x90Pa!\x12\x81a\nDV[\x9A\x99PPPPPPPPPPV[a!(a\"\xD3V[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06|V[a!\x96\x81a#2V[PV[a!\xA1a\"\xD3V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`@Q\x7F'`\x07<|\xD8\xCA\xC51\xD7\xF6C\xBE\xCB\xFB\xB7M\x8B\x81VD>\xAC\xF8yb%2\xDB\xBB<\xD5\x90a!\xEC\x90\x83\x90a%\xB2V[`@Q\x80\x91\x03\x90\xA1PV[a!\x96\x813a#\x82V[a\"\x0B\x82\x82a\x12\x88V[a\t\x88W`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x86R\x92R\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U\x90Q3\x92\x85\x91\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x91\x90\xA4PPV[a\"v\x82\x82a\x12\x88V[\x15a\t\x88W`\0\x82\x81R`\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[3a\"\xDCa\x12yV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06|V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a#\x8C\x82\x82a\x12\x88V[a\t\x88Wa#\x99\x81a#\xDBV[a#\xA4\x83` a#\xEDV[`@Q` \x01a#\xB5\x92\x91\x90a7\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x06|\x91`\x04\x01a8bV[``a\x04\x85`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a#\xFC\x83`\x02a8\x8BV[a$\x07\x90`\x02a8\xA2V[`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x1EWa$\x1Ea%\xC6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$HW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a$cWa$ca6\x17V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a$\x92Wa$\x92a6\x17V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a$\xB6\x84`\x02a8\x8BV[a$\xC1\x90`\x01a8\xA2V[\x90P[`\x01\x81\x11\x15a%9Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a$\xF5Wa$\xF5a6\x17V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a%\x0BWa%\x0Ba6\x17V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a%2\x81a8\xB5V[\x90Pa$\xC4V[P\x83\x15a\x06\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x06|V[`\0` \x82\x84\x03\x12\x15a%\x9AW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x06\xA3W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@R\x90V[`@Qa\x01`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@Qa\x01\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\x94Wa&\x94a%\xC6V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a&\xB5Wa&\xB5a%\xC6V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a&\xD0W`\0\x80\xFD[\x815a&\xE3a&\xDE\x82a&\x9CV[a&lV[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x86\x01\x01\x92P\x85\x83\x11\x15a'\x05W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW``\x81\x88\x03\x12\x15a'\"W`\0\x80\xFD[a'*a%\xDCV[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x82\x015`\xFF\x81\x16\x81\x14a'LW`\0\x80\xFD[`@\x82\x01R\x83R` \x90\x92\x01\x91``\x01a'\nV[P\x95\x94PPPPPV[`\0\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a'\x85Wa'\x85a%\xC6V[P`\x1F\x83\x01`\x1F\x19\x16` \x01a'\x9A\x81a&lV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15a'\xAFW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a'\xD7W`\0\x80\xFD[\x815a'\xE5a&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x07W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a(*W`\0\x80\xFD[\x86\x01`?\x81\x01\x88\x13a(;W`\0\x80\xFD[a(M\x88` \x83\x015`@\x84\x01a'kV[\x84RP` \x92\x83\x01\x92\x01a(\x0CV[`\0\x82`\x1F\x83\x01\x12a(mW`\0\x80\xFD[\x815a({a&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x9DW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805\x83R` \x92\x83\x01\x92\x01a(\xA2V[`\0\x82`\x1F\x83\x01\x12a(\xCBW`\0\x80\xFD[\x815a(\xD9a&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\xFBW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x1EW`\0\x80\xFD[a)-\x88` \x83\x8A\x01\x01a(\\V[\x84RP` \x92\x83\x01\x92\x01a)\0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x96W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a)bW`\0\x80\xFD[\x815a)pa&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a)\x92W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805a)\xAA\x81a)<V[\x83R` \x92\x83\x01\x92\x01a)\x97V[\x805\x80\x15\x15\x81\x14a)\xC8W`\0\x80\xFD[\x91\x90PV[`\0a\x01`\x82\x84\x03\x12\x15a)\xE0W`\0\x80\xFD[a)\xE8a&\x04V[\x825\x81R\x90P` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x07W`\0\x80\xFD[a*\x13\x84\x82\x85\x01a'\xC6V[` \x83\x01RP`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*2W`\0\x80\xFD[a*>\x84\x82\x85\x01a(\xBAV[`@\x83\x01RP``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*]W`\0\x80\xFD[a*i\x84\x82\x85\x01a)QV[``\x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x88W`\0\x80\xFD[a*\x94\x84\x82\x85\x01a(\xBAV[`\x80\x83\x01RP`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xB3W`\0\x80\xFD[a*\xBF\x84\x82\x85\x01a(\\V[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xDEW`\0\x80\xFD[a*\xEA\x84\x82\x85\x01a'\xC6V[`\xC0\x83\x01RP`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\tW`\0\x80\xFD[a+\x15\x84\x82\x85\x01a'\xC6V[`\xE0\x83\x01RPa\x01\0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+5W`\0\x80\xFD[a+A\x84\x82\x85\x01a(\xBAV[a\x01\0\x83\x01RPa+Ua\x01 \x83\x01a)\xB8V[a\x01 \x82\x01Ra+ha\x01@\x83\x01a)\xB8V[a\x01@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a+\x87W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a+\x9DW`\0\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a+\xAFW`\0\x80\xFD[a+\xB7a%\xDCV[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xDEW`\0\x80\xFD[a+\xEA\x87\x82\x85\x01a&\xBFV[`@\x83\x01RP\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x0CW`\0\x80\xFD[a,\x18\x85\x82\x86\x01a)\xCDV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a,:W`\0\x80\xFD[\x855a,E\x81a)<V[\x94P` \x86\x015a,U\x81a)<V[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,wW`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a,\x88W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x9EW`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a,\xB0W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95PPP` \x01\x91\x90V[`\0` \x82\x84\x03\x12\x15a,\xD3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xEDW`\0\x80\xFD[\x825\x91P` \x83\x015a,\xFF\x81a)<V[\x80\x91PP\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12a-\x1BW`\0\x80\xFD[a\x06\xA3\x83\x835` \x85\x01a'kV[`\0` \x82\x84\x03\x12\x15a-<W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-RW`\0\x80\xFD[\x82\x01a\x01\x80\x81\x85\x03\x12\x15a-eW`\0\x80\xFD[a-ma&'V[\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x8AW`\0\x80\xFD[a-\x96\x86\x82\x85\x01a-\nV[` \x83\x01RP`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xB5W`\0\x80\xFD[a-\xC1\x86\x82\x85\x01a'\xC6V[`@\x83\x01RP``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xE0W`\0\x80\xFD[a-\xEC\x86\x82\x85\x01a(\xBAV[``\x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x0BW`\0\x80\xFD[a.\x17\x86\x82\x85\x01a)QV[`\x80\x83\x01RP`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.6W`\0\x80\xFD[a.B\x86\x82\x85\x01a(\xBAV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.aW`\0\x80\xFD[a.m\x86\x82\x85\x01a(\\V[`\xC0\x83\x01RP`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x8CW`\0\x80\xFD[a.\x98\x86\x82\x85\x01a'\xC6V[`\xE0\x83\x01RPa\x01\0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xB8W`\0\x80\xFD[a.\xC4\x86\x82\x85\x01a'\xC6V[a\x01\0\x83\x01RPa\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xE5W`\0\x80\xFD[a.\xF1\x86\x82\x85\x01a(\xBAV[a\x01 \x83\x01RPa/\x05a\x01@\x83\x01a)\xB8V[a\x01@\x82\x01Ra/\x18a\x01`\x83\x01a)\xB8V[a\x01`\x82\x01R\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a/7W`\0\x80\xFD[\x815a/Ea&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a/gW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x8AW`\0\x80\xFD[a/\x99\x88` \x83\x8A\x01\x01a-\nV[\x84RP` \x92\x83\x01\x92\x01a/lV[`\0\x80`@\x83\x85\x03\x12\x15a/\xBBW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xD8W`\0\x80\xFD[a,\x18\x85\x82\x86\x01a/&V[`\0\x80`@\x83\x85\x03\x12\x15a/\xF7W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a0\rW`\0\x80\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15a0\x1FW`\0\x80\xFD[a0'a&JV[\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0DW`\0\x80\xFD[a0P\x87\x82\x85\x01a-\nV[` \x83\x01RP`@\x82\x81\x015\x90\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0yW`\0\x80\xFD[a0\x85\x87\x82\x85\x01a&\xBFV[``\x83\x01RP`\x80\x82\x015\x91Pa0\x9B\x82a)<V[`\x80\x81\x01\x91\x90\x91R\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x0CW`\0\x80\xFD[`\x03\x81\x10a0\xDCWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x04\x85\x82\x84a0\xBEV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a1\rW`\0\x80\xFD[\x895\x98P` \x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1*W`\0\x80\xFD[a16\x8C\x82\x8D\x01a-\nV[\x98PP`@\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1RW`\0\x80\xFD[a1^\x8C\x82\x8D\x01a(\\V[\x97PP``\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1zW`\0\x80\xFD[a1\x86\x8C\x82\x8D\x01a'\xC6V[\x96PP`\x80\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xA2W`\0\x80\xFD[a1\xAE\x8C\x82\x8D\x01a'\xC6V[\x95PP`\xA0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xCAW`\0\x80\xFD[a1\xD6\x8C\x82\x8D\x01a(\xBAV[\x94PP`\xC0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xF2W`\0\x80\xFD[a1\xFE\x8C\x82\x8D\x01a/&V[\x93PPa2\r`\xE0\x8B\x01a)\xB8V[\x91Pa2\x1Ca\x01\0\x8B\x01a)\xB8V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a2HW`\0\x80\xFD[\x885\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2eW`\0\x80\xFD[a2q\x8B\x82\x8C\x01a-\nV[\x97PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x8DW`\0\x80\xFD[a2\x99\x8B\x82\x8C\x01a(\\V[\x96PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xB5W`\0\x80\xFD[a2\xC1\x8B\x82\x8C\x01a'\xC6V[\x95PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xDDW`\0\x80\xFD[a2\xE9\x8B\x82\x8C\x01a'\xC6V[\x94PP`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x05W`\0\x80\xFD[a3\x11\x8B\x82\x8C\x01a(\xBAV[\x93PPa3 `\xC0\x8A\x01a)\xB8V[\x91Pa3.`\xE0\x8A\x01a)\xB8V[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a3OW`\0\x80\xFD[\x815a\x06\xA3\x81a)<V[`\0` \x82\x84\x03\x12\x15a3lW`\0\x80\xFD[PQ\x91\x90PV[\x82\x81R`@\x81\x01a\x06\xA3` \x83\x01\x84a0\xBEV[`\0` \x82\x84\x03\x12\x15a3\x99W`\0\x80\xFD[\x81Qa\x06\xA3\x81a)<V[` \x80\x82R`Z\x90\x82\x01R\x7FPKPHelper: only the Domain Walle`@\x82\x01R\x7Ft registry is allowed to mint do``\x82\x01Rymain wallets, who are you?`0\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0[\x83\x81\x10\x15a4?W\x81\x81\x01Q\x83\x82\x01R` \x01a4'V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra4`\x81` \x86\x01` \x86\x01a4$V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x05\xEB`@\x83\x01\x84a4HV[` \x80\x82R`6\x90\x82\x01R\x7FPKPHelper: ipfs cid and scope ar`@\x82\x01Ru\x0EL/$\r\x8C\xAD\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`S\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`5\x90\x82\x01R\x7FPKPHelper: address and scope arr`@\x82\x01Rt\x0C/$\r\x8C\xAD\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`[\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`;\x90\x82\x01R`\0\x80Q` a8\xCD\x839\x81Q\x91R`@\x82\x01Rz\r,\x84\x0C.NL/$\r\x8C\xAD\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`+\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`?\x90\x82\x01R`\0\x80Q` a8\xCD\x839\x81Q\x91R`@\x82\x01R\x7Fpubkey array lengths must match\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`?\x90\x82\x01R`\0\x80Q` a8\xCD\x839\x81Q\x91R`@\x82\x01R\x7Fscopes array lengths must match\0``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a6_W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a6AV[P\x93\x94\x93PPPPV[\x83\x81R``` \x82\x01R`\0a6\x82``\x83\x01\x85a4HV[\x82\x81\x03`@\x84\x01Ra6\x94\x81\x85a6-V[\x96\x95PPPPPPV[\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a6\xC8\x90\x83\x01\x84a6-V[\x95\x94PPPPPV[\x83\x81R``` \x82\x01R\x82Q``\x82\x01R`\0` \x84\x01Q```\x80\x84\x01Ra6\xFD`\xC0\x84\x01\x82a4HV[\x90P`@\x85\x01Q`_\x19\x84\x83\x03\x01`\xA0\x85\x01Ra7\x1A\x82\x82a4HV[\x91PP\x82\x81\x03`@\x84\x01Ra6\x94\x81\x85a6-V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0a7r`\xC0\x83\x01\x87a4HV[``\x83\x01\x86\x90R\x82\x81\x03`\x80\x84\x01R\x84Q\x80\x82R` \x80\x87\x01\x92\x01\x90`\0[\x81\x81\x10\x15a7\xCEW\x83Q\x80Q\x84R` \x81\x01Q` \x85\x01R`\xFF`@\x82\x01Q\x16`@\x85\x01RP``\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa7\x91V[PP`\x01`\x01`\xA0\x1B\x03\x85\x16`\xA0\x85\x01R\x91Pa7\xE8\x90PV[\x97\x96PPPPPPPV[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa8%\x81`\x17\x85\x01` \x88\x01a4$V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa8V\x81`(\x84\x01` \x88\x01a4$V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x06\xA3` \x83\x01\x84a4HV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x85Wa\x04\x85a8uV[\x80\x82\x01\x80\x82\x11\x15a\x04\x85Wa\x04\x85a8uV[`\0\x81a8\xC4Wa8\xC4a8uV[P`\0\x19\x01\x90V\xFEPKPHelper: auth method type and \xA2dipfsX\"\x12 \t\xDC\x917<\xBE\xD6#I\x87\xFF\x9E\xA9\xBA\xC8\x0C\xF4\x06(\xBD\xA5\x98\xC8v\xF9\xB2?\xC0=\xA6\xA8\xD2dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static PKPHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01<W`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\x01AW\x80c\x0E\x9E\xD6\x8B\x14a\x01vW\x80c\x13\xAFA\x1B\x14a\x01\x98W\x80c\x15\x0Bz\x02\x14a\x01\xB9W\x80c /rO\x14a\x01\xF2W\x80c$\x8A\x9C\xA3\x14a\x02\x05W\x80c+U5Q\x14a\x02%W\x80c//\xF1]\x14a\x02GW\x80c2vU\x8C\x14a\x02gW\x80c6V\x8A\xBE\x14a\x02|W\x80cPC\x02l\x14a\x02\x9CW\x80cP\xD1{^\x14a\x02\xB1W\x80cqP\x18\xA6\x14a\x02\xD1W\x80cs\xCCA\x11\x14a\x02\xE6W\x80cw\x8F\xE5r\x14a\x02\xFBW\x80cx..\xA5\x14a\x03\x0EW\x80c\x8D\xA5\xCB[\x14a\x03.W\x80c\x91\xD1HT\x14a\x03CW\x80c\x91\xEEO\xD5\x14a\x03cW\x80c\x9D\xCA\x002\x14a\x03vW\x80c\xA2\x17\xFD\xDF\x14a\x03\xA4W\x80c\xCA\xEA\xD0\xC7\x14a\x03\xB9W\x80c\xD5Gt\x1F\x14a\x03\xCEW\x80c\xDB\x0B\xF93\x14a\x03\xEEW\x80c\xE4\xF1\x1D\xF6\x14a\x04\x01W\x80c\xF2\xFD\xE3\x8B\x14a\x04\x14W\x80c\xF9]q\xB1\x14a\x044W[`\0\x80\xFD[4\x80\x15a\x01MW`\0\x80\xFD[Pa\x01aa\x01\\6`\x04a%\x88V[a\x04TV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x82W`\0\x80\xFD[Pa\x01\x8Ba\x04\x8BV[`@Qa\x01m\x91\x90a%\xB2V[a\x01\xABa\x01\xA66`\x04a+tV[a\x05vV[`@Q\x90\x81R` \x01a\x01mV[4\x80\x15a\x01\xC5W`\0\x80\xFD[Pa\x01\xD9a\x01\xD46`\x04a,\"V[a\x05\xF3V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01mV[a\x01\xABa\x02\x006`\x04a+tV[a\x06\x97V[4\x80\x15a\x02\x11W`\0\x80\xFD[Pa\x01\xABa\x02 6`\x04a,\xC1V[a\x06\xAAV[4\x80\x15a\x021W`\0\x80\xFD[Pa\x02Ea\x02@6`\x04a,\xC1V[a\x06\xC0V[\0[4\x80\x15a\x02SW`\0\x80\xFD[Pa\x02Ea\x02b6`\x04a,\xDAV[a\x08\x9BV[4\x80\x15a\x02sW`\0\x80\xFD[Pa\x01\x8Ba\x08\xBCV[4\x80\x15a\x02\x88W`\0\x80\xFD[Pa\x02Ea\x02\x976`\x04a,\xDAV[a\t\x0EV[4\x80\x15a\x02\xA8W`\0\x80\xFD[Pa\x01\x8Ba\t\x8CV[4\x80\x15a\x02\xBDW`\0\x80\xFD[P`\x02Ta\x01\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xDDW`\0\x80\xFD[Pa\x02Ea\t\xDEV[4\x80\x15a\x02\xF2W`\0\x80\xFD[Pa\x01\x8Ba\t\xF2V[a\x01\xABa\x03\t6`\x04a-*V[a\nDV[4\x80\x15a\x03\x1AW`\0\x80\xFD[Pa\x02Ea\x03)6`\x04a/\xA8V[a\x10VV[4\x80\x15a\x03:W`\0\x80\xFD[Pa\x01\x8Ba\x12yV[4\x80\x15a\x03OW`\0\x80\xFD[Pa\x01aa\x03^6`\x04a,\xDAV[a\x12\x88V[a\x01\xABa\x03q6`\x04a/\xE4V[a\x12\xB3V[4\x80\x15a\x03\x82W`\0\x80\xFD[P`\x02Ta\x03\x97\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x01m\x91\x90a0\xE0V[4\x80\x15a\x03\xB0W`\0\x80\xFD[Pa\x01\xAB`\0\x81V[4\x80\x15a\x03\xC5W`\0\x80\xFD[Pa\x01\x8Ba\x19UV[4\x80\x15a\x03\xDAW`\0\x80\xFD[Pa\x02Ea\x03\xE96`\x04a,\xDAV[a\x19\xA7V[a\x01\xABa\x03\xFC6`\x04a0\xEEV[a\x19\xC3V[a\x01\xABa\x04\x0F6`\x04a2+V[a\x1F\xCDV[4\x80\x15a\x04 W`\0\x80\xFD[Pa\x02Ea\x04/6`\x04a3=V[a! V[4\x80\x15a\x04@W`\0\x80\xFD[Pa\x02Ea\x04O6`\x04a3=V[a!\x99V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x85WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[`\x02T`@\x80Qc\xDA\x19\xDD\xFB`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\xDA\x19\xDD\xFB\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x01\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x050\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05q\x91\x90a3\x87V[\x90P\x90V[`\0\x80`@Q\x80`\xA0\x01`@R\x80\x85`\0\x01Q\x81R` \x01`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01knaga-keyset1`\xA0\x1B\x81RP\x81R` \x01\x85` \x01Q\x81R` \x01\x85`@\x01Q\x81R` \x01a\x05\xD4a\x04\x8BV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x05\xEB\x81\x84a\x12\xB3V[\x94\x93PPPPV[`\0a\x05\xFDa\x19UV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FPKPHelper: only accepts transfer`D\x82\x01Ry\x1C\xC8\x19\x9C\x9B\xDBH\x1D\x1A\x19H\x14\x12\xD4\x13\x91\x95\x08\x18\xDB\xDB\x9D\x1C\x98X\xDD`2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[Pc\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[`\0a\x06\xA3\x83\x83a\x05vV[\x93\x92PPPV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x01T\x90V[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x07\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x076\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x07e\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA6\x91\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a3\xA4V[`\0a\x07\xE0a\t\x8CV[`@Qc\xB6:vw`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB6:vw\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x089W=`\0\x80>=`\0\xFD[PP`@Qc(\xCD\x10\xC7`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92PcQ\x9A!\x8E\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x7FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x93W=`\0\x80>=`\0\xFD[PPPPPPV[a\x08\xA4\x82a\x06\xAAV[a\x08\xAD\x81a!\xF7V[a\x08\xB7\x83\x83a\"\x01V[PPPV[`\x02T`@\x80Qc\x12\x0E_\x07`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x90r\xF88\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\t~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x06|V[a\t\x88\x82\x82a\"lV[PPV[`\x02T`@\x80Qc\x16\xF7k\xBF`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c\x16\xF7k\xBF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[a\t\xE6a\"\xD3V[a\t\xF0`\0a#2V[V[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[`\0\x80a\nOa\x19UV[\x83Q` \x85\x01Q`@Qc?\xF8\x06\x97`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92c\x7F\xF0\r.\x924\x92a\n\x84\x92`\x04\x01a4tV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\n\xA2W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC7\x91\x90a3ZV[\x90P\x82``\x01QQ\x83`@\x01QQ\x14a\n\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\x8DV[\x82`\xA0\x01QQ\x83`\x80\x01QQ\x14a\x0B\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\xE3V[\x82`\xE0\x01QQ\x83`\xC0\x01QQ\x14a\x0BDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a58V[\x82a\x01\0\x01QQ\x83`\xC0\x01QQ\x14a\x0BnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\x81V[\x82a\x01 \x01QQ\x83`\xC0\x01QQ\x14a\x0B\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\xCCV[`@\x83\x01QQ\x15a\x0CdW`\0[\x83`@\x01QQ\x81\x10\x15a\x0CbWa\x0B\xBBa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x8AC\x15x\x83\x86`@\x01Q\x84\x81Q\x81\x10a\x0B\xE0Wa\x0B\xE0a6\x17V[` \x02` \x01\x01Q\x87``\x01Q\x85\x81Q\x81\x10a\x0B\xFEWa\x0B\xFEa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C$\x93\x92\x91\x90a6iV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C>W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0CRW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x0B\xA6\x90PV[P[`\x80\x83\x01QQ\x15a\r0W`\0[\x83`\x80\x01QQ\x81\x10\x15a\r.Wa\x0C\x87a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x86`\x80\x01Q\x84\x81Q\x81\x10a\x0C\xACWa\x0C\xACa6\x17V[` \x02` \x01\x01Q\x87`\xA0\x01Q\x85\x81Q\x81\x10a\x0C\xCAWa\x0C\xCAa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xF0\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\x1EW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x0Cr\x90PV[P[`\xC0\x83\x01QQ\x15a\x0ERW`\0[\x83`\xC0\x01QQ\x81\x10\x15a\x0EPWa\rSa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x88`\xC0\x01Q\x86\x81Q\x81\x10a\r\x83Wa\r\x83a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x88`\xE0\x01Q\x86\x81Q\x81\x10a\r\xA6Wa\r\xA6a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x88a\x01\0\x01Q\x86\x81Q\x81\x10a\r\xCAWa\r\xCAa6\x17V[` \x02` \x01\x01Q\x81RP\x87a\x01 \x01Q\x85\x81Q\x81\x10a\r\xECWa\r\xECa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x12\x93\x92\x91\x90a6\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E@W=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\r>\x90PV[P[`\0a\x0E\\a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCA\x91\x90a3\x87V[\x90P\x83a\x01@\x01Q\x15a\x0FlWa\x0E\xDFa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x83`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x1AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F9\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FSW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[PPPP[\x83a\x01`\x01Q\x15a\x0F\xE5Wa\x0F\x7Fa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xAE\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xC8W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xDCW=`\0\x80>=`\0\xFD[PPPPa\x10OV[a\x0F\xEDa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x1C\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x106W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10JW=`\0\x80>=`\0\xFD[PPPP[P\x92\x91PPV[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x10\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCC\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x10\xFB\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11<\x91\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a3\xA4V[`\0a\x11va\t\x8CV[\x82Q\x90\x91P\x15a\x08\xB7W\x80`\x01`\x01`\xA0\x1B\x03\x16c\x85^\xEC\"\x84\x84`\0\x81Q\x81\x10a\x11\xA3Wa\x11\xA3a6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xC8\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xE2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF6W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x90\0\xFE\xE1\x84\x84`\x01\x81Q\x81\x10a\x12\x1DWa\x12\x1Da6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12B\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12pW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0\x91\x82R`\x01` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[\x80Q\x82Q`\0\x91\x14a\x13-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FPKPHelper: Claim key type must m`D\x82\x01R\x7Fatch Auth Method data key type\0\0`d\x82\x01R`\x84\x01a\x06|V[`\x01`\0a\x139a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cq\xAA\x9A\xCF4\x84\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Q\x8B``\x01Q\x8C`\x80\x01Q`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x83\x96\x95\x94\x93\x92\x91\x90a7SV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x13\xA1W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xC6\x91\x90a3ZV[\x90P\x83`@\x01QQ\x84` \x01QQ\x14a\x13\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\x8DV[\x83`\x80\x01QQ\x84``\x01QQ\x14a\x14\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a4\xE3V[\x83`\xC0\x01QQ\x84`\xA0\x01QQ\x14a\x14CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a58V[\x83`\xE0\x01QQ\x84`\xA0\x01QQ\x14a\x14lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\x81V[\x83a\x01\0\x01QQ\x84`\xA0\x01QQ\x14a\x14\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\xCCV[` \x84\x01QQ\x15a\x15bW`\0[\x84` \x01QQ\x81\x10\x15a\x15`Wa\x14\xB9a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x8AC\x15x\x83\x87` \x01Q\x84\x81Q\x81\x10a\x14\xDEWa\x14\xDEa6\x17V[` \x02` \x01\x01Q\x88`@\x01Q\x85\x81Q\x81\x10a\x14\xFCWa\x14\xFCa6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\"\x93\x92\x91\x90a6iV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15PW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x14\xA4\x90PV[P[``\x84\x01QQ\x15a\x16.W`\0[\x84``\x01QQ\x81\x10\x15a\x16,Wa\x15\x85a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x87``\x01Q\x84\x81Q\x81\x10a\x15\xAAWa\x15\xAAa6\x17V[` \x02` \x01\x01Q\x88`\x80\x01Q\x85\x81Q\x81\x10a\x15\xC8Wa\x15\xC8a6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xEE\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x08W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x1CW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x15p\x90PV[P[`\xA0\x84\x01QQ\x15a\x17OW`\0[\x84`\xA0\x01QQ\x81\x10\x15a\x17MWa\x16Qa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x89`\xA0\x01Q\x86\x81Q\x81\x10a\x16\x81Wa\x16\x81a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x89`\xC0\x01Q\x86\x81Q\x81\x10a\x16\xA4Wa\x16\xA4a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x89`\xE0\x01Q\x86\x81Q\x81\x10a\x16\xC7Wa\x16\xC7a6\x17V[` \x02` \x01\x01Q\x81RP\x88a\x01\0\x01Q\x85\x81Q\x81\x10a\x16\xE9Wa\x16\xE9a6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x0F\x93\x92\x91\x90a6\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17)W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17=W=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x16<\x90PV[P[`\0a\x17Ya\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x86\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC7\x91\x90a3\x87V[\x90P\x84a\x01 \x01Q\x15a\x18iWa\x17\xDCa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x83`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x17W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x186\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18PW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18dW=`\0\x80>=`\0\xFD[PPPP[\x84a\x01@\x01Q\x15a\x18\xE2Wa\x18|a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xAB\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x18\xD9W=`\0\x80>=`\0\xFD[PPPPa\x19LV[a\x18\xEAa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x19\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x193W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19GW=`\0\x80>=`\0\xFD[PPPP[P\x94\x93PPPPV[`\x02T`@\x80Qc,\x0B\x8B\xF7`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91c,\x0B\x8B\xF7\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\xDDW=`\0\x80>=`\0\xFD[a\x19\xB0\x82a\x06\xAAV[a\x19\xB9\x81a!\xF7V[a\x08\xB7\x83\x83a\"lV[`\x02T`@\x80Qc!\x0Bs\x9D`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8E\x8D\xFD\x16\x91\x83\x91cB\x16\xE7:\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1A\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A9\x91\x90a3ZV[`\x02T`@Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81Ra\x1Ah\x92\x91`\x01`\xA0\x1B\x90\x04`\xFF\x16\x90`\x04\x01a3sV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA9\x91\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a3\xA4V[`\0a\x1A\xE3a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xF0\r.4\x8D\x8D`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x11\x92\x91\x90a4tV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x1B/W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BT\x91\x90a3ZV[\x90P\x87Q\x89Q\x14a\x1BwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a58V[\x86Q\x89Q\x14a\x1B\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\x81V[\x85Q\x89Q\x14a\x1B\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06|\x90a5\xCCV[\x88Q\x15a\x1C\xC1W`\0[\x89Q\x81\x10\x15a\x1C\xBFWa\x1B\xD4a\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xD44\x9B\x83`@Q\x80``\x01`@R\x80\x8E\x86\x81Q\x81\x10a\x1C\0Wa\x1C\0a6\x17V[` \x02` \x01\x01Q\x81R` \x01\x8D\x86\x81Q\x81\x10a\x1C\x1FWa\x1C\x1Fa6\x17V[` \x02` \x01\x01Q\x81R` \x01\x8C\x86\x81Q\x81\x10a\x1C>Wa\x1C>a6\x17V[` \x02` \x01\x01Q\x81RP\x8A\x85\x81Q\x81\x10a\x1C[Wa\x1C[a6\x17V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x81\x93\x92\x91\x90a6\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\xAFW=`\0\x80>=`\0\xFD[PP`\x01\x90\x92\x01\x91Pa\x1B\xC3\x90PV[P[`\0a\x1C\xCBa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\xBDI\x86\xA0\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xF8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D9\x91\x90a3\x87V[\x90P\x84\x15a\x1D\xD6Wa\x1DIa\x08\xBCV[`\x01`\x01`\xA0\x1B\x03\x16c\x16c\xC1!\x83\x83`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\xA3\x93\x92\x91\x90a6\x9EV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xBDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D\xD1W=`\0\x80>=`\0\xFD[PPPP[\x83\x15a\x1EJWa\x1D\xE4a\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E0\x83\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x13\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1EAW=`\0\x80>=`\0\xFD[PPPPa\x1E\xB4V[a\x1ERa\x19UV[`\x01`\x01`\xA0\x1B\x03\x16cB\x84.\x0E03\x85`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x81\x93\x92\x91\x90a7/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\x9BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\xAFW=`\0\x80>=`\0\xFD[PPPP[\x85Q\x15a\x1F\xBEWa\x1E\xC3a\t\x8CV[`\x01`\x01`\xA0\x1B\x03\x16c\x85^\xEC\"\x83\x88`\0\x81Q\x81\x10a\x1E\xE5Wa\x1E\xE5a6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\n\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F8W=`\0\x80>=`\0\xFD[PPPPa\x1FDa\t\x8CV[`\x01`\x01`\xA0\x1B\x03\x16c\x90\0\xFE\xE1\x83\x88`\x01\x81Q\x81\x10a\x1FfWa\x1Ffa6\x17V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\x8B\x92\x91\x90a4tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xB9W=`\0\x80>=`\0\xFD[PPPP[P\x9A\x99PPPPPPPPPPV[`\0\x80`@Q\x80a\x01\x80\x01`@R\x80\x8B\x81R` \x01\x8A\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x81\x11\x15a \x01Wa \x01a%\xC6V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a 4W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x1FW\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a oW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a ZW\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xA0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x01`\0`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xDBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xC6W\x90P[P\x81R` \x01\x89\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x15\x15\x81R` \x01\x84\x15\x15\x81RP\x90Pa!\x12\x81a\nDV[\x9A\x99PPPPPPPPPPV[a!(a\"\xD3V[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06|V[a!\x96\x81a#2V[PV[a!\xA1a\"\xD3V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U`@Q\x7F'`\x07<|\xD8\xCA\xC51\xD7\xF6C\xBE\xCB\xFB\xB7M\x8B\x81VD>\xAC\xF8yb%2\xDB\xBB<\xD5\x90a!\xEC\x90\x83\x90a%\xB2V[`@Q\x80\x91\x03\x90\xA1PV[a!\x96\x813a#\x82V[a\"\x0B\x82\x82a\x12\x88V[a\t\x88W`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x86R\x92R\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U\x90Q3\x92\x85\x91\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r\x91\x90\xA4PPV[a\"v\x82\x82a\x12\x88V[\x15a\t\x88W`\0\x82\x81R`\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[3a\"\xDCa\x12yV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06|V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a#\x8C\x82\x82a\x12\x88V[a\t\x88Wa#\x99\x81a#\xDBV[a#\xA4\x83` a#\xEDV[`@Q` \x01a#\xB5\x92\x91\x90a7\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x06|\x91`\x04\x01a8bV[``a\x04\x85`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a#\xFC\x83`\x02a8\x8BV[a$\x07\x90`\x02a8\xA2V[`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x1EWa$\x1Ea%\xC6V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$HW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a$cWa$ca6\x17V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a$\x92Wa$\x92a6\x17V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a$\xB6\x84`\x02a8\x8BV[a$\xC1\x90`\x01a8\xA2V[\x90P[`\x01\x81\x11\x15a%9Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a$\xF5Wa$\xF5a6\x17V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a%\x0BWa%\x0Ba6\x17V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a%2\x81a8\xB5V[\x90Pa$\xC4V[P\x83\x15a\x06\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x06|V[`\0` \x82\x84\x03\x12\x15a%\x9AW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x06\xA3W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@R\x90V[`@Qa\x01`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@Qa\x01\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC6V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\x94Wa&\x94a%\xC6V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a&\xB5Wa&\xB5a%\xC6V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a&\xD0W`\0\x80\xFD[\x815a&\xE3a&\xDE\x82a&\x9CV[a&lV[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x86\x01\x01\x92P\x85\x83\x11\x15a'\x05W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW``\x81\x88\x03\x12\x15a'\"W`\0\x80\xFD[a'*a%\xDCV[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x82\x015`\xFF\x81\x16\x81\x14a'LW`\0\x80\xFD[`@\x82\x01R\x83R` \x90\x92\x01\x91``\x01a'\nV[P\x95\x94PPPPPV[`\0\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a'\x85Wa'\x85a%\xC6V[P`\x1F\x83\x01`\x1F\x19\x16` \x01a'\x9A\x81a&lV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15a'\xAFW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a'\xD7W`\0\x80\xFD[\x815a'\xE5a&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x07W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a(*W`\0\x80\xFD[\x86\x01`?\x81\x01\x88\x13a(;W`\0\x80\xFD[a(M\x88` \x83\x015`@\x84\x01a'kV[\x84RP` \x92\x83\x01\x92\x01a(\x0CV[`\0\x82`\x1F\x83\x01\x12a(mW`\0\x80\xFD[\x815a({a&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x9DW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805\x83R` \x92\x83\x01\x92\x01a(\xA2V[`\0\x82`\x1F\x83\x01\x12a(\xCBW`\0\x80\xFD[\x815a(\xD9a&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\xFBW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x1EW`\0\x80\xFD[a)-\x88` \x83\x8A\x01\x01a(\\V[\x84RP` \x92\x83\x01\x92\x01a)\0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x96W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a)bW`\0\x80\xFD[\x815a)pa&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a)\x92W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805a)\xAA\x81a)<V[\x83R` \x92\x83\x01\x92\x01a)\x97V[\x805\x80\x15\x15\x81\x14a)\xC8W`\0\x80\xFD[\x91\x90PV[`\0a\x01`\x82\x84\x03\x12\x15a)\xE0W`\0\x80\xFD[a)\xE8a&\x04V[\x825\x81R\x90P` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x07W`\0\x80\xFD[a*\x13\x84\x82\x85\x01a'\xC6V[` \x83\x01RP`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*2W`\0\x80\xFD[a*>\x84\x82\x85\x01a(\xBAV[`@\x83\x01RP``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*]W`\0\x80\xFD[a*i\x84\x82\x85\x01a)QV[``\x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\x88W`\0\x80\xFD[a*\x94\x84\x82\x85\x01a(\xBAV[`\x80\x83\x01RP`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xB3W`\0\x80\xFD[a*\xBF\x84\x82\x85\x01a(\\V[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xDEW`\0\x80\xFD[a*\xEA\x84\x82\x85\x01a'\xC6V[`\xC0\x83\x01RP`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\tW`\0\x80\xFD[a+\x15\x84\x82\x85\x01a'\xC6V[`\xE0\x83\x01RPa\x01\0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+5W`\0\x80\xFD[a+A\x84\x82\x85\x01a(\xBAV[a\x01\0\x83\x01RPa+Ua\x01 \x83\x01a)\xB8V[a\x01 \x82\x01Ra+ha\x01@\x83\x01a)\xB8V[a\x01@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a+\x87W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a+\x9DW`\0\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a+\xAFW`\0\x80\xFD[a+\xB7a%\xDCV[\x815\x81R` \x80\x83\x015\x90\x82\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xDEW`\0\x80\xFD[a+\xEA\x87\x82\x85\x01a&\xBFV[`@\x83\x01RP\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x0CW`\0\x80\xFD[a,\x18\x85\x82\x86\x01a)\xCDV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a,:W`\0\x80\xFD[\x855a,E\x81a)<V[\x94P` \x86\x015a,U\x81a)<V[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,wW`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a,\x88W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x9EW`\0\x80\xFD[\x88` \x82\x84\x01\x01\x11\x15a,\xB0W`\0\x80\xFD[\x95\x98\x94\x97P\x92\x95PPP` \x01\x91\x90V[`\0` \x82\x84\x03\x12\x15a,\xD3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xEDW`\0\x80\xFD[\x825\x91P` \x83\x015a,\xFF\x81a)<V[\x80\x91PP\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12a-\x1BW`\0\x80\xFD[a\x06\xA3\x83\x835` \x85\x01a'kV[`\0` \x82\x84\x03\x12\x15a-<W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-RW`\0\x80\xFD[\x82\x01a\x01\x80\x81\x85\x03\x12\x15a-eW`\0\x80\xFD[a-ma&'V[\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x8AW`\0\x80\xFD[a-\x96\x86\x82\x85\x01a-\nV[` \x83\x01RP`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xB5W`\0\x80\xFD[a-\xC1\x86\x82\x85\x01a'\xC6V[`@\x83\x01RP``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xE0W`\0\x80\xFD[a-\xEC\x86\x82\x85\x01a(\xBAV[``\x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x0BW`\0\x80\xFD[a.\x17\x86\x82\x85\x01a)QV[`\x80\x83\x01RP`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.6W`\0\x80\xFD[a.B\x86\x82\x85\x01a(\xBAV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.aW`\0\x80\xFD[a.m\x86\x82\x85\x01a(\\V[`\xC0\x83\x01RP`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x8CW`\0\x80\xFD[a.\x98\x86\x82\x85\x01a'\xC6V[`\xE0\x83\x01RPa\x01\0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xB8W`\0\x80\xFD[a.\xC4\x86\x82\x85\x01a'\xC6V[a\x01\0\x83\x01RPa\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xE5W`\0\x80\xFD[a.\xF1\x86\x82\x85\x01a(\xBAV[a\x01 \x83\x01RPa/\x05a\x01@\x83\x01a)\xB8V[a\x01@\x82\x01Ra/\x18a\x01`\x83\x01a)\xB8V[a\x01`\x82\x01R\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a/7W`\0\x80\xFD[\x815a/Ea&\xDE\x82a&\x9CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a/gW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a'aW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x8AW`\0\x80\xFD[a/\x99\x88` \x83\x8A\x01\x01a-\nV[\x84RP` \x92\x83\x01\x92\x01a/lV[`\0\x80`@\x83\x85\x03\x12\x15a/\xBBW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xD8W`\0\x80\xFD[a,\x18\x85\x82\x86\x01a/&V[`\0\x80`@\x83\x85\x03\x12\x15a/\xF7W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a0\rW`\0\x80\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15a0\x1FW`\0\x80\xFD[a0'a&JV[\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0DW`\0\x80\xFD[a0P\x87\x82\x85\x01a-\nV[` \x83\x01RP`@\x82\x81\x015\x90\x82\x01R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0yW`\0\x80\xFD[a0\x85\x87\x82\x85\x01a&\xBFV[``\x83\x01RP`\x80\x82\x015\x91Pa0\x9B\x82a)<V[`\x80\x81\x01\x91\x90\x91R\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x0CW`\0\x80\xFD[`\x03\x81\x10a0\xDCWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x04\x85\x82\x84a0\xBEV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a1\rW`\0\x80\xFD[\x895\x98P` \x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1*W`\0\x80\xFD[a16\x8C\x82\x8D\x01a-\nV[\x98PP`@\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1RW`\0\x80\xFD[a1^\x8C\x82\x8D\x01a(\\V[\x97PP``\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1zW`\0\x80\xFD[a1\x86\x8C\x82\x8D\x01a'\xC6V[\x96PP`\x80\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xA2W`\0\x80\xFD[a1\xAE\x8C\x82\x8D\x01a'\xC6V[\x95PP`\xA0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xCAW`\0\x80\xFD[a1\xD6\x8C\x82\x8D\x01a(\xBAV[\x94PP`\xC0\x8A\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xF2W`\0\x80\xFD[a1\xFE\x8C\x82\x8D\x01a/&V[\x93PPa2\r`\xE0\x8B\x01a)\xB8V[\x91Pa2\x1Ca\x01\0\x8B\x01a)\xB8V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a2HW`\0\x80\xFD[\x885\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2eW`\0\x80\xFD[a2q\x8B\x82\x8C\x01a-\nV[\x97PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x8DW`\0\x80\xFD[a2\x99\x8B\x82\x8C\x01a(\\V[\x96PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xB5W`\0\x80\xFD[a2\xC1\x8B\x82\x8C\x01a'\xC6V[\x95PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xDDW`\0\x80\xFD[a2\xE9\x8B\x82\x8C\x01a'\xC6V[\x94PP`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x05W`\0\x80\xFD[a3\x11\x8B\x82\x8C\x01a(\xBAV[\x93PPa3 `\xC0\x8A\x01a)\xB8V[\x91Pa3.`\xE0\x8A\x01a)\xB8V[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0` \x82\x84\x03\x12\x15a3OW`\0\x80\xFD[\x815a\x06\xA3\x81a)<V[`\0` \x82\x84\x03\x12\x15a3lW`\0\x80\xFD[PQ\x91\x90PV[\x82\x81R`@\x81\x01a\x06\xA3` \x83\x01\x84a0\xBEV[`\0` \x82\x84\x03\x12\x15a3\x99W`\0\x80\xFD[\x81Qa\x06\xA3\x81a)<V[` \x80\x82R`Z\x90\x82\x01R\x7FPKPHelper: only the Domain Walle`@\x82\x01R\x7Ft registry is allowed to mint do``\x82\x01Rymain wallets, who are you?`0\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0[\x83\x81\x10\x15a4?W\x81\x81\x01Q\x83\x82\x01R` \x01a4'V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra4`\x81` \x86\x01` \x86\x01a4$V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x05\xEB`@\x83\x01\x84a4HV[` \x80\x82R`6\x90\x82\x01R\x7FPKPHelper: ipfs cid and scope ar`@\x82\x01Ru\x0EL/$\r\x8C\xAD\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`S\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`5\x90\x82\x01R\x7FPKPHelper: address and scope arr`@\x82\x01Rt\x0C/$\r\x8C\xAD\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`[\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`;\x90\x82\x01R`\0\x80Q` a8\xCD\x839\x81Q\x91R`@\x82\x01Rz\r,\x84\x0C.NL/$\r\x8C\xAD\xCC\xEE\x8D\x0Ed\r\xAE\xAEn\x84\r\xAC.\x8Cm`+\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`?\x90\x82\x01R`\0\x80Q` a8\xCD\x839\x81Q\x91R`@\x82\x01R\x7Fpubkey array lengths must match\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`?\x90\x82\x01R`\0\x80Q` a8\xCD\x839\x81Q\x91R`@\x82\x01R\x7Fscopes array lengths must match\0``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a6_W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a6AV[P\x93\x94\x93PPPPV[\x83\x81R``` \x82\x01R`\0a6\x82``\x83\x01\x85a4HV[\x82\x81\x03`@\x84\x01Ra6\x94\x81\x85a6-V[\x96\x95PPPPPPV[\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a6\xC8\x90\x83\x01\x84a6-V[\x95\x94PPPPPV[\x83\x81R``` \x82\x01R\x82Q``\x82\x01R`\0` \x84\x01Q```\x80\x84\x01Ra6\xFD`\xC0\x84\x01\x82a4HV[\x90P`@\x85\x01Q`_\x19\x84\x83\x03\x01`\xA0\x85\x01Ra7\x1A\x82\x82a4HV[\x91PP\x82\x81\x03`@\x84\x01Ra6\x94\x81\x85a6-V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0a7r`\xC0\x83\x01\x87a4HV[``\x83\x01\x86\x90R\x82\x81\x03`\x80\x84\x01R\x84Q\x80\x82R` \x80\x87\x01\x92\x01\x90`\0[\x81\x81\x10\x15a7\xCEW\x83Q\x80Q\x84R` \x81\x01Q` \x85\x01R`\xFF`@\x82\x01Q\x16`@\x85\x01RP``\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa7\x91V[PP`\x01`\x01`\xA0\x1B\x03\x85\x16`\xA0\x85\x01R\x91Pa7\xE8\x90PV[\x97\x96PPPPPPPV[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa8%\x81`\x17\x85\x01` \x88\x01a4$V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa8V\x81`(\x84\x01` \x88\x01a4$V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x06\xA3` \x83\x01\x84a4HV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x85Wa\x04\x85a8uV[\x80\x82\x01\x80\x82\x11\x15a\x04\x85Wa\x04\x85a8uV[`\0\x81a8\xC4Wa8\xC4a8uV[P`\0\x19\x01\x90V\xFEPKPHelper: auth method type and \xA2dipfsX\"\x12 \t\xDC\x917<\xBE\xD6#I\x87\xFF\x9E\xA9\xBA\xC8\x0C\xF4\x06(\xBD\xA5\x98\xC8v\xF9\xB2?\xC0=\xA6\xA8\xD2dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static PKPHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PKPHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PKPHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PKPHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PKPHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PKPHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PKPHelper)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PKPHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PKPHELPER_ABI.clone(),
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
                PKPHELPER_ABI.clone(),
                PKPHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimAndMintNextAndAddAuthMethods` (0x202f724f) function
        pub fn claim_and_mint_next_and_add_auth_methods(
            &self,
            claim_material: ClaimMaterial,
            auth_method_data: AuthMethodData,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([32, 47, 114, 79], (claim_material, auth_method_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimAndMintNextAndAddAuthMethodsWithTypes` (0x13af411b) function
        pub fn claim_and_mint_next_and_add_auth_methods_with_types(
            &self,
            claim_material: ClaimMaterial,
            auth_method_data: AuthMethodData,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([19, 175, 65, 27], (claim_material, auth_method_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimAndMintNextAndAddAuthMethodsWithTypesV2` (0x91ee4fd5) function
        pub fn claim_and_mint_next_and_add_auth_methods_with_types_v2(
            &self,
            claim_material: ClaimMaterialV2,
            auth_method_data: AuthMethodData,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([145, 238, 79, 213], (claim_material, auth_method_data))
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
        ///Calls the contract's `getDomainWalletRegistry` (0x73cc4111) function
        pub fn get_domain_wallet_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([115, 204, 65, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPKPNftMetdataAddress` (0x5043026c) function
        pub fn get_pkp_nft_metdata_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 67, 2, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPkpNftAddress` (0xcaead0c7) function
        pub fn get_pkp_nft_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([202, 234, 208, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPkpPermissionsAddress` (0x3276558c) function
        pub fn get_pkp_permissions_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([50, 118, 85, 140], ())
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
        ///Calls the contract's `getStakingAddress` (0x0e9ed68b) function
        pub fn get_staking_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([14, 158, 214, 139], ())
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
        ///Calls the contract's `mintNextAndAddAuthMethods` (0xe4f11df6) function
        pub fn mint_next_and_add_auth_methods(
            &self,
            key_type: ::ethers::core::types::U256,
            key_set_id: ::std::string::String,
            permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
            permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
            permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
            permitted_auth_method_scopes: ::std::vec::Vec<
                ::std::vec::Vec<::ethers::core::types::U256>,
            >,
            add_pkp_eth_address_as_permitted_address: bool,
            send_pkp_to_itself: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [228, 241, 29, 246],
                    (
                        key_type,
                        key_set_id,
                        permitted_auth_method_types,
                        permitted_auth_method_ids,
                        permitted_auth_method_pubkeys,
                        permitted_auth_method_scopes,
                        add_pkp_eth_address_as_permitted_address,
                        send_pkp_to_itself,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintNextAndAddAuthMethodsWithTypes` (0x778fe572) function
        pub fn mint_next_and_add_auth_methods_with_types(
            &self,
            params: MintNextAndAddAuthMethodsWithTypesParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 143, 229, 114], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintNextAndAddDomainWalletMetadata` (0xdb0bf933) function
        pub fn mint_next_and_add_domain_wallet_metadata(
            &self,
            key_type: ::ethers::core::types::U256,
            key_set_id: ::std::string::String,
            permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
            permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
            permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
            permitted_auth_method_scopes: ::std::vec::Vec<
                ::std::vec::Vec<::ethers::core::types::U256>,
            >,
            nft_metadata: ::std::vec::Vec<::std::string::String>,
            add_pkp_eth_address_as_permitted_address: bool,
            send_pkp_to_itself: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [219, 11, 249, 51],
                    (
                        key_type,
                        key_set_id,
                        permitted_auth_method_types,
                        permitted_auth_method_ids,
                        permitted_auth_method_pubkeys,
                        permitted_auth_method_scopes,
                        nft_metadata,
                        add_pkp_eth_address_as_permitted_address,
                        send_pkp_to_itself,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePkpMetadata` (0x2b553551) function
        pub fn remove_pkp_metadata(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 85, 53, 81], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
            new_resolver_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 93, 113, 177], new_resolver_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPkpMetadata` (0x782e2ea5) function
        pub fn set_pkp_metadata(
            &self,
            token_id: ::ethers::core::types::U256,
            nft_metadata: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 46, 46, 165], (token_id, nft_metadata))
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ContractResolverAddressSet` event
        pub fn contract_resolver_address_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractResolverAddressSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PKPHelperEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PKPHelper<M> {
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
        name = "ContractResolverAddressSet",
        abi = "ContractResolverAddressSet(address)"
    )]
    pub struct ContractResolverAddressSetFilter {
        pub new_resolver_address: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum PKPHelperEvents {
        ContractResolverAddressSetFilter(ContractResolverAddressSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PKPHelperEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ContractResolverAddressSetFilter::decode_log(log) {
                return Ok(PKPHelperEvents::ContractResolverAddressSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PKPHelperEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(PKPHelperEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(PKPHelperEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(PKPHelperEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PKPHelperEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractResolverAddressSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ContractResolverAddressSetFilter> for PKPHelperEvents {
        fn from(value: ContractResolverAddressSetFilter) -> Self {
            Self::ContractResolverAddressSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PKPHelperEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for PKPHelperEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for PKPHelperEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for PKPHelperEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `claimAndMintNextAndAddAuthMethods` function with signature `claimAndMintNextAndAddAuthMethods((uint256,bytes32,(bytes32,bytes32,uint8)[]),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x202f724f`
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
        name = "claimAndMintNextAndAddAuthMethods",
        abi = "claimAndMintNextAndAddAuthMethods((uint256,bytes32,(bytes32,bytes32,uint8)[]),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))"
    )]
    pub struct ClaimAndMintNextAndAddAuthMethodsCall {
        pub claim_material: ClaimMaterial,
        pub auth_method_data: AuthMethodData,
    }
    ///Container type for all input parameters for the `claimAndMintNextAndAddAuthMethodsWithTypes` function with signature `claimAndMintNextAndAddAuthMethodsWithTypes((uint256,bytes32,(bytes32,bytes32,uint8)[]),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x13af411b`
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
        name = "claimAndMintNextAndAddAuthMethodsWithTypes",
        abi = "claimAndMintNextAndAddAuthMethodsWithTypes((uint256,bytes32,(bytes32,bytes32,uint8)[]),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))"
    )]
    pub struct ClaimAndMintNextAndAddAuthMethodsWithTypesCall {
        pub claim_material: ClaimMaterial,
        pub auth_method_data: AuthMethodData,
    }
    ///Container type for all input parameters for the `claimAndMintNextAndAddAuthMethodsWithTypesV2` function with signature `claimAndMintNextAndAddAuthMethodsWithTypesV2((uint256,string,bytes32,(bytes32,bytes32,uint8)[],address),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x91ee4fd5`
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
        name = "claimAndMintNextAndAddAuthMethodsWithTypesV2",
        abi = "claimAndMintNextAndAddAuthMethodsWithTypesV2((uint256,string,bytes32,(bytes32,bytes32,uint8)[],address),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))"
    )]
    pub struct ClaimAndMintNextAndAddAuthMethodsWithTypesV2Call {
        pub claim_material: ClaimMaterialV2,
        pub auth_method_data: AuthMethodData,
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
    ///Container type for all input parameters for the `getDomainWalletRegistry` function with signature `getDomainWalletRegistry()` and selector `0x73cc4111`
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
    #[ethcall(name = "getDomainWalletRegistry", abi = "getDomainWalletRegistry()")]
    pub struct GetDomainWalletRegistryCall;
    ///Container type for all input parameters for the `getPKPNftMetdataAddress` function with signature `getPKPNftMetdataAddress()` and selector `0x5043026c`
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
    #[ethcall(name = "getPKPNftMetdataAddress", abi = "getPKPNftMetdataAddress()")]
    pub struct GetPKPNftMetdataAddressCall;
    ///Container type for all input parameters for the `getPkpNftAddress` function with signature `getPkpNftAddress()` and selector `0xcaead0c7`
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
    #[ethcall(name = "getPkpNftAddress", abi = "getPkpNftAddress()")]
    pub struct GetPkpNftAddressCall;
    ///Container type for all input parameters for the `getPkpPermissionsAddress` function with signature `getPkpPermissionsAddress()` and selector `0x3276558c`
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
    #[ethcall(name = "getPkpPermissionsAddress", abi = "getPkpPermissionsAddress()")]
    pub struct GetPkpPermissionsAddressCall;
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
    ///Container type for all input parameters for the `getStakingAddress` function with signature `getStakingAddress()` and selector `0x0e9ed68b`
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
    #[ethcall(name = "getStakingAddress", abi = "getStakingAddress()")]
    pub struct GetStakingAddressCall;
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
    ///Container type for all input parameters for the `mintNextAndAddAuthMethods` function with signature `mintNextAndAddAuthMethods(uint256,string,uint256[],bytes[],bytes[],uint256[][],bool,bool)` and selector `0xe4f11df6`
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
        name = "mintNextAndAddAuthMethods",
        abi = "mintNextAndAddAuthMethods(uint256,string,uint256[],bytes[],bytes[],uint256[][],bool,bool)"
    )]
    pub struct MintNextAndAddAuthMethodsCall {
        pub key_type: ::ethers::core::types::U256,
        pub key_set_id: ::std::string::String,
        pub permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub add_pkp_eth_address_as_permitted_address: bool,
        pub send_pkp_to_itself: bool,
    }
    ///Container type for all input parameters for the `mintNextAndAddAuthMethodsWithTypes` function with signature `mintNextAndAddAuthMethodsWithTypes((uint256,string,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x778fe572`
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
        name = "mintNextAndAddAuthMethodsWithTypes",
        abi = "mintNextAndAddAuthMethodsWithTypes((uint256,string,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))"
    )]
    pub struct MintNextAndAddAuthMethodsWithTypesCall {
        pub params: MintNextAndAddAuthMethodsWithTypesParams,
    }
    ///Container type for all input parameters for the `mintNextAndAddDomainWalletMetadata` function with signature `mintNextAndAddDomainWalletMetadata(uint256,string,uint256[],bytes[],bytes[],uint256[][],string[],bool,bool)` and selector `0xdb0bf933`
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
        name = "mintNextAndAddDomainWalletMetadata",
        abi = "mintNextAndAddDomainWalletMetadata(uint256,string,uint256[],bytes[],bytes[],uint256[][],string[],bool,bool)"
    )]
    pub struct MintNextAndAddDomainWalletMetadataCall {
        pub key_type: ::ethers::core::types::U256,
        pub key_set_id: ::std::string::String,
        pub permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub nft_metadata: ::std::vec::Vec<::std::string::String>,
        pub add_pkp_eth_address_as_permitted_address: bool,
        pub send_pkp_to_itself: bool,
    }
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `removePkpMetadata` function with signature `removePkpMetadata(uint256)` and selector `0x2b553551`
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
    #[ethcall(name = "removePkpMetadata", abi = "removePkpMetadata(uint256)")]
    pub struct RemovePkpMetadataCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
        pub new_resolver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPkpMetadata` function with signature `setPkpMetadata(uint256,string[])` and selector `0x782e2ea5`
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
    #[ethcall(name = "setPkpMetadata", abi = "setPkpMetadata(uint256,string[])")]
    pub struct SetPkpMetadataCall {
        pub token_id: ::ethers::core::types::U256,
        pub nft_metadata: ::std::vec::Vec<::std::string::String>,
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum PKPHelperCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        ClaimAndMintNextAndAddAuthMethods(ClaimAndMintNextAndAddAuthMethodsCall),
        ClaimAndMintNextAndAddAuthMethodsWithTypes(
            ClaimAndMintNextAndAddAuthMethodsWithTypesCall,
        ),
        ClaimAndMintNextAndAddAuthMethodsWithTypesV2(
            ClaimAndMintNextAndAddAuthMethodsWithTypesV2Call,
        ),
        ContractResolver(ContractResolverCall),
        Env(EnvCall),
        GetDomainWalletRegistry(GetDomainWalletRegistryCall),
        GetPKPNftMetdataAddress(GetPKPNftMetdataAddressCall),
        GetPkpNftAddress(GetPkpNftAddressCall),
        GetPkpPermissionsAddress(GetPkpPermissionsAddressCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetStakingAddress(GetStakingAddressCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        MintNextAndAddAuthMethods(MintNextAndAddAuthMethodsCall),
        MintNextAndAddAuthMethodsWithTypes(MintNextAndAddAuthMethodsWithTypesCall),
        MintNextAndAddDomainWalletMetadata(MintNextAndAddDomainWalletMetadataCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        RemovePkpMetadata(RemovePkpMetadataCall),
        RenounceOwnership(RenounceOwnershipCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetContractResolver(SetContractResolverCall),
        SetPkpMetadata(SetPkpMetadataCall),
        SupportsInterface(SupportsInterfaceCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for PKPHelperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <ClaimAndMintNextAndAddAuthMethodsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimAndMintNextAndAddAuthMethods(decoded));
            }
            if let Ok(decoded) = <ClaimAndMintNextAndAddAuthMethodsWithTypesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimAndMintNextAndAddAuthMethodsWithTypes(decoded));
            }
            if let Ok(decoded) = <ClaimAndMintNextAndAddAuthMethodsWithTypesV2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimAndMintNextAndAddAuthMethodsWithTypesV2(decoded));
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
            if let Ok(decoded) = <GetDomainWalletRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDomainWalletRegistry(decoded));
            }
            if let Ok(decoded) = <GetPKPNftMetdataAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPKPNftMetdataAddress(decoded));
            }
            if let Ok(decoded) = <GetPkpNftAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPkpNftAddress(decoded));
            }
            if let Ok(decoded) = <GetPkpPermissionsAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPkpPermissionsAddress(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetStakingAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakingAddress(decoded));
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
            if let Ok(decoded) = <MintNextAndAddAuthMethodsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintNextAndAddAuthMethods(decoded));
            }
            if let Ok(decoded) = <MintNextAndAddAuthMethodsWithTypesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintNextAndAddAuthMethodsWithTypes(decoded));
            }
            if let Ok(decoded) = <MintNextAndAddDomainWalletMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintNextAndAddDomainWalletMetadata(decoded));
            }
            if let Ok(decoded) = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemovePkpMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemovePkpMetadata(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
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
            if let Ok(decoded) = <SetPkpMetadataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPkpMetadata(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PKPHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimAndMintNextAndAddAuthMethods(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimAndMintNextAndAddAuthMethodsWithTypes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimAndMintNextAndAddAuthMethodsWithTypesV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Env(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDomainWalletRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPKPNftMetdataAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPkpNftAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPkpPermissionsAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakingAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintNextAndAddAuthMethods(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintNextAndAddAuthMethodsWithTypes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintNextAndAddDomainWalletMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemovePkpMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetContractResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPkpMetadata(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PKPHelperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimAndMintNextAndAddAuthMethods(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimAndMintNextAndAddAuthMethodsWithTypes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimAndMintNextAndAddAuthMethodsWithTypesV2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Env(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDomainWalletRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPKPNftMetdataAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPkpNftAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPkpPermissionsAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakingAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintNextAndAddAuthMethods(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintNextAndAddAuthMethodsWithTypes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintNextAndAddDomainWalletMetadata(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePkpMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetContractResolver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPkpMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for PKPHelperCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<ClaimAndMintNextAndAddAuthMethodsCall>
    for PKPHelperCalls {
        fn from(value: ClaimAndMintNextAndAddAuthMethodsCall) -> Self {
            Self::ClaimAndMintNextAndAddAuthMethods(value)
        }
    }
    impl ::core::convert::From<ClaimAndMintNextAndAddAuthMethodsWithTypesCall>
    for PKPHelperCalls {
        fn from(value: ClaimAndMintNextAndAddAuthMethodsWithTypesCall) -> Self {
            Self::ClaimAndMintNextAndAddAuthMethodsWithTypes(value)
        }
    }
    impl ::core::convert::From<ClaimAndMintNextAndAddAuthMethodsWithTypesV2Call>
    for PKPHelperCalls {
        fn from(value: ClaimAndMintNextAndAddAuthMethodsWithTypesV2Call) -> Self {
            Self::ClaimAndMintNextAndAddAuthMethodsWithTypesV2(value)
        }
    }
    impl ::core::convert::From<ContractResolverCall> for PKPHelperCalls {
        fn from(value: ContractResolverCall) -> Self {
            Self::ContractResolver(value)
        }
    }
    impl ::core::convert::From<EnvCall> for PKPHelperCalls {
        fn from(value: EnvCall) -> Self {
            Self::Env(value)
        }
    }
    impl ::core::convert::From<GetDomainWalletRegistryCall> for PKPHelperCalls {
        fn from(value: GetDomainWalletRegistryCall) -> Self {
            Self::GetDomainWalletRegistry(value)
        }
    }
    impl ::core::convert::From<GetPKPNftMetdataAddressCall> for PKPHelperCalls {
        fn from(value: GetPKPNftMetdataAddressCall) -> Self {
            Self::GetPKPNftMetdataAddress(value)
        }
    }
    impl ::core::convert::From<GetPkpNftAddressCall> for PKPHelperCalls {
        fn from(value: GetPkpNftAddressCall) -> Self {
            Self::GetPkpNftAddress(value)
        }
    }
    impl ::core::convert::From<GetPkpPermissionsAddressCall> for PKPHelperCalls {
        fn from(value: GetPkpPermissionsAddressCall) -> Self {
            Self::GetPkpPermissionsAddress(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for PKPHelperCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetStakingAddressCall> for PKPHelperCalls {
        fn from(value: GetStakingAddressCall) -> Self {
            Self::GetStakingAddress(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for PKPHelperCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for PKPHelperCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<MintNextAndAddAuthMethodsCall> for PKPHelperCalls {
        fn from(value: MintNextAndAddAuthMethodsCall) -> Self {
            Self::MintNextAndAddAuthMethods(value)
        }
    }
    impl ::core::convert::From<MintNextAndAddAuthMethodsWithTypesCall>
    for PKPHelperCalls {
        fn from(value: MintNextAndAddAuthMethodsWithTypesCall) -> Self {
            Self::MintNextAndAddAuthMethodsWithTypes(value)
        }
    }
    impl ::core::convert::From<MintNextAndAddDomainWalletMetadataCall>
    for PKPHelperCalls {
        fn from(value: MintNextAndAddDomainWalletMetadataCall) -> Self {
            Self::MintNextAndAddDomainWalletMetadata(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for PKPHelperCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PKPHelperCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemovePkpMetadataCall> for PKPHelperCalls {
        fn from(value: RemovePkpMetadataCall) -> Self {
            Self::RemovePkpMetadata(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PKPHelperCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for PKPHelperCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for PKPHelperCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetContractResolverCall> for PKPHelperCalls {
        fn from(value: SetContractResolverCall) -> Self {
            Self::SetContractResolver(value)
        }
    }
    impl ::core::convert::From<SetPkpMetadataCall> for PKPHelperCalls {
        fn from(value: SetPkpMetadataCall) -> Self {
            Self::SetPkpMetadata(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for PKPHelperCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PKPHelperCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
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
    ///Container type for all return fields from the `claimAndMintNextAndAddAuthMethods` function with signature `claimAndMintNextAndAddAuthMethods((uint256,bytes32,(bytes32,bytes32,uint8)[]),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x202f724f`
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
    pub struct ClaimAndMintNextAndAddAuthMethodsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `claimAndMintNextAndAddAuthMethodsWithTypes` function with signature `claimAndMintNextAndAddAuthMethodsWithTypes((uint256,bytes32,(bytes32,bytes32,uint8)[]),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x13af411b`
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
    pub struct ClaimAndMintNextAndAddAuthMethodsWithTypesReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `claimAndMintNextAndAddAuthMethodsWithTypesV2` function with signature `claimAndMintNextAndAddAuthMethodsWithTypesV2((uint256,string,bytes32,(bytes32,bytes32,uint8)[],address),(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x91ee4fd5`
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
    pub struct ClaimAndMintNextAndAddAuthMethodsWithTypesV2Return(
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `getDomainWalletRegistry` function with signature `getDomainWalletRegistry()` and selector `0x73cc4111`
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
    pub struct GetDomainWalletRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPKPNftMetdataAddress` function with signature `getPKPNftMetdataAddress()` and selector `0x5043026c`
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
    pub struct GetPKPNftMetdataAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPkpNftAddress` function with signature `getPkpNftAddress()` and selector `0xcaead0c7`
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
    pub struct GetPkpNftAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPkpPermissionsAddress` function with signature `getPkpPermissionsAddress()` and selector `0x3276558c`
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
    pub struct GetPkpPermissionsAddressReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getStakingAddress` function with signature `getStakingAddress()` and selector `0x0e9ed68b`
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
    pub struct GetStakingAddressReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `mintNextAndAddAuthMethods` function with signature `mintNextAndAddAuthMethods(uint256,string,uint256[],bytes[],bytes[],uint256[][],bool,bool)` and selector `0xe4f11df6`
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
    pub struct MintNextAndAddAuthMethodsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintNextAndAddAuthMethodsWithTypes` function with signature `mintNextAndAddAuthMethodsWithTypes((uint256,string,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool))` and selector `0x778fe572`
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
    pub struct MintNextAndAddAuthMethodsWithTypesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintNextAndAddDomainWalletMetadata` function with signature `mintNextAndAddDomainWalletMetadata(uint256,string,uint256[],bytes[],bytes[],uint256[][],string[],bool,bool)` and selector `0xdb0bf933`
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
    pub struct MintNextAndAddDomainWalletMetadataReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
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
    ///`Signature(bytes32,bytes32,uint8)`
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
    pub struct Signature {
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
    }
    ///`ClaimMaterial(uint256,bytes32,(bytes32,bytes32,uint8)[])`
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
    pub struct ClaimMaterial {
        pub key_type: ::ethers::core::types::U256,
        pub derived_key_id: [u8; 32],
        pub signatures: ::std::vec::Vec<Signature>,
    }
    ///`ClaimMaterialV2(uint256,string,bytes32,(bytes32,bytes32,uint8)[],address)`
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
    pub struct ClaimMaterialV2 {
        pub key_type: ::ethers::core::types::U256,
        pub key_set_id: ::std::string::String,
        pub derived_key_id: [u8; 32],
        pub signatures: ::std::vec::Vec<Signature>,
        pub staking_contract_address: ::ethers::core::types::Address,
    }
    ///`AuthMethodData(uint256,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool)`
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
    pub struct AuthMethodData {
        pub key_type: ::ethers::core::types::U256,
        pub permitted_ipfs_ci_ds: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_ipfs_cid_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub permitted_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub permitted_address_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub add_pkp_eth_address_as_permitted_address: bool,
        pub send_pkp_to_itself: bool,
    }
    ///`MintNextAndAddAuthMethodsWithTypesParams(uint256,string,bytes[],uint256[][],address[],uint256[][],uint256[],bytes[],bytes[],uint256[][],bool,bool)`
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
    pub struct MintNextAndAddAuthMethodsWithTypesParams {
        pub key_type: ::ethers::core::types::U256,
        pub key_set_id: ::std::string::String,
        pub permitted_ipfs_ci_ds: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_ipfs_cid_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub permitted_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub permitted_addresses_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub permitted_auth_method_types: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permitted_auth_method_ids: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_pubkeys: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub permitted_auth_method_scopes: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::U256>,
        >,
        pub add_pkp_eth_address_as_permitted_address: bool,
        pub send_pkp_to_itself: bool,
    }
}
