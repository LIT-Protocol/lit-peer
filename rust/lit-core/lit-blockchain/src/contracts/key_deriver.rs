pub use key_deriver::*;
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
pub mod key_deriver {
    const _: () = {
        ::core::include_bytes!(
            "../../abis/KeyDeriver.json",
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
                    ::std::borrow::ToOwned::to_owned("HD_KDF"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HD_KDF"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static KEYDERIVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x05\xEE\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cb\xE4\xC4d\x14a\0;W\x80c\xA3,+\x99\x14a\0`W[`\0\x80\xFD[a\0C`\xF5\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0sa\0n6`\x04a\x02\x9EV[a\0\x81V[`@Qa\0W\x92\x91\x90a\x04CV[`\0```\0a\0\x92\x86\x86\x86a\x01\0V[\x90P`\0\x80`\xF5`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\0\xB0\x91\x90a\x04\x7FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\0\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xF0V[``\x91P[P\x90\x99\x90\x98P\x96PPPPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R``\x91\x80[\x85Q\x81\x10\x15a\x01\x9AW\x84\x86\x82\x81Q\x81\x10a\x011Wa\x011a\x04\x9BV[` \x02` \x01\x01Q` \x01Q\x03a\x01\x92W\x82\x86\x82\x81Q\x81\x10a\x01UWa\x01Ua\x04\x9BV[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\x01r\x92\x91\x90a\x04\xB1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\x01\x8E\x90a\x04\xE0V[\x92PP[`\x01\x01a\x01\x15V[P\x83`\x02\x03a\x01\xACW`\x01\x93Pa\x01\xB9V[\x83`\x03\x03a\x01\xB9W`\0\x93P[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\x05\x8E`+\x919\x80Q`@Q\x91\x92P`\xF8\x87\x90\x1B\x91`\x01`\xE5\x1B\x91`\xE0\x90\x81\x1B\x91\x90\x86\x90\x1B\x90`\0\x90a\x02\x10\x90\x86\x90\x86\x90\x8F\x90\x87\x90\x8B\x90\x88\x90\x8F\x90` \x01a\x05\x13V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x9C\x9BPPPPPPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02hWa\x02ha\x020V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\x96Wa\x02\x96a\x020V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02\xB3W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xD0W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x02\xE1W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xFAWa\x02\xFAa\x020V[\x80`\x05\x1Ba\x03\n` \x82\x01a\x02nV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x89\x84\x11\x15a\x03&W`\0\x80\xFD[` \x85\x01\x92P[\x83\x83\x10\x15a\x04\nW\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03KW`\0\x80\xFD[\x85\x01`@\x81\x8C\x03`\x1F\x19\x01\x12\x15a\x03aW`\0\x80\xFD[a\x03ia\x02FV[` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\x82W`\0\x80\xFD[\x82\x01` \x81\x01\x90`?\x01\x8D\x13a\x03\x97W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xB0Wa\x03\xB0a\x020V[a\x03\xC3`\x1F\x82\x01`\x1F\x19\x16` \x01a\x02nV[\x81\x81R\x8E` \x83\x85\x01\x01\x11\x15a\x03\xD8W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x92\x82\x01\x83\x01R\x83R`@\x93\x90\x93\x015\x82\x84\x01RP\x83R\x92\x83\x01\x92\x91\x90\x91\x01\x90a\x03-V[\x96\x99\x96\x98PPPP`@\x94\x90\x94\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x04:W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\"V[PP`\0\x91\x01RV[\x82\x15\x15\x81R`@` \x82\x01R`\0\x82Q\x80`@\x84\x01Ra\x04j\x81``\x85\x01` \x87\x01a\x04\x1FV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01``\x01\x93\x92PPPV[`\0\x82Qa\x04\x91\x81\x84` \x87\x01a\x04\x1FV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x83Qa\x04\xC3\x81\x84` \x88\x01a\x04\x1FV[\x83Q\x90\x83\x01\x90a\x04\xD7\x81\x83` \x88\x01a\x04\x1FV[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x05\nWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x01\x92\x91PPV[`\x01`\x01`\xF8\x1B\x03\x19\x88\x16\x81R`\x01`\x01`\xE0\x1B\x03\x19\x87\x81\x16`\x01\x83\x01R`\x05\x82\x01\x87\x90R\x85\x16`%\x82\x01R\x83Q`\0\x90a\x05U\x81`)\x85\x01` \x89\x01a\x04\x1FV[`\x01`\x01`\xE0\x1B\x03\x19\x85\x16`)\x91\x84\x01\x91\x82\x01R\x83Qa\x05|\x81`-\x84\x01` \x88\x01a\x04\x1FV[\x01`-\x01\x99\x98PPPPPPPPPV\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 ].\x87\x92D\x05\xC7Z7\xCB1\x99\x10\x9D\xC5\x84wu\xCE(\x13|^\x1C\xA4\x05r\x0C\xFA;\xEA=dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static KEYDERIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cb\xE4\xC4d\x14a\0;W\x80c\xA3,+\x99\x14a\0`W[`\0\x80\xFD[a\0C`\xF5\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0sa\0n6`\x04a\x02\x9EV[a\0\x81V[`@Qa\0W\x92\x91\x90a\x04CV[`\0```\0a\0\x92\x86\x86\x86a\x01\0V[\x90P`\0\x80`\xF5`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\0\xB0\x91\x90a\x04\x7FV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\0\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xF0V[``\x91P[P\x90\x99\x90\x98P\x96PPPPPPPV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R``\x91\x80[\x85Q\x81\x10\x15a\x01\x9AW\x84\x86\x82\x81Q\x81\x10a\x011Wa\x011a\x04\x9BV[` \x02` \x01\x01Q` \x01Q\x03a\x01\x92W\x82\x86\x82\x81Q\x81\x10a\x01UWa\x01Ua\x04\x9BV[` \x02` \x01\x01Q`\0\x01Q`@Q` \x01a\x01r\x92\x91\x90a\x04\xB1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P\x81\x80a\x01\x8E\x90a\x04\xE0V[\x92PP[`\x01\x01a\x01\x15V[P\x83`\x02\x03a\x01\xACW`\x01\x93Pa\x01\xB9V[\x83`\x03\x03a\x01\xB9W`\0\x93P[`\0`@Q\x80``\x01`@R\x80`+\x81R` \x01a\x05\x8E`+\x919\x80Q`@Q\x91\x92P`\xF8\x87\x90\x1B\x91`\x01`\xE5\x1B\x91`\xE0\x90\x81\x1B\x91\x90\x86\x90\x1B\x90`\0\x90a\x02\x10\x90\x86\x90\x86\x90\x8F\x90\x87\x90\x8B\x90\x88\x90\x8F\x90` \x01a\x05\x13V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x9C\x9BPPPPPPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02hWa\x02ha\x020V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\x96Wa\x02\x96a\x020V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02\xB3W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xD0W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x02\xE1W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\xFAWa\x02\xFAa\x020V[\x80`\x05\x1Ba\x03\n` \x82\x01a\x02nV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x89\x84\x11\x15a\x03&W`\0\x80\xFD[` \x85\x01\x92P[\x83\x83\x10\x15a\x04\nW\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03KW`\0\x80\xFD[\x85\x01`@\x81\x8C\x03`\x1F\x19\x01\x12\x15a\x03aW`\0\x80\xFD[a\x03ia\x02FV[` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\x82W`\0\x80\xFD[\x82\x01` \x81\x01\x90`?\x01\x8D\x13a\x03\x97W`\0\x80\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xB0Wa\x03\xB0a\x020V[a\x03\xC3`\x1F\x82\x01`\x1F\x19\x16` \x01a\x02nV[\x81\x81R\x8E` \x83\x85\x01\x01\x11\x15a\x03\xD8W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x92\x82\x01\x83\x01R\x83R`@\x93\x90\x93\x015\x82\x84\x01RP\x83R\x92\x83\x01\x92\x91\x90\x91\x01\x90a\x03-V[\x96\x99\x96\x98PPPP`@\x94\x90\x94\x015\x93PPPV[`\0[\x83\x81\x10\x15a\x04:W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\"V[PP`\0\x91\x01RV[\x82\x15\x15\x81R`@` \x82\x01R`\0\x82Q\x80`@\x84\x01Ra\x04j\x81``\x85\x01` \x87\x01a\x04\x1FV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01``\x01\x93\x92PPPV[`\0\x82Qa\x04\x91\x81\x84` \x87\x01a\x04\x1FV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x83Qa\x04\xC3\x81\x84` \x88\x01a\x04\x1FV[\x83Q\x90\x83\x01\x90a\x04\xD7\x81\x83` \x88\x01a\x04\x1FV[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x05\nWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x01\x92\x91PPV[`\x01`\x01`\xF8\x1B\x03\x19\x88\x16\x81R`\x01`\x01`\xE0\x1B\x03\x19\x87\x81\x16`\x01\x83\x01R`\x05\x82\x01\x87\x90R\x85\x16`%\x82\x01R\x83Q`\0\x90a\x05U\x81`)\x85\x01` \x89\x01a\x04\x1FV[`\x01`\x01`\xE0\x1B\x03\x19\x85\x16`)\x91\x84\x01\x91\x82\x01R\x83Qa\x05|\x81`-\x84\x01` \x88\x01a\x04\x1FV[\x01`-\x01\x99\x98PPPPPPPPPV\xFELIT_HD_KEY_ID_K256_XMD:SHA-256_SSWU_RO_NUL_\xA2dipfsX\"\x12 ].\x87\x92D\x05\xC7Z7\xCB1\x99\x10\x9D\xC5\x84wu\xCE(\x13|^\x1C\xA4\x05r\x0C\xFA;\xEA=dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static KEYDERIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct KeyDeriver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KeyDeriver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KeyDeriver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KeyDeriver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KeyDeriver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(KeyDeriver)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KeyDeriver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KEYDERIVER_ABI.clone(),
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
                KEYDERIVER_ABI.clone(),
                KEYDERIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `HD_KDF` (0x62e4c464) function
        pub fn hd_kdf(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([98, 228, 196, 100], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KeyDeriver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `HD_KDF` function with signature `HD_KDF()` and selector `0x62e4c464`
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
    #[ethcall(name = "HD_KDF", abi = "HD_KDF()")]
    pub struct HdKdfCall;
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
    pub enum KeyDeriverCalls {
        HdKdf(HdKdfCall),
        ComputeHDPubKey(ComputeHDPubKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for KeyDeriverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HdKdfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HdKdf(decoded));
            }
            if let Ok(decoded) = <ComputeHDPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeHDPubKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KeyDeriverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HdKdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeHDPubKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for KeyDeriverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HdKdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeHDPubKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HdKdfCall> for KeyDeriverCalls {
        fn from(value: HdKdfCall) -> Self {
            Self::HdKdf(value)
        }
    }
    impl ::core::convert::From<ComputeHDPubKeyCall> for KeyDeriverCalls {
        fn from(value: ComputeHDPubKeyCall) -> Self {
            Self::ComputeHDPubKey(value)
        }
    }
    ///Container type for all return fields from the `HD_KDF` function with signature `HD_KDF()` and selector `0x62e4c464`
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
    pub struct HdKdfReturn(pub ::ethers::core::types::Address);
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
