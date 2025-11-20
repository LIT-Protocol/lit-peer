pub use release_register::*;
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
pub mod release_register {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/ReleaseRegister.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("env"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACTIVATOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ACTIVATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("BURNER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BURNER_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("CREATOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CREATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("DEACTIVATOR_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEACTIVATOR_ROLE"),
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
                    ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_RO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_RO"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_SSH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_SSH"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RELEASE_OPTION_USERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RELEASE_OPTION_USERS",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addAllowedAdminSigningPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addAllowedAdminSigningPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
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
                    ::std::borrow::ToOwned::to_owned("addAllowedSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAllowedSubnet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("burnRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("createRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Status",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Type",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platform"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Platform",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("options"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id_key_digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("public_key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("date"),
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
                    ::std::borrow::ToOwned::to_owned("getActiveRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActiveRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Type",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platform"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Platform",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getActiveReleases"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getActiveReleases"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCreatorDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCreatorDomain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getRelease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRelease"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReleaseRegister.Release",
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
                    ::std::borrow::ToOwned::to_owned("hasAllowedAdminSigningPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hasAllowedAdminSigningPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("hasAllowedAuthorKeyDigest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "hasAllowedAuthorKeyDigest",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("hasAllowedEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasAllowedEnv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
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
                    ::std::borrow::ToOwned::to_owned("hasAllowedSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasAllowedSubnet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("hasCreatorInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasCreatorInit"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("initCreator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initCreator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnetId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("domain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("authorKeyDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "removeAllowedAdminSigningPublicKey",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeAllowedAdminSigningPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                        ::std::borrow::ToOwned::to_owned("enum ReleaseRegister.Env"),
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
                    ::std::borrow::ToOwned::to_owned("removeAllowedSubnet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeAllowedSubnet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
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
                    ::std::borrow::ToOwned::to_owned("setReleaseStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReleaseStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ReleaseRegister.Status",
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
                    ::std::borrow::ToOwned::to_owned(
                        "AllowedAdminSigningPublicKeyAdded",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAdminSigningPublicKeyAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AllowedAdminSigningPublicKeyRemoved",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAdminSigningPublicKeyRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedAuthorKeyDigestAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAuthorKeyDigestAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedAuthorKeyDigestRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedAuthorKeyDigestRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("AllowedSubnetAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AllowedSubnetAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AllowedSubnetRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AllowedSubnetRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("subnet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InitCreator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("InitCreator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("domain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("authorKeyDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseBurned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReleaseBurned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReleaseCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("env"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("typ"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("date"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platform"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("options"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id_key_digest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("public_key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("cid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseStatusChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReleaseStatusChange",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("releaseId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ActivatorRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ActivatorRoleRequired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AdminRoleRequired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BurnerRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BurnerRoleRequired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreatorRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CreatorRoleRequired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeactivatorRoleRequired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeactivatorRoleRequired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEnv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidEnv"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidStatus"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReleaseNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ReleaseNotFound"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static RELEASEREGISTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa3\xEE8\x03\x80a3\xEE\x839\x81\x01`@\x81\x90Ra\0/\x91a\x02\xC1V[a\0G`\0\x80Q` a3\xCE\x839\x81Q\x91R3a\x01\xCAV[a\0_`\0\x80Q` a3\xCE\x839\x81Q\x91R\x80a\x01\xD8V[a\0\x97\x7F<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB6`\0\x80Q` a3\xCE\x839\x81Q\x91Ra\x01\xD8V[a\0\xCF\x7F\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<`\0\x80Q` a3\xCE\x839\x81Q\x91Ra\x01\xD8V[a\x01\x07\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF`\0\x80Q` a3\xCE\x839\x81Q\x91Ra\x01\xD8V[a\x01?\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC8`\0\x80Q` a3\xCE\x839\x81Q\x91Ra\x01\xD8V[`\x01`\x03`\0\x83`\x02\x81\x11\x15a\x01WWa\x01Wa\x02\xE9V[`\x02\x81\x11\x15a\x01hWa\x01ha\x02\xE9V[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x01\xBC\x91\x90a\x02\xFFV[`@Q\x80\x91\x03\x90\xA1Pa\x03'V[a\x01\xD4\x82\x82a\x02#V[PPV[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\x01\xD4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x02}3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0` \x82\x84\x03\x12\x15a\x02\xD3W`\0\x80\xFD[\x81Q`\x03\x81\x10a\x02\xE2W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x03!WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[a0\x98\x80a\x036`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA6W`\x005`\xE0\x1C\x80cp\xE6ZE\x11a\0\xEAW\x80cp\xE6ZE\x14a\x03!W\x80ct\xBC\x819\x14a\x034W\x80cu\xB28\xFC\x14a\x03GW\x80c\x7Fi\x8E\x92\x14a\x03\\W\x80c\x8A\xED\xA2Z\x14a\x03|W\x80c\x8D\xEB8\x93\x14a\x03\x91W\x80c\x91\xD1HT\x14a\x03\xA4W\x80c\x9B\xB4\xE2\xF7\x14a\x03\xB7W\x80c\xA0\x90\x83\0\x14a\x03\xCCW\x80c\xA2\x17\xFD\xDF\x14a\x03\xF8W\x80c\xADv\x93\x94\x14a\x04\0W\x80c\xBC|\xA3\x17\x14a\x04\x08W\x80c\xD5Gt\x1F\x14a\x04\x1BW\x80c\xD6\xBCbm\x14a\x04.W\x80c\xDB-0;\x14a\x04AW\x80c\xE1\xC0\xAF\x08\x14a\x04TW\x80c\xF2\xDC\x99\x16\x14a\x04gW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xABW\x80c\x02>\x92\x88\x14a\x01\xD3W\x80c\x08t\n;\x14a\x01\xE8W\x80c\x0E\t+\x18\x14a\x02\x0BW\x80c\x0E\x1EY\xDD\x14a\x02\x1EW\x80c\x19r@e\x14a\x023W\x80c\x1B\xD5d\xDC\x14a\x02FW\x80c$\x8A\x9C\xA3\x14a\x02YW\x80c&\t\xE5\x86\x14a\x02lW\x80c'}\xCE\xAF\x14a\x02\x7FW\x80c(,Q\xF3\x14a\x02\x87W\x80c*\xE7\x9Bm\x14a\x02\xAEW\x80c//\xF1]\x14a\x02\xC1W\x80c6V\x8A\xBE\x14a\x02\xD4W\x80c:\xCD\x1E\xA3\x14a\x02\xE7W\x80c=\xC6\xC8X\x14a\x02\xEFW\x80cE\x8B\xE7\xDC\x14a\x02\xFAW[`\0\x80\xFD[a\x01\xBEa\x01\xB96`\x04a&+V[a\x04zV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xDBa\x04\xB1V[`@Qa\x01\xCA\x91\x90a&UV[a\x01\xFD`\0\x80Q` a0#\x839\x81Q\x91R\x81V[`@Q\x90\x81R` \x01a\x01\xCAV[a\x01\xBEa\x02\x196`\x04a';V[a\x04\xC2V[a\x021a\x02,6`\x04a'\x94V[a\x04\xEDV[\0[a\x021a\x02A6`\x04a'\xE2V[a\n\x94V[a\x01\xBEa\x02T6`\x04a)\x07V[a\x10\x19V[a\x01\xFDa\x02g6`\x04a)\"V[a\x10\\V[a\x021a\x02z6`\x04a)\"V[a\x10qV[a\x01\xFD`\x04\x81V[a\x01\xFD\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC8\x81V[a\x01\xFDa\x02\xBC6`\x04a);V[a\x11\xABV[a\x021a\x02\xCF6`\x04a)\xC2V[a\x11\xF4V[a\x021a\x02\xE26`\x04a)\xC2V[a\x12\x10V[a\x01\xFD`\x08\x81V[`\x01T`\xFF\x16a\x01\xBEV[a\x01\xFD\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF\x81V[a\x021a\x03/6`\x04a';V[a\x12\x8EV[a\x021a\x03B6`\x04a)\x07V[a\x13#V[a\x01\xFD`\0\x80Q` a0C\x839\x81Q\x91R\x81V[a\x03oa\x03j6`\x04a)\"V[a\x13\xD5V[`@Qa\x01\xCA\x91\x90a*\x91V[a\x01\xFD`\0\x80Q` a0\x03\x839\x81Q\x91R\x81V[a\x021a\x03\x9F6`\x04a)\x07V[a\x17\x1DV[a\x01\xBEa\x03\xB26`\x04a)\xC2V[a\x17\xC1V[a\x03\xBFa\x17\xEAV[`@Qa\x01\xCA\x91\x90a+tV[a\x01\xBEa\x03\xDA6`\x04a+\x87V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[a\x01\xFD`\0\x81V[a\x01\xFD`\x02\x81V[a\x021a\x04\x166`\x04a+\xA2V[a\x18|V[a\x021a\x04)6`\x04a)\xC2V[a\x1A\x1FV[a\x01\xBEa\x04<6`\x04a';V[a\x1A;V[a\x021a\x04O6`\x04a+\x87V[a\x1AMV[a\x021a\x04b6`\x04a';V[a\x1A\xD2V[a\x021a\x04u6`\x04a+\x87V[a\x1B[V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\xABWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[``a\x04\xBD`\ta\x1B\xE3V[\x90P\x90V[`\0`\x05\x82`@Qa\x04\xD4\x91\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x92\x91PPV[`\x02\x81`\x03\x81\x11\x15a\x05\x01Wa\x05\x01a)\xEEV[\x03a\x05@Wa\x05\x1E`\0\x80Q` a0#\x839\x81Q\x91R3a\x17\xC1V[a\x05;W`@Qc.\x10\x89\x1F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xB9V[`\x03\x81`\x03\x81\x11\x15a\x05TWa\x05Ta)\xEEV[\x03a\x05\xA0Wa\x05\x83\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF3a\x17\xC1V[a\x05;W`@Qc\t>\xFA\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x07\xA9/\x19`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x05\xDCWa\x05\xDCa)\xEEV[\x03a\x05\xFAW`@Qc\x05\x0C\xC7\xFF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x07` R`@\x80\x82 \x81Qa\x01@\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x06+Wa\x06+a)\xEEV[`\x03\x81\x11\x15a\x06<Wa\x06<a)\xEEV[\x81R\x81T` \x90\x91\x01\x90a\x01\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x06_Wa\x06_a)\xEEV[`\x02\x81\x11\x15a\x06pWa\x06pa)\xEEV[\x81R\x81T` \x90\x91\x01\x90b\x01\0\0\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x06\x94Wa\x06\x94a)\xEEV[`\x03\x81\x11\x15a\x06\xA5Wa\x06\xA5a)\xEEV[\x81R` \x01`\x01\x82\x01\x80Ta\x06\xB9\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xE5\x90a,FV[\x80\x15a\x072W\x80`\x1F\x10a\x07\x07Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x072V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`@\x90\x91\x01\x90`\xFF\x16\x80\x15a\x07`Wa\x07`a)\xEEV[\x80\x15a\x07nWa\x07na)\xEEV[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x07\x8C\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xB8\x90a,FV[\x80\x15a\x08\x05W\x80`\x1F\x10a\x07\xDAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x05V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xE8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x08\x1E\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08J\x90a,FV[\x80\x15a\x08\x97W\x80`\x1F\x10a\x08lWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08zW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x08\xB0\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDC\x90a,FV[\x80\x15a\t)W\x80`\x1F\x10a\x08\xFEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t)V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a\tGWa\tGa)\xEEV[\x82`\x03\x81\x11\x15a\tYWa\tYa)\xEEV[\x14a\t\xF3W\x82a\t{\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x85`\xA0\x01Qa\x11\xABV[\x03a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FMust replace active release befo`D\x82\x01R\x7Fre changing status from Active\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x07` R`@\x90 \x80T\x83\x91\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\n\x1EWa\n\x1Ea)\xEEV[\x02\x17\x90UP\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x83\x83`@Qa\nT\x92\x91\x90a,\x80V[`@Q\x80\x91\x03\x90\xA1`\x02\x82`\x03\x81\x11\x15a\npWa\npa)\xEEV[\x03a\n\x83Wa\n~\x83a\x1B\xF7V[PPPV[a\n\x8E`\t\x84a eV[PPPPV[a\n\xAC`\0\x80Q` a0\x03\x839\x81Q\x91R3a\x17\xC1V[a\n\xC9W`@Qc\x80Q\x0F\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x8A`\x03\x81\x11\x15a\n\xDDWa\n\xDDa)\xEEV[\x03a\x0B\x1CWa\n\xFA`\0\x80Q` a0#\x839\x81Q\x91R3a\x17\xC1V[a\x0B\x17W`@Qc.\x10\x89\x1F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0BnV[`\x01\x8A`\x03\x81\x11\x15a\x0B0Wa\x0B0a)\xEEV[\x14\x15\x80\x15a\x0BPWP`\x03\x8A`\x03\x81\x11\x15a\x0BMWa\x0BMa)\xEEV[\x14\x15[\x15a\x0BnW`@Qc\x07\xA9/\x19`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x8A`\x02\x81\x11\x15a\x0B\x84Wa\x0B\x84a)\xEEV[`\x02\x81\x11\x15a\x0B\x95Wa\x0B\x95a)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x0B\xCBW`@Qc29\xEA_`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04`\0a\x0B\xDA\x8D`\x04a qV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x0C\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FThe provided subnet (within the `D\x82\x01R\x7Frelease id) is not valid for thi`d\x82\x01Ri\x1C\xC8\x18\xDB\xDB\x9D\x1C\x98X\xDD`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\t\xEAV[`\0\x89`\x02\x81\x11\x15a\x0C\x95Wa\x0C\x95a)\xEEV[\x14\x15\x80\x15a\x0C\xB5WP`\x01\x89`\x02\x81\x11\x15a\x0C\xB2Wa\x0C\xB2a)\xEEV[\x14\x15[\x15a\r8W`\0\x8B\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x0C\xDDWa\x0C\xDDa)\xEEV[\x14a\r8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FA release with this ID already e`D\x82\x01Rdxists`\xD8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[`\x02\x89`\x02\x81\x11\x15a\rLWa\rLa)\xEEV[\x03a\r\xB8W`\x02\x85\x16`\0\x03a\r\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FThe RO option is required for pr`D\x82\x01Rjod releases`\xA8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[\x80`\0\x03a\r\xC3WPB[`@Q\x80a\x01@\x01`@R\x80\x8B`\x03\x81\x11\x15a\r\xE1Wa\r\xE1a)\xEEV[\x81R` \x01\x8A`\x02\x81\x11\x15a\r\xF8Wa\r\xF8a)\xEEV[\x81R` \x01\x89`\x03\x81\x11\x15a\x0E\x0FWa\x0E\x0Fa)\xEEV[\x81R` \x01\x88\x81R` \x01\x82\x81R` \x01\x87`\0\x81\x11\x15a\x0E2Wa\x0E2a)\xEEV[\x81R` \x80\x82\x01\x88\x90R`@\x80\x83\x01\x88\x90R``\x83\x01\x87\x90R`\x80\x90\x92\x01\x85\x90R`\0\x8E\x81R`\x07\x90\x91R \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x0E|Wa\x0E|a)\xEEV[\x02\x17\x90UP` \x82\x01Q\x81T\x82\x90a\xFF\0\x19\x16a\x01\0\x83`\x02\x81\x11\x15a\x0E\xA4Wa\x0E\xA4a)\xEEV[\x02\x17\x90UP`@\x82\x01Q\x81T\x82\x90b\xFF\0\0\x19\x16b\x01\0\0\x83`\x03\x81\x11\x15a\x0E\xCEWa\x0E\xCEa)\xEEV[\x02\x17\x90UP``\x82\x01Q`\x01\x82\x01\x90a\x0E\xE7\x90\x82a,\xE4V[P`\x80\x82\x01Q`\x02\x82\x01U`\xA0\x82\x01Q`\x03\x82\x01\x80T`\xFF\x19\x16`\x01\x83\x80\x15a\x0F\x12Wa\x0F\x12a)\xEEV[\x02\x17\x90UP`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x0F5\x90\x82a,\xE4V[Pa\x01\0\x82\x01Q`\x06\x82\x01\x90a\x0FK\x90\x82a,\xE4V[Pa\x01 \x82\x01Q`\x07\x82\x01\x90a\x0Fa\x90\x82a,\xE4V[P\x90PP\x7F\x90\xA8b\xCC\x16\xEB\xCB\x9BT\x9C\x93 \x13\xF7|B-\xF1\x17\xE2MH\xD7\xD2j}\x90\xFB\xC4<<\x8F\x8B\x8B\x8B\x8B\x8B\x86\x8C\x8C\x8C\x8C\x8C`@Qa\x0F\xA8\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a-\xA2V[`@Q\x80\x91\x03\x90\xA1`\x02\x8A`\x03\x81\x11\x15a\x0F\xC4Wa\x0F\xC4a)\xEEV[\x03a\x10\x0CWa\x0F\xD2\x8Ba\x1B\xF7V[\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x8B\x8B`@Qa\x10\x03\x92\x91\x90a,\x80V[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPPV[`\0`\x03`\0\x83`\x02\x81\x11\x15a\x101Wa\x101a)\xEEV[`\x02\x81\x11\x15a\x10BWa\x10Ba)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x92\x91PPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x10\x9B\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC83a\x17\xC1V[a\x10\xB8W`@Qc=;+Y`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x10\xDBWa\x10\xDBa)\xEEV[\x03a\x10\xF9W`@Qc\x05\x0C\xC7\xFF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x07` R`@\x81 \x80Tb\xFF\xFF\xFF\x19\x16\x81U\x90a\x11\x1F`\x01\x83\x01\x82a%tV[`\0`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80T`\xFF\x19\x16\x90U`\x04\x83\x01\x81\x90Ua\x11I\x90`\x05\x84\x01\x90a%tV[a\x11W`\x06\x83\x01`\0a%tV[a\x11e`\x07\x83\x01`\0a%tV[Pa\x11s\x90P`\t\x82a eV[P`@Q\x81\x81R\x7F\xE4\xA7\xF7\xB4\x82Q\xADp\xE6?\x80\x07X\xA4E\xB0\x03\x86\xE2\xFA\x98\xD5\xAF\xCE\x96\xA5F\xE8\xFC\xE2\x11N\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\0\x80\x85\x85\x85\x85`@Q` \x01a\x11\xC5\x94\x93\x92\x91\x90a.PV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x90\x81R`\x08\x90\x92R\x90 T\x96\x95PPPPPPV[a\x11\xFD\x82a\x10\\V[a\x12\x06\x81a \xCEV[a\n~\x83\x83a \xDBV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x12\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\t\xEAV[a\x12\x8A\x82\x82a!_V[PPV[a\x12\xA6`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x12\xC3W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x82`@Qa\x12\xD5\x91\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T\x92\x15\x15`\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x7F\xE7s^\x9FV\x9F\xE6\x16qf*\x88)\xDBw\xDE8\xEFaLw\xB1\xB16\xC9X\xAF\xF7\x81\xDF|u\x90a\x11\xA0\x90\x83\x90a+tV[a\x13;`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x13XW`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x83`\x02\x81\x11\x15a\x13pWa\x13pa)\xEEV[`\x02\x81\x11\x15a\x13\x81Wa\x13\x81a)\xEEV[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x11\xA0\x91\x90a.\xB6V[a\x13\xDDa%\xAEV[`\0\x82\x81R`\x07` R`@\x90\x81\x90 \x81Qa\x01@\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x14\x0FWa\x14\x0Fa)\xEEV[`\x03\x81\x11\x15a\x14 Wa\x14 a)\xEEV[\x81R\x81T` \x90\x91\x01\x90a\x01\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14CWa\x14Ca)\xEEV[`\x02\x81\x11\x15a\x14TWa\x14Ta)\xEEV[\x81R\x81T` \x90\x91\x01\x90b\x01\0\0\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x14xWa\x14xa)\xEEV[`\x03\x81\x11\x15a\x14\x89Wa\x14\x89a)\xEEV[\x81R` \x01`\x01\x82\x01\x80Ta\x14\x9D\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xC9\x90a,FV[\x80\x15a\x15\x16W\x80`\x1F\x10a\x14\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x16V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xF9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`@\x90\x91\x01\x90`\xFF\x16\x80\x15a\x15DWa\x15Da)\xEEV[\x80\x15a\x15RWa\x15Ra)\xEEV[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x15p\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x9C\x90a,FV[\x80\x15a\x15\xE9W\x80`\x1F\x10a\x15\xBEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xE9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xCCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x16\x02\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16.\x90a,FV[\x80\x15a\x16{W\x80`\x1F\x10a\x16PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16{V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x16\x94\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xC0\x90a,FV[\x80\x15a\x17\rW\x80`\x1F\x10a\x16\xE2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[a\x175`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x17RW`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x82`\x02\x81\x11\x15a\x17hWa\x17ha)\xEEV[`\x02\x81\x11\x15a\x17yWa\x17ya)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\xFF\x19\x16\x90UQ\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x90a\x11\xA0\x90\x83\x90a.\xB6V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x02\x80Ta\x17\xF9\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18%\x90a,FV[\x80\x15a\x18rW\x80`\x1F\x10a\x18GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[a\x18\x94`\0\x80Q` a0\x03\x839\x81Q\x91R3a\x17\xC1V[a\x18\xB1W`@Qc\x80Q\x0F\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x85`\x02\x81\x11\x15a\x18\xC7Wa\x18\xC7a)\xEEV[`\x02\x81\x11\x15a\x18\xD8Wa\x18\xD8a)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x19\x0EW`@Qc29\xEA_`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02a\x19\x1A\x83\x82a,\xE4V[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x90Q`\x06\x90a\x19S\x90\x84\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T\x92\x15\x15`\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x7F\x18\xF09lm\x01\x87mv\x10\xD9hw\xB4O\x01z\xB4\xCA\"e\xB10y\xB1\x0E\x0B\xE6\xB6\xAF0\xD4\x90a\x19\xA1\x90\x83\x90a+tV[`@Q\x80\x91\x03\x90\xA1\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x83`@Qa\x19\xD8\x91\x90a.\xC4V[`@Q\x80\x91\x03\x90\xA1\x7F[Y\x0C\xCE\xAB\xBE\xB2\x8C\xD1nA\xA8\x11\xBAF\xDF[\xB8,\x98\x92\r^\x7F\xF7~O\xA9\xDBp\x9B3\x82\x82`@Qa\x1A\x11\x92\x91\x90a.\xD8V[`@Q\x80\x91\x03\x90\xA1PPPPV[a\x1A(\x82a\x10\\V[a\x1A1\x81a \xCEV[a\n~\x83\x83a!_V[`\0`\x06\x82`@Qa\x04\xD4\x91\x90a,*V[a\x1Ae`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x1A\x82W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\xF5\x80\xA1\xEA\x01\xC0\xED>\xC5\xE4y|V\x0E\xE7\x04l5\x91X\x8C\x0F\xEB\x81O_-N\xCAP',\x90a\x11\xA0\x90\x83\x90a.\xC4V[a\x1A\xEA`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x1B\x07W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x81`@Qa\x1B\x17\x91\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T`\xFF\x19\x16\x90U\x7F\xF10B%]\x0BX\xF7\xAC\xC8, \x1A\x0C\xD4\xD84\xB0x=\xD7\xB3\xC2\x9C\xCA\x9B\x96\x84\rc?t\x90a\x11\xA0\x90\x83\x90a+tV[a\x1Bs`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x1B\x90W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x90a\x11\xA0\x90\x83\x90a.\xC4V[```\0a\x1B\xF0\x83a!\xC4V[\x93\x92PPPV[`\0\x81\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x1C\x1AWa\x1C\x1Aa)\xEEV[\x03a\x1C8W`@Qc\x05\x0C\xC7\xFF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x07` R`@\x80\x82 \x81Qa\x01@\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x1CiWa\x1Cia)\xEEV[`\x03\x81\x11\x15a\x1CzWa\x1Cza)\xEEV[\x81R\x81T` \x90\x91\x01\x90a\x01\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1C\x9DWa\x1C\x9Da)\xEEV[`\x02\x81\x11\x15a\x1C\xAEWa\x1C\xAEa)\xEEV[\x81R\x81T` \x90\x91\x01\x90b\x01\0\0\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1C\xD2Wa\x1C\xD2a)\xEEV[`\x03\x81\x11\x15a\x1C\xE3Wa\x1C\xE3a)\xEEV[\x81R` \x01`\x01\x82\x01\x80Ta\x1C\xF7\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D#\x90a,FV[\x80\x15a\x1DpW\x80`\x1F\x10a\x1DEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1DpV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1DSW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`@\x90\x91\x01\x90`\xFF\x16\x80\x15a\x1D\x9EWa\x1D\x9Ea)\xEEV[\x80\x15a\x1D\xACWa\x1D\xACa)\xEEV[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x1D\xCA\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xF6\x90a,FV[\x80\x15a\x1ECW\x80`\x1F\x10a\x1E\x18Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1ECV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E&W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x1E\\\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\x88\x90a,FV[\x80\x15a\x1E\xD5W\x80`\x1F\x10a\x1E\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x1E\xEE\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\x1A\x90a,FV[\x80\x15a\x1FgW\x80`\x1F\x10a\x1F<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1FgV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1FJW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a\x1F\x85Wa\x1F\x85a)\xEEV[\x81Q`\x03\x81\x11\x15a\x1F\x98Wa\x1F\x98a)\xEEV[\x14a\x1F\xB6W`@Qc\x07\xA9/\x19`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84`\xA0\x01Q`@Q` \x01a\x1F\xDF\x94\x93\x92\x91\x90a.PV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x08\x90\x93R\x91 T\x90\x91P\x80\x15a AW`\0\x81\x81R`\x07` R`@\x90 `\x02\x01T`\x80\x84\x01Q\x11\x15a <W`\0\x82\x81R`\x08` R`@\x90 \x84\x90U[a SV[`\0\x82\x81R`\x08` R`@\x90 \x84\x90U[a ^`\t\x85a\" V[PPPPPV[`\0a\x1B\xF0\x83\x83a\",V[`\0\x80`\0[`\x14\x81\x10\x15a \xC3Wa \x8B\x81`\x08a/\x13V[\x85a \x96\x83\x87a/*V[` \x81\x10a \xA6Wa \xA6a/=V[\x1A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x91\x90\x91\x17\x90`\x01\x01a wV[P``\x1C\x93\x92PPPV[a \xD8\x813a#\x1FV[PV[a \xE5\x82\x82a\x17\xC1V[a\x12\x8AW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua!\x1B3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a!i\x82\x82a\x17\xC1V[\x15a\x12\x8AW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\"\x14W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\"\0W[PPPPP\x90P\x91\x90PV[`\0a\x1B\xF0\x83\x83a#xV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a#\x15W`\0a\"P`\x01\x83a/SV[\x85T\x90\x91P`\0\x90a\"d\x90`\x01\x90a/SV[\x90P\x81\x81\x14a\"\xC9W`\0\x86`\0\x01\x82\x81T\x81\x10a\"\x84Wa\"\x84a/=V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\"\xA7Wa\"\xA7a/=V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\"\xDAWa\"\xDAa/fV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x04\xABV[`\0\x91PPa\x04\xABV[a#)\x82\x82a\x17\xC1V[a\x12\x8AWa#6\x81a#\xC7V[a#A\x83` a#\xD9V[`@Q` \x01a#R\x92\x91\x90a/|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\t\xEA\x91`\x04\x01a+tV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta#\xBFWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04\xABV[P`\0a\x04\xABV[``a\x04\xAB`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a#\xE8\x83`\x02a/\x13V[a#\xF3\x90`\x02a/*V[`\x01`\x01`@\x1B\x03\x81\x11\x15a$\nWa$\na&\x98V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$4W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a$OWa$Oa/=V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a$~Wa$~a/=V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a$\xA2\x84`\x02a/\x13V[a$\xAD\x90`\x01a/*V[\x90P[`\x01\x81\x11\x15a%%Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a$\xE1Wa$\xE1a/=V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a$\xF7Wa$\xF7a/=V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a%\x1E\x81a/\xEBV[\x90Pa$\xB0V[P\x83\x15a\x1B\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\t\xEAV[P\x80Ta%\x80\x90a,FV[`\0\x82U\x80`\x1F\x10a%\x90WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a \xD8\x91\x90a&\x12V[`@\x80Qa\x01@\x81\x01\x90\x91R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x80\x81\x11\x15a%\xF0Wa%\xF0a)\xEEV[\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[[\x80\x82\x11\x15a&'W`\0\x81U`\x01\x01a&\x13V[P\x90V[`\0` \x82\x84\x03\x12\x15a&=W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1B\xF0W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a&\x8DW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&oV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a&\xBFW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xD8Wa&\xD8a&\x98V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a'\x06Wa'\x06a&\x98V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a'\x1EW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'MW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'cW`\0\x80\xFD[a'o\x84\x82\x85\x01a&\xAEV[\x94\x93PPPPV[`\x04\x81\x10a \xD8W`\0\x80\xFD[\x805a'\x8F\x81a'wV[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xA7W`\0\x80\xFD[\x825\x91P` \x83\x015a'\xB9\x81a'wV[\x80\x91PP\x92P\x92\x90PV[\x805`\x03\x81\x10a'\x8FW`\0\x80\xFD[\x805`\x01\x81\x10a'\x8FW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a(\x04W`\0\x80\xFD[\x8B5\x9APa(\x14` \x8D\x01a'\x84V[\x99Pa(\"`@\x8D\x01a'\xC4V[\x98Pa(0``\x8D\x01a'\x84V[\x97P`\x80\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(KW`\0\x80\xFD[a(W\x8E\x82\x8F\x01a&\xAEV[\x97PPa(f`\xA0\x8D\x01a'\xD3V[\x95P`\xC0\x8C\x015\x94P`\xE0\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x88W`\0\x80\xFD[a(\x94\x8E\x82\x8F\x01a&\xAEV[\x94PPa\x01\0\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xB1W`\0\x80\xFD[a(\xBD\x8E\x82\x8F\x01a&\xAEV[\x93PPa\x01 \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xDAW`\0\x80\xFD[a(\xE6\x8E\x82\x8F\x01a&\xAEV[\x92PP`\0a\x01@\x8D\x015\x90P\x80\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0` \x82\x84\x03\x12\x15a)\x19W`\0\x80\xFD[a\x1B\xF0\x82a'\xC4V[`\0` \x82\x84\x03\x12\x15a)4W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)QW`\0\x80\xFD[a)Z\x85a'\xC4V[\x93P` \x85\x015a)j\x81a'wV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x85W`\0\x80\xFD[a)\x91\x87\x82\x88\x01a&\xAEV[\x92PPa)\xA0``\x86\x01a'\xD3V[\x90P\x92\x95\x91\x94P\x92PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\x8FW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a)\xD5W`\0\x80\xFD[\x825\x91Pa)\xE5` \x84\x01a)\xABV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x04\x81\x10a \xD8Wa \xD8a)\xEEV[a*\x1D\x81a*\x04V[\x90RV[`\x03\x81\x10a*\x1DWa*\x1Da)\xEEV[`\0[\x83\x81\x10\x15a*LW\x81\x81\x01Q\x83\x82\x01R` \x01a*4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*m\x81` \x86\x01` \x86\x01a*1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x81\x10a*\x1DWa*\x1Da)\xEEV[` \x81Ra*\xA3` \x82\x01\x83Qa*\x14V[`\0` \x83\x01Qa*\xB7`@\x84\x01\x82a*!V[P`@\x83\x01Qa*\xCA``\x84\x01\x82a*\x14V[P``\x83\x01Qa\x01@`\x80\x84\x01Ra*\xE6a\x01`\x84\x01\x82a*UV[\x90P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Qa+\x04`\xC0\x85\x01\x82a*\x81V[P`\xC0\x84\x01Q`\xE0\x84\x01R`\xE0\x84\x01Q`\x1F\x19\x84\x83\x03\x01a\x01\0\x85\x01Ra++\x82\x82a*UV[\x91PPa\x01\0\x84\x01Q`\x1F\x19\x84\x83\x03\x01a\x01 \x85\x01Ra+K\x82\x82a*UV[\x91PPa\x01 \x84\x01Q`\x1F\x19\x84\x83\x03\x01a\x01@\x85\x01Ra+k\x82\x82a*UV[\x95\x94PPPPPV[` \x81R`\0a\x1B\xF0` \x83\x01\x84a*UV[`\0` \x82\x84\x03\x12\x15a+\x99W`\0\x80\xFD[a\x1B\xF0\x82a)\xABV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xB8W`\0\x80\xFD[a+\xC1\x85a'\xC4V[\x93Pa+\xCF` \x86\x01a)\xABV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xEAW`\0\x80\xFD[a+\xF6\x87\x82\x88\x01a&\xAEV[\x92PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x12W`\0\x80\xFD[a,\x1E\x87\x82\x88\x01a&\xAEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x82Qa,<\x81\x84` \x87\x01a*1V[\x91\x90\x91\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a,ZW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,zWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x82\x81R`@\x81\x01a,\x90\x83a*\x04V[\x82` \x83\x01R\x93\x92PPPV[`\x1F\x82\x11\x15a\n~W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a,\xC4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a ^W`\0\x81U`\x01\x01a,\xD0V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xFDWa,\xFDa&\x98V[a-\x11\x81a-\x0B\x84Ta,FV[\x84a,\x9DV[` `\x1F\x82\x11`\x01\x81\x14a-EW`\0\x83\x15a--WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua ^V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a-uW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a-UV[P\x84\x82\x10\x15a-\x93W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x8B\x81Ra-\xAE\x8Ba*\x04V[\x8A` \x82\x01Ra-\xC1`@\x82\x01\x8Ba*!V[a-\xCA\x89a*\x04V[\x88``\x82\x01Ra\x01``\x80\x82\x01R`\0a-\xE8a\x01`\x83\x01\x8Aa*UV[\x88`\xA0\x84\x01Ra-\xFB`\xC0\x84\x01\x89a*\x81V[\x86`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Ra.\x14\x81\x87a*UV[\x90P\x82\x81\x03a\x01 \x84\x01Ra.)\x81\x86a*UV[\x90P\x82\x81\x03a\x01@\x84\x01Ra.>\x81\x85a*UV[\x9E\x9DPPPPPPPPPPPPPPV[`\0`\x03\x86\x10a.bWa.ba)\xEEV[\x85`\xF8\x1B\x82Ra.q\x85a*\x04V[\x84`\xF8\x1B`\x01\x83\x01R\x83Qa.\x8D\x81`\x02\x85\x01` \x88\x01a*1V[\x82\x01`\x01\x84\x10a.\x9FWa.\x9Fa)\xEEV[`\xF8\x93\x90\x93\x1B`\x02\x84\x01RPP`\x03\x01\x93\x92PPPV[` \x81\x01a\x04\xAB\x82\x84a*!V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@\x81R`\0a.\xEB`@\x83\x01\x85a*UV[\x82\x81\x03` \x84\x01Ra+k\x81\x85a*UV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xABWa\x04\xABa.\xFDV[\x80\x82\x01\x80\x82\x11\x15a\x04\xABWa\x04\xABa.\xFDV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xABWa\x04\xABa.\xFDV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa/\xAE\x81`\x17\x85\x01` \x88\x01a*1V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa/\xDF\x81`(\x84\x01` \x88\x01a*1V[\x01`(\x01\x94\x93PPPPV[`\0\x81a/\xFAWa/\xFAa.\xFDV[P`\0\x19\x01\x90V\xFE<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB6\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\xA2dipfsX\"\x12 \x9Fc\x06\x8F\xDC\x11\xDF\x99;Z\xEDw\x94\xC2\x16\xFB&n\x1E\x9F\x01\xBE\x1B\x1Bu2\x11\xB3Z\xBD\x1A7dsolcC\0\x08\x1C\x003\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB";
    /// The bytecode of the contract.
    pub static RELEASEREGISTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA6W`\x005`\xE0\x1C\x80cp\xE6ZE\x11a\0\xEAW\x80cp\xE6ZE\x14a\x03!W\x80ct\xBC\x819\x14a\x034W\x80cu\xB28\xFC\x14a\x03GW\x80c\x7Fi\x8E\x92\x14a\x03\\W\x80c\x8A\xED\xA2Z\x14a\x03|W\x80c\x8D\xEB8\x93\x14a\x03\x91W\x80c\x91\xD1HT\x14a\x03\xA4W\x80c\x9B\xB4\xE2\xF7\x14a\x03\xB7W\x80c\xA0\x90\x83\0\x14a\x03\xCCW\x80c\xA2\x17\xFD\xDF\x14a\x03\xF8W\x80c\xADv\x93\x94\x14a\x04\0W\x80c\xBC|\xA3\x17\x14a\x04\x08W\x80c\xD5Gt\x1F\x14a\x04\x1BW\x80c\xD6\xBCbm\x14a\x04.W\x80c\xDB-0;\x14a\x04AW\x80c\xE1\xC0\xAF\x08\x14a\x04TW\x80c\xF2\xDC\x99\x16\x14a\x04gW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\xABW\x80c\x02>\x92\x88\x14a\x01\xD3W\x80c\x08t\n;\x14a\x01\xE8W\x80c\x0E\t+\x18\x14a\x02\x0BW\x80c\x0E\x1EY\xDD\x14a\x02\x1EW\x80c\x19r@e\x14a\x023W\x80c\x1B\xD5d\xDC\x14a\x02FW\x80c$\x8A\x9C\xA3\x14a\x02YW\x80c&\t\xE5\x86\x14a\x02lW\x80c'}\xCE\xAF\x14a\x02\x7FW\x80c(,Q\xF3\x14a\x02\x87W\x80c*\xE7\x9Bm\x14a\x02\xAEW\x80c//\xF1]\x14a\x02\xC1W\x80c6V\x8A\xBE\x14a\x02\xD4W\x80c:\xCD\x1E\xA3\x14a\x02\xE7W\x80c=\xC6\xC8X\x14a\x02\xEFW\x80cE\x8B\xE7\xDC\x14a\x02\xFAW[`\0\x80\xFD[a\x01\xBEa\x01\xB96`\x04a&+V[a\x04zV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xDBa\x04\xB1V[`@Qa\x01\xCA\x91\x90a&UV[a\x01\xFD`\0\x80Q` a0#\x839\x81Q\x91R\x81V[`@Q\x90\x81R` \x01a\x01\xCAV[a\x01\xBEa\x02\x196`\x04a';V[a\x04\xC2V[a\x021a\x02,6`\x04a'\x94V[a\x04\xEDV[\0[a\x021a\x02A6`\x04a'\xE2V[a\n\x94V[a\x01\xBEa\x02T6`\x04a)\x07V[a\x10\x19V[a\x01\xFDa\x02g6`\x04a)\"V[a\x10\\V[a\x021a\x02z6`\x04a)\"V[a\x10qV[a\x01\xFD`\x04\x81V[a\x01\xFD\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC8\x81V[a\x01\xFDa\x02\xBC6`\x04a);V[a\x11\xABV[a\x021a\x02\xCF6`\x04a)\xC2V[a\x11\xF4V[a\x021a\x02\xE26`\x04a)\xC2V[a\x12\x10V[a\x01\xFD`\x08\x81V[`\x01T`\xFF\x16a\x01\xBEV[a\x01\xFD\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF\x81V[a\x021a\x03/6`\x04a';V[a\x12\x8EV[a\x021a\x03B6`\x04a)\x07V[a\x13#V[a\x01\xFD`\0\x80Q` a0C\x839\x81Q\x91R\x81V[a\x03oa\x03j6`\x04a)\"V[a\x13\xD5V[`@Qa\x01\xCA\x91\x90a*\x91V[a\x01\xFD`\0\x80Q` a0\x03\x839\x81Q\x91R\x81V[a\x021a\x03\x9F6`\x04a)\x07V[a\x17\x1DV[a\x01\xBEa\x03\xB26`\x04a)\xC2V[a\x17\xC1V[a\x03\xBFa\x17\xEAV[`@Qa\x01\xCA\x91\x90a+tV[a\x01\xBEa\x03\xDA6`\x04a+\x87V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[a\x01\xFD`\0\x81V[a\x01\xFD`\x02\x81V[a\x021a\x04\x166`\x04a+\xA2V[a\x18|V[a\x021a\x04)6`\x04a)\xC2V[a\x1A\x1FV[a\x01\xBEa\x04<6`\x04a';V[a\x1A;V[a\x021a\x04O6`\x04a+\x87V[a\x1AMV[a\x021a\x04b6`\x04a';V[a\x1A\xD2V[a\x021a\x04u6`\x04a+\x87V[a\x1B[V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\xABWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[``a\x04\xBD`\ta\x1B\xE3V[\x90P\x90V[`\0`\x05\x82`@Qa\x04\xD4\x91\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x92\x91PPV[`\x02\x81`\x03\x81\x11\x15a\x05\x01Wa\x05\x01a)\xEEV[\x03a\x05@Wa\x05\x1E`\0\x80Q` a0#\x839\x81Q\x91R3a\x17\xC1V[a\x05;W`@Qc.\x10\x89\x1F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xB9V[`\x03\x81`\x03\x81\x11\x15a\x05TWa\x05Ta)\xEEV[\x03a\x05\xA0Wa\x05\x83\x7FP\xA3\xDC\xCCG68r\xDDF\xDEb\xB5\x92s\x98\x9E\xDBr\x90\x1A\xDE\xA0\xB9a\xD5#+\xF9\xA1\xFE\xBF3a\x17\xC1V[a\x05;W`@Qc\t>\xFA\xF9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x07\xA9/\x19`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x05\xDCWa\x05\xDCa)\xEEV[\x03a\x05\xFAW`@Qc\x05\x0C\xC7\xFF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x07` R`@\x80\x82 \x81Qa\x01@\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x06+Wa\x06+a)\xEEV[`\x03\x81\x11\x15a\x06<Wa\x06<a)\xEEV[\x81R\x81T` \x90\x91\x01\x90a\x01\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x06_Wa\x06_a)\xEEV[`\x02\x81\x11\x15a\x06pWa\x06pa)\xEEV[\x81R\x81T` \x90\x91\x01\x90b\x01\0\0\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x06\x94Wa\x06\x94a)\xEEV[`\x03\x81\x11\x15a\x06\xA5Wa\x06\xA5a)\xEEV[\x81R` \x01`\x01\x82\x01\x80Ta\x06\xB9\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xE5\x90a,FV[\x80\x15a\x072W\x80`\x1F\x10a\x07\x07Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x072V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x15W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`@\x90\x91\x01\x90`\xFF\x16\x80\x15a\x07`Wa\x07`a)\xEEV[\x80\x15a\x07nWa\x07na)\xEEV[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x07\x8C\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xB8\x90a,FV[\x80\x15a\x08\x05W\x80`\x1F\x10a\x07\xDAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x05V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xE8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x08\x1E\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08J\x90a,FV[\x80\x15a\x08\x97W\x80`\x1F\x10a\x08lWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08zW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x08\xB0\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDC\x90a,FV[\x80\x15a\t)W\x80`\x1F\x10a\x08\xFEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t)V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a\tGWa\tGa)\xEEV[\x82`\x03\x81\x11\x15a\tYWa\tYa)\xEEV[\x14a\t\xF3W\x82a\t{\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x85`\xA0\x01Qa\x11\xABV[\x03a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FMust replace active release befo`D\x82\x01R\x7Fre changing status from Active\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x07` R`@\x90 \x80T\x83\x91\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\n\x1EWa\n\x1Ea)\xEEV[\x02\x17\x90UP\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x83\x83`@Qa\nT\x92\x91\x90a,\x80V[`@Q\x80\x91\x03\x90\xA1`\x02\x82`\x03\x81\x11\x15a\npWa\npa)\xEEV[\x03a\n\x83Wa\n~\x83a\x1B\xF7V[PPPV[a\n\x8E`\t\x84a eV[PPPPV[a\n\xAC`\0\x80Q` a0\x03\x839\x81Q\x91R3a\x17\xC1V[a\n\xC9W`@Qc\x80Q\x0F\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x8A`\x03\x81\x11\x15a\n\xDDWa\n\xDDa)\xEEV[\x03a\x0B\x1CWa\n\xFA`\0\x80Q` a0#\x839\x81Q\x91R3a\x17\xC1V[a\x0B\x17W`@Qc.\x10\x89\x1F`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0BnV[`\x01\x8A`\x03\x81\x11\x15a\x0B0Wa\x0B0a)\xEEV[\x14\x15\x80\x15a\x0BPWP`\x03\x8A`\x03\x81\x11\x15a\x0BMWa\x0BMa)\xEEV[\x14\x15[\x15a\x0BnW`@Qc\x07\xA9/\x19`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x8A`\x02\x81\x11\x15a\x0B\x84Wa\x0B\x84a)\xEEV[`\x02\x81\x11\x15a\x0B\x95Wa\x0B\x95a)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x0B\xCBW`@Qc29\xEA_`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04`\0a\x0B\xDA\x8D`\x04a qV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x0C\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FThe provided subnet (within the `D\x82\x01R\x7Frelease id) is not valid for thi`d\x82\x01Ri\x1C\xC8\x18\xDB\xDB\x9D\x1C\x98X\xDD`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\t\xEAV[`\0\x89`\x02\x81\x11\x15a\x0C\x95Wa\x0C\x95a)\xEEV[\x14\x15\x80\x15a\x0C\xB5WP`\x01\x89`\x02\x81\x11\x15a\x0C\xB2Wa\x0C\xB2a)\xEEV[\x14\x15[\x15a\r8W`\0\x8B\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x0C\xDDWa\x0C\xDDa)\xEEV[\x14a\r8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FA release with this ID already e`D\x82\x01Rdxists`\xD8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[`\x02\x89`\x02\x81\x11\x15a\rLWa\rLa)\xEEV[\x03a\r\xB8W`\x02\x85\x16`\0\x03a\r\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FThe RO option is required for pr`D\x82\x01Rjod releases`\xA8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[\x80`\0\x03a\r\xC3WPB[`@Q\x80a\x01@\x01`@R\x80\x8B`\x03\x81\x11\x15a\r\xE1Wa\r\xE1a)\xEEV[\x81R` \x01\x8A`\x02\x81\x11\x15a\r\xF8Wa\r\xF8a)\xEEV[\x81R` \x01\x89`\x03\x81\x11\x15a\x0E\x0FWa\x0E\x0Fa)\xEEV[\x81R` \x01\x88\x81R` \x01\x82\x81R` \x01\x87`\0\x81\x11\x15a\x0E2Wa\x0E2a)\xEEV[\x81R` \x80\x82\x01\x88\x90R`@\x80\x83\x01\x88\x90R``\x83\x01\x87\x90R`\x80\x90\x92\x01\x85\x90R`\0\x8E\x81R`\x07\x90\x91R \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x0E|Wa\x0E|a)\xEEV[\x02\x17\x90UP` \x82\x01Q\x81T\x82\x90a\xFF\0\x19\x16a\x01\0\x83`\x02\x81\x11\x15a\x0E\xA4Wa\x0E\xA4a)\xEEV[\x02\x17\x90UP`@\x82\x01Q\x81T\x82\x90b\xFF\0\0\x19\x16b\x01\0\0\x83`\x03\x81\x11\x15a\x0E\xCEWa\x0E\xCEa)\xEEV[\x02\x17\x90UP``\x82\x01Q`\x01\x82\x01\x90a\x0E\xE7\x90\x82a,\xE4V[P`\x80\x82\x01Q`\x02\x82\x01U`\xA0\x82\x01Q`\x03\x82\x01\x80T`\xFF\x19\x16`\x01\x83\x80\x15a\x0F\x12Wa\x0F\x12a)\xEEV[\x02\x17\x90UP`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x0F5\x90\x82a,\xE4V[Pa\x01\0\x82\x01Q`\x06\x82\x01\x90a\x0FK\x90\x82a,\xE4V[Pa\x01 \x82\x01Q`\x07\x82\x01\x90a\x0Fa\x90\x82a,\xE4V[P\x90PP\x7F\x90\xA8b\xCC\x16\xEB\xCB\x9BT\x9C\x93 \x13\xF7|B-\xF1\x17\xE2MH\xD7\xD2j}\x90\xFB\xC4<<\x8F\x8B\x8B\x8B\x8B\x8B\x86\x8C\x8C\x8C\x8C\x8C`@Qa\x0F\xA8\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a-\xA2V[`@Q\x80\x91\x03\x90\xA1`\x02\x8A`\x03\x81\x11\x15a\x0F\xC4Wa\x0F\xC4a)\xEEV[\x03a\x10\x0CWa\x0F\xD2\x8Ba\x1B\xF7V[\x7Fj\xE2O\xA34\\\xD7H\x8C5\xE1\x1CRx\x05\x9A\nT_\xD0\x08\xC3=-9m\x91{i\xBE\xD5\xBF\x8B\x8B`@Qa\x10\x03\x92\x91\x90a,\x80V[`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPPV[`\0`\x03`\0\x83`\x02\x81\x11\x15a\x101Wa\x101a)\xEEV[`\x02\x81\x11\x15a\x10BWa\x10Ba)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x92\x91PPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x10\x9B\x7F\x96g\xE8\x07\x08\xB6\xEE\xEB\0S\xFA\x0C\xCAD\xE0(\xFFT\x8E*\x9F\x02\x9E\xDF\xEA\xC8|\x11\x8B\x08\xB7\xC83a\x17\xC1V[a\x10\xB8W`@Qc=;+Y`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x10\xDBWa\x10\xDBa)\xEEV[\x03a\x10\xF9W`@Qc\x05\x0C\xC7\xFF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x07` R`@\x81 \x80Tb\xFF\xFF\xFF\x19\x16\x81U\x90a\x11\x1F`\x01\x83\x01\x82a%tV[`\0`\x02\x83\x01\x81\x90U`\x03\x83\x01\x80T`\xFF\x19\x16\x90U`\x04\x83\x01\x81\x90Ua\x11I\x90`\x05\x84\x01\x90a%tV[a\x11W`\x06\x83\x01`\0a%tV[a\x11e`\x07\x83\x01`\0a%tV[Pa\x11s\x90P`\t\x82a eV[P`@Q\x81\x81R\x7F\xE4\xA7\xF7\xB4\x82Q\xADp\xE6?\x80\x07X\xA4E\xB0\x03\x86\xE2\xFA\x98\xD5\xAF\xCE\x96\xA5F\xE8\xFC\xE2\x11N\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\0\x80\x85\x85\x85\x85`@Q` \x01a\x11\xC5\x94\x93\x92\x91\x90a.PV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x90\x81R`\x08\x90\x92R\x90 T\x96\x95PPPPPPV[a\x11\xFD\x82a\x10\\V[a\x12\x06\x81a \xCEV[a\n~\x83\x83a \xDBV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x12\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\t\xEAV[a\x12\x8A\x82\x82a!_V[PPV[a\x12\xA6`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x12\xC3W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x05\x82`@Qa\x12\xD5\x91\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T\x92\x15\x15`\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x7F\xE7s^\x9FV\x9F\xE6\x16qf*\x88)\xDBw\xDE8\xEFaLw\xB1\xB16\xC9X\xAF\xF7\x81\xDF|u\x90a\x11\xA0\x90\x83\x90a+tV[a\x13;`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x13XW`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x03`\0\x83`\x02\x81\x11\x15a\x13pWa\x13pa)\xEEV[`\x02\x81\x11\x15a\x13\x81Wa\x13\x81a)\xEEV[\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x83\x9A\xD2t=@b\xDFW\x9E\xDF8\x18\xF6B\xB7\x1E\xE0h\x8A5\xD6\xBCD8\xEFS\x14\xCE\xCE\x80\x15\x81`@Qa\x11\xA0\x91\x90a.\xB6V[a\x13\xDDa%\xAEV[`\0\x82\x81R`\x07` R`@\x90\x81\x90 \x81Qa\x01@\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x14\x0FWa\x14\x0Fa)\xEEV[`\x03\x81\x11\x15a\x14 Wa\x14 a)\xEEV[\x81R\x81T` \x90\x91\x01\x90a\x01\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14CWa\x14Ca)\xEEV[`\x02\x81\x11\x15a\x14TWa\x14Ta)\xEEV[\x81R\x81T` \x90\x91\x01\x90b\x01\0\0\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x14xWa\x14xa)\xEEV[`\x03\x81\x11\x15a\x14\x89Wa\x14\x89a)\xEEV[\x81R` \x01`\x01\x82\x01\x80Ta\x14\x9D\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xC9\x90a,FV[\x80\x15a\x15\x16W\x80`\x1F\x10a\x14\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x16V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xF9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`@\x90\x91\x01\x90`\xFF\x16\x80\x15a\x15DWa\x15Da)\xEEV[\x80\x15a\x15RWa\x15Ra)\xEEV[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x15p\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x9C\x90a,FV[\x80\x15a\x15\xE9W\x80`\x1F\x10a\x15\xBEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xE9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xCCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x16\x02\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16.\x90a,FV[\x80\x15a\x16{W\x80`\x1F\x10a\x16PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16{V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16^W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x16\x94\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xC0\x90a,FV[\x80\x15a\x17\rW\x80`\x1F\x10a\x16\xE2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[a\x175`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x17RW`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x82`\x02\x81\x11\x15a\x17hWa\x17ha)\xEEV[`\x02\x81\x11\x15a\x17yWa\x17ya)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\xFF\x19\x16\x90UQ\x7F?\x17\x8F\x17\xDA\xE6\xCA\xF8\xCA\t\xC4\x85u\x02\xBA\xF7tN\x85\x97\xDEB\xD6Ydv\xFE\x9E\x06\xB8\xADG\x90a\x11\xA0\x90\x83\x90a.\xB6V[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x02\x80Ta\x17\xF9\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18%\x90a,FV[\x80\x15a\x18rW\x80`\x1F\x10a\x18GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[a\x18\x94`\0\x80Q` a0\x03\x839\x81Q\x91R3a\x17\xC1V[a\x18\xB1W`@Qc\x80Q\x0F\xE1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03`\0\x85`\x02\x81\x11\x15a\x18\xC7Wa\x18\xC7a)\xEEV[`\x02\x81\x11\x15a\x18\xD8Wa\x18\xD8a)\xEEV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15\x15`\x01\x14a\x19\x0EW`@Qc29\xEA_`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02a\x19\x1A\x83\x82a,\xE4V[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x90Q`\x06\x90a\x19S\x90\x84\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T\x92\x15\x15`\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x7F\x18\xF09lm\x01\x87mv\x10\xD9hw\xB4O\x01z\xB4\xCA\"e\xB10y\xB1\x0E\x0B\xE6\xB6\xAF0\xD4\x90a\x19\xA1\x90\x83\x90a+tV[`@Q\x80\x91\x03\x90\xA1\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x83`@Qa\x19\xD8\x91\x90a.\xC4V[`@Q\x80\x91\x03\x90\xA1\x7F[Y\x0C\xCE\xAB\xBE\xB2\x8C\xD1nA\xA8\x11\xBAF\xDF[\xB8,\x98\x92\r^\x7F\xF7~O\xA9\xDBp\x9B3\x82\x82`@Qa\x1A\x11\x92\x91\x90a.\xD8V[`@Q\x80\x91\x03\x90\xA1PPPPV[a\x1A(\x82a\x10\\V[a\x1A1\x81a \xCEV[a\n~\x83\x83a!_V[`\0`\x06\x82`@Qa\x04\xD4\x91\x90a,*V[a\x1Ae`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x1A\x82W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\xF5\x80\xA1\xEA\x01\xC0\xED>\xC5\xE4y|V\x0E\xE7\x04l5\x91X\x8C\x0F\xEB\x81O_-N\xCAP',\x90a\x11\xA0\x90\x83\x90a.\xC4V[a\x1A\xEA`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x1B\x07W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05\x81`@Qa\x1B\x17\x91\x90a,*V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x81 \x80T`\xFF\x19\x16\x90U\x7F\xF10B%]\x0BX\xF7\xAC\xC8, \x1A\x0C\xD4\xD84\xB0x=\xD7\xB3\xC2\x9C\xCA\x9B\x96\x84\rc?t\x90a\x11\xA0\x90\x83\x90a+tV[a\x1Bs`\0\x80Q` a0C\x839\x81Q\x91R3a\x17\xC1V[a\x1B\x90W`@QcdH|%`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x1A{xD\x16\xB56r\x84J\x12\x94zYin\x83Zm\x8D\xFF\xBB\x0CF0\xA4\xD0H\x12\xBDbx\x90a\x11\xA0\x90\x83\x90a.\xC4V[```\0a\x1B\xF0\x83a!\xC4V[\x93\x92PPPV[`\0\x81\x81R`\x07` R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x1C\x1AWa\x1C\x1Aa)\xEEV[\x03a\x1C8W`@Qc\x05\x0C\xC7\xFF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x07` R`@\x80\x82 \x81Qa\x01@\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x03\x81\x11\x15a\x1CiWa\x1Cia)\xEEV[`\x03\x81\x11\x15a\x1CzWa\x1Cza)\xEEV[\x81R\x81T` \x90\x91\x01\x90a\x01\0\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1C\x9DWa\x1C\x9Da)\xEEV[`\x02\x81\x11\x15a\x1C\xAEWa\x1C\xAEa)\xEEV[\x81R\x81T` \x90\x91\x01\x90b\x01\0\0\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x1C\xD2Wa\x1C\xD2a)\xEEV[`\x03\x81\x11\x15a\x1C\xE3Wa\x1C\xE3a)\xEEV[\x81R` \x01`\x01\x82\x01\x80Ta\x1C\xF7\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D#\x90a,FV[\x80\x15a\x1DpW\x80`\x1F\x10a\x1DEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1DpV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1DSW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01T` \x82\x01R`\x03\x82\x01T`@\x90\x91\x01\x90`\xFF\x16\x80\x15a\x1D\x9EWa\x1D\x9Ea)\xEEV[\x80\x15a\x1D\xACWa\x1D\xACa)\xEEV[\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01\x80Ta\x1D\xCA\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xF6\x90a,FV[\x80\x15a\x1ECW\x80`\x1F\x10a\x1E\x18Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1ECV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E&W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01\x80Ta\x1E\\\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\x88\x90a,FV[\x80\x15a\x1E\xD5W\x80`\x1F\x10a\x1E\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x07\x82\x01\x80Ta\x1E\xEE\x90a,FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\x1A\x90a,FV[\x80\x15a\x1FgW\x80`\x1F\x10a\x1F<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1FgV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1FJW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\x02`\x03\x81\x11\x15a\x1F\x85Wa\x1F\x85a)\xEEV[\x81Q`\x03\x81\x11\x15a\x1F\x98Wa\x1F\x98a)\xEEV[\x14a\x1F\xB6W`@Qc\x07\xA9/\x19`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84`\xA0\x01Q`@Q` \x01a\x1F\xDF\x94\x93\x92\x91\x90a.PV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x08\x90\x93R\x91 T\x90\x91P\x80\x15a AW`\0\x81\x81R`\x07` R`@\x90 `\x02\x01T`\x80\x84\x01Q\x11\x15a <W`\0\x82\x81R`\x08` R`@\x90 \x84\x90U[a SV[`\0\x82\x81R`\x08` R`@\x90 \x84\x90U[a ^`\t\x85a\" V[PPPPPV[`\0a\x1B\xF0\x83\x83a\",V[`\0\x80`\0[`\x14\x81\x10\x15a \xC3Wa \x8B\x81`\x08a/\x13V[\x85a \x96\x83\x87a/*V[` \x81\x10a \xA6Wa \xA6a/=V[\x1A`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x1C\x91\x90\x91\x17\x90`\x01\x01a wV[P``\x1C\x93\x92PPPV[a \xD8\x813a#\x1FV[PV[a \xE5\x82\x82a\x17\xC1V[a\x12\x8AW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua!\x1B3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a!i\x82\x82a\x17\xC1V[\x15a\x12\x8AW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[``\x81`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\"\x14W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\"\0W[PPPPP\x90P\x91\x90PV[`\0a\x1B\xF0\x83\x83a#xV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a#\x15W`\0a\"P`\x01\x83a/SV[\x85T\x90\x91P`\0\x90a\"d\x90`\x01\x90a/SV[\x90P\x81\x81\x14a\"\xC9W`\0\x86`\0\x01\x82\x81T\x81\x10a\"\x84Wa\"\x84a/=V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\"\xA7Wa\"\xA7a/=V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\"\xDAWa\"\xDAa/fV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x04\xABV[`\0\x91PPa\x04\xABV[a#)\x82\x82a\x17\xC1V[a\x12\x8AWa#6\x81a#\xC7V[a#A\x83` a#\xD9V[`@Q` \x01a#R\x92\x91\x90a/|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\t\xEA\x91`\x04\x01a+tV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta#\xBFWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04\xABV[P`\0a\x04\xABV[``a\x04\xAB`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a#\xE8\x83`\x02a/\x13V[a#\xF3\x90`\x02a/*V[`\x01`\x01`@\x1B\x03\x81\x11\x15a$\nWa$\na&\x98V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$4W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a$OWa$Oa/=V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a$~Wa$~a/=V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a$\xA2\x84`\x02a/\x13V[a$\xAD\x90`\x01a/*V[\x90P[`\x01\x81\x11\x15a%%Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a$\xE1Wa$\xE1a/=V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a$\xF7Wa$\xF7a/=V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a%\x1E\x81a/\xEBV[\x90Pa$\xB0V[P\x83\x15a\x1B\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\t\xEAV[P\x80Ta%\x80\x90a,FV[`\0\x82U\x80`\x1F\x10a%\x90WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a \xD8\x91\x90a&\x12V[`@\x80Qa\x01@\x81\x01\x90\x91R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x80\x81\x11\x15a%\xF0Wa%\xF0a)\xEEV[\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[[\x80\x82\x11\x15a&'W`\0\x81U`\x01\x01a&\x13V[P\x90V[`\0` \x82\x84\x03\x12\x15a&=W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1B\xF0W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a&\x8DW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&oV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a&\xBFW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xD8Wa&\xD8a&\x98V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a'\x06Wa'\x06a&\x98V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a'\x1EW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a'MW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'cW`\0\x80\xFD[a'o\x84\x82\x85\x01a&\xAEV[\x94\x93PPPPV[`\x04\x81\x10a \xD8W`\0\x80\xFD[\x805a'\x8F\x81a'wV[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xA7W`\0\x80\xFD[\x825\x91P` \x83\x015a'\xB9\x81a'wV[\x80\x91PP\x92P\x92\x90PV[\x805`\x03\x81\x10a'\x8FW`\0\x80\xFD[\x805`\x01\x81\x10a'\x8FW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a(\x04W`\0\x80\xFD[\x8B5\x9APa(\x14` \x8D\x01a'\x84V[\x99Pa(\"`@\x8D\x01a'\xC4V[\x98Pa(0``\x8D\x01a'\x84V[\x97P`\x80\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(KW`\0\x80\xFD[a(W\x8E\x82\x8F\x01a&\xAEV[\x97PPa(f`\xA0\x8D\x01a'\xD3V[\x95P`\xC0\x8C\x015\x94P`\xE0\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\x88W`\0\x80\xFD[a(\x94\x8E\x82\x8F\x01a&\xAEV[\x94PPa\x01\0\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xB1W`\0\x80\xFD[a(\xBD\x8E\x82\x8F\x01a&\xAEV[\x93PPa\x01 \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xDAW`\0\x80\xFD[a(\xE6\x8E\x82\x8F\x01a&\xAEV[\x92PP`\0a\x01@\x8D\x015\x90P\x80\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0` \x82\x84\x03\x12\x15a)\x19W`\0\x80\xFD[a\x1B\xF0\x82a'\xC4V[`\0` \x82\x84\x03\x12\x15a)4W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)QW`\0\x80\xFD[a)Z\x85a'\xC4V[\x93P` \x85\x015a)j\x81a'wV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x85W`\0\x80\xFD[a)\x91\x87\x82\x88\x01a&\xAEV[\x92PPa)\xA0``\x86\x01a'\xD3V[\x90P\x92\x95\x91\x94P\x92PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\x8FW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a)\xD5W`\0\x80\xFD[\x825\x91Pa)\xE5` \x84\x01a)\xABV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x04\x81\x10a \xD8Wa \xD8a)\xEEV[a*\x1D\x81a*\x04V[\x90RV[`\x03\x81\x10a*\x1DWa*\x1Da)\xEEV[`\0[\x83\x81\x10\x15a*LW\x81\x81\x01Q\x83\x82\x01R` \x01a*4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra*m\x81` \x86\x01` \x86\x01a*1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x81\x10a*\x1DWa*\x1Da)\xEEV[` \x81Ra*\xA3` \x82\x01\x83Qa*\x14V[`\0` \x83\x01Qa*\xB7`@\x84\x01\x82a*!V[P`@\x83\x01Qa*\xCA``\x84\x01\x82a*\x14V[P``\x83\x01Qa\x01@`\x80\x84\x01Ra*\xE6a\x01`\x84\x01\x82a*UV[\x90P`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Qa+\x04`\xC0\x85\x01\x82a*\x81V[P`\xC0\x84\x01Q`\xE0\x84\x01R`\xE0\x84\x01Q`\x1F\x19\x84\x83\x03\x01a\x01\0\x85\x01Ra++\x82\x82a*UV[\x91PPa\x01\0\x84\x01Q`\x1F\x19\x84\x83\x03\x01a\x01 \x85\x01Ra+K\x82\x82a*UV[\x91PPa\x01 \x84\x01Q`\x1F\x19\x84\x83\x03\x01a\x01@\x85\x01Ra+k\x82\x82a*UV[\x95\x94PPPPPV[` \x81R`\0a\x1B\xF0` \x83\x01\x84a*UV[`\0` \x82\x84\x03\x12\x15a+\x99W`\0\x80\xFD[a\x1B\xF0\x82a)\xABV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xB8W`\0\x80\xFD[a+\xC1\x85a'\xC4V[\x93Pa+\xCF` \x86\x01a)\xABV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xEAW`\0\x80\xFD[a+\xF6\x87\x82\x88\x01a&\xAEV[\x92PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x12W`\0\x80\xFD[a,\x1E\x87\x82\x88\x01a&\xAEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x82Qa,<\x81\x84` \x87\x01a*1V[\x91\x90\x91\x01\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a,ZW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a,zWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x82\x81R`@\x81\x01a,\x90\x83a*\x04V[\x82` \x83\x01R\x93\x92PPPV[`\x1F\x82\x11\x15a\n~W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a,\xC4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a ^W`\0\x81U`\x01\x01a,\xD0V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a,\xFDWa,\xFDa&\x98V[a-\x11\x81a-\x0B\x84Ta,FV[\x84a,\x9DV[` `\x1F\x82\x11`\x01\x81\x14a-EW`\0\x83\x15a--WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua ^V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a-uW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a-UV[P\x84\x82\x10\x15a-\x93W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x8B\x81Ra-\xAE\x8Ba*\x04V[\x8A` \x82\x01Ra-\xC1`@\x82\x01\x8Ba*!V[a-\xCA\x89a*\x04V[\x88``\x82\x01Ra\x01``\x80\x82\x01R`\0a-\xE8a\x01`\x83\x01\x8Aa*UV[\x88`\xA0\x84\x01Ra-\xFB`\xC0\x84\x01\x89a*\x81V[\x86`\xE0\x84\x01R\x82\x81\x03a\x01\0\x84\x01Ra.\x14\x81\x87a*UV[\x90P\x82\x81\x03a\x01 \x84\x01Ra.)\x81\x86a*UV[\x90P\x82\x81\x03a\x01@\x84\x01Ra.>\x81\x85a*UV[\x9E\x9DPPPPPPPPPPPPPPV[`\0`\x03\x86\x10a.bWa.ba)\xEEV[\x85`\xF8\x1B\x82Ra.q\x85a*\x04V[\x84`\xF8\x1B`\x01\x83\x01R\x83Qa.\x8D\x81`\x02\x85\x01` \x88\x01a*1V[\x82\x01`\x01\x84\x10a.\x9FWa.\x9Fa)\xEEV[`\xF8\x93\x90\x93\x1B`\x02\x84\x01RPP`\x03\x01\x93\x92PPPV[` \x81\x01a\x04\xAB\x82\x84a*!V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@\x81R`\0a.\xEB`@\x83\x01\x85a*UV[\x82\x81\x03` \x84\x01Ra+k\x81\x85a*UV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xABWa\x04\xABa.\xFDV[\x80\x82\x01\x80\x82\x11\x15a\x04\xABWa\x04\xABa.\xFDV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xABWa\x04\xABa.\xFDV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[v\x02\x0B\x1B\x1B+\x9B\x9A\x1B{s\xA3\x93{a\xD1\x03\x0B\x1B\x1B{\xABs\xA1`M\x1B\x81R`\0\x83Qa/\xAE\x81`\x17\x85\x01` \x88\x01a*1V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa/\xDF\x81`(\x84\x01` \x88\x01a*1V[\x01`(\x01\x94\x93PPPPV[`\0\x81a/\xFAWa/\xFAa.\xFDV[P`\0\x19\x01\x90V\xFE<%\x19\xC4H}GqHr\xF9,\xF9\nP\xC2_]\xEA\xEC'\x89\xDC*I{\x12r\xDFa\x1D\xB6\xCE\x1F\x15i(#\xE8\xA9\xD7|\xA8\xC1\xB7\xA2\xCC\x14_\xFD\0\x87P\xEE\x9D?\x86\x04\xF9\xC5.\xEE\xA7<\xDF\x8BLR\x0F\xFE\x19|SC\xC6\xF5\xAE\xC5\x95p\x15\x1E\xF9\xA4\x92\xF2\xC6$\xFDE\xDD\xDEa5\xECB\xA2dipfsX\"\x12 \x9Fc\x06\x8F\xDC\x11\xDF\x99;Z\xEDw\x94\xC2\x16\xFB&n\x1E\x9F\x01\xBE\x1B\x1Bu2\x11\xB3Z\xBD\x1A7dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static RELEASEREGISTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ReleaseRegister<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ReleaseRegister<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ReleaseRegister<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ReleaseRegister<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ReleaseRegister<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ReleaseRegister))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ReleaseRegister<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RELEASEREGISTER_ABI.clone(),
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
                RELEASEREGISTER_ABI.clone(),
                RELEASEREGISTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ACTIVATOR_ROLE` (0x08740a3b) function
        pub fn activator_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 116, 10, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ADMIN_ROLE` (0x75b238fc) function
        pub fn admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([117, 178, 56, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BURNER_ROLE` (0x282c51f3) function
        pub fn burner_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([40, 44, 81, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CREATOR_ROLE` (0x8aeda25a) function
        pub fn creator_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([138, 237, 162, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEACTIVATOR_ROLE` (0x458be7dc) function
        pub fn deactivator_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([69, 139, 231, 220], ())
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
        ///Calls the contract's `RELEASE_OPTION_RO` (0xad769394) function
        pub fn release_option_ro(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 118, 147, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELEASE_OPTION_SSH` (0x3acd1ea3) function
        pub fn release_option_ssh(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 205, 30, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RELEASE_OPTION_USERS` (0x277dceaf) function
        pub fn release_option_users(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 125, 206, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAllowedAdminSigningPublicKey` (0x70e65a45) function
        pub fn add_allowed_admin_signing_public_key(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 230, 90, 69], pub_key)
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
        ///Calls the contract's `addAllowedSubnet` (0xf2dc9916) function
        pub fn add_allowed_subnet(
            &self,
            subnet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 220, 153, 22], subnet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnRelease` (0x2609e586) function
        pub fn burn_release(
            &self,
            release_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 9, 229, 134], release_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createRelease` (0x19724065) function
        pub fn create_release(
            &self,
            release_id: [u8; 32],
            status: u8,
            env: u8,
            typ: u8,
            kind: ::ethers::core::types::Bytes,
            platform: u8,
            options: ::ethers::core::types::U256,
            id_key_digest: ::ethers::core::types::Bytes,
            public_key: ::ethers::core::types::Bytes,
            cid: ::ethers::core::types::Bytes,
            date: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [25, 114, 64, 101],
                    (
                        release_id,
                        status,
                        env,
                        typ,
                        kind,
                        platform,
                        options,
                        id_key_digest,
                        public_key,
                        cid,
                        date,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActiveRelease` (0x2ae79b6d) function
        pub fn get_active_release(
            &self,
            env: u8,
            typ: u8,
            kind: ::ethers::core::types::Bytes,
            platform: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([42, 231, 155, 109], (env, typ, kind, platform))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActiveReleases` (0x023e9288) function
        pub fn get_active_releases(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([2, 62, 146, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCreatorDomain` (0x9bb4e2f7) function
        pub fn get_creator_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([155, 180, 226, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRelease` (0x7f698e92) function
        pub fn get_release(
            &self,
            release_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Release> {
            self.0
                .method_hash([127, 105, 142, 146], release_id)
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
        ///Calls the contract's `hasAllowedAdminSigningPublicKey` (0x0e092b18) function
        pub fn has_allowed_admin_signing_public_key(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 9, 43, 24], pub_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllowedAuthorKeyDigest` (0xd6bc626d) function
        pub fn has_allowed_author_key_digest(
            &self,
            digest: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([214, 188, 98, 109], digest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllowedEnv` (0x1bd564dc) function
        pub fn has_allowed_env(
            &self,
            env: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 213, 100, 220], env)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAllowedSubnet` (0xa0908300) function
        pub fn has_allowed_subnet(
            &self,
            subnet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([160, 144, 131, 0], subnet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasCreatorInit` (0x3dc6c858) function
        pub fn has_creator_init(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 198, 200, 88], ())
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
        ///Calls the contract's `initCreator` (0xbc7ca317) function
        pub fn init_creator(
            &self,
            env: u8,
            subnet_id: ::ethers::core::types::Address,
            domain: ::ethers::core::types::Bytes,
            author_key_digest: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [188, 124, 163, 23],
                    (env, subnet_id, domain, author_key_digest),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAllowedAdminSigningPublicKey` (0xe1c0af08) function
        pub fn remove_allowed_admin_signing_public_key(
            &self,
            pub_key: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 192, 175, 8], pub_key)
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
        ///Calls the contract's `removeAllowedSubnet` (0xdb2d303b) function
        pub fn remove_allowed_subnet(
            &self,
            subnet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 45, 48, 59], subnet)
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
        ///Calls the contract's `setReleaseStatus` (0x0e1e59dd) function
        pub fn set_release_status(
            &self,
            release_id: [u8; 32],
            status: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 30, 89, 221], (release_id, status))
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
        ///Gets the contract's `AllowedAdminSigningPublicKeyAdded` event
        pub fn allowed_admin_signing_public_key_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAdminSigningPublicKeyAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedAdminSigningPublicKeyRemoved` event
        pub fn allowed_admin_signing_public_key_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAdminSigningPublicKeyRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedAuthorKeyDigestAdded` event
        pub fn allowed_author_key_digest_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAuthorKeyDigestAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedAuthorKeyDigestRemoved` event
        pub fn allowed_author_key_digest_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedAuthorKeyDigestRemovedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `AllowedSubnetAdded` event
        pub fn allowed_subnet_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedSubnetAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowedSubnetRemoved` event
        pub fn allowed_subnet_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowedSubnetRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InitCreator` event
        pub fn init_creator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitCreatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReleaseBurned` event
        pub fn release_burned_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseBurnedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReleaseCreated` event
        pub fn release_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReleaseStatusChange` event
        pub fn release_status_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseStatusChangeFilter,
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
            ReleaseRegisterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ReleaseRegister<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ActivatorRoleRequired` with signature `ActivatorRoleRequired()` and selector `0x5c21123e`
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
    #[etherror(name = "ActivatorRoleRequired", abi = "ActivatorRoleRequired()")]
    pub struct ActivatorRoleRequired;
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
    ///Custom Error type `BurnerRoleRequired` with signature `BurnerRoleRequired()` and selector `0xf4ecad64`
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
    #[etherror(name = "BurnerRoleRequired", abi = "BurnerRoleRequired()")]
    pub struct BurnerRoleRequired;
    ///Custom Error type `CreatorRoleRequired` with signature `CreatorRoleRequired()` and selector `0x80510fe1`
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
    #[etherror(name = "CreatorRoleRequired", abi = "CreatorRoleRequired()")]
    pub struct CreatorRoleRequired;
    ///Custom Error type `DeactivatorRoleRequired` with signature `DeactivatorRoleRequired()` and selector `0x093efaf9`
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
    #[etherror(name = "DeactivatorRoleRequired", abi = "DeactivatorRoleRequired()")]
    pub struct DeactivatorRoleRequired;
    ///Custom Error type `InvalidEnv` with signature `InvalidEnv()` and selector `0xc8e7a97c`
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
    #[etherror(name = "InvalidEnv", abi = "InvalidEnv()")]
    pub struct InvalidEnv;
    ///Custom Error type `InvalidStatus` with signature `InvalidStatus()` and selector `0xf525e320`
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
    #[etherror(name = "InvalidStatus", abi = "InvalidStatus()")]
    pub struct InvalidStatus;
    ///Custom Error type `ReleaseNotFound` with signature `ReleaseNotFound()` and selector `0x28663ff8`
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
    #[etherror(name = "ReleaseNotFound", abi = "ReleaseNotFound()")]
    pub struct ReleaseNotFound;
    ///Container type for all of the contract's custom errors
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
    pub enum ReleaseRegisterErrors {
        ActivatorRoleRequired(ActivatorRoleRequired),
        AdminRoleRequired(AdminRoleRequired),
        BurnerRoleRequired(BurnerRoleRequired),
        CreatorRoleRequired(CreatorRoleRequired),
        DeactivatorRoleRequired(DeactivatorRoleRequired),
        InvalidEnv(InvalidEnv),
        InvalidStatus(InvalidStatus),
        ReleaseNotFound(ReleaseNotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ReleaseRegisterErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ActivatorRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActivatorRoleRequired(decoded));
            }
            if let Ok(decoded) = <AdminRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminRoleRequired(decoded));
            }
            if let Ok(decoded) = <BurnerRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnerRoleRequired(decoded));
            }
            if let Ok(decoded) = <CreatorRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatorRoleRequired(decoded));
            }
            if let Ok(decoded) = <DeactivatorRoleRequired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivatorRoleRequired(decoded));
            }
            if let Ok(decoded) = <InvalidEnv as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidEnv(decoded));
            }
            if let Ok(decoded) = <InvalidStatus as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidStatus(decoded));
            }
            if let Ok(decoded) = <ReleaseNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseNotFound(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReleaseRegisterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ActivatorRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnerRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatorRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivatorRoleRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ReleaseRegisterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ActivatorRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AdminRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BurnerRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CreatorRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DeactivatorRoleRequired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEnv as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidStatus as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReleaseNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ReleaseRegisterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActivatorRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdminRoleRequired(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnerRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreatorRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeactivatorRoleRequired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ReleaseRegisterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ActivatorRoleRequired> for ReleaseRegisterErrors {
        fn from(value: ActivatorRoleRequired) -> Self {
            Self::ActivatorRoleRequired(value)
        }
    }
    impl ::core::convert::From<AdminRoleRequired> for ReleaseRegisterErrors {
        fn from(value: AdminRoleRequired) -> Self {
            Self::AdminRoleRequired(value)
        }
    }
    impl ::core::convert::From<BurnerRoleRequired> for ReleaseRegisterErrors {
        fn from(value: BurnerRoleRequired) -> Self {
            Self::BurnerRoleRequired(value)
        }
    }
    impl ::core::convert::From<CreatorRoleRequired> for ReleaseRegisterErrors {
        fn from(value: CreatorRoleRequired) -> Self {
            Self::CreatorRoleRequired(value)
        }
    }
    impl ::core::convert::From<DeactivatorRoleRequired> for ReleaseRegisterErrors {
        fn from(value: DeactivatorRoleRequired) -> Self {
            Self::DeactivatorRoleRequired(value)
        }
    }
    impl ::core::convert::From<InvalidEnv> for ReleaseRegisterErrors {
        fn from(value: InvalidEnv) -> Self {
            Self::InvalidEnv(value)
        }
    }
    impl ::core::convert::From<InvalidStatus> for ReleaseRegisterErrors {
        fn from(value: InvalidStatus) -> Self {
            Self::InvalidStatus(value)
        }
    }
    impl ::core::convert::From<ReleaseNotFound> for ReleaseRegisterErrors {
        fn from(value: ReleaseNotFound) -> Self {
            Self::ReleaseNotFound(value)
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
        name = "AllowedAdminSigningPublicKeyAdded",
        abi = "AllowedAdminSigningPublicKeyAdded(bytes)"
    )]
    pub struct AllowedAdminSigningPublicKeyAddedFilter {
        pub pub_key: ::ethers::core::types::Bytes,
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
        name = "AllowedAdminSigningPublicKeyRemoved",
        abi = "AllowedAdminSigningPublicKeyRemoved(bytes)"
    )]
    pub struct AllowedAdminSigningPublicKeyRemovedFilter {
        pub pub_key: ::ethers::core::types::Bytes,
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
        name = "AllowedAuthorKeyDigestAdded",
        abi = "AllowedAuthorKeyDigestAdded(bytes)"
    )]
    pub struct AllowedAuthorKeyDigestAddedFilter {
        pub digest: ::ethers::core::types::Bytes,
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
        name = "AllowedAuthorKeyDigestRemoved",
        abi = "AllowedAuthorKeyDigestRemoved(bytes)"
    )]
    pub struct AllowedAuthorKeyDigestRemovedFilter {
        pub digest: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "AllowedSubnetAdded", abi = "AllowedSubnetAdded(address)")]
    pub struct AllowedSubnetAddedFilter {
        pub subnet: ::ethers::core::types::Address,
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
    #[ethevent(name = "AllowedSubnetRemoved", abi = "AllowedSubnetRemoved(address)")]
    pub struct AllowedSubnetRemovedFilter {
        pub subnet: ::ethers::core::types::Address,
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
    #[ethevent(name = "InitCreator", abi = "InitCreator(bytes,bytes)")]
    pub struct InitCreatorFilter {
        pub domain: ::ethers::core::types::Bytes,
        pub author_key_digest: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "ReleaseBurned", abi = "ReleaseBurned(bytes32)")]
    pub struct ReleaseBurnedFilter {
        pub release_id: [u8; 32],
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
        name = "ReleaseCreated",
        abi = "ReleaseCreated(bytes32,uint8,uint8,uint8,bytes,uint256,uint8,uint256,bytes,bytes,bytes)"
    )]
    pub struct ReleaseCreatedFilter {
        pub release_id: [u8; 32],
        pub status: u8,
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub date: ::ethers::core::types::U256,
        pub platform: u8,
        pub options: ::ethers::core::types::U256,
        pub id_key_digest: ::ethers::core::types::Bytes,
        pub public_key: ::ethers::core::types::Bytes,
        pub cid: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "ReleaseStatusChange", abi = "ReleaseStatusChange(bytes32,uint8)")]
    pub struct ReleaseStatusChangeFilter {
        pub release_id: [u8; 32],
        pub status: u8,
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
    pub enum ReleaseRegisterEvents {
        AllowedAdminSigningPublicKeyAddedFilter(AllowedAdminSigningPublicKeyAddedFilter),
        AllowedAdminSigningPublicKeyRemovedFilter(
            AllowedAdminSigningPublicKeyRemovedFilter,
        ),
        AllowedAuthorKeyDigestAddedFilter(AllowedAuthorKeyDigestAddedFilter),
        AllowedAuthorKeyDigestRemovedFilter(AllowedAuthorKeyDigestRemovedFilter),
        AllowedEnvAddedFilter(AllowedEnvAddedFilter),
        AllowedEnvRemovedFilter(AllowedEnvRemovedFilter),
        AllowedSubnetAddedFilter(AllowedSubnetAddedFilter),
        AllowedSubnetRemovedFilter(AllowedSubnetRemovedFilter),
        InitCreatorFilter(InitCreatorFilter),
        ReleaseBurnedFilter(ReleaseBurnedFilter),
        ReleaseCreatedFilter(ReleaseCreatedFilter),
        ReleaseStatusChangeFilter(ReleaseStatusChangeFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ReleaseRegisterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllowedAdminSigningPublicKeyAddedFilter::decode_log(
                log,
            ) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAdminSigningPublicKeyAddedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AllowedAdminSigningPublicKeyRemovedFilter::decode_log(
                log,
            ) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAdminSigningPublicKeyRemovedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AllowedAuthorKeyDigestAddedFilter::decode_log(log) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAuthorKeyDigestAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = AllowedAuthorKeyDigestRemovedFilter::decode_log(log) {
                return Ok(
                    ReleaseRegisterEvents::AllowedAuthorKeyDigestRemovedFilter(decoded),
                );
            }
            if let Ok(decoded) = AllowedEnvAddedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedEnvAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowedEnvRemovedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedEnvRemovedFilter(decoded));
            }
            if let Ok(decoded) = AllowedSubnetAddedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedSubnetAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowedSubnetRemovedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::AllowedSubnetRemovedFilter(decoded));
            }
            if let Ok(decoded) = InitCreatorFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::InitCreatorFilter(decoded));
            }
            if let Ok(decoded) = ReleaseBurnedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::ReleaseBurnedFilter(decoded));
            }
            if let Ok(decoded) = ReleaseCreatedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::ReleaseCreatedFilter(decoded));
            }
            if let Ok(decoded) = ReleaseStatusChangeFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::ReleaseStatusChangeFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ReleaseRegisterEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ReleaseRegisterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowedAdminSigningPublicKeyAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedAdminSigningPublicKeyRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedAuthorKeyDigestAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedAuthorKeyDigestRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedEnvAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedEnvRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedSubnetAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedSubnetRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitCreatorFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseBurnedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReleaseCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReleaseStatusChangeFilter(element) => {
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
    impl ::core::convert::From<AllowedAdminSigningPublicKeyAddedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAdminSigningPublicKeyAddedFilter) -> Self {
            Self::AllowedAdminSigningPublicKeyAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedAdminSigningPublicKeyRemovedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAdminSigningPublicKeyRemovedFilter) -> Self {
            Self::AllowedAdminSigningPublicKeyRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedAuthorKeyDigestAddedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAuthorKeyDigestAddedFilter) -> Self {
            Self::AllowedAuthorKeyDigestAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedAuthorKeyDigestRemovedFilter>
    for ReleaseRegisterEvents {
        fn from(value: AllowedAuthorKeyDigestRemovedFilter) -> Self {
            Self::AllowedAuthorKeyDigestRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedEnvAddedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedEnvAddedFilter) -> Self {
            Self::AllowedEnvAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedEnvRemovedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedEnvRemovedFilter) -> Self {
            Self::AllowedEnvRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedSubnetAddedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedSubnetAddedFilter) -> Self {
            Self::AllowedSubnetAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowedSubnetRemovedFilter> for ReleaseRegisterEvents {
        fn from(value: AllowedSubnetRemovedFilter) -> Self {
            Self::AllowedSubnetRemovedFilter(value)
        }
    }
    impl ::core::convert::From<InitCreatorFilter> for ReleaseRegisterEvents {
        fn from(value: InitCreatorFilter) -> Self {
            Self::InitCreatorFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseBurnedFilter> for ReleaseRegisterEvents {
        fn from(value: ReleaseBurnedFilter) -> Self {
            Self::ReleaseBurnedFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseCreatedFilter> for ReleaseRegisterEvents {
        fn from(value: ReleaseCreatedFilter) -> Self {
            Self::ReleaseCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseStatusChangeFilter> for ReleaseRegisterEvents {
        fn from(value: ReleaseStatusChangeFilter) -> Self {
            Self::ReleaseStatusChangeFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ReleaseRegisterEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ReleaseRegisterEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ReleaseRegisterEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ACTIVATOR_ROLE` function with signature `ACTIVATOR_ROLE()` and selector `0x08740a3b`
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
    #[ethcall(name = "ACTIVATOR_ROLE", abi = "ACTIVATOR_ROLE()")]
    pub struct ActivatorRoleCall;
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
    ///Container type for all input parameters for the `BURNER_ROLE` function with signature `BURNER_ROLE()` and selector `0x282c51f3`
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
    #[ethcall(name = "BURNER_ROLE", abi = "BURNER_ROLE()")]
    pub struct BurnerRoleCall;
    ///Container type for all input parameters for the `CREATOR_ROLE` function with signature `CREATOR_ROLE()` and selector `0x8aeda25a`
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
    #[ethcall(name = "CREATOR_ROLE", abi = "CREATOR_ROLE()")]
    pub struct CreatorRoleCall;
    ///Container type for all input parameters for the `DEACTIVATOR_ROLE` function with signature `DEACTIVATOR_ROLE()` and selector `0x458be7dc`
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
    #[ethcall(name = "DEACTIVATOR_ROLE", abi = "DEACTIVATOR_ROLE()")]
    pub struct DeactivatorRoleCall;
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
    ///Container type for all input parameters for the `RELEASE_OPTION_RO` function with signature `RELEASE_OPTION_RO()` and selector `0xad769394`
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
    #[ethcall(name = "RELEASE_OPTION_RO", abi = "RELEASE_OPTION_RO()")]
    pub struct ReleaseOptionRoCall;
    ///Container type for all input parameters for the `RELEASE_OPTION_SSH` function with signature `RELEASE_OPTION_SSH()` and selector `0x3acd1ea3`
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
    #[ethcall(name = "RELEASE_OPTION_SSH", abi = "RELEASE_OPTION_SSH()")]
    pub struct ReleaseOptionSshCall;
    ///Container type for all input parameters for the `RELEASE_OPTION_USERS` function with signature `RELEASE_OPTION_USERS()` and selector `0x277dceaf`
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
    #[ethcall(name = "RELEASE_OPTION_USERS", abi = "RELEASE_OPTION_USERS()")]
    pub struct ReleaseOptionUsersCall;
    ///Container type for all input parameters for the `addAllowedAdminSigningPublicKey` function with signature `addAllowedAdminSigningPublicKey(bytes)` and selector `0x70e65a45`
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
        name = "addAllowedAdminSigningPublicKey",
        abi = "addAllowedAdminSigningPublicKey(bytes)"
    )]
    pub struct AddAllowedAdminSigningPublicKeyCall {
        pub pub_key: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `addAllowedSubnet` function with signature `addAllowedSubnet(address)` and selector `0xf2dc9916`
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
    #[ethcall(name = "addAllowedSubnet", abi = "addAllowedSubnet(address)")]
    pub struct AddAllowedSubnetCall {
        pub subnet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burnRelease` function with signature `burnRelease(bytes32)` and selector `0x2609e586`
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
    #[ethcall(name = "burnRelease", abi = "burnRelease(bytes32)")]
    pub struct BurnReleaseCall {
        pub release_id: [u8; 32],
    }
    ///Container type for all input parameters for the `createRelease` function with signature `createRelease(bytes32,uint8,uint8,uint8,bytes,uint8,uint256,bytes,bytes,bytes,uint256)` and selector `0x19724065`
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
        name = "createRelease",
        abi = "createRelease(bytes32,uint8,uint8,uint8,bytes,uint8,uint256,bytes,bytes,bytes,uint256)"
    )]
    pub struct CreateReleaseCall {
        pub release_id: [u8; 32],
        pub status: u8,
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub platform: u8,
        pub options: ::ethers::core::types::U256,
        pub id_key_digest: ::ethers::core::types::Bytes,
        pub public_key: ::ethers::core::types::Bytes,
        pub cid: ::ethers::core::types::Bytes,
        pub date: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getActiveRelease` function with signature `getActiveRelease(uint8,uint8,bytes,uint8)` and selector `0x2ae79b6d`
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
        name = "getActiveRelease",
        abi = "getActiveRelease(uint8,uint8,bytes,uint8)"
    )]
    pub struct GetActiveReleaseCall {
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub platform: u8,
    }
    ///Container type for all input parameters for the `getActiveReleases` function with signature `getActiveReleases()` and selector `0x023e9288`
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
    #[ethcall(name = "getActiveReleases", abi = "getActiveReleases()")]
    pub struct GetActiveReleasesCall;
    ///Container type for all input parameters for the `getCreatorDomain` function with signature `getCreatorDomain()` and selector `0x9bb4e2f7`
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
    #[ethcall(name = "getCreatorDomain", abi = "getCreatorDomain()")]
    pub struct GetCreatorDomainCall;
    ///Container type for all input parameters for the `getRelease` function with signature `getRelease(bytes32)` and selector `0x7f698e92`
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
    #[ethcall(name = "getRelease", abi = "getRelease(bytes32)")]
    pub struct GetReleaseCall {
        pub release_id: [u8; 32],
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
    ///Container type for all input parameters for the `hasAllowedAdminSigningPublicKey` function with signature `hasAllowedAdminSigningPublicKey(bytes)` and selector `0x0e092b18`
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
        name = "hasAllowedAdminSigningPublicKey",
        abi = "hasAllowedAdminSigningPublicKey(bytes)"
    )]
    pub struct HasAllowedAdminSigningPublicKeyCall {
        pub pub_key: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasAllowedAuthorKeyDigest` function with signature `hasAllowedAuthorKeyDigest(bytes)` and selector `0xd6bc626d`
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
        name = "hasAllowedAuthorKeyDigest",
        abi = "hasAllowedAuthorKeyDigest(bytes)"
    )]
    pub struct HasAllowedAuthorKeyDigestCall {
        pub digest: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasAllowedEnv` function with signature `hasAllowedEnv(uint8)` and selector `0x1bd564dc`
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
    #[ethcall(name = "hasAllowedEnv", abi = "hasAllowedEnv(uint8)")]
    pub struct HasAllowedEnvCall {
        pub env: u8,
    }
    ///Container type for all input parameters for the `hasAllowedSubnet` function with signature `hasAllowedSubnet(address)` and selector `0xa0908300`
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
    #[ethcall(name = "hasAllowedSubnet", abi = "hasAllowedSubnet(address)")]
    pub struct HasAllowedSubnetCall {
        pub subnet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasCreatorInit` function with signature `hasCreatorInit()` and selector `0x3dc6c858`
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
    #[ethcall(name = "hasCreatorInit", abi = "hasCreatorInit()")]
    pub struct HasCreatorInitCall;
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
    ///Container type for all input parameters for the `initCreator` function with signature `initCreator(uint8,address,bytes,bytes)` and selector `0xbc7ca317`
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
    #[ethcall(name = "initCreator", abi = "initCreator(uint8,address,bytes,bytes)")]
    pub struct InitCreatorCall {
        pub env: u8,
        pub subnet_id: ::ethers::core::types::Address,
        pub domain: ::ethers::core::types::Bytes,
        pub author_key_digest: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `removeAllowedAdminSigningPublicKey` function with signature `removeAllowedAdminSigningPublicKey(bytes)` and selector `0xe1c0af08`
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
        name = "removeAllowedAdminSigningPublicKey",
        abi = "removeAllowedAdminSigningPublicKey(bytes)"
    )]
    pub struct RemoveAllowedAdminSigningPublicKeyCall {
        pub pub_key: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `removeAllowedSubnet` function with signature `removeAllowedSubnet(address)` and selector `0xdb2d303b`
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
    #[ethcall(name = "removeAllowedSubnet", abi = "removeAllowedSubnet(address)")]
    pub struct RemoveAllowedSubnetCall {
        pub subnet: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `setReleaseStatus` function with signature `setReleaseStatus(bytes32,uint8)` and selector `0x0e1e59dd`
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
    #[ethcall(name = "setReleaseStatus", abi = "setReleaseStatus(bytes32,uint8)")]
    pub struct SetReleaseStatusCall {
        pub release_id: [u8; 32],
        pub status: u8,
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
    pub enum ReleaseRegisterCalls {
        ActivatorRole(ActivatorRoleCall),
        AdminRole(AdminRoleCall),
        BurnerRole(BurnerRoleCall),
        CreatorRole(CreatorRoleCall),
        DeactivatorRole(DeactivatorRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        ReleaseOptionRo(ReleaseOptionRoCall),
        ReleaseOptionSsh(ReleaseOptionSshCall),
        ReleaseOptionUsers(ReleaseOptionUsersCall),
        AddAllowedAdminSigningPublicKey(AddAllowedAdminSigningPublicKeyCall),
        AddAllowedEnv(AddAllowedEnvCall),
        AddAllowedSubnet(AddAllowedSubnetCall),
        BurnRelease(BurnReleaseCall),
        CreateRelease(CreateReleaseCall),
        GetActiveRelease(GetActiveReleaseCall),
        GetActiveReleases(GetActiveReleasesCall),
        GetCreatorDomain(GetCreatorDomainCall),
        GetRelease(GetReleaseCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasAllowedAdminSigningPublicKey(HasAllowedAdminSigningPublicKeyCall),
        HasAllowedAuthorKeyDigest(HasAllowedAuthorKeyDigestCall),
        HasAllowedEnv(HasAllowedEnvCall),
        HasAllowedSubnet(HasAllowedSubnetCall),
        HasCreatorInit(HasCreatorInitCall),
        HasRole(HasRoleCall),
        InitCreator(InitCreatorCall),
        RemoveAllowedAdminSigningPublicKey(RemoveAllowedAdminSigningPublicKeyCall),
        RemoveAllowedEnv(RemoveAllowedEnvCall),
        RemoveAllowedSubnet(RemoveAllowedSubnetCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetReleaseStatus(SetReleaseStatusCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReleaseRegisterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ActivatorRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActivatorRole(decoded));
            }
            if let Ok(decoded) = <AdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminRole(decoded));
            }
            if let Ok(decoded) = <BurnerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnerRole(decoded));
            }
            if let Ok(decoded) = <CreatorRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreatorRole(decoded));
            }
            if let Ok(decoded) = <DeactivatorRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivatorRole(decoded));
            }
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <ReleaseOptionRoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseOptionRo(decoded));
            }
            if let Ok(decoded) = <ReleaseOptionSshCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseOptionSsh(decoded));
            }
            if let Ok(decoded) = <ReleaseOptionUsersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReleaseOptionUsers(decoded));
            }
            if let Ok(decoded) = <AddAllowedAdminSigningPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAllowedAdminSigningPublicKey(decoded));
            }
            if let Ok(decoded) = <AddAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAllowedEnv(decoded));
            }
            if let Ok(decoded) = <AddAllowedSubnetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAllowedSubnet(decoded));
            }
            if let Ok(decoded) = <BurnReleaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnRelease(decoded));
            }
            if let Ok(decoded) = <CreateReleaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateRelease(decoded));
            }
            if let Ok(decoded) = <GetActiveReleaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetActiveRelease(decoded));
            }
            if let Ok(decoded) = <GetActiveReleasesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetActiveReleases(decoded));
            }
            if let Ok(decoded) = <GetCreatorDomainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCreatorDomain(decoded));
            }
            if let Ok(decoded) = <GetReleaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRelease(decoded));
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
            if let Ok(decoded) = <HasAllowedAdminSigningPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasAllowedAdminSigningPublicKey(decoded));
            }
            if let Ok(decoded) = <HasAllowedAuthorKeyDigestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasAllowedAuthorKeyDigest(decoded));
            }
            if let Ok(decoded) = <HasAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasAllowedEnv(decoded));
            }
            if let Ok(decoded) = <HasAllowedSubnetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasAllowedSubnet(decoded));
            }
            if let Ok(decoded) = <HasCreatorInitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasCreatorInit(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <InitCreatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitCreator(decoded));
            }
            if let Ok(decoded) = <RemoveAllowedAdminSigningPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAllowedAdminSigningPublicKey(decoded));
            }
            if let Ok(decoded) = <RemoveAllowedEnvCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAllowedEnv(decoded));
            }
            if let Ok(decoded) = <RemoveAllowedSubnetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAllowedSubnet(decoded));
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
            if let Ok(decoded) = <SetReleaseStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetReleaseStatus(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReleaseRegisterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActivatorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivatorRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseOptionRo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseOptionSsh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseOptionUsers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedAdminSigningPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowedSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActiveRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetActiveReleases(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCreatorDomain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRelease(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedAdminSigningPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedAuthorKeyDigest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAllowedSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasCreatorInit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitCreator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedAdminSigningPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedEnv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowedSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReleaseStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReleaseRegisterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActivatorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeactivatorRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseOptionRo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseOptionSsh(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseOptionUsers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddAllowedAdminSigningPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAllowedSubnet(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActiveRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActiveReleases(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCreatorDomain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRelease(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasAllowedAdminSigningPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasAllowedAuthorKeyDigest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasAllowedSubnet(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasCreatorInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitCreator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowedAdminSigningPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveAllowedEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowedSubnet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReleaseStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ActivatorRoleCall> for ReleaseRegisterCalls {
        fn from(value: ActivatorRoleCall) -> Self {
            Self::ActivatorRole(value)
        }
    }
    impl ::core::convert::From<AdminRoleCall> for ReleaseRegisterCalls {
        fn from(value: AdminRoleCall) -> Self {
            Self::AdminRole(value)
        }
    }
    impl ::core::convert::From<BurnerRoleCall> for ReleaseRegisterCalls {
        fn from(value: BurnerRoleCall) -> Self {
            Self::BurnerRole(value)
        }
    }
    impl ::core::convert::From<CreatorRoleCall> for ReleaseRegisterCalls {
        fn from(value: CreatorRoleCall) -> Self {
            Self::CreatorRole(value)
        }
    }
    impl ::core::convert::From<DeactivatorRoleCall> for ReleaseRegisterCalls {
        fn from(value: DeactivatorRoleCall) -> Self {
            Self::DeactivatorRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ReleaseRegisterCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<ReleaseOptionRoCall> for ReleaseRegisterCalls {
        fn from(value: ReleaseOptionRoCall) -> Self {
            Self::ReleaseOptionRo(value)
        }
    }
    impl ::core::convert::From<ReleaseOptionSshCall> for ReleaseRegisterCalls {
        fn from(value: ReleaseOptionSshCall) -> Self {
            Self::ReleaseOptionSsh(value)
        }
    }
    impl ::core::convert::From<ReleaseOptionUsersCall> for ReleaseRegisterCalls {
        fn from(value: ReleaseOptionUsersCall) -> Self {
            Self::ReleaseOptionUsers(value)
        }
    }
    impl ::core::convert::From<AddAllowedAdminSigningPublicKeyCall>
    for ReleaseRegisterCalls {
        fn from(value: AddAllowedAdminSigningPublicKeyCall) -> Self {
            Self::AddAllowedAdminSigningPublicKey(value)
        }
    }
    impl ::core::convert::From<AddAllowedEnvCall> for ReleaseRegisterCalls {
        fn from(value: AddAllowedEnvCall) -> Self {
            Self::AddAllowedEnv(value)
        }
    }
    impl ::core::convert::From<AddAllowedSubnetCall> for ReleaseRegisterCalls {
        fn from(value: AddAllowedSubnetCall) -> Self {
            Self::AddAllowedSubnet(value)
        }
    }
    impl ::core::convert::From<BurnReleaseCall> for ReleaseRegisterCalls {
        fn from(value: BurnReleaseCall) -> Self {
            Self::BurnRelease(value)
        }
    }
    impl ::core::convert::From<CreateReleaseCall> for ReleaseRegisterCalls {
        fn from(value: CreateReleaseCall) -> Self {
            Self::CreateRelease(value)
        }
    }
    impl ::core::convert::From<GetActiveReleaseCall> for ReleaseRegisterCalls {
        fn from(value: GetActiveReleaseCall) -> Self {
            Self::GetActiveRelease(value)
        }
    }
    impl ::core::convert::From<GetActiveReleasesCall> for ReleaseRegisterCalls {
        fn from(value: GetActiveReleasesCall) -> Self {
            Self::GetActiveReleases(value)
        }
    }
    impl ::core::convert::From<GetCreatorDomainCall> for ReleaseRegisterCalls {
        fn from(value: GetCreatorDomainCall) -> Self {
            Self::GetCreatorDomain(value)
        }
    }
    impl ::core::convert::From<GetReleaseCall> for ReleaseRegisterCalls {
        fn from(value: GetReleaseCall) -> Self {
            Self::GetRelease(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ReleaseRegisterCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ReleaseRegisterCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasAllowedAdminSigningPublicKeyCall>
    for ReleaseRegisterCalls {
        fn from(value: HasAllowedAdminSigningPublicKeyCall) -> Self {
            Self::HasAllowedAdminSigningPublicKey(value)
        }
    }
    impl ::core::convert::From<HasAllowedAuthorKeyDigestCall> for ReleaseRegisterCalls {
        fn from(value: HasAllowedAuthorKeyDigestCall) -> Self {
            Self::HasAllowedAuthorKeyDigest(value)
        }
    }
    impl ::core::convert::From<HasAllowedEnvCall> for ReleaseRegisterCalls {
        fn from(value: HasAllowedEnvCall) -> Self {
            Self::HasAllowedEnv(value)
        }
    }
    impl ::core::convert::From<HasAllowedSubnetCall> for ReleaseRegisterCalls {
        fn from(value: HasAllowedSubnetCall) -> Self {
            Self::HasAllowedSubnet(value)
        }
    }
    impl ::core::convert::From<HasCreatorInitCall> for ReleaseRegisterCalls {
        fn from(value: HasCreatorInitCall) -> Self {
            Self::HasCreatorInit(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ReleaseRegisterCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitCreatorCall> for ReleaseRegisterCalls {
        fn from(value: InitCreatorCall) -> Self {
            Self::InitCreator(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedAdminSigningPublicKeyCall>
    for ReleaseRegisterCalls {
        fn from(value: RemoveAllowedAdminSigningPublicKeyCall) -> Self {
            Self::RemoveAllowedAdminSigningPublicKey(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedEnvCall> for ReleaseRegisterCalls {
        fn from(value: RemoveAllowedEnvCall) -> Self {
            Self::RemoveAllowedEnv(value)
        }
    }
    impl ::core::convert::From<RemoveAllowedSubnetCall> for ReleaseRegisterCalls {
        fn from(value: RemoveAllowedSubnetCall) -> Self {
            Self::RemoveAllowedSubnet(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ReleaseRegisterCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ReleaseRegisterCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetReleaseStatusCall> for ReleaseRegisterCalls {
        fn from(value: SetReleaseStatusCall) -> Self {
            Self::SetReleaseStatus(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ReleaseRegisterCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `ACTIVATOR_ROLE` function with signature `ACTIVATOR_ROLE()` and selector `0x08740a3b`
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
    pub struct ActivatorRoleReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `BURNER_ROLE` function with signature `BURNER_ROLE()` and selector `0x282c51f3`
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
    pub struct BurnerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CREATOR_ROLE` function with signature `CREATOR_ROLE()` and selector `0x8aeda25a`
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
    pub struct CreatorRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEACTIVATOR_ROLE` function with signature `DEACTIVATOR_ROLE()` and selector `0x458be7dc`
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
    pub struct DeactivatorRoleReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `RELEASE_OPTION_RO` function with signature `RELEASE_OPTION_RO()` and selector `0xad769394`
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
    pub struct ReleaseOptionRoReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RELEASE_OPTION_SSH` function with signature `RELEASE_OPTION_SSH()` and selector `0x3acd1ea3`
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
    pub struct ReleaseOptionSshReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RELEASE_OPTION_USERS` function with signature `RELEASE_OPTION_USERS()` and selector `0x277dceaf`
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
    pub struct ReleaseOptionUsersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getActiveRelease` function with signature `getActiveRelease(uint8,uint8,bytes,uint8)` and selector `0x2ae79b6d`
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
    pub struct GetActiveReleaseReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getActiveReleases` function with signature `getActiveReleases()` and selector `0x023e9288`
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
    pub struct GetActiveReleasesReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getCreatorDomain` function with signature `getCreatorDomain()` and selector `0x9bb4e2f7`
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
    pub struct GetCreatorDomainReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getRelease` function with signature `getRelease(bytes32)` and selector `0x7f698e92`
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
    pub struct GetReleaseReturn(pub Release);
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
    ///Container type for all return fields from the `hasAllowedAdminSigningPublicKey` function with signature `hasAllowedAdminSigningPublicKey(bytes)` and selector `0x0e092b18`
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
    pub struct HasAllowedAdminSigningPublicKeyReturn(pub bool);
    ///Container type for all return fields from the `hasAllowedAuthorKeyDigest` function with signature `hasAllowedAuthorKeyDigest(bytes)` and selector `0xd6bc626d`
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
    pub struct HasAllowedAuthorKeyDigestReturn(pub bool);
    ///Container type for all return fields from the `hasAllowedEnv` function with signature `hasAllowedEnv(uint8)` and selector `0x1bd564dc`
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
    pub struct HasAllowedEnvReturn(pub bool);
    ///Container type for all return fields from the `hasAllowedSubnet` function with signature `hasAllowedSubnet(address)` and selector `0xa0908300`
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
    pub struct HasAllowedSubnetReturn(pub bool);
    ///Container type for all return fields from the `hasCreatorInit` function with signature `hasCreatorInit()` and selector `0x3dc6c858`
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
    pub struct HasCreatorInitReturn(pub bool);
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
    ///`Release(uint8,uint8,uint8,bytes,uint256,uint8,uint256,bytes,bytes,bytes)`
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
    pub struct Release {
        pub status: u8,
        pub env: u8,
        pub typ: u8,
        pub kind: ::ethers::core::types::Bytes,
        pub date: ::ethers::core::types::U256,
        pub platform: u8,
        pub options: ::ethers::core::types::U256,
        pub id_key_digest: ::ethers::core::types::Bytes,
        pub public_key: ::ethers::core::types::Bytes,
        pub cid: ::ethers::core::types::Bytes,
    }
}
