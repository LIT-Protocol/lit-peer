pub use forwarder::*;
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
pub mod forwarder {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/Forwarder.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        1usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes1"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("req"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Forwarder.ForwardRequest",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("req"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Forwarder.ForwardRequest",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EIP712DomainChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EIP712DomainChanged",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignatureDoesNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SignatureDoesNotMatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringTooLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StringTooLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("str"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransactionRevertedSilently"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransactionRevertedSilently",
                            ),
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
    pub static FORWARDER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x82R`\x0F\x81Rn#\xA9\xA7;\x19\x10#7\xB9;\xB0\xB922\xB9`\x89\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x05\x83Rd0.0.1`\xD8\x1B\x90\x83\x01R\x90a\0`\x82`\0a\x01\nV[a\x01 Ra\0o\x81`\x01a\x01\nV[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Ra\0\xFC`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0Ra\x03SV[`\0` \x83Q\x10\x15a\x01&Wa\x01\x1F\x83a\x01=V[\x90Pa\x017V[\x81a\x011\x84\x82a\x02#V[P`\xFF\x90P[\x92\x91PPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15a\x01qW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\x01h\x91\x90a\x02\xE1V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\x01|\x82a\x03/V[\x17\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xAEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xCEWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\x1EW\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xFBWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02\x1BW`\0\x81U`\x01\x01a\x02\x07V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02<Wa\x02<a\x01\x84V[a\x02P\x81a\x02J\x84Ta\x01\x9AV[\x84a\x01\xD4V[` `\x1F\x82\x11`\x01\x81\x14a\x02\x84W`\0\x83\x15a\x02lWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02\x1BV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xB4W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\x94V[P\x84\x82\x10\x15a\x02\xD2W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x03\x0FW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x02\xF2V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x01\xCEW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x0E?a\x03\xAD`\09`\0a\x03s\x01R`\0a\x03I\x01R`\0a\x07Y\x01R`\0a\x071\x01R`\0a\x06\x8C\x01R`\0a\x06\xB6\x01R`\0a\x06\xE0\x01Ra\x0E?`\0\xF3\xFE`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c-\x035\xAB\x14a\0DW\x80cG\x15?\x82\x14a\0\x8DW\x80c\x84\xB0\x19n\x14a\0\xAEW\x80c\xBF];\xDB\x14a\0\xD6W[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[Pa\0za\0_6`\x04a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\x9B6`\x04a\nNV[a\x01\x06V[`@Qa\0\x84\x92\x91\x90a\x0B@V[4\x80\x15a\0\xBAW`\0\x80\xFD[Pa\0\xC3a\x03;V[`@Qa\0\x84\x97\x96\x95\x94\x93\x92\x91\x90a\x0BcV[4\x80\x15a\0\xE2W`\0\x80\xFD[Pa\0\xF6a\0\xF16`\x04a\nNV[a\x03\xC3V[`@Q\x90\x15\x15\x81R` \x01a\0\x84V[`\0``a\x01\x15\x85\x85\x85a\x03\xC3V[a\x012W`@Qc+\x84\xC4O`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x85\x015`\x02`\0a\x01H` \x89\x01\x89a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x11\x15a\x01\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNonce is not strictly increasing`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xCB`\x80\x86\x015`\x01a\x0B\xFBV[`\x02`\0a\x01\xDC` \x89\x01\x89a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\0\x80\x86` \x01` \x81\x01\x90a\x02\x16\x91\x90a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16``\x88\x015`@\x89\x015a\x026`\xA0\x8B\x01\x8Ba\x0C\x1CV[a\x02C` \x8D\x01\x8Da\n\x1EV[`@Q` \x01a\x02U\x93\x92\x91\x90a\x0CbV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x02o\x91a\x0C\x83V[`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80`\0\x81\x14a\x02\xADW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xB2V[``\x91P[P\x91P\x91P\x81a\x03\x12W`D\x81Q\x10\x15a\x02\xDFW`@Qc\x04&\xC1\x89`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02\xF9\x91\x90a\x0C\xB5V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xB3\x91\x90a\r^V[a\x03!`?``\x89\x015a\rqV[Z\x11a\x03/Wa\x03/a\r\x93V[\x90\x96\x90\x95P\x93PPPPV[`\0``\x80\x82\x80\x80\x83a\x03n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x05BV[a\x03\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x05BV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0\x80a\x04\xD6\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04\xD0\x92P\x7F\xDD\x8FKp\xB0\xF49>\x88\x9B\xD3\x91(\xA3\x06(\xA7\x8Ba\x81j\x9E\xB8\x19\x97Y\xE7\xA3Ie~H\x91Pa\x043\x90P` \x8A\x01\x8Aa\n\x1EV[a\x04C`@\x8B\x01` \x8C\x01a\n\x1EV[`@\x8B\x015``\x8C\x015`\x80\x8D\x015a\x04_`\xA0\x8F\x01\x8Fa\x0C\x1CV[`@Qa\x04m\x92\x91\x90a\r\xA9V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x98\x90\x98R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x90\x82\x01R\x94\x90\x93\x16``\x85\x01R`\x80\x84\x01\x91\x90\x91R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x05\xEFV[\x90a\x06\x1CV[\x90P`\x80\x85\x015`\x02`\0a\x04\xEE` \x89\x01\x89a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14\x80\x15a\x059WPa\x05$` \x86\x01\x86a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[\x95\x94PPPPPV[```\xFF\x83\x14a\x05\\Wa\x05U\x83a\x06@V[\x90Pa\x05\xE9V[\x81\x80Ta\x05h\x90a\r\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x94\x90a\r\xB9V[\x80\x15a\x05\xE1W\x80`\x1F\x10a\x05\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xE1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0a\x05\xE9a\x05\xFCa\x06\x7FV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x06+\x85\x85a\x07\xAFV[\x91P\x91Pa\x068\x81a\x07\xF4V[P\x93\x92PPPV[```\0a\x06M\x83a\t<V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x06\xD8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x07\x02WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x07\xAA`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x90P\x90V[`\0\x80\x82Q`A\x03a\x07\xE5W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x07\xD9\x87\x82\x85\x85a\tdV[\x94P\x94PPPPa\x07\xEDV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x08\x08Wa\x08\x08a\r\xF3V[\x03a\x08\x10WPV[`\x01\x81`\x04\x81\x11\x15a\x08$Wa\x08$a\r\xF3V[\x03a\x08lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01RwECDSA: invalid signature`@\x1B`D\x82\x01R`d\x01a\x01\xB3V[`\x02\x81`\x04\x81\x11\x15a\x08\x80Wa\x08\x80a\r\xF3V[\x03a\x08\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x01\xB3V[`\x03\x81`\x04\x81\x11\x15a\x08\xE1Wa\x08\xE1a\r\xF3V[\x03a\t9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x01\xB3V[PV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x05\xE9W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a\t\x91WP`\0\x90P`\x03a\n\x15V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\t\xE5W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\x0EW`\0`\x01\x92P\x92PPa\n\x15V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a\n0W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nGW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\ncW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\nyW`\0\x80\xFD[\x84\x01`\xC0\x81\x87\x03\x12\x15a\n\x8BW`\0\x80\xFD[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xA6W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\n\xB7W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCDW`\0\x80\xFD[\x86` \x82\x84\x01\x01\x11\x15a\n\xDFW`\0\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[`\0[\x83\x81\x10\x15a\x0B\x0BW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\xF3V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0B,\x81` \x86\x01` \x86\x01a\n\xF0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x0B[`@\x83\x01\x84a\x0B\x14V[\x94\x93PPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a\x0B\x82`\xE0\x83\x01\x89a\x0B\x14V[\x82\x81\x03`@\x84\x01Ra\x0B\x94\x81\x89a\x0B\x14V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a\x0B\xEAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xCCV[P\x90\x9B\x9APPPPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\xE9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0C3W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0CMW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x07\xEDW`\0\x80\xFD[\x82\x84\x827``\x91\x90\x91\x1B`\x01`\x01``\x1B\x03\x19\x16\x91\x01\x90\x81R`\x14\x01\x91\x90PV[`\0\x82Qa\x0C\x95\x81\x84` \x87\x01a\n\xF0V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\xC7W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xDDW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0C\xEEW`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\x07Wa\r\x07a\x0C\x9FV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\r5Wa\r5a\x0C\x9FV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\rMW`\0\x80\xFD[a\x059\x82` \x83\x01` \x86\x01a\n\xF0V[` \x81R`\0a\nG` \x83\x01\x84a\x0B\x14V[`\0\x82a\r\x8EWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\r\xCDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\xEDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA2\xFDD)>1\x0B\xEE\xF4\x80\xE2\x9Ai\x96\xD9i\xEBN|#.\x94:\x8D{\x1E\x04\xBC\xA8\xD9aUdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static FORWARDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c-\x035\xAB\x14a\0DW\x80cG\x15?\x82\x14a\0\x8DW\x80c\x84\xB0\x19n\x14a\0\xAEW\x80c\xBF];\xDB\x14a\0\xD6W[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[Pa\0za\0_6`\x04a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\x9B6`\x04a\nNV[a\x01\x06V[`@Qa\0\x84\x92\x91\x90a\x0B@V[4\x80\x15a\0\xBAW`\0\x80\xFD[Pa\0\xC3a\x03;V[`@Qa\0\x84\x97\x96\x95\x94\x93\x92\x91\x90a\x0BcV[4\x80\x15a\0\xE2W`\0\x80\xFD[Pa\0\xF6a\0\xF16`\x04a\nNV[a\x03\xC3V[`@Q\x90\x15\x15\x81R` \x01a\0\x84V[`\0``a\x01\x15\x85\x85\x85a\x03\xC3V[a\x012W`@Qc+\x84\xC4O`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x85\x015`\x02`\0a\x01H` \x89\x01\x89a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x11\x15a\x01\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FNonce is not strictly increasing`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xCB`\x80\x86\x015`\x01a\x0B\xFBV[`\x02`\0a\x01\xDC` \x89\x01\x89a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP`\0\x80\x86` \x01` \x81\x01\x90a\x02\x16\x91\x90a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16``\x88\x015`@\x89\x015a\x026`\xA0\x8B\x01\x8Ba\x0C\x1CV[a\x02C` \x8D\x01\x8Da\n\x1EV[`@Q` \x01a\x02U\x93\x92\x91\x90a\x0CbV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x02o\x91a\x0C\x83V[`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP=\x80`\0\x81\x14a\x02\xADW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xB2V[``\x91P[P\x91P\x91P\x81a\x03\x12W`D\x81Q\x10\x15a\x02\xDFW`@Qc\x04&\xC1\x89`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x02\xF9\x91\x90a\x0C\xB5V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xB3\x91\x90a\r^V[a\x03!`?``\x89\x015a\rqV[Z\x11a\x03/Wa\x03/a\r\x93V[\x90\x96\x90\x95P\x93PPPPV[`\0``\x80\x82\x80\x80\x83a\x03n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x05BV[a\x03\x99\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x05BV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0\x80a\x04\xD6\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04\xD0\x92P\x7F\xDD\x8FKp\xB0\xF49>\x88\x9B\xD3\x91(\xA3\x06(\xA7\x8Ba\x81j\x9E\xB8\x19\x97Y\xE7\xA3Ie~H\x91Pa\x043\x90P` \x8A\x01\x8Aa\n\x1EV[a\x04C`@\x8B\x01` \x8C\x01a\n\x1EV[`@\x8B\x015``\x8C\x015`\x80\x8D\x015a\x04_`\xA0\x8F\x01\x8Fa\x0C\x1CV[`@Qa\x04m\x92\x91\x90a\r\xA9V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x98\x90\x98R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x90\x82\x01R\x94\x90\x93\x16``\x85\x01R`\x80\x84\x01\x91\x90\x91R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x05\xEFV[\x90a\x06\x1CV[\x90P`\x80\x85\x015`\x02`\0a\x04\xEE` \x89\x01\x89a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14\x80\x15a\x059WPa\x05$` \x86\x01\x86a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[\x95\x94PPPPPV[```\xFF\x83\x14a\x05\\Wa\x05U\x83a\x06@V[\x90Pa\x05\xE9V[\x81\x80Ta\x05h\x90a\r\xB9V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x94\x90a\r\xB9V[\x80\x15a\x05\xE1W\x80`\x1F\x10a\x05\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xE1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0a\x05\xE9a\x05\xFCa\x06\x7FV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x06+\x85\x85a\x07\xAFV[\x91P\x91Pa\x068\x81a\x07\xF4V[P\x93\x92PPPV[```\0a\x06M\x83a\t<V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\x06\xD8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\x07\x02WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x07\xAA`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x90P\x90V[`\0\x80\x82Q`A\x03a\x07\xE5W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x07\xD9\x87\x82\x85\x85a\tdV[\x94P\x94PPPPa\x07\xEDV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x08\x08Wa\x08\x08a\r\xF3V[\x03a\x08\x10WPV[`\x01\x81`\x04\x81\x11\x15a\x08$Wa\x08$a\r\xF3V[\x03a\x08lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01RwECDSA: invalid signature`@\x1B`D\x82\x01R`d\x01a\x01\xB3V[`\x02\x81`\x04\x81\x11\x15a\x08\x80Wa\x08\x80a\r\xF3V[\x03a\x08\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x01\xB3V[`\x03\x81`\x04\x81\x11\x15a\x08\xE1Wa\x08\xE1a\r\xF3V[\x03a\t9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x01\xB3V[PV[`\0`\xFF\x82\x16`\x1F\x81\x11\x15a\x05\xE9W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a\t\x91WP`\0\x90P`\x03a\n\x15V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\t\xE5W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\x0EW`\0`\x01\x92P\x92PPa\n\x15V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0` \x82\x84\x03\x12\x15a\n0W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nGW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\ncW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\nyW`\0\x80\xFD[\x84\x01`\xC0\x81\x87\x03\x12\x15a\n\x8BW`\0\x80\xFD[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xA6W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\n\xB7W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCDW`\0\x80\xFD[\x86` \x82\x84\x01\x01\x11\x15a\n\xDFW`\0\x80\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[`\0[\x83\x81\x10\x15a\x0B\x0BW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\xF3V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0B,\x81` \x86\x01` \x86\x01a\n\xF0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\x0B[`@\x83\x01\x84a\x0B\x14V[\x94\x93PPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a\x0B\x82`\xE0\x83\x01\x89a\x0B\x14V[\x82\x81\x03`@\x84\x01Ra\x0B\x94\x81\x89a\x0B\x14V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a\x0B\xEAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xCCV[P\x90\x9B\x9APPPPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\xE9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0C3W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0CMW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x07\xEDW`\0\x80\xFD[\x82\x84\x827``\x91\x90\x91\x1B`\x01`\x01``\x1B\x03\x19\x16\x91\x01\x90\x81R`\x14\x01\x91\x90PV[`\0\x82Qa\x0C\x95\x81\x84` \x87\x01a\n\xF0V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\xC7W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xDDW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0C\xEEW`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\x07Wa\r\x07a\x0C\x9FV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\r5Wa\r5a\x0C\x9FV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\rMW`\0\x80\xFD[a\x059\x82` \x83\x01` \x86\x01a\n\xF0V[` \x81R`\0a\nG` \x83\x01\x84a\x0B\x14V[`\0\x82a\r\x8EWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\r\xCDW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\r\xEDWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA2\xFDD)>1\x0B\xEE\xF4\x80\xE2\x9Ai\x96\xD9i\xEBN|#.\x94:\x8D{\x1E\x04\xBC\xA8\xD9aUdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static FORWARDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Forwarder<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Forwarder<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Forwarder<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Forwarder<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Forwarder<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Forwarder)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Forwarder<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FORWARDER_ABI.clone(),
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
                FORWARDER_ABI.clone(),
                FORWARDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x47153f82) function
        pub fn execute(
            &self,
            req: ForwardRequest,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([71, 21, 63, 130], (req, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            from: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 3, 53, 171], from)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0xbf5d3bdb) function
        pub fn verify(
            &self,
            req: ForwardRequest,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 93, 59, 219], (req, signature))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Forwarder<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
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
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
    ///Custom Error type `SignatureDoesNotMatch` with signature `SignatureDoesNotMatch()` and selector `0x5709889e`
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
    #[etherror(name = "SignatureDoesNotMatch", abi = "SignatureDoesNotMatch()")]
    pub struct SignatureDoesNotMatch;
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
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
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
    ///Custom Error type `TransactionRevertedSilently` with signature `TransactionRevertedSilently()` and selector `0x0426c189`
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
    #[etherror(
        name = "TransactionRevertedSilently",
        abi = "TransactionRevertedSilently()"
    )]
    pub struct TransactionRevertedSilently;
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
    pub enum ForwarderErrors {
        InvalidShortString(InvalidShortString),
        SignatureDoesNotMatch(SignatureDoesNotMatch),
        StringTooLong(StringTooLong),
        TransactionRevertedSilently(TransactionRevertedSilently),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ForwarderErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded) = <SignatureDoesNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignatureDoesNotMatch(decoded));
            }
            if let Ok(decoded) = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StringTooLong(decoded));
            }
            if let Ok(decoded) = <TransactionRevertedSilently as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransactionRevertedSilently(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ForwarderErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidShortString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignatureDoesNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringTooLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransactionRevertedSilently(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ForwarderErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidShortString as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SignatureDoesNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringTooLong as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransactionRevertedSilently as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ForwarderErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidShortString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignatureDoesNotMatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactionRevertedSilently(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ForwarderErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for ForwarderErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<SignatureDoesNotMatch> for ForwarderErrors {
        fn from(value: SignatureDoesNotMatch) -> Self {
            Self::SignatureDoesNotMatch(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for ForwarderErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
        }
    }
    impl ::core::convert::From<TransactionRevertedSilently> for ForwarderErrors {
        fn from(value: TransactionRevertedSilently) -> Self {
            Self::TransactionRevertedSilently(value)
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
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all input parameters for the `execute` function with signature `execute((address,address,uint256,uint256,uint256,bytes),bytes)` and selector `0x47153f82`
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
        name = "execute",
        abi = "execute((address,address,uint256,uint256,uint256,bytes),bytes)"
    )]
    pub struct ExecuteCall {
        pub req: ForwardRequest,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub from: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verify` function with signature `verify((address,address,uint256,uint256,uint256,bytes),bytes)` and selector `0xbf5d3bdb`
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
        name = "verify",
        abi = "verify((address,address,uint256,uint256,uint256,bytes),bytes)"
    )]
    pub struct VerifyCall {
        pub req: ForwardRequest,
        pub signature: ::ethers::core::types::Bytes,
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
    pub enum ForwarderCalls {
        Eip712Domain(Eip712DomainCall),
        Execute(ExecuteCall),
        GetNonce(GetNonceCall),
        Verify(VerifyCall),
    }
    impl ::ethers::core::abi::AbiDecode for ForwarderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ForwarderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Eip712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ForwarderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for ForwarderCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for ForwarderCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for ForwarderCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for ForwarderCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `execute` function with signature `execute((address,address,uint256,uint256,uint256,bytes),bytes)` and selector `0x47153f82`
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
    pub struct ExecuteReturn(pub bool, pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    pub struct GetNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `verify` function with signature `verify((address,address,uint256,uint256,uint256,bytes),bytes)` and selector `0xbf5d3bdb`
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
    pub struct VerifyReturn(pub bool);
    ///`ForwardRequest(address,address,uint256,uint256,uint256,bytes)`
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
    pub struct ForwardRequest {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub gas: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
}
