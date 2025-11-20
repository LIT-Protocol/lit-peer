pub use wlit::*;
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
pub mod wlit {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/WLIT.json",
        );
    };
    #[allow(deprecated)]
        fn __abi_functions() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::Function>> {
        
        std::collections::BTreeMap::from(
        [
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("guy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("burnFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBalanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBalanceOf"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getTrustedForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTrustedForwarder",
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setTrustedForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setTrustedForwarder",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("forwarder"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dst"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("src"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dst"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                (
                    ::std::borrow::ToOwned::to_owned("trustedForwarder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trustedForwarder"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
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
            ]
        )
        }
        
        #[allow(deprecated)]
        fn __abi_events() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::Event>> {
        
        std::collections::BTreeMap::from(
        [
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("src"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("guy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dst"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("src"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dst"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("src"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("wad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAllowance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalance",
                            ),
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
                        name: ::std::borrow::ToOwned::to_owned("name_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("symbol_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from(__abi_functions()),
            events: ::core::convert::From::from(__abi_events()),
            errors: ::core::convert::From::from(__abi_errors()),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static WLIT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x03\x80T`\xFF\x19\x16`\x12\x17\x90U4\x80\x15a\0\x1DW`\0\x80\xFD[P`@Qa\x0F\xC68\x03\x80a\x0F\xC6\x839\x81\x01`@\x81\x90Ra\0<\x91a\x01oV[a\0E3a\0fV[`\x01a\0Q\x83\x82a\x02aV[P`\x02a\0^\x82\x82a\x02aV[PPPa\x03\x1FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\0\xDDW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xF6Wa\0\xF6a\0\xB6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01$Wa\x01$a\0\xB6V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x01<W`\0\x80\xFD[`\0[\x82\x81\x10\x15a\x01[W` \x81\x86\x01\x81\x01Q\x83\x83\x01\x82\x01R\x01a\x01?V[P`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x82W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x98W`\0\x80\xFD[a\x01\xA4\x85\x82\x86\x01a\0\xCCV[` \x85\x01Q\x90\x93P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xC2W`\0\x80\xFD[a\x01\xCE\x85\x82\x86\x01a\0\xCCV[\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xECW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\x0CWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\\W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x029WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02YW`\0\x81U`\x01\x01a\x02EV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02zWa\x02za\0\xB6V[a\x02\x8E\x81a\x02\x88\x84Ta\x01\xD8V[\x84a\x02\x12V[` `\x1F\x82\x11`\x01\x81\x14a\x02\xC2W`\0\x83\x15a\x02\xAAWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02YV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xF2W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xD2V[P\x84\x82\x10\x15a\x03\x10W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x0C\x98\x80a\x03.`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xF3W`\x005`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x01\nW\x80c\t^\xA7\xB3\x14a\x015W\x80c\x18\x16\r\xDD\x14a\x01eW\x80c#\xB8r\xDD\x14a\x01\x82W\x80c.\x1A}M\x14a\x01\xA2W\x80c1<\xE5g\x14a\x01\xC2W\x80cB\x96lh\x14a\x01\xEEW\x80cp\xA0\x821\x14a\x02\x0EW\x80cqP\x18\xA6\x14a\x02;W\x80cy\xCCg\x90\x14a\x02PW\x80c}\xA0\xA8w\x14a\x02pW\x80c\x8D\xA5\xCB[\x14a\x02\xADW\x80c\x95\xD8\x9BA\x14a\x02\xC2W\x80c\x9B\x96\xEE\xCE\x14a\x02\xD7W\x80c\xA9\x05\x9C\xBB\x14a\x03\rW\x80c\xCE\x1B\x81_\x14a\x03-W\x80c\xD0\xE3\r\xB0\x14a\x01\x02W\x80c\xDAt\"(\x14a\x03PW\x80c\xDDb\xED>\x14a\x03pW\x80c\xF2\xFD\xE3\x8B\x14a\x03\xA8Wa\x01\x02V[6a\x01\x02Wa\x01\0a\x03\xC8V[\0[a\x01\0a\x03\xC8V[4\x80\x15a\x01\x16W`\0\x80\xFD[Pa\x01\x1Fa\x04VV[`@Qa\x01,\x91\x90a\n\xB4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01AW`\0\x80\xFD[Pa\x01Ua\x01P6`\x04a\x0B\x1EV[a\x04\xE4V[`@Q\x90\x15\x15\x81R` \x01a\x01,V[4\x80\x15a\x01qW`\0\x80\xFD[PG[`@Q\x90\x81R` \x01a\x01,V[4\x80\x15a\x01\x8EW`\0\x80\xFD[Pa\x01Ua\x01\x9D6`\x04a\x0BHV[a\x05wV[4\x80\x15a\x01\xAEW`\0\x80\xFD[Pa\x01\0a\x01\xBD6`\x04a\x0B\x85V[a\x07\x8DV[4\x80\x15a\x01\xCEW`\0\x80\xFD[P`\x03Ta\x01\xDC\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01,V[4\x80\x15a\x01\xFAW`\0\x80\xFD[Pa\x01\0a\x02\t6`\x04a\x0B\x85V[a\x08\xAAV[4\x80\x15a\x02\x1AW`\0\x80\xFD[Pa\x01ta\x02)6`\x04a\x0B\x9EV[`\x04` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02GW`\0\x80\xFD[Pa\x01\0a\x08\xC1V[4\x80\x15a\x02\\W`\0\x80\xFD[Pa\x01\0a\x02k6`\x04a\x0B\x1EV[a\x08\xD5V[4\x80\x15a\x02|W`\0\x80\xFD[P`\x03Ta\x02\x95\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01,V[4\x80\x15a\x02\xB9W`\0\x80\xFD[Pa\x02\x95a\x08\xE6V[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x01\x1Fa\x08\xF5V[4\x80\x15a\x02\xE3W`\0\x80\xFD[Pa\x01ta\x02\xF26`\x04a\x0B\x9EV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[4\x80\x15a\x03\x19W`\0\x80\xFD[Pa\x01Ua\x03(6`\x04a\x0B\x1EV[a\t\x02V[4\x80\x15a\x039W`\0\x80\xFD[P`\x03Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02\x95V[4\x80\x15a\x03\\W`\0\x80\xFD[Pa\x01\0a\x03k6`\x04a\x0B\x9EV[a\t\x1DV[4\x80\x15a\x03|W`\0\x80\xFD[Pa\x01ta\x03\x8B6`\x04a\x0B\xB9V[`\x05` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x03\xB4W`\0\x80\xFD[Pa\x01\0a\x03\xC36`\x04a\x0B\x9EV[a\tMV[4`\x04`\0a\x03\xD5a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x04\x04\x91\x90a\x0C\x02V[\x90\x91UPa\x04\x12\x90Pa\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C4`@Qa\x04L\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2V[`\x01\x80Ta\x04c\x90a\x0C\x15V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x8F\x90a\x0C\x15V[\x80\x15a\x04\xDCW\x80`\x1F\x10a\x04\xB1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xDCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xBFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x81`\x05`\0a\x04\xF3a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x88\x16\x80\x82R\x91\x90\x93R\x91 \x91\x90\x91Ua\x05+a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x05e\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x81 T\x82\x11\x15a\x05\xB0W`@Qc\x1E\x9A\xCF\x17`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xB8a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x06\x1CWP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x05` R`@\x81 `\0\x19\x91a\x05\xF8a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15[\x15a\x06\xDAW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x05` R`@\x81 \x83\x91a\x06Ca\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x10\x15a\x06\x83W`@Qc\x13\xBE%+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x05` R`@\x81 \x83\x91a\x06\xA5a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x06\xD4\x91\x90a\x0COV[\x90\x91UPP[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x84\x92\x90a\x07\x02\x90\x84\x90a\x0COV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x84\x92\x90a\x07/\x90\x84\x90a\x0C\x02V[\x92PP\x81\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x07{\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x93\x92PPPV[\x80`\x04`\0a\x07\x9Aa\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x10\x15a\x07\xDAW`@Qc\x1E\x9A\xCF\x17`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x04`\0a\x07\xE7a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x08\x16\x91\x90a\x0COV[\x90\x91UPa\x08$\x90Pa\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFC\x82\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x08\\W=`\0\x80>=`\0\xFD[Pa\x08ea\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x7F\xCFS,\x15\xF0\xA6\xDB\x0B\xD6\xD0\xE08\xBE\xA7\x1D0\xD8\x08\xC7\xD9\x8C\xB3\xBFrh\xA9[\xF5\x08\x1Be\x82`@Qa\x08\x9F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PV[a\x08\xBDa\x08\xB5a\t\xCBV[`\0\x83a\x05wV[PPV[a\x08\xC9a\n\x05V[a\x08\xD3`\0a\ndV[V[a\x08\xE1\x82`\0\x83a\x05wV[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x02\x80Ta\x04c\x90a\x0C\x15V[`\0a\t\x16a\t\x0Fa\t\xCBV[\x84\x84a\x05wV[\x93\x92PPPV[a\t%a\n\x05V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[a\tUa\n\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\t\xC8\x81a\ndV[PV[`\x03T`\0\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15a\t\xEEWP`\x146\x10\x15[\x15a\n\0WP`\x13\x196\x015``\x1C\x90V[P3\x90V[3a\n\x0Ea\x08\xE6V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xB6V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\n\xE2W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\n\xC5V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x19W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B1W`\0\x80\xFD[a\x0B:\x83a\x0B\x02V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B]W`\0\x80\xFD[a\x0Bf\x84a\x0B\x02V[\x92Pa\x0Bt` \x85\x01a\x0B\x02V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x97W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\xB0W`\0\x80\xFD[a\t\x16\x82a\x0B\x02V[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xCCW`\0\x80\xFD[a\x0B\xD5\x83a\x0B\x02V[\x91Pa\x0B\xE3` \x84\x01a\x0B\x02V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05qWa\x05qa\x0B\xECV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C)W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0CIWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x05qWa\x05qa\x0B\xECV\xFE\xA2dipfsX\"\x12 +\xFE\x15\xB3\xDB\xEC\x16O\x1F\xBE\xAC\x8B\xBB#\xD2\x0ERg\xA2\xA6&5^\x1C\xC3\xCA\xAC\xC4FY\xC1\xD2dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static WLIT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xF3W`\x005`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x01\nW\x80c\t^\xA7\xB3\x14a\x015W\x80c\x18\x16\r\xDD\x14a\x01eW\x80c#\xB8r\xDD\x14a\x01\x82W\x80c.\x1A}M\x14a\x01\xA2W\x80c1<\xE5g\x14a\x01\xC2W\x80cB\x96lh\x14a\x01\xEEW\x80cp\xA0\x821\x14a\x02\x0EW\x80cqP\x18\xA6\x14a\x02;W\x80cy\xCCg\x90\x14a\x02PW\x80c}\xA0\xA8w\x14a\x02pW\x80c\x8D\xA5\xCB[\x14a\x02\xADW\x80c\x95\xD8\x9BA\x14a\x02\xC2W\x80c\x9B\x96\xEE\xCE\x14a\x02\xD7W\x80c\xA9\x05\x9C\xBB\x14a\x03\rW\x80c\xCE\x1B\x81_\x14a\x03-W\x80c\xD0\xE3\r\xB0\x14a\x01\x02W\x80c\xDAt\"(\x14a\x03PW\x80c\xDDb\xED>\x14a\x03pW\x80c\xF2\xFD\xE3\x8B\x14a\x03\xA8Wa\x01\x02V[6a\x01\x02Wa\x01\0a\x03\xC8V[\0[a\x01\0a\x03\xC8V[4\x80\x15a\x01\x16W`\0\x80\xFD[Pa\x01\x1Fa\x04VV[`@Qa\x01,\x91\x90a\n\xB4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01AW`\0\x80\xFD[Pa\x01Ua\x01P6`\x04a\x0B\x1EV[a\x04\xE4V[`@Q\x90\x15\x15\x81R` \x01a\x01,V[4\x80\x15a\x01qW`\0\x80\xFD[PG[`@Q\x90\x81R` \x01a\x01,V[4\x80\x15a\x01\x8EW`\0\x80\xFD[Pa\x01Ua\x01\x9D6`\x04a\x0BHV[a\x05wV[4\x80\x15a\x01\xAEW`\0\x80\xFD[Pa\x01\0a\x01\xBD6`\x04a\x0B\x85V[a\x07\x8DV[4\x80\x15a\x01\xCEW`\0\x80\xFD[P`\x03Ta\x01\xDC\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01,V[4\x80\x15a\x01\xFAW`\0\x80\xFD[Pa\x01\0a\x02\t6`\x04a\x0B\x85V[a\x08\xAAV[4\x80\x15a\x02\x1AW`\0\x80\xFD[Pa\x01ta\x02)6`\x04a\x0B\x9EV[`\x04` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02GW`\0\x80\xFD[Pa\x01\0a\x08\xC1V[4\x80\x15a\x02\\W`\0\x80\xFD[Pa\x01\0a\x02k6`\x04a\x0B\x1EV[a\x08\xD5V[4\x80\x15a\x02|W`\0\x80\xFD[P`\x03Ta\x02\x95\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01,V[4\x80\x15a\x02\xB9W`\0\x80\xFD[Pa\x02\x95a\x08\xE6V[4\x80\x15a\x02\xCEW`\0\x80\xFD[Pa\x01\x1Fa\x08\xF5V[4\x80\x15a\x02\xE3W`\0\x80\xFD[Pa\x01ta\x02\xF26`\x04a\x0B\x9EV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[4\x80\x15a\x03\x19W`\0\x80\xFD[Pa\x01Ua\x03(6`\x04a\x0B\x1EV[a\t\x02V[4\x80\x15a\x039W`\0\x80\xFD[P`\x03Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x02\x95V[4\x80\x15a\x03\\W`\0\x80\xFD[Pa\x01\0a\x03k6`\x04a\x0B\x9EV[a\t\x1DV[4\x80\x15a\x03|W`\0\x80\xFD[Pa\x01ta\x03\x8B6`\x04a\x0B\xB9V[`\x05` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x03\xB4W`\0\x80\xFD[Pa\x01\0a\x03\xC36`\x04a\x0B\x9EV[a\tMV[4`\x04`\0a\x03\xD5a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x04\x04\x91\x90a\x0C\x02V[\x90\x91UPa\x04\x12\x90Pa\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C4`@Qa\x04L\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2V[`\x01\x80Ta\x04c\x90a\x0C\x15V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x8F\x90a\x0C\x15V[\x80\x15a\x04\xDCW\x80`\x1F\x10a\x04\xB1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xDCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xBFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x81`\x05`\0a\x04\xF3a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x91\x88\x16\x80\x82R\x91\x90\x93R\x91 \x91\x90\x91Ua\x05+a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x05e\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x81 T\x82\x11\x15a\x05\xB0W`@Qc\x1E\x9A\xCF\x17`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xB8a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x06\x1CWP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x05` R`@\x81 `\0\x19\x91a\x05\xF8a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14\x15[\x15a\x06\xDAW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x05` R`@\x81 \x83\x91a\x06Ca\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x10\x15a\x06\x83W`@Qc\x13\xBE%+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x05` R`@\x81 \x83\x91a\x06\xA5a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x06\xD4\x91\x90a\x0COV[\x90\x91UPP[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x84\x92\x90a\x07\x02\x90\x84\x90a\x0COV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x84\x92\x90a\x07/\x90\x84\x90a\x0C\x02V[\x92PP\x81\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa\x07{\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x93\x92PPPV[\x80`\x04`\0a\x07\x9Aa\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x10\x15a\x07\xDAW`@Qc\x1E\x9A\xCF\x17`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x04`\0a\x07\xE7a\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x08\x16\x91\x90a\x0COV[\x90\x91UPa\x08$\x90Pa\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFC\x82\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x08\\W=`\0\x80>=`\0\xFD[Pa\x08ea\t\xCBV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x7F\xCFS,\x15\xF0\xA6\xDB\x0B\xD6\xD0\xE08\xBE\xA7\x1D0\xD8\x08\xC7\xD9\x8C\xB3\xBFrh\xA9[\xF5\x08\x1Be\x82`@Qa\x08\x9F\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PV[a\x08\xBDa\x08\xB5a\t\xCBV[`\0\x83a\x05wV[PPV[a\x08\xC9a\n\x05V[a\x08\xD3`\0a\ndV[V[a\x08\xE1\x82`\0\x83a\x05wV[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x02\x80Ta\x04c\x90a\x0C\x15V[`\0a\t\x16a\t\x0Fa\t\xCBV[\x84\x84a\x05wV[\x93\x92PPPV[a\t%a\n\x05V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[a\tUa\n\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\t\xC8\x81a\ndV[PV[`\x03T`\0\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15a\t\xEEWP`\x146\x10\x15[\x15a\n\0WP`\x13\x196\x015``\x1C\x90V[P3\x90V[3a\n\x0Ea\x08\xE6V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\t\xB6V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\n\xE2W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\n\xC5V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x19W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B1W`\0\x80\xFD[a\x0B:\x83a\x0B\x02V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B]W`\0\x80\xFD[a\x0Bf\x84a\x0B\x02V[\x92Pa\x0Bt` \x85\x01a\x0B\x02V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x97W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\xB0W`\0\x80\xFD[a\t\x16\x82a\x0B\x02V[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xCCW`\0\x80\xFD[a\x0B\xD5\x83a\x0B\x02V[\x91Pa\x0B\xE3` \x84\x01a\x0B\x02V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05qWa\x05qa\x0B\xECV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C)W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0CIWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x05qWa\x05qa\x0B\xECV\xFE\xA2dipfsX\"\x12 +\xFE\x15\xB3\xDB\xEC\x16O\x1F\xBE\xAC\x8B\xBB#\xD2\x0ERg\xA2\xA6&5^\x1C\xC3\xCA\xAC\xC4FY\xC1\xD2dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static WLIT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct WLIT<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WLIT<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WLIT<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WLIT<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WLIT<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WLIT)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WLIT<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    WLIT_ABI.clone(),
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
                WLIT_ABI.clone(),
                WLIT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            guy: ::ethers::core::types::Address,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (guy, wad))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnFrom` (0x79cc6790) function
        pub fn burn_from(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 204, 103, 144], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xd0e30db0) function
        pub fn deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalanceOf` (0x9b96eece) function
        pub fn get_balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 150, 238, 206], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTrustedForwarder` (0xce1b815f) function
        pub fn get_trusted_forwarder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([206, 27, 129, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrustedForwarder` (0xda742228) function
        pub fn set_trusted_forwarder(
            &self,
            forwarder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 116, 34, 40], forwarder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            dst: ::ethers::core::types::Address,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, wad))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            src: ::ethers::core::types::Address,
            dst: ::ethers::core::types::Address,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, wad))
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
        ///Calls the contract's `trustedForwarder` (0x7da0a877) function
        pub fn trusted_forwarder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 160, 168, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], wad)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
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
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdrawal` event
        pub fn withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WLITEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for WLIT<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientAllowance` with signature `InsufficientAllowance()` and selector `0x13be252b`
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
    #[etherror(name = "InsufficientAllowance", abi = "InsufficientAllowance()")]
    pub struct InsufficientAllowance;
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
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
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
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
    pub enum WLITErrors {
        InsufficientAllowance(InsufficientAllowance),
        InsufficientBalance(InsufficientBalance),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for WLITErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAllowance(decoded));
            }
            if let Ok(decoded) = <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientBalance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WLITErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for WLITErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for WLITErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for WLITErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InsufficientAllowance> for WLITErrors {
        fn from(value: InsufficientAllowance) -> Self {
            Self::InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for WLITErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub src: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub guy: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub src: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(address,uint256)")]
    pub struct WithdrawalFilter {
        #[ethevent(indexed)]
        pub src: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
    pub enum WLITEvents {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ::ethers::contract::EthLogDecode for WLITEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(WLITEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(WLITEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(WLITEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(WLITEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(WLITEvents::WithdrawalFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WLITEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for WLITEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for WLITEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for WLITEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for WLITEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalFilter> for WLITEvents {
        fn from(value: WithdrawalFilter) -> Self {
            Self::WithdrawalFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub guy: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
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
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `burnFrom` function with signature `burnFrom(address,uint256)` and selector `0x79cc6790`
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
    #[ethcall(name = "burnFrom", abi = "burnFrom(address,uint256)")]
    pub struct BurnFromCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `0xd0e30db0`
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
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    ///Container type for all input parameters for the `getBalanceOf` function with signature `getBalanceOf(address)` and selector `0x9b96eece`
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
    #[ethcall(name = "getBalanceOf", abi = "getBalanceOf(address)")]
    pub struct GetBalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTrustedForwarder` function with signature `getTrustedForwarder()` and selector `0xce1b815f`
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
    #[ethcall(name = "getTrustedForwarder", abi = "getTrustedForwarder()")]
    pub struct GetTrustedForwarderCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    ///Container type for all input parameters for the `setTrustedForwarder` function with signature `setTrustedForwarder(address)` and selector `0xda742228`
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
    #[ethcall(name = "setTrustedForwarder", abi = "setTrustedForwarder(address)")]
    pub struct SetTrustedForwarderCall {
        pub forwarder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub src: ::ethers::core::types::Address,
        pub dst: ::ethers::core::types::Address,
        pub wad: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `trustedForwarder` function with signature `trustedForwarder()` and selector `0x7da0a877`
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
    #[ethcall(name = "trustedForwarder", abi = "trustedForwarder()")]
    pub struct TrustedForwarderCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub wad: ::ethers::core::types::U256,
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
    pub enum WLITCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnFrom(BurnFromCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        GetBalanceOf(GetBalanceOfCall),
        GetTrustedForwarder(GetTrustedForwarderCall),
        Name(NameCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetTrustedForwarder(SetTrustedForwarderCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedForwarder(TrustedForwarderCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for WLITCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <BurnFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnFrom(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <GetBalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBalanceOf(decoded));
            }
            if let Ok(decoded) = <GetTrustedForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTrustedForwarder(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetTrustedForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTrustedForwarder(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TrustedForwarderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrustedForwarder(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WLITCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTrustedForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTrustedForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustedForwarder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WLITCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTrustedForwarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedForwarder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedForwarder(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for WLITCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for WLITCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for WLITCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for WLITCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<BurnFromCall> for WLITCalls {
        fn from(value: BurnFromCall) -> Self {
            Self::BurnFrom(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for WLITCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DepositCall> for WLITCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetBalanceOfCall> for WLITCalls {
        fn from(value: GetBalanceOfCall) -> Self {
            Self::GetBalanceOf(value)
        }
    }
    impl ::core::convert::From<GetTrustedForwarderCall> for WLITCalls {
        fn from(value: GetTrustedForwarderCall) -> Self {
            Self::GetTrustedForwarder(value)
        }
    }
    impl ::core::convert::From<NameCall> for WLITCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for WLITCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for WLITCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetTrustedForwarderCall> for WLITCalls {
        fn from(value: SetTrustedForwarderCall) -> Self {
            Self::SetTrustedForwarder(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for WLITCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for WLITCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for WLITCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for WLITCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for WLITCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrustedForwarderCall> for WLITCalls {
        fn from(value: TrustedForwarderCall) -> Self {
            Self::TrustedForwarder(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for WLITCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `getBalanceOf` function with signature `getBalanceOf(address)` and selector `0x9b96eece`
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
    pub struct GetBalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTrustedForwarder` function with signature `getTrustedForwarder()` and selector `0xce1b815f`
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
    pub struct GetTrustedForwarderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `trustedForwarder` function with signature `trustedForwarder()` and selector `0x7da0a877`
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
    pub struct TrustedForwarderReturn(pub ::ethers::core::types::Address);
}
