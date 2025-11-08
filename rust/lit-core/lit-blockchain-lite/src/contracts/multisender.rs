pub use multisender::*;
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
pub mod multisender {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/Multisender.json",
        );
    };
    #[allow(deprecated)]
        fn __abi_functions() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::Function>> {
        
        std::collections::BTreeMap::from(
        [
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
                    ::std::borrow::ToOwned::to_owned("sendEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendEth"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipients"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipients"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenContract"),
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
                    ::std::borrow::ToOwned::to_owned("sendTokensExact"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendTokensExact"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipients"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountPerRecipient",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenContract"),
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
            ]
        )
        }
        
        #[allow(deprecated)]
        fn __abi_events() -> std::collections::BTreeMap<String, Vec<::ethers::core::abi::ethabi::Event>> {
        
        std::collections::BTreeMap::from(
        [
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
            ]
        )
        }
        
        #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        
        
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from(__abi_functions()),
            events: ::core::convert::From::from(__abi_events()),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MULTISENDER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x173`\x1BV[`kV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\t\x80\x80a\0z`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0kW`\x005`\xE0\x1C\x80c;/\xE7\x81\x14a\0pW\x80c<\xCF\xD6\x0B\x14a\0\x85W\x80cI\xDFr\x8C\x14a\0\x9AW\x80cn\xCF\x13\x86\x14a\0\xBAW\x80cqP\x18\xA6\x14a\0\xDAW\x80c\x8D\xA5\xCB[\x14a\0\xEFW\x80c\xBE\xBF\xEF\x03\x14a\x01\x1AW\x80c\xF2\xFD\xE3\x8B\x14a\x01:W[`\0\x80\xFD[a\0\x83a\0~6`\x04a\x07RV[a\x01ZV[\0[4\x80\x15a\0\x91W`\0\x80\xFD[Pa\0\x83a\x01\xE1V[4\x80\x15a\0\xA6W`\0\x80\xFD[Pa\0\x83a\0\xB56`\x04a\x07\xAFV[a\x02\x18V[4\x80\x15a\0\xC6W`\0\x80\xFD[Pa\0\x83a\0\xD56`\x04a\x07\xD1V[a\x03\x06V[4\x80\x15a\0\xE6W`\0\x80\xFD[Pa\0\x83a\x04GV[4\x80\x15a\0\xFBW`\0\x80\xFD[Pa\x01\x04a\x04[V[`@Qa\x01\x11\x91\x90a\x08$V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01&W`\0\x80\xFD[Pa\0\x83a\x0156`\x04a\x088V[a\x04jV[4\x80\x15a\x01FW`\0\x80\xFD[Pa\0\x83a\x01U6`\x04a\x07\xAFV[a\x05\xE2V[`\0a\x01f\x824a\x08\x93V[\x90P`\0[\x82\x81\x10\x15a\x01\xDBW\x83\x83\x82\x81\x81\x10a\x01\x85Wa\x01\x85a\x08\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x01\x9A\x91\x90a\x07\xAFV[`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFC\x83\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xD2W=`\0\x80>=`\0\xFD[P`\x01\x01a\x01kV[PPPPV[a\x01\xE9a\x06XV[`@Q3\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02\x15W=`\0\x80>=`\0\xFD[PV[a\x02 a\x06XV[`@Qcp\xA0\x821`\xE0\x1B\x81R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90a\x02Q\x900\x90`\x04\x01a\x08$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x92\x91\x90a\x08\xCBV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90a\x02\xC3\x903\x90\x85\x90`\x04\x01a\x08\xE4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDB\x91\x90a\x08\xFDV[`@Qcp\xA0\x821`\xE0\x1B\x81R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90a\x037\x900\x90`\x04\x01a\x08$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03x\x91\x90a\x08\xCBV[\x90P`\0a\x03\x87\x85`\na\t\x1FV[a\x03\x91\x90\x83a\x08\x93V[\x90P`\0[\x85\x81\x10\x15a\x04>W\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x88\x88\x84\x81\x81\x10a\x03\xBFWa\x03\xBFa\x08\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x03\xD4\x91\x90a\x07\xAFV[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xF2\x92\x91\x90a\x08\xE4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x045\x91\x90a\x08\xFDV[P`\x01\x01a\x03\x96V[PPPPPPPV[a\x04Oa\x06XV[a\x04Y`\0a\x06\xB7V[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x81`\0a\x04w\x85\x84a\t\x1FV[\x90P\x80\x82`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA6\x91\x90a\x08$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE7\x91\x90a\x08\xCBV[\x10\x15a\x057W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01RyInsufficient token balance`0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x85\x81\x10\x15a\x04>W\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x88\x88\x84\x81\x81\x10a\x05cWa\x05ca\x08\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x05x\x91\x90a\x07\xAFV[\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x96\x92\x91\x90a\x08\xE4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD9\x91\x90a\x08\xFDV[P`\x01\x01a\x05:V[a\x05\xEAa\x06XV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05.V[a\x02\x15\x81a\x06\xB7V[3a\x06aa\x04[V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05.V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x19W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x070W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x07KW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07eW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07{W`\0\x80\xFD[a\x07\x87\x85\x82\x86\x01a\x07\x07V[\x90\x96\x90\x95P\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xAAW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07\xC1W`\0\x80\xFD[a\x07\xCA\x82a\x07\x93V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07\xE6W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xFCW`\0\x80\xFD[a\x08\x08\x86\x82\x87\x01a\x07\x07V[\x90\x94P\x92Pa\x08\x1B\x90P` \x85\x01a\x07\x93V[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x08NW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08dW`\0\x80\xFD[a\x08p\x87\x82\x88\x01a\x07\x07V[\x90\x95P\x93Pa\x08\x83\x90P` \x86\x01a\x07\x93V[\x93\x96\x92\x95P\x92\x93`@\x015\x92PPV[`\0\x82a\x08\xB0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xDDW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t\x0FW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xCAW`\0\x80\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\tDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x91\x0E\x81\xC5\xE1\\\x13\x8D{u\x987\xE0\xAF\xBF=q\xADO08(\xD7\xABdc\xD4\x01\xCD\xE1\x85\xDAdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static MULTISENDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0kW`\x005`\xE0\x1C\x80c;/\xE7\x81\x14a\0pW\x80c<\xCF\xD6\x0B\x14a\0\x85W\x80cI\xDFr\x8C\x14a\0\x9AW\x80cn\xCF\x13\x86\x14a\0\xBAW\x80cqP\x18\xA6\x14a\0\xDAW\x80c\x8D\xA5\xCB[\x14a\0\xEFW\x80c\xBE\xBF\xEF\x03\x14a\x01\x1AW\x80c\xF2\xFD\xE3\x8B\x14a\x01:W[`\0\x80\xFD[a\0\x83a\0~6`\x04a\x07RV[a\x01ZV[\0[4\x80\x15a\0\x91W`\0\x80\xFD[Pa\0\x83a\x01\xE1V[4\x80\x15a\0\xA6W`\0\x80\xFD[Pa\0\x83a\0\xB56`\x04a\x07\xAFV[a\x02\x18V[4\x80\x15a\0\xC6W`\0\x80\xFD[Pa\0\x83a\0\xD56`\x04a\x07\xD1V[a\x03\x06V[4\x80\x15a\0\xE6W`\0\x80\xFD[Pa\0\x83a\x04GV[4\x80\x15a\0\xFBW`\0\x80\xFD[Pa\x01\x04a\x04[V[`@Qa\x01\x11\x91\x90a\x08$V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01&W`\0\x80\xFD[Pa\0\x83a\x0156`\x04a\x088V[a\x04jV[4\x80\x15a\x01FW`\0\x80\xFD[Pa\0\x83a\x01U6`\x04a\x07\xAFV[a\x05\xE2V[`\0a\x01f\x824a\x08\x93V[\x90P`\0[\x82\x81\x10\x15a\x01\xDBW\x83\x83\x82\x81\x81\x10a\x01\x85Wa\x01\x85a\x08\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x01\x9A\x91\x90a\x07\xAFV[`\x01`\x01`\xA0\x1B\x03\x16a\x08\xFC\x83\x90\x81\x15\x02\x90`@Q`\0`@Q\x80\x83\x03\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01\xD2W=`\0\x80>=`\0\xFD[P`\x01\x01a\x01kV[PPPPV[a\x01\xE9a\x06XV[`@Q3\x90G\x80\x15a\x08\xFC\x02\x91`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02\x15W=`\0\x80>=`\0\xFD[PV[a\x02 a\x06XV[`@Qcp\xA0\x821`\xE0\x1B\x81R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90a\x02Q\x900\x90`\x04\x01a\x08$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x92\x91\x90a\x08\xCBV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90a\x02\xC3\x903\x90\x85\x90`\x04\x01a\x08\xE4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDB\x91\x90a\x08\xFDV[`@Qcp\xA0\x821`\xE0\x1B\x81R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90a\x037\x900\x90`\x04\x01a\x08$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03x\x91\x90a\x08\xCBV[\x90P`\0a\x03\x87\x85`\na\t\x1FV[a\x03\x91\x90\x83a\x08\x93V[\x90P`\0[\x85\x81\x10\x15a\x04>W\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x88\x88\x84\x81\x81\x10a\x03\xBFWa\x03\xBFa\x08\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x03\xD4\x91\x90a\x07\xAFV[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xF2\x92\x91\x90a\x08\xE4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x045\x91\x90a\x08\xFDV[P`\x01\x01a\x03\x96V[PPPPPPPV[a\x04Oa\x06XV[a\x04Y`\0a\x06\xB7V[V[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x81`\0a\x04w\x85\x84a\t\x1FV[\x90P\x80\x82`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x8210`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA6\x91\x90a\x08$V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE7\x91\x90a\x08\xCBV[\x10\x15a\x057W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01RyInsufficient token balance`0\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x85\x81\x10\x15a\x04>W\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x88\x88\x84\x81\x81\x10a\x05cWa\x05ca\x08\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x05x\x91\x90a\x07\xAFV[\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x96\x92\x91\x90a\x08\xE4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD9\x91\x90a\x08\xFDV[P`\x01\x01a\x05:V[a\x05\xEAa\x06XV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05.V[a\x02\x15\x81a\x06\xB7V[3a\x06aa\x04[V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05.V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x07\x19W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x070W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x07KW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07eW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07{W`\0\x80\xFD[a\x07\x87\x85\x82\x86\x01a\x07\x07V[\x90\x96\x90\x95P\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xAAW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07\xC1W`\0\x80\xFD[a\x07\xCA\x82a\x07\x93V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07\xE6W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xFCW`\0\x80\xFD[a\x08\x08\x86\x82\x87\x01a\x07\x07V[\x90\x94P\x92Pa\x08\x1B\x90P` \x85\x01a\x07\x93V[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x08NW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08dW`\0\x80\xFD[a\x08p\x87\x82\x88\x01a\x07\x07V[\x90\x95P\x93Pa\x08\x83\x90P` \x86\x01a\x07\x93V[\x93\x96\x92\x95P\x92\x93`@\x015\x92PPV[`\0\x82a\x08\xB0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xDDW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t\x0FW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xCAW`\0\x80\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\tDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x91\x0E\x81\xC5\xE1\\\x13\x8D{u\x987\xE0\xAF\xBF=q\xADO08(\xD7\xABdc\xD4\x01\xCD\xE1\x85\xDAdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static MULTISENDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Multisender<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Multisender<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Multisender<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Multisender<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Multisender<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Multisender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Multisender<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTISENDER_ABI.clone(),
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
                MULTISENDER_ABI.clone(),
                MULTISENDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `sendEth` (0x3b2fe781) function
        pub fn send_eth(
            &self,
            recipients: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 47, 231, 129], recipients)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendTokens` (0x6ecf1386) function
        pub fn send_tokens(
            &self,
            recipients: ::std::vec::Vec<::ethers::core::types::Address>,
            token_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 207, 19, 134], (recipients, token_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendTokensExact` (0xbebfef03) function
        pub fn send_tokens_exact(
            &self,
            recipients: ::std::vec::Vec<::ethers::core::types::Address>,
            token_contract: ::ethers::core::types::Address,
            amount_per_recipient: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [190, 191, 239, 3],
                    (recipients, token_contract, amount_per_recipient),
                )
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
        ///Calls the contract's `withdraw` (0x3ccfd60b) function
        pub fn withdraw(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTokens` (0x49df728c) function
        pub fn withdraw_tokens(
            &self,
            token_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 223, 114, 140], token_contract)
                .expect("method not found (this should never happen)")
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Multisender<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `sendEth` function with signature `sendEth(address[])` and selector `0x3b2fe781`
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
    #[ethcall(name = "sendEth", abi = "sendEth(address[])")]
    pub struct SendEthCall {
        pub recipients: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `sendTokens` function with signature `sendTokens(address[],address)` and selector `0x6ecf1386`
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
    #[ethcall(name = "sendTokens", abi = "sendTokens(address[],address)")]
    pub struct SendTokensCall {
        pub recipients: ::std::vec::Vec<::ethers::core::types::Address>,
        pub token_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sendTokensExact` function with signature `sendTokensExact(address[],address,uint256)` and selector `0xbebfef03`
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
        name = "sendTokensExact",
        abi = "sendTokensExact(address[],address,uint256)"
    )]
    pub struct SendTokensExactCall {
        pub recipients: ::std::vec::Vec<::ethers::core::types::Address>,
        pub token_contract: ::ethers::core::types::Address,
        pub amount_per_recipient: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw()` and selector `0x3ccfd60b`
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
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    ///Container type for all input parameters for the `withdrawTokens` function with signature `withdrawTokens(address)` and selector `0x49df728c`
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
    #[ethcall(name = "withdrawTokens", abi = "withdrawTokens(address)")]
    pub struct WithdrawTokensCall {
        pub token_contract: ::ethers::core::types::Address,
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
    pub enum MultisenderCalls {
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SendEth(SendEthCall),
        SendTokens(SendTokensCall),
        SendTokensExact(SendTokensExactCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
        WithdrawTokens(WithdrawTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultisenderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <SendEthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendEth(decoded));
            }
            if let Ok(decoded) = <SendTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendTokens(decoded));
            }
            if let Ok(decoded) = <SendTokensExactCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendTokensExact(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultisenderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendTokensExact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MultisenderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendTokensExact(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnerCall> for MultisenderCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MultisenderCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SendEthCall> for MultisenderCalls {
        fn from(value: SendEthCall) -> Self {
            Self::SendEth(value)
        }
    }
    impl ::core::convert::From<SendTokensCall> for MultisenderCalls {
        fn from(value: SendTokensCall) -> Self {
            Self::SendTokens(value)
        }
    }
    impl ::core::convert::From<SendTokensExactCall> for MultisenderCalls {
        fn from(value: SendTokensExactCall) -> Self {
            Self::SendTokensExact(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MultisenderCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MultisenderCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawTokensCall> for MultisenderCalls {
        fn from(value: WithdrawTokensCall) -> Self {
            Self::WithdrawTokens(value)
        }
    }
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
}
