#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 24usize] = [
        "System",
        "RandomnessCollectiveFlip",
        "Timestamp",
        "Balances",
        "ValidatorSet",
        "Session",
        "Aura",
        "Grandpa",
        "TransactionPayment",
        "Sudo",
        "Authorship",
        "TfgridModule",
        "SmartContractModule",
        "TFTBridgeModule",
        "TFTPriceModule",
        "Scheduler",
        "BurningModule",
        "TFKVStore",
        "Council",
        "CouncilMembership",
        "RuntimeUpgrade",
        "Validator",
        "Dao",
        "Utility",
    ];
    #[derive(:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 3)]
        Balances(balances::Event),
        #[codec(index = 4)]
        ValidatorSet(validator_set::Event),
        #[codec(index = 5)]
        Session(session::Event),
        #[codec(index = 7)]
        Grandpa(grandpa::Event),
        #[codec(index = 8)]
        TransactionPayment(transaction_payment::Event),
        #[codec(index = 9)]
        Sudo(sudo::Event),
        #[codec(index = 11)]
        TfgridModule(tfgrid_module::Event),
        #[codec(index = 12)]
        SmartContractModule(smart_contract_module::Event),
        #[codec(index = 13)]
        TFTBridgeModule(tft_bridge_module::Event),
        #[codec(index = 14)]
        TFTPriceModule(tft_price_module::Event),
        #[codec(index = 15)]
        Scheduler(scheduler::Event),
        #[codec(index = 16)]
        BurningModule(burning_module::Event),
        #[codec(index = 17)]
        TFKVStore(tfkv_store::Event),
        #[codec(index = 18)]
        Council(council::Event),
        #[codec(index = 19)]
        CouncilMembership(council_membership::Event),
        #[codec(index = 21)]
        Validator(validator::Event),
        #[codec(index = 22)]
        Dao(dao::Event),
        #[codec(index = 23)]
        Utility(utility::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<Remark> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark",
                        Remark { remark },
                        [
                            101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
                            147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8,
                            146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8,
                            160u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_heap_pages",
                        SetHeapPages { pages },
                        [
                            43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
                            86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
                            125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code",
                        SetCode { code },
                        [
                            27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
                            244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
                            4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_code_without_checks",
                        SetCodeWithoutChecks { code },
                        [
                            102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
                            159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
                            40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::StaticTxPayload<SetStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "set_storage",
                        SetStorage { items },
                        [
                            74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
                            45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
                            140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::StaticTxPayload<KillStorage> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_storage",
                        KillStorage { keys },
                        [
                            174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
                            85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
                            67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
                            15u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "kill_prefix",
                        KillPrefix { prefix, subkeys },
                        [
                            203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
                            193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
                            80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
                            48u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
                    ::subxt::tx::StaticTxPayload::new(
                        "System",
                        "remark_with_event",
                        RemarkWithEvent { remark },
                        [
                            123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
                            134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
                            136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
                            46u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::utils::AccountId32,
                pub hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
                            114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
                            42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::AccountInfo<
                            ::core::primitive::u32,
                            runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
                            114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
                            42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
                            222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
                            41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_support::dispatch::PerDispatchClass<
                            runtime_types::sp_weights::weight_v2::Weight,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            120u8, 67u8, 71u8, 163u8, 36u8, 202u8, 52u8, 106u8, 143u8, 155u8,
                            144u8, 87u8, 142u8, 241u8, 232u8, 183u8, 56u8, 235u8, 27u8, 237u8,
                            20u8, 202u8, 33u8, 85u8, 189u8, 0u8, 28u8, 52u8, 198u8, 40u8, 219u8,
                            54u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
                            254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
                            219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
                            134u8, 60u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
                            21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
                            80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
                            21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
                            80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Number",
                        vec![],
                        [
                            228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
                            246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
                            36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
                            164u8, 191u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::H256>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
                            169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
                            15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
                            36u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_runtime::generic::digest::Digest,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Digest",
                        vec![],
                        [
                            83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
                            31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
                            216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::frame_system::EventRecord<
                                runtime_types::tfchain_runtime::RuntimeEvent,
                                ::subxt::utils::H256,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "Events",
                        vec![],
                        [
                            118u8, 12u8, 23u8, 40u8, 81u8, 73u8, 192u8, 20u8, 107u8, 95u8, 14u8,
                            58u8, 103u8, 14u8, 200u8, 108u8, 153u8, 65u8, 201u8, 228u8, 97u8,
                            123u8, 135u8, 86u8, 163u8, 190u8, 20u8, 218u8, 78u8, 0u8, 77u8, 209u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
                            203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
                            161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
                            112u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
                            63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
                            111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
                            38u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
                            63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
                            111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
                            38u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
                            126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
                            185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
                            178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
                            83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
                            210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
                            155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
                            103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
                            201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
                            172u8, 93u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockWeights,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockWeights",
                        [
                            118u8, 253u8, 239u8, 217u8, 145u8, 115u8, 85u8, 86u8, 172u8, 248u8,
                            139u8, 32u8, 158u8, 126u8, 172u8, 188u8, 197u8, 105u8, 145u8, 235u8,
                            171u8, 50u8, 31u8, 225u8, 167u8, 187u8, 241u8, 87u8, 6u8, 17u8, 234u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::frame_system::limits::BlockLength,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockLength",
                        [
                            116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8,
                            150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8,
                            12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8,
                            236u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_weights::RuntimeDbWeight>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "DbWeight",
                        [
                            124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8,
                            199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8,
                            20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8,
                            118u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "Version",
                        [
                            93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
                            47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
                            165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub fn random_material(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::subxt::utils::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        vec![],
                        [
                            152u8, 126u8, 73u8, 88u8, 54u8, 147u8, 6u8, 19u8, 214u8, 40u8, 159u8,
                            30u8, 236u8, 61u8, 240u8, 65u8, 178u8, 94u8, 146u8, 152u8, 135u8,
                            252u8, 160u8, 86u8, 123u8, 114u8, 251u8, 140u8, 98u8, 143u8, 217u8,
                            242u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<Set> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Timestamp",
                        "set",
                        Set { now },
                        [
                            6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8,
                            102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8,
                            214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
                            221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
                            185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
                            13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
                            27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Transfer {
                pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetBalance {
                pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceTransfer {
                pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TransferAll {
                pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceUnreserve {
                pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                #[doc = "---------------------------------"]
                #[doc = "- Origin account is already in memory, so no DB operations for them."]
                #[doc = "# </weight>"]
                pub fn transfer(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<Transfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer",
                        Transfer { dest, value },
                        [
                            111u8, 222u8, 32u8, 56u8, 171u8, 77u8, 252u8, 29u8, 194u8, 155u8,
                            200u8, 192u8, 198u8, 81u8, 23u8, 115u8, 236u8, 91u8, 218u8, 114u8,
                            107u8, 141u8, 138u8, 100u8, 237u8, 21u8, 58u8, 172u8, 3u8, 20u8, 216u8,
                            38u8,
                        ],
                    )
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SetBalance> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "set_balance",
                        SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        },
                        [
                            234u8, 215u8, 97u8, 98u8, 243u8, 199u8, 57u8, 76u8, 59u8, 161u8, 118u8,
                            207u8, 34u8, 197u8, 198u8, 61u8, 231u8, 210u8, 169u8, 235u8, 150u8,
                            137u8, 173u8, 49u8, 28u8, 77u8, 84u8, 149u8, 143u8, 210u8, 139u8,
                            193u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_transfer",
                        ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            79u8, 174u8, 212u8, 108u8, 184u8, 33u8, 170u8, 29u8, 232u8, 254u8,
                            195u8, 218u8, 221u8, 134u8, 57u8, 99u8, 6u8, 70u8, 181u8, 227u8, 56u8,
                            239u8, 243u8, 158u8, 157u8, 245u8, 36u8, 162u8, 11u8, 237u8, 147u8,
                            15u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_keep_alive",
                        TransferKeepAlive { dest, value },
                        [
                            112u8, 179u8, 75u8, 168u8, 193u8, 221u8, 9u8, 82u8, 190u8, 113u8,
                            253u8, 13u8, 130u8, 134u8, 170u8, 216u8, 136u8, 111u8, 242u8, 220u8,
                            202u8, 112u8, 47u8, 79u8, 73u8, 244u8, 226u8, 59u8, 240u8, 188u8,
                            210u8, 208u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). # <weight>"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                #[doc = "  #</weight>"]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<TransferAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "transfer_all",
                        TransferAll { dest, keep_alive },
                        [
                            46u8, 129u8, 29u8, 177u8, 221u8, 107u8, 245u8, 69u8, 238u8, 126u8,
                            145u8, 26u8, 219u8, 208u8, 14u8, 80u8, 149u8, 1u8, 214u8, 63u8, 67u8,
                            201u8, 144u8, 45u8, 129u8, 145u8, 174u8, 71u8, 238u8, 113u8, 208u8,
                            34u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Balances",
                        "force_unreserve",
                        ForceUnreserve { who, amount },
                        [
                            160u8, 146u8, 137u8, 76u8, 157u8, 187u8, 66u8, 148u8, 207u8, 76u8,
                            32u8, 254u8, 82u8, 215u8, 35u8, 161u8, 213u8, 52u8, 32u8, 98u8, 102u8,
                            106u8, 234u8, 123u8, 6u8, 175u8, 184u8, 188u8, 174u8, 106u8, 176u8,
                            78u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::utils::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::utils::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
                            156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
                            20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            74u8, 203u8, 111u8, 142u8, 225u8, 104u8, 173u8, 51u8, 226u8, 12u8,
                            85u8, 135u8, 41u8, 206u8, 177u8, 238u8, 94u8, 246u8, 184u8, 250u8,
                            140u8, 213u8, 91u8, 118u8, 163u8, 111u8, 211u8, 46u8, 204u8, 160u8,
                            154u8, 21u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
                            40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
                            217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
                            40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
                            217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
                            58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
                            58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
                            136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
                            167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
                            183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_balances::ReserveData<
                                [::core::primitive::u8; 8usize],
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
                            167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
                            183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod validator_set {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddValidator {
                pub validator_id: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveValidator {
                pub validator_id: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddValidatorAgain {
                pub validator_id: ::subxt::utils::AccountId32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Add a new validator."]
                #[doc = ""]
                #[doc = "New validator's session keys should be set in Session pallet before"]
                #[doc = "calling this."]
                #[doc = ""]
                #[doc = "The origin can be configured using the `AddRemoveOrigin` type in the"]
                #[doc = "host runtime. Can also be set to sudo/root."]
                pub fn add_validator(
                    &self,
                    validator_id: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<AddValidator> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ValidatorSet",
                        "add_validator",
                        AddValidator { validator_id },
                        [
                            114u8, 142u8, 186u8, 6u8, 8u8, 126u8, 38u8, 17u8, 89u8, 151u8, 62u8,
                            107u8, 193u8, 121u8, 19u8, 252u8, 135u8, 250u8, 70u8, 51u8, 168u8,
                            35u8, 84u8, 160u8, 173u8, 76u8, 6u8, 157u8, 169u8, 155u8, 2u8, 79u8,
                        ],
                    )
                }
                #[doc = "Remove a validator."]
                #[doc = ""]
                #[doc = "The origin can be configured using the `AddRemoveOrigin` type in the"]
                #[doc = "host runtime. Can also be set to sudo/root."]
                pub fn remove_validator(
                    &self,
                    validator_id: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveValidator> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ValidatorSet",
                        "remove_validator",
                        RemoveValidator { validator_id },
                        [
                            54u8, 212u8, 36u8, 143u8, 40u8, 163u8, 214u8, 2u8, 121u8, 252u8, 181u8,
                            205u8, 247u8, 199u8, 228u8, 6u8, 171u8, 128u8, 29u8, 10u8, 164u8, 41u8,
                            129u8, 148u8, 27u8, 41u8, 254u8, 151u8, 17u8, 121u8, 19u8, 193u8,
                        ],
                    )
                }
                #[doc = "Add an approved validator again when it comes back online."]
                #[doc = ""]
                #[doc = "For this call, the dispatch origin must be the validator itself."]
                pub fn add_validator_again(
                    &self,
                    validator_id: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<AddValidatorAgain> {
                    ::subxt::tx::StaticTxPayload::new(
                        "ValidatorSet",
                        "add_validator_again",
                        AddValidatorAgain { validator_id },
                        [
                            49u8, 228u8, 183u8, 242u8, 121u8, 225u8, 111u8, 16u8, 190u8, 242u8,
                            190u8, 36u8, 175u8, 95u8, 186u8, 43u8, 104u8, 112u8, 116u8, 161u8,
                            28u8, 86u8, 108u8, 146u8, 49u8, 134u8, 12u8, 93u8, 98u8, 75u8, 191u8,
                            61u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::substrate_validator_set::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New validator addition initiated. Effective in ~2 sessions."]
            pub struct ValidatorAdditionInitiated(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for ValidatorAdditionInitiated {
                const PALLET: &'static str = "ValidatorSet";
                const EVENT: &'static str = "ValidatorAdditionInitiated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Validator removal initiated. Effective in ~2 sessions."]
            pub struct ValidatorRemovalInitiated(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for ValidatorRemovalInitiated {
                const PALLET: &'static str = "ValidatorSet";
                const EVENT: &'static str = "ValidatorRemovalInitiated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ValidatorSet",
                        "Validators",
                        vec![],
                        [
                            144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
                            242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
                            32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
                            68u8,
                        ],
                    )
                }
                pub fn approved_validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ValidatorSet",
                        "ApprovedValidators",
                        vec![],
                        [
                            10u8, 30u8, 95u8, 7u8, 178u8, 137u8, 216u8, 239u8, 132u8, 72u8, 120u8,
                            141u8, 163u8, 24u8, 73u8, 148u8, 109u8, 60u8, 210u8, 207u8, 54u8,
                            239u8, 2u8, 60u8, 199u8, 217u8, 164u8, 205u8, 162u8, 126u8, 136u8,
                            133u8,
                        ],
                    )
                }
                pub fn offline_validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "ValidatorSet",
                        "OfflineValidators",
                        vec![],
                        [
                            213u8, 104u8, 52u8, 225u8, 165u8, 188u8, 134u8, 209u8, 8u8, 111u8,
                            30u8, 113u8, 76u8, 93u8, 201u8, 192u8, 227u8, 189u8, 74u8, 134u8,
                            210u8, 89u8, 210u8, 177u8, 169u8, 138u8, 238u8, 169u8, 13u8, 192u8,
                            49u8, 146u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetKeys {
                pub keys: runtime_types::tfchain_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PurgeKeys;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Sets the session key(s) of the function caller to `keys`."]
                #[doc = "Allows an account to set its session key prior to becoming a validator."]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be signed."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
                #[doc = "  `T::Keys::key_ids()` which is fixed."]
                #[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
                #[doc = "- DbWrites: `origin account`, `NextKeys`"]
                #[doc = "- DbReads per key id: `KeyOwner`"]
                #[doc = "- DbWrites per key id: `KeyOwner`"]
                #[doc = "# </weight>"]
                pub fn set_keys(
                    &self,
                    keys: runtime_types::tfchain_runtime::opaque::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetKeys> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Session",
                        "set_keys",
                        SetKeys { keys, proof },
                        [
                            229u8, 28u8, 176u8, 203u8, 230u8, 60u8, 48u8, 16u8, 105u8, 168u8,
                            172u8, 55u8, 34u8, 137u8, 68u8, 166u8, 132u8, 16u8, 89u8, 229u8, 253u8,
                            251u8, 253u8, 247u8, 107u8, 36u8, 229u8, 113u8, 40u8, 208u8, 63u8,
                            202u8,
                        ],
                    )
                }
                #[doc = "Removes any session key(s) of the function caller."]
                #[doc = ""]
                #[doc = "This doesn't take effect until the next session."]
                #[doc = ""]
                #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                #[doc = "usually means being a stash account)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
                #[doc = "  of `T::Keys::key_ids()` which is fixed."]
                #[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
                #[doc = "- DbWrites: `NextKeys`, `origin account`"]
                #[doc = "- DbWrites per key id: `KeyOwner`"]
                #[doc = "# </weight>"]
                pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Session",
                        "purge_keys",
                        PurgeKeys {},
                        [
                            200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
                            35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
                            71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "New session has happened. Note that the argument is the session index, not the"]
            #[doc = "block number as the type might suggest."]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current set of validators."]
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "Validators",
                        vec![],
                        [
                            144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
                            242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
                            32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
                            68u8,
                        ],
                    )
                }
                #[doc = " Current index of the session."]
                pub fn current_index(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "CurrentIndex",
                        vec![],
                        [
                            148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
                            183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
                            159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
                            130u8, 40u8,
                        ],
                    )
                }
                #[doc = " True if the underlying economic identities or weighting behind the validators"]
                #[doc = " has changed in the queued validator set."]
                pub fn queued_changed(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "QueuedChanged",
                        vec![],
                        [
                            105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
                            244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
                            247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
                        ],
                    )
                }
                #[doc = " The queued keys for the next session. When the next session begins, these keys"]
                #[doc = " will be used to determine the validator's session keys."]
                pub fn queued_keys(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<(
                            ::subxt::utils::AccountId32,
                            runtime_types::tfchain_runtime::opaque::SessionKeys,
                        )>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "QueuedKeys",
                        vec![],
                        [
                            95u8, 84u8, 36u8, 56u8, 145u8, 53u8, 123u8, 153u8, 96u8, 11u8, 89u8,
                            233u8, 31u8, 10u8, 130u8, 105u8, 86u8, 7u8, 1u8, 143u8, 205u8, 142u8,
                            178u8, 226u8, 71u8, 160u8, 15u8, 6u8, 166u8, 57u8, 242u8, 210u8,
                        ],
                    )
                }
                #[doc = " Indices of disabled validators."]
                #[doc = ""]
                #[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
                #[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
                #[doc = " a new set of identities."]
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "DisabledValidators",
                        vec![],
                        [
                            135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
                            189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
                            110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
                            84u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_runtime::opaque::SessionKeys,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "NextKeys",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            253u8, 126u8, 143u8, 57u8, 151u8, 37u8, 203u8, 108u8, 167u8, 250u8,
                            156u8, 52u8, 246u8, 237u8, 145u8, 173u8, 30u8, 195u8, 235u8, 161u8,
                            37u8, 232u8, 83u8, 208u8, 121u8, 250u8, 203u8, 142u8, 222u8, 3u8,
                            213u8, 12u8,
                        ],
                    )
                }
                #[doc = " The next session keys for a validator."]
                pub fn next_keys_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_runtime::opaque::SessionKeys,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "NextKeys",
                        Vec::new(),
                        [
                            253u8, 126u8, 143u8, 57u8, 151u8, 37u8, 203u8, 108u8, 167u8, 250u8,
                            156u8, 52u8, 246u8, 237u8, 145u8, 173u8, 30u8, 195u8, 235u8, 161u8,
                            37u8, 232u8, 83u8, 208u8, 121u8, 250u8, 203u8, 142u8, 222u8, 3u8,
                            213u8, 12u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner(
                    &self,
                    _0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "KeyOwner",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            &(_0.borrow(), _1.borrow()),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
                            199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
                            0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
                        ],
                    )
                }
                #[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
                pub fn key_owner_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Session",
                        "KeyOwner",
                        Vec::new(),
                        [
                            4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
                            199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
                            0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "report_equivocation",
                        ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            99u8, 59u8, 234u8, 30u8, 150u8, 187u8, 133u8, 167u8, 92u8, 34u8, 231u8,
                            208u8, 141u8, 40u8, 182u8, 200u8, 82u8, 198u8, 254u8, 56u8, 72u8, 77u8,
                            41u8, 186u8, 80u8, 213u8, 78u8, 214u8, 215u8, 225u8, 187u8, 28u8,
                        ],
                    )
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            145u8, 84u8, 164u8, 4u8, 14u8, 22u8, 157u8, 100u8, 5u8, 21u8, 60u8,
                            65u8, 183u8, 32u8, 212u8, 33u8, 183u8, 167u8, 54u8, 57u8, 204u8, 4u8,
                            28u8, 71u8, 250u8, 151u8, 1u8, 206u8, 222u8, 102u8, 89u8, 55u8,
                        ],
                    )
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<NoteStalled> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Grandpa",
                        "note_stalled",
                        NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            197u8, 236u8, 137u8, 32u8, 46u8, 200u8, 144u8, 13u8, 89u8, 181u8,
                            235u8, 73u8, 167u8, 131u8, 174u8, 93u8, 42u8, 136u8, 238u8, 59u8,
                            129u8, 60u8, 83u8, 100u8, 5u8, 182u8, 99u8, 250u8, 145u8, 180u8, 1u8,
                            199u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "State",
                        vec![],
                        [
                            211u8, 149u8, 114u8, 217u8, 206u8, 194u8, 115u8, 67u8, 12u8, 218u8,
                            246u8, 213u8, 208u8, 29u8, 216u8, 104u8, 2u8, 39u8, 123u8, 172u8,
                            252u8, 210u8, 52u8, 129u8, 147u8, 237u8, 244u8, 68u8, 252u8, 169u8,
                            97u8, 148u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "PendingChange",
                        vec![],
                        [
                            178u8, 24u8, 140u8, 7u8, 8u8, 196u8, 18u8, 58u8, 3u8, 226u8, 181u8,
                            47u8, 155u8, 160u8, 70u8, 12u8, 75u8, 189u8, 38u8, 255u8, 104u8, 141u8,
                            64u8, 34u8, 134u8, 201u8, 102u8, 21u8, 75u8, 81u8, 218u8, 60u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "NextForced",
                        vec![],
                        [
                            99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
                            6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
                            196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "Stalled",
                        vec![],
                        [
                            219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
                            189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
                            109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "CurrentSetId",
                        vec![],
                        [
                            129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
                            178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
                            52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
                            108u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "SetIdSession",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Grandpa",
                        "SetIdSession",
                        Vec::new(),
                        [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::utils::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
                            197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
                            237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
                            180u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_transaction_payment::Releases,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
                            200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
                            58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetKey {
                pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SudoAs {
                pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo(
                    &self,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<Sudo> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo",
                        Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            207u8, 110u8, 39u8, 177u8, 149u8, 81u8, 241u8, 138u8, 46u8, 32u8,
                            198u8, 94u8, 179u8, 114u8, 143u8, 216u8, 250u8, 79u8, 22u8, 124u8,
                            170u8, 147u8, 70u8, 171u8, 70u8, 141u8, 65u8, 4u8, 47u8, 215u8, 184u8,
                            33u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- The weight of this call is defined by the caller."]
                #[doc = "# </weight>"]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo_unchecked_weight",
                        SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            195u8, 19u8, 145u8, 155u8, 127u8, 0u8, 106u8, 74u8, 17u8, 115u8, 47u8,
                            213u8, 252u8, 8u8, 184u8, 91u8, 120u8, 81u8, 151u8, 177u8, 83u8, 25u8,
                            73u8, 241u8, 59u8, 205u8, 11u8, 63u8, 222u8, 139u8, 39u8, 11u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB change."]
                #[doc = "# </weight>"]
                pub fn set_key(
                    &self,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<SetKey> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "set_key",
                        SetKey { new },
                        [
                            23u8, 224u8, 218u8, 169u8, 8u8, 28u8, 111u8, 199u8, 26u8, 88u8, 225u8,
                            105u8, 17u8, 19u8, 87u8, 156u8, 97u8, 67u8, 89u8, 173u8, 70u8, 0u8,
                            5u8, 246u8, 198u8, 135u8, 182u8, 180u8, 44u8, 9u8, 212u8, 95u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<SudoAs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Sudo",
                        "sudo_as",
                        SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            108u8, 61u8, 199u8, 78u8, 75u8, 247u8, 161u8, 42u8, 152u8, 52u8, 221u8,
                            126u8, 110u8, 118u8, 120u8, 113u8, 123u8, 82u8, 66u8, 44u8, 151u8,
                            193u8, 239u8, 41u8, 85u8, 136u8, 26u8, 20u8, 203u8, 165u8, 52u8, 146u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8,
                            34u8, 108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8,
                            80u8, 199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Provide a set of uncles."]
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<SetUncles> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Authorship",
                        "set_uncles",
                        SetUncles { new_uncles },
                        [
                            181u8, 70u8, 222u8, 83u8, 154u8, 215u8, 200u8, 64u8, 154u8, 228u8,
                            115u8, 247u8, 117u8, 89u8, 229u8, 102u8, 128u8, 189u8, 90u8, 60u8,
                            223u8, 19u8, 111u8, 172u8, 5u8, 223u8, 132u8, 37u8, 235u8, 119u8, 42u8,
                            64u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Uncles"]
                pub fn uncles(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_authorship::UncleEntryItem<
                                ::core::primitive::u32,
                                ::subxt::utils::H256,
                                ::subxt::utils::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Authorship",
                        "Uncles",
                        vec![],
                        [
                            193u8, 226u8, 196u8, 151u8, 233u8, 82u8, 60u8, 164u8, 27u8, 156u8,
                            231u8, 51u8, 79u8, 134u8, 170u8, 166u8, 71u8, 120u8, 250u8, 255u8,
                            52u8, 168u8, 74u8, 199u8, 122u8, 253u8, 248u8, 178u8, 39u8, 233u8,
                            132u8, 67u8,
                        ],
                    )
                }
                #[doc = " Author of current block."]
                pub fn author(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Authorship",
                        "Author",
                        vec![],
                        [
                            149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8,
                            25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8,
                            181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8,
                            113u8,
                        ],
                    )
                }
                #[doc = " Whether uncles were already set in this block."]
                pub fn did_set_uncles(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Authorship",
                        "DidSetUncles",
                        vec![],
                        [
                            64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
                            126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
                            83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The number of blocks back we should accept uncles."]
                #[doc = " This means that we will deal with uncle-parents that are"]
                #[doc = " `UncleGenerations + 1` before `now`."]
                pub fn uncle_generations(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Authorship",
                        "UncleGenerations",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod tfgrid_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetStorageVersion {
                pub version: runtime_types::pallet_tfgrid::types::StorageVersion,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateFarm {
                pub name:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                    runtime_types::tfchain_support::types::IP4,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateFarm {
                pub id: ::core::primitive::u32,
                pub name:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddStellarPayoutV2address {
                pub farm_id: ::core::primitive::u32,
                pub stellar_address: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetFarmCertification {
                pub farm_id: ::core::primitive::u32,
                pub certification: runtime_types::tfchain_support::types::FarmCertification,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddFarmIp {
                pub id: ::core::primitive::u32,
                pub ip:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub gw:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveFarmIp {
                pub id: ::core::primitive::u32,
                pub ip:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct DeleteFarm {
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateNode {
                pub farm_id: ::core::primitive::u32,
                pub resources: runtime_types::tfchain_support::resources::Resources,
                pub location: runtime_types::pallet_tfgrid::types::LocationInput<
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                >,
                pub interfaces: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                    runtime_types::tfchain_support::types::Interface<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    >,
                >,
                pub secure_boot: ::core::primitive::bool,
                pub virtualized: ::core::primitive::bool,
                pub serial_number: ::core::option::Option<
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateNode {
                pub node_id: ::core::primitive::u32,
                pub farm_id: ::core::primitive::u32,
                pub resources: runtime_types::tfchain_support::resources::Resources,
                pub location: runtime_types::pallet_tfgrid::types::LocationInput<
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                >,
                pub interfaces: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                    runtime_types::tfchain_support::types::Interface<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    >,
                >,
                pub secure_boot: ::core::primitive::bool,
                pub virtualized: ::core::primitive::bool,
                pub serial_number: ::core::option::Option<
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetNodeCertification {
                pub node_id: ::core::primitive::u32,
                pub node_certification: runtime_types::tfchain_support::types::NodeCertification,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ReportUptime {
                pub uptime: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddNodePublicConfig {
                pub farm_id: ::core::primitive::u32,
                pub node_id: ::core::primitive::u32,
                pub public_config:
                    ::core::option::Option<runtime_types::tfchain_support::types::PublicConfig>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct DeleteNode {
                pub id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateEntity {
                pub target: ::subxt::utils::AccountId32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub country:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub city:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub signature: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateEntity {
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub country:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub city:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DeleteEntity;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateTwin {
                pub ip:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateTwin {
                pub ip:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddTwinEntity {
                pub twin_id: ::core::primitive::u32,
                pub entity_id: ::core::primitive::u32,
                pub signature: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DeleteTwinEntity {
                pub twin_id: ::core::primitive::u32,
                pub entity_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct DeleteTwin {
                pub twin_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreatePricingPolicy {
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub su: runtime_types::pallet_tfgrid::types::Policy,
                pub cu: runtime_types::pallet_tfgrid::types::Policy,
                pub nu: runtime_types::pallet_tfgrid::types::Policy,
                pub ipu: runtime_types::pallet_tfgrid::types::Policy,
                pub unique_name: runtime_types::pallet_tfgrid::types::Policy,
                pub domain_name: runtime_types::pallet_tfgrid::types::Policy,
                pub foundation_account: ::subxt::utils::AccountId32,
                pub certified_sales_account: ::subxt::utils::AccountId32,
                pub discount_for_dedication_nodes: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdatePricingPolicy {
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub su: runtime_types::pallet_tfgrid::types::Policy,
                pub cu: runtime_types::pallet_tfgrid::types::Policy,
                pub nu: runtime_types::pallet_tfgrid::types::Policy,
                pub ipu: runtime_types::pallet_tfgrid::types::Policy,
                pub unique_name: runtime_types::pallet_tfgrid::types::Policy,
                pub domain_name: runtime_types::pallet_tfgrid::types::Policy,
                pub foundation_account: ::subxt::utils::AccountId32,
                pub certified_sales_account: ::subxt::utils::AccountId32,
                pub discount_for_dedication_nodes: ::core::primitive::u8,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateFarmingPolicy {
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub su: ::core::primitive::u32,
                pub cu: ::core::primitive::u32,
                pub nu: ::core::primitive::u32,
                pub ipv4: ::core::primitive::u32,
                pub minimal_uptime: ::core::primitive::u16,
                pub policy_end: ::core::primitive::u32,
                pub immutable: ::core::primitive::bool,
                pub default: ::core::primitive::bool,
                pub node_certification: runtime_types::tfchain_support::types::NodeCertification,
                pub farm_certification: runtime_types::tfchain_support::types::FarmCertification,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UserAcceptTc {
                pub document_link:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub document_hash:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct DeleteNodeFarm {
                pub node_id: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetFarmDedicated {
                pub farm_id: ::core::primitive::u32,
                pub dedicated: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceResetFarmIp {
                pub farm_id: ::core::primitive::u32,
                pub ip:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetConnectionPrice {
                pub price: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddNodeCertifier {
                pub who: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveNodeCertifier {
                pub who: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateFarmingPolicy {
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub su: ::core::primitive::u32,
                pub cu: ::core::primitive::u32,
                pub nu: ::core::primitive::u32,
                pub ipv4: ::core::primitive::u32,
                pub minimal_uptime: ::core::primitive::u16,
                pub policy_end: ::core::primitive::u32,
                pub default: ::core::primitive::bool,
                pub node_certification: runtime_types::tfchain_support::types::NodeCertification,
                pub farm_certification: runtime_types::tfchain_support::types::FarmCertification,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AttachPolicyToFarm {
                pub farm_id: ::core::primitive::u32,
                pub limits: ::core::option::Option<
                    runtime_types::tfchain_support::types::FarmingPolicyLimit,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetZosVersion {
                pub zos_version: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set_storage_version(
                    &self,
                    version: runtime_types::pallet_tfgrid::types::StorageVersion,
                ) -> ::subxt::tx::StaticTxPayload<SetStorageVersion> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "set_storage_version",
                        SetStorageVersion { version },
                        [
                            63u8, 163u8, 229u8, 4u8, 15u8, 253u8, 237u8, 9u8, 47u8, 63u8, 178u8,
                            211u8, 29u8, 230u8, 39u8, 189u8, 140u8, 167u8, 19u8, 36u8, 30u8, 52u8,
                            106u8, 165u8, 55u8, 228u8, 109u8, 123u8, 37u8, 124u8, 140u8, 168u8,
                        ],
                    )
                }
                pub fn create_farm(
                    &self,
                    name: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        runtime_types::tfchain_support::types::IP4,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<CreateFarm> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "create_farm",
                        CreateFarm { name, public_ips },
                        [
                            7u8, 32u8, 98u8, 79u8, 74u8, 224u8, 112u8, 128u8, 167u8, 72u8, 82u8,
                            160u8, 29u8, 209u8, 158u8, 204u8, 124u8, 0u8, 48u8, 116u8, 129u8, 74u8,
                            163u8, 234u8, 110u8, 227u8, 28u8, 84u8, 46u8, 62u8, 19u8, 232u8,
                        ],
                    )
                }
                pub fn update_farm(
                    &self,
                    id: ::core::primitive::u32,
                    name: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateFarm> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "update_farm",
                        UpdateFarm { id, name },
                        [
                            249u8, 13u8, 33u8, 235u8, 142u8, 94u8, 60u8, 222u8, 105u8, 253u8, 40u8,
                            206u8, 158u8, 167u8, 30u8, 235u8, 128u8, 12u8, 12u8, 173u8, 31u8, 72u8,
                            108u8, 32u8, 81u8, 128u8, 7u8, 212u8, 8u8, 50u8, 114u8, 29u8,
                        ],
                    )
                }
                pub fn add_stellar_payout_v2address(
                    &self,
                    farm_id: ::core::primitive::u32,
                    stellar_address: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<AddStellarPayoutV2address> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "add_stellar_payout_v2address",
                        AddStellarPayoutV2address {
                            farm_id,
                            stellar_address,
                        },
                        [
                            149u8, 72u8, 116u8, 127u8, 86u8, 29u8, 91u8, 191u8, 86u8, 102u8, 74u8,
                            126u8, 75u8, 226u8, 168u8, 190u8, 20u8, 249u8, 26u8, 151u8, 142u8,
                            178u8, 229u8, 132u8, 209u8, 156u8, 226u8, 177u8, 236u8, 234u8, 25u8,
                            180u8,
                        ],
                    )
                }
                pub fn set_farm_certification(
                    &self,
                    farm_id: ::core::primitive::u32,
                    certification: runtime_types::tfchain_support::types::FarmCertification,
                ) -> ::subxt::tx::StaticTxPayload<SetFarmCertification> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "set_farm_certification",
                        SetFarmCertification {
                            farm_id,
                            certification,
                        },
                        [
                            116u8, 90u8, 212u8, 85u8, 200u8, 122u8, 60u8, 198u8, 20u8, 197u8,
                            195u8, 214u8, 75u8, 224u8, 52u8, 159u8, 90u8, 49u8, 94u8, 62u8, 123u8,
                            56u8, 135u8, 229u8, 72u8, 132u8, 166u8, 122u8, 8u8, 145u8, 61u8, 194u8,
                        ],
                    )
                }
                pub fn add_farm_ip(
                    &self,
                    id: ::core::primitive::u32,
                    ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    gw: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<AddFarmIp> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "add_farm_ip",
                        AddFarmIp { id, ip, gw },
                        [
                            52u8, 245u8, 35u8, 245u8, 217u8, 100u8, 247u8, 93u8, 0u8, 119u8, 111u8,
                            86u8, 117u8, 80u8, 214u8, 171u8, 238u8, 79u8, 209u8, 230u8, 61u8, 9u8,
                            32u8, 105u8, 66u8, 149u8, 178u8, 204u8, 204u8, 77u8, 146u8, 160u8,
                        ],
                    )
                }
                pub fn remove_farm_ip(
                    &self,
                    id: ::core::primitive::u32,
                    ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<RemoveFarmIp> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "remove_farm_ip",
                        RemoveFarmIp { id, ip },
                        [
                            156u8, 186u8, 247u8, 52u8, 224u8, 50u8, 45u8, 190u8, 17u8, 204u8,
                            252u8, 203u8, 28u8, 76u8, 134u8, 77u8, 65u8, 52u8, 168u8, 220u8, 121u8,
                            124u8, 121u8, 148u8, 38u8, 105u8, 99u8, 17u8, 182u8, 16u8, 216u8,
                            163u8,
                        ],
                    )
                }
                pub fn delete_farm(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DeleteFarm> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "delete_farm",
                        DeleteFarm { id },
                        [
                            249u8, 238u8, 102u8, 205u8, 89u8, 92u8, 90u8, 114u8, 180u8, 227u8,
                            53u8, 88u8, 171u8, 191u8, 65u8, 164u8, 242u8, 3u8, 83u8, 57u8, 254u8,
                            219u8, 77u8, 109u8, 76u8, 160u8, 64u8, 211u8, 113u8, 230u8, 224u8,
                            15u8,
                        ],
                    )
                }
                pub fn create_node(
                    &self,
                    farm_id: ::core::primitive::u32,
                    resources: runtime_types::tfchain_support::resources::Resources,
                    location: runtime_types::pallet_tfgrid::types::LocationInput<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    interfaces: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        runtime_types::tfchain_support::types::Interface<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                            >,
                        >,
                    >,
                    secure_boot: ::core::primitive::bool,
                    virtualized: ::core::primitive::bool,
                    serial_number: ::core::option::Option<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<CreateNode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "create_node",
                        CreateNode {
                            farm_id,
                            resources,
                            location,
                            interfaces,
                            secure_boot,
                            virtualized,
                            serial_number,
                        },
                        [
                            22u8, 101u8, 125u8, 52u8, 227u8, 146u8, 198u8, 194u8, 59u8, 212u8,
                            29u8, 6u8, 239u8, 187u8, 112u8, 206u8, 131u8, 57u8, 122u8, 11u8, 42u8,
                            144u8, 245u8, 148u8, 143u8, 125u8, 236u8, 65u8, 145u8, 85u8, 65u8,
                            150u8,
                        ],
                    )
                }
                pub fn update_node(
                    &self,
                    node_id: ::core::primitive::u32,
                    farm_id: ::core::primitive::u32,
                    resources: runtime_types::tfchain_support::resources::Resources,
                    location: runtime_types::pallet_tfgrid::types::LocationInput<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    interfaces: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        runtime_types::tfchain_support::types::Interface<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                            >,
                        >,
                    >,
                    secure_boot: ::core::primitive::bool,
                    virtualized: ::core::primitive::bool,
                    serial_number: ::core::option::Option<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateNode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "update_node",
                        UpdateNode {
                            node_id,
                            farm_id,
                            resources,
                            location,
                            interfaces,
                            secure_boot,
                            virtualized,
                            serial_number,
                        },
                        [
                            168u8, 37u8, 17u8, 29u8, 173u8, 90u8, 58u8, 177u8, 96u8, 205u8, 96u8,
                            0u8, 18u8, 107u8, 215u8, 67u8, 117u8, 172u8, 45u8, 61u8, 131u8, 124u8,
                            212u8, 253u8, 145u8, 30u8, 3u8, 47u8, 193u8, 210u8, 70u8, 43u8,
                        ],
                    )
                }
                pub fn set_node_certification(
                    &self,
                    node_id: ::core::primitive::u32,
                    node_certification: runtime_types::tfchain_support::types::NodeCertification,
                ) -> ::subxt::tx::StaticTxPayload<SetNodeCertification> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "set_node_certification",
                        SetNodeCertification {
                            node_id,
                            node_certification,
                        },
                        [
                            171u8, 13u8, 82u8, 133u8, 121u8, 184u8, 22u8, 176u8, 44u8, 228u8, 38u8,
                            65u8, 185u8, 252u8, 121u8, 176u8, 226u8, 177u8, 124u8, 0u8, 75u8, 28u8,
                            32u8, 149u8, 195u8, 87u8, 202u8, 5u8, 79u8, 125u8, 92u8, 3u8,
                        ],
                    )
                }
                pub fn report_uptime(
                    &self,
                    uptime: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ReportUptime> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "report_uptime",
                        ReportUptime { uptime },
                        [
                            251u8, 94u8, 190u8, 15u8, 207u8, 61u8, 118u8, 248u8, 38u8, 54u8, 145u8,
                            244u8, 109u8, 145u8, 106u8, 84u8, 213u8, 21u8, 190u8, 37u8, 44u8, 92u8,
                            240u8, 130u8, 60u8, 39u8, 132u8, 91u8, 23u8, 61u8, 194u8, 155u8,
                        ],
                    )
                }
                pub fn add_node_public_config(
                    &self,
                    farm_id: ::core::primitive::u32,
                    node_id: ::core::primitive::u32,
                    public_config: ::core::option::Option<
                        runtime_types::tfchain_support::types::PublicConfig,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<AddNodePublicConfig> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "add_node_public_config",
                        AddNodePublicConfig {
                            farm_id,
                            node_id,
                            public_config,
                        },
                        [
                            99u8, 158u8, 169u8, 3u8, 78u8, 111u8, 39u8, 119u8, 103u8, 129u8, 39u8,
                            61u8, 249u8, 108u8, 72u8, 152u8, 43u8, 44u8, 249u8, 94u8, 28u8, 37u8,
                            213u8, 228u8, 141u8, 133u8, 255u8, 153u8, 183u8, 231u8, 14u8, 68u8,
                        ],
                    )
                }
                pub fn delete_node(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DeleteNode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "delete_node",
                        DeleteNode { id },
                        [
                            140u8, 83u8, 79u8, 212u8, 208u8, 201u8, 177u8, 80u8, 174u8, 47u8,
                            133u8, 145u8, 168u8, 80u8, 120u8, 191u8, 231u8, 159u8, 217u8, 235u8,
                            32u8, 233u8, 229u8, 230u8, 116u8, 248u8, 73u8, 125u8, 66u8, 115u8,
                            207u8, 245u8,
                        ],
                    )
                }
                pub fn create_entity(
                    &self,
                    target: ::subxt::utils::AccountId32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    country: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    city: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    signature: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<CreateEntity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "create_entity",
                        CreateEntity {
                            target,
                            name,
                            country,
                            city,
                            signature,
                        },
                        [
                            243u8, 92u8, 62u8, 192u8, 146u8, 210u8, 234u8, 7u8, 30u8, 109u8, 25u8,
                            91u8, 129u8, 4u8, 252u8, 143u8, 125u8, 211u8, 97u8, 45u8, 16u8, 88u8,
                            82u8, 105u8, 239u8, 134u8, 95u8, 70u8, 209u8, 248u8, 44u8, 210u8,
                        ],
                    )
                }
                pub fn update_entity(
                    &self,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    country: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    city: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateEntity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "update_entity",
                        UpdateEntity {
                            name,
                            country,
                            city,
                        },
                        [
                            36u8, 142u8, 168u8, 47u8, 70u8, 116u8, 178u8, 156u8, 186u8, 216u8,
                            107u8, 153u8, 218u8, 253u8, 191u8, 140u8, 198u8, 98u8, 204u8, 219u8,
                            34u8, 251u8, 145u8, 142u8, 111u8, 156u8, 117u8, 176u8, 83u8, 130u8,
                            181u8, 66u8,
                        ],
                    )
                }
                pub fn delete_entity(&self) -> ::subxt::tx::StaticTxPayload<DeleteEntity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "delete_entity",
                        DeleteEntity {},
                        [
                            202u8, 110u8, 143u8, 217u8, 149u8, 217u8, 39u8, 65u8, 73u8, 188u8,
                            104u8, 165u8, 24u8, 90u8, 187u8, 14u8, 115u8, 176u8, 198u8, 88u8,
                            180u8, 114u8, 134u8, 167u8, 24u8, 148u8, 77u8, 102u8, 43u8, 40u8,
                            199u8, 100u8,
                        ],
                    )
                }
                pub fn create_twin(
                    &self,
                    ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<CreateTwin> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "create_twin",
                        CreateTwin { ip },
                        [
                            44u8, 204u8, 47u8, 159u8, 12u8, 225u8, 39u8, 240u8, 144u8, 185u8, 41u8,
                            129u8, 49u8, 65u8, 125u8, 175u8, 142u8, 201u8, 38u8, 222u8, 165u8,
                            130u8, 164u8, 72u8, 75u8, 232u8, 96u8, 201u8, 154u8, 53u8, 222u8, 0u8,
                        ],
                    )
                }
                pub fn update_twin(
                    &self,
                    ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateTwin> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "update_twin",
                        UpdateTwin { ip },
                        [
                            16u8, 220u8, 158u8, 7u8, 254u8, 187u8, 19u8, 86u8, 47u8, 147u8, 74u8,
                            173u8, 118u8, 199u8, 197u8, 58u8, 166u8, 238u8, 103u8, 117u8, 15u8,
                            153u8, 139u8, 71u8, 73u8, 54u8, 51u8, 219u8, 239u8, 252u8, 249u8, 75u8,
                        ],
                    )
                }
                pub fn add_twin_entity(
                    &self,
                    twin_id: ::core::primitive::u32,
                    entity_id: ::core::primitive::u32,
                    signature: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<AddTwinEntity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "add_twin_entity",
                        AddTwinEntity {
                            twin_id,
                            entity_id,
                            signature,
                        },
                        [
                            253u8, 40u8, 130u8, 253u8, 252u8, 131u8, 165u8, 176u8, 117u8, 226u8,
                            238u8, 85u8, 1u8, 19u8, 233u8, 74u8, 23u8, 165u8, 143u8, 71u8, 146u8,
                            86u8, 51u8, 71u8, 0u8, 125u8, 252u8, 35u8, 171u8, 227u8, 38u8, 71u8,
                        ],
                    )
                }
                pub fn delete_twin_entity(
                    &self,
                    twin_id: ::core::primitive::u32,
                    entity_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DeleteTwinEntity> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "delete_twin_entity",
                        DeleteTwinEntity { twin_id, entity_id },
                        [
                            42u8, 147u8, 2u8, 43u8, 213u8, 116u8, 126u8, 84u8, 10u8, 32u8, 42u8,
                            193u8, 243u8, 55u8, 77u8, 119u8, 214u8, 38u8, 145u8, 254u8, 174u8,
                            132u8, 167u8, 241u8, 20u8, 5u8, 156u8, 139u8, 116u8, 53u8, 33u8, 33u8,
                        ],
                    )
                }
                pub fn delete_twin(
                    &self,
                    twin_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DeleteTwin> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "delete_twin",
                        DeleteTwin { twin_id },
                        [
                            165u8, 138u8, 198u8, 30u8, 45u8, 194u8, 101u8, 16u8, 55u8, 169u8,
                            230u8, 157u8, 58u8, 151u8, 5u8, 104u8, 188u8, 236u8, 92u8, 46u8, 124u8,
                            231u8, 75u8, 63u8, 195u8, 7u8, 221u8, 207u8, 44u8, 11u8, 18u8, 116u8,
                        ],
                    )
                }
                pub fn create_pricing_policy(
                    &self,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    su: runtime_types::pallet_tfgrid::types::Policy,
                    cu: runtime_types::pallet_tfgrid::types::Policy,
                    nu: runtime_types::pallet_tfgrid::types::Policy,
                    ipu: runtime_types::pallet_tfgrid::types::Policy,
                    unique_name: runtime_types::pallet_tfgrid::types::Policy,
                    domain_name: runtime_types::pallet_tfgrid::types::Policy,
                    foundation_account: ::subxt::utils::AccountId32,
                    certified_sales_account: ::subxt::utils::AccountId32,
                    discount_for_dedication_nodes: ::core::primitive::u8,
                ) -> ::subxt::tx::StaticTxPayload<CreatePricingPolicy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "create_pricing_policy",
                        CreatePricingPolicy {
                            name,
                            su,
                            cu,
                            nu,
                            ipu,
                            unique_name,
                            domain_name,
                            foundation_account,
                            certified_sales_account,
                            discount_for_dedication_nodes,
                        },
                        [
                            203u8, 124u8, 135u8, 110u8, 19u8, 245u8, 223u8, 89u8, 246u8, 175u8,
                            111u8, 22u8, 176u8, 30u8, 26u8, 98u8, 59u8, 154u8, 159u8, 3u8, 169u8,
                            131u8, 184u8, 220u8, 90u8, 118u8, 33u8, 0u8, 162u8, 188u8, 162u8, 22u8,
                        ],
                    )
                }
                pub fn update_pricing_policy(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    su: runtime_types::pallet_tfgrid::types::Policy,
                    cu: runtime_types::pallet_tfgrid::types::Policy,
                    nu: runtime_types::pallet_tfgrid::types::Policy,
                    ipu: runtime_types::pallet_tfgrid::types::Policy,
                    unique_name: runtime_types::pallet_tfgrid::types::Policy,
                    domain_name: runtime_types::pallet_tfgrid::types::Policy,
                    foundation_account: ::subxt::utils::AccountId32,
                    certified_sales_account: ::subxt::utils::AccountId32,
                    discount_for_dedication_nodes: ::core::primitive::u8,
                ) -> ::subxt::tx::StaticTxPayload<UpdatePricingPolicy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "update_pricing_policy",
                        UpdatePricingPolicy {
                            id,
                            name,
                            su,
                            cu,
                            nu,
                            ipu,
                            unique_name,
                            domain_name,
                            foundation_account,
                            certified_sales_account,
                            discount_for_dedication_nodes,
                        },
                        [
                            250u8, 218u8, 165u8, 30u8, 133u8, 47u8, 247u8, 26u8, 88u8, 91u8, 128u8,
                            201u8, 168u8, 71u8, 241u8, 206u8, 166u8, 160u8, 146u8, 215u8, 212u8,
                            187u8, 24u8, 92u8, 134u8, 17u8, 120u8, 97u8, 151u8, 65u8, 127u8, 163u8,
                        ],
                    )
                }
                pub fn create_farming_policy(
                    &self,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    su: ::core::primitive::u32,
                    cu: ::core::primitive::u32,
                    nu: ::core::primitive::u32,
                    ipv4: ::core::primitive::u32,
                    minimal_uptime: ::core::primitive::u16,
                    policy_end: ::core::primitive::u32,
                    immutable: ::core::primitive::bool,
                    default: ::core::primitive::bool,
                    node_certification: runtime_types::tfchain_support::types::NodeCertification,
                    farm_certification: runtime_types::tfchain_support::types::FarmCertification,
                ) -> ::subxt::tx::StaticTxPayload<CreateFarmingPolicy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "create_farming_policy",
                        CreateFarmingPolicy {
                            name,
                            su,
                            cu,
                            nu,
                            ipv4,
                            minimal_uptime,
                            policy_end,
                            immutable,
                            default,
                            node_certification,
                            farm_certification,
                        },
                        [
                            54u8, 13u8, 239u8, 154u8, 129u8, 28u8, 63u8, 159u8, 109u8, 51u8, 12u8,
                            117u8, 243u8, 224u8, 117u8, 114u8, 254u8, 53u8, 23u8, 90u8, 157u8,
                            178u8, 70u8, 27u8, 156u8, 29u8, 76u8, 105u8, 86u8, 104u8, 92u8, 41u8,
                        ],
                    )
                }
                pub fn user_accept_tc(
                    &self,
                    document_link: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    document_hash: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UserAcceptTc> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "user_accept_tc",
                        UserAcceptTc {
                            document_link,
                            document_hash,
                        },
                        [
                            136u8, 237u8, 179u8, 129u8, 88u8, 25u8, 65u8, 27u8, 181u8, 250u8, 72u8,
                            82u8, 148u8, 221u8, 208u8, 82u8, 59u8, 81u8, 166u8, 84u8, 66u8, 11u8,
                            253u8, 196u8, 219u8, 227u8, 70u8, 199u8, 44u8, 38u8, 50u8, 97u8,
                        ],
                    )
                }
                pub fn delete_node_farm(
                    &self,
                    node_id: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<DeleteNodeFarm> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "delete_node_farm",
                        DeleteNodeFarm { node_id },
                        [
                            71u8, 218u8, 135u8, 73u8, 75u8, 174u8, 16u8, 53u8, 93u8, 243u8, 125u8,
                            0u8, 162u8, 246u8, 253u8, 174u8, 24u8, 28u8, 53u8, 219u8, 199u8, 242u8,
                            108u8, 143u8, 240u8, 9u8, 88u8, 10u8, 78u8, 67u8, 237u8, 164u8,
                        ],
                    )
                }
                pub fn set_farm_dedicated(
                    &self,
                    farm_id: ::core::primitive::u32,
                    dedicated: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<SetFarmDedicated> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "set_farm_dedicated",
                        SetFarmDedicated { farm_id, dedicated },
                        [
                            206u8, 116u8, 222u8, 236u8, 150u8, 136u8, 115u8, 92u8, 18u8, 157u8,
                            206u8, 55u8, 54u8, 207u8, 176u8, 192u8, 204u8, 101u8, 35u8, 45u8, 87u8,
                            223u8, 232u8, 164u8, 68u8, 239u8, 1u8, 107u8, 80u8, 90u8, 84u8, 228u8,
                        ],
                    )
                }
                pub fn force_reset_farm_ip(
                    &self,
                    farm_id: ::core::primitive::u32,
                    ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ForceResetFarmIp> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "force_reset_farm_ip",
                        ForceResetFarmIp { farm_id, ip },
                        [
                            48u8, 80u8, 15u8, 126u8, 82u8, 47u8, 179u8, 3u8, 140u8, 4u8, 176u8,
                            77u8, 109u8, 241u8, 39u8, 115u8, 21u8, 243u8, 74u8, 21u8, 246u8, 140u8,
                            10u8, 88u8, 26u8, 113u8, 90u8, 4u8, 17u8, 102u8, 170u8, 33u8,
                        ],
                    )
                }
                pub fn set_connection_price(
                    &self,
                    price: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetConnectionPrice> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "set_connection_price",
                        SetConnectionPrice { price },
                        [
                            55u8, 65u8, 178u8, 50u8, 251u8, 254u8, 116u8, 55u8, 52u8, 152u8, 218u8,
                            47u8, 222u8, 134u8, 162u8, 91u8, 19u8, 218u8, 103u8, 233u8, 241u8,
                            199u8, 116u8, 239u8, 2u8, 191u8, 65u8, 175u8, 87u8, 7u8, 192u8, 114u8,
                        ],
                    )
                }
                pub fn add_node_certifier(
                    &self,
                    who: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<AddNodeCertifier> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "add_node_certifier",
                        AddNodeCertifier { who },
                        [
                            200u8, 65u8, 254u8, 184u8, 184u8, 155u8, 22u8, 251u8, 53u8, 229u8,
                            195u8, 106u8, 183u8, 176u8, 233u8, 143u8, 106u8, 36u8, 25u8, 28u8,
                            82u8, 221u8, 135u8, 198u8, 23u8, 41u8, 211u8, 189u8, 91u8, 185u8, 85u8,
                            236u8,
                        ],
                    )
                }
                pub fn remove_node_certifier(
                    &self,
                    who: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveNodeCertifier> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "remove_node_certifier",
                        RemoveNodeCertifier { who },
                        [
                            152u8, 218u8, 68u8, 7u8, 244u8, 191u8, 93u8, 16u8, 222u8, 21u8, 10u8,
                            44u8, 5u8, 237u8, 152u8, 23u8, 104u8, 21u8, 189u8, 174u8, 53u8, 167u8,
                            154u8, 76u8, 241u8, 60u8, 78u8, 135u8, 107u8, 31u8, 96u8, 165u8,
                        ],
                    )
                }
                pub fn update_farming_policy(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    su: ::core::primitive::u32,
                    cu: ::core::primitive::u32,
                    nu: ::core::primitive::u32,
                    ipv4: ::core::primitive::u32,
                    minimal_uptime: ::core::primitive::u16,
                    policy_end: ::core::primitive::u32,
                    default: ::core::primitive::bool,
                    node_certification: runtime_types::tfchain_support::types::NodeCertification,
                    farm_certification: runtime_types::tfchain_support::types::FarmCertification,
                ) -> ::subxt::tx::StaticTxPayload<UpdateFarmingPolicy> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "update_farming_policy",
                        UpdateFarmingPolicy {
                            id,
                            name,
                            su,
                            cu,
                            nu,
                            ipv4,
                            minimal_uptime,
                            policy_end,
                            default,
                            node_certification,
                            farm_certification,
                        },
                        [
                            14u8, 192u8, 18u8, 219u8, 108u8, 229u8, 205u8, 96u8, 69u8, 162u8,
                            138u8, 47u8, 189u8, 212u8, 165u8, 13u8, 250u8, 5u8, 79u8, 140u8, 131u8,
                            77u8, 98u8, 91u8, 223u8, 186u8, 25u8, 52u8, 22u8, 185u8, 212u8, 32u8,
                        ],
                    )
                }
                pub fn attach_policy_to_farm(
                    &self,
                    farm_id: ::core::primitive::u32,
                    limits: ::core::option::Option<
                        runtime_types::tfchain_support::types::FarmingPolicyLimit,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<AttachPolicyToFarm> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "attach_policy_to_farm",
                        AttachPolicyToFarm { farm_id, limits },
                        [
                            52u8, 86u8, 40u8, 169u8, 87u8, 253u8, 113u8, 58u8, 47u8, 201u8, 143u8,
                            236u8, 189u8, 251u8, 244u8, 7u8, 17u8, 24u8, 102u8, 52u8, 250u8, 158u8,
                            30u8, 248u8, 34u8, 1u8, 240u8, 98u8, 165u8, 152u8, 238u8, 46u8,
                        ],
                    )
                }
                pub fn set_zos_version(
                    &self,
                    zos_version: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetZosVersion> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TfgridModule",
                        "set_zos_version",
                        SetZosVersion { zos_version },
                        [
                            252u8, 66u8, 161u8, 46u8, 125u8, 153u8, 76u8, 143u8, 177u8, 213u8,
                            65u8, 19u8, 106u8, 126u8, 251u8, 32u8, 151u8, 55u8, 5u8, 75u8, 202u8,
                            17u8, 126u8, 162u8, 244u8, 20u8, 172u8, 128u8, 58u8, 185u8, 111u8,
                            114u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_tfgrid::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmStored(
                pub  runtime_types::tfchain_support::types::Farm<
                    runtime_types::pallet_tfgrid::farm::FarmName,
                >,
            );
            impl ::subxt::events::StaticEvent for FarmStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmUpdated(
                pub  runtime_types::tfchain_support::types::Farm<
                    runtime_types::pallet_tfgrid::farm::FarmName,
                >,
            );
            impl ::subxt::events::StaticEvent for FarmUpdated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct FarmDeleted(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for FarmDeleted {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmDeleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeStored(
                pub  runtime_types::tfchain_support::types::Node<
                    runtime_types::pallet_tfgrid::node::Location,
                    runtime_types::tfchain_support::types::Interface<
                        runtime_types::pallet_tfgrid::interface::InterfaceName,
                        runtime_types::pallet_tfgrid::interface::InterfaceMac,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_tfgrid::interface::InterfaceIp,
                        >,
                    >,
                    runtime_types::pallet_tfgrid::node::SerialNumber,
                >,
            );
            impl ::subxt::events::StaticEvent for NodeStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeUpdated(
                pub  runtime_types::tfchain_support::types::Node<
                    runtime_types::pallet_tfgrid::node::Location,
                    runtime_types::tfchain_support::types::Interface<
                        runtime_types::pallet_tfgrid::interface::InterfaceName,
                        runtime_types::pallet_tfgrid::interface::InterfaceMac,
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::pallet_tfgrid::interface::InterfaceIp,
                        >,
                    >,
                    runtime_types::pallet_tfgrid::node::SerialNumber,
                >,
            );
            impl ::subxt::events::StaticEvent for NodeUpdated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct NodeDeleted(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for NodeDeleted {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeDeleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeUptimeReported(
                pub ::core::primitive::u32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for NodeUptimeReported {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeUptimeReported";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodePublicConfigStored(
                pub ::core::primitive::u32,
                pub ::core::option::Option<runtime_types::tfchain_support::types::PublicConfig>,
            );
            impl ::subxt::events::StaticEvent for NodePublicConfigStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodePublicConfigStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EntityStored(
                pub  runtime_types::pallet_tfgrid::types::Entity<
                    ::subxt::utils::AccountId32,
                    runtime_types::pallet_tfgrid::node::CityName,
                    runtime_types::pallet_tfgrid::node::CountryName,
                >,
            );
            impl ::subxt::events::StaticEvent for EntityStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "EntityStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EntityUpdated(
                pub  runtime_types::pallet_tfgrid::types::Entity<
                    ::subxt::utils::AccountId32,
                    runtime_types::pallet_tfgrid::node::CityName,
                    runtime_types::pallet_tfgrid::node::CountryName,
                >,
            );
            impl ::subxt::events::StaticEvent for EntityUpdated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "EntityUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct EntityDeleted(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for EntityDeleted {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "EntityDeleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TwinStored(
                pub  runtime_types::pallet_tfgrid::types::Twin<
                    runtime_types::pallet_tfgrid::twin::TwinIp,
                    ::subxt::utils::AccountId32,
                >,
            );
            impl ::subxt::events::StaticEvent for TwinStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "TwinStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TwinUpdated(
                pub  runtime_types::pallet_tfgrid::types::Twin<
                    runtime_types::pallet_tfgrid::twin::TwinIp,
                    ::subxt::utils::AccountId32,
                >,
            );
            impl ::subxt::events::StaticEvent for TwinUpdated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "TwinUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TwinEntityStored(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::events::StaticEvent for TwinEntityStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "TwinEntityStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct TwinEntityRemoved(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for TwinEntityRemoved {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "TwinEntityRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct TwinDeleted(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for TwinDeleted {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "TwinDeleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct PricingPolicyStored(
                pub runtime_types::pallet_tfgrid::types::PricingPolicy<::subxt::utils::AccountId32>,
            );
            impl ::subxt::events::StaticEvent for PricingPolicyStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "PricingPolicyStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmingPolicyStored(
                pub runtime_types::pallet_tfgrid::types::FarmingPolicy<::core::primitive::u32>,
            );
            impl ::subxt::events::StaticEvent for FarmingPolicyStored {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmingPolicyStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmPayoutV2AddressRegistered(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::events::StaticEvent for FarmPayoutV2AddressRegistered {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmPayoutV2AddressRegistered";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct FarmMarkedAsDedicated(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for FarmMarkedAsDedicated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmMarkedAsDedicated";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ConnectionPriceSet(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for ConnectionPriceSet {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "ConnectionPriceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeCertificationSet(
                pub ::core::primitive::u32,
                pub runtime_types::tfchain_support::types::NodeCertification,
            );
            impl ::subxt::events::StaticEvent for NodeCertificationSet {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeCertificationSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeCertifierAdded(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for NodeCertifierAdded {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeCertifierAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeCertifierRemoved(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for NodeCertifierRemoved {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "NodeCertifierRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmingPolicyUpdated(
                pub runtime_types::pallet_tfgrid::types::FarmingPolicy<::core::primitive::u32>,
            );
            impl ::subxt::events::StaticEvent for FarmingPolicyUpdated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmingPolicyUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmingPolicySet(
                pub ::core::primitive::u32,
                pub  ::core::option::Option<
                    runtime_types::tfchain_support::types::FarmingPolicyLimit,
                >,
            );
            impl ::subxt::events::StaticEvent for FarmingPolicySet {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmingPolicySet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct FarmCertificationSet(
                pub ::core::primitive::u32,
                pub runtime_types::tfchain_support::types::FarmCertification,
            );
            impl ::subxt::events::StaticEvent for FarmCertificationSet {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "FarmCertificationSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ZosVersionUpdated(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::events::StaticEvent for ZosVersionUpdated {
                const PALLET: &'static str = "TfgridModule";
                const EVENT: &'static str = "ZosVersionUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn farms(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_support::types::Farm<
                            runtime_types::pallet_tfgrid::farm::FarmName,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Farms",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            78u8, 175u8, 6u8, 22u8, 212u8, 118u8, 51u8, 235u8, 172u8, 204u8, 25u8,
                            131u8, 193u8, 170u8, 98u8, 153u8, 229u8, 189u8, 127u8, 167u8, 91u8,
                            221u8, 241u8, 75u8, 92u8, 201u8, 86u8, 164u8, 146u8, 165u8, 152u8,
                            234u8,
                        ],
                    )
                }
                pub fn farms_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_support::types::Farm<
                            runtime_types::pallet_tfgrid::farm::FarmName,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Farms",
                        Vec::new(),
                        [
                            78u8, 175u8, 6u8, 22u8, 212u8, 118u8, 51u8, 235u8, 172u8, 204u8, 25u8,
                            131u8, 193u8, 170u8, 98u8, 153u8, 229u8, 189u8, 127u8, 167u8, 91u8,
                            221u8, 241u8, 75u8, 92u8, 201u8, 86u8, 164u8, 146u8, 165u8, 152u8,
                            234u8,
                        ],
                    )
                }
                pub fn nodes_by_farm_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "NodesByFarmID",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            165u8, 22u8, 165u8, 8u8, 102u8, 105u8, 214u8, 160u8, 145u8, 121u8, 1u8,
                            124u8, 68u8, 8u8, 249u8, 47u8, 83u8, 26u8, 30u8, 89u8, 105u8, 137u8,
                            205u8, 32u8, 47u8, 45u8, 44u8, 207u8, 105u8, 68u8, 137u8, 227u8,
                        ],
                    )
                }
                pub fn nodes_by_farm_id_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "NodesByFarmID",
                        Vec::new(),
                        [
                            165u8, 22u8, 165u8, 8u8, 102u8, 105u8, 214u8, 160u8, 145u8, 121u8, 1u8,
                            124u8, 68u8, 8u8, 249u8, 47u8, 83u8, 26u8, 30u8, 89u8, 105u8, 137u8,
                            205u8, 32u8, 47u8, 45u8, 44u8, 207u8, 105u8, 68u8, 137u8, 227u8,
                        ],
                    )
                }
                pub fn farm_id_by_name(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmIdByName",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            123u8, 37u8, 72u8, 225u8, 218u8, 26u8, 212u8, 226u8, 204u8, 44u8,
                            148u8, 36u8, 22u8, 93u8, 245u8, 189u8, 98u8, 165u8, 196u8, 102u8, 61u8,
                            160u8, 230u8, 237u8, 148u8, 215u8, 142u8, 104u8, 9u8, 191u8, 104u8,
                            123u8,
                        ],
                    )
                }
                pub fn farm_id_by_name_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmIdByName",
                        Vec::new(),
                        [
                            123u8, 37u8, 72u8, 225u8, 218u8, 26u8, 212u8, 226u8, 204u8, 44u8,
                            148u8, 36u8, 22u8, 93u8, 245u8, 189u8, 98u8, 165u8, 196u8, 102u8, 61u8,
                            160u8, 230u8, 237u8, 148u8, 215u8, 142u8, 104u8, 9u8, 191u8, 104u8,
                            123u8,
                        ],
                    )
                }
                pub fn farm_payout_v2_address_by_farm_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmPayoutV2AddressByFarmID",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            212u8, 133u8, 245u8, 23u8, 219u8, 107u8, 25u8, 193u8, 234u8, 52u8,
                            62u8, 143u8, 178u8, 68u8, 47u8, 96u8, 209u8, 223u8, 95u8, 255u8, 42u8,
                            130u8, 207u8, 116u8, 229u8, 70u8, 161u8, 4u8, 43u8, 135u8, 87u8, 208u8,
                        ],
                    )
                }
                pub fn farm_payout_v2_address_by_farm_id_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmPayoutV2AddressByFarmID",
                        Vec::new(),
                        [
                            212u8, 133u8, 245u8, 23u8, 219u8, 107u8, 25u8, 193u8, 234u8, 52u8,
                            62u8, 143u8, 178u8, 68u8, 47u8, 96u8, 209u8, 223u8, 95u8, 255u8, 42u8,
                            130u8, 207u8, 116u8, 229u8, 70u8, 161u8, 4u8, 43u8, 135u8, 87u8, 208u8,
                        ],
                    )
                }
                pub fn nodes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_support::types::Node<
                            runtime_types::pallet_tfgrid::node::Location,
                            runtime_types::tfchain_support::types::Interface<
                                runtime_types::pallet_tfgrid::interface::InterfaceName,
                                runtime_types::pallet_tfgrid::interface::InterfaceMac,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    runtime_types::pallet_tfgrid::interface::InterfaceIp,
                                >,
                            >,
                            runtime_types::pallet_tfgrid::node::SerialNumber,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Nodes",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            141u8, 44u8, 20u8, 68u8, 25u8, 29u8, 67u8, 95u8, 161u8, 97u8, 81u8,
                            80u8, 196u8, 0u8, 247u8, 51u8, 57u8, 24u8, 20u8, 38u8, 36u8, 236u8,
                            55u8, 193u8, 115u8, 197u8, 245u8, 72u8, 248u8, 156u8, 117u8, 135u8,
                        ],
                    )
                }
                pub fn nodes_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_support::types::Node<
                            runtime_types::pallet_tfgrid::node::Location,
                            runtime_types::tfchain_support::types::Interface<
                                runtime_types::pallet_tfgrid::interface::InterfaceName,
                                runtime_types::pallet_tfgrid::interface::InterfaceMac,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    runtime_types::pallet_tfgrid::interface::InterfaceIp,
                                >,
                            >,
                            runtime_types::pallet_tfgrid::node::SerialNumber,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Nodes",
                        Vec::new(),
                        [
                            141u8, 44u8, 20u8, 68u8, 25u8, 29u8, 67u8, 95u8, 161u8, 97u8, 81u8,
                            80u8, 196u8, 0u8, 247u8, 51u8, 57u8, 24u8, 20u8, 38u8, 36u8, 236u8,
                            55u8, 193u8, 115u8, 197u8, 245u8, 72u8, 248u8, 156u8, 117u8, 135u8,
                        ],
                    )
                }
                pub fn node_id_by_twin_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "NodeIdByTwinID",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            239u8, 102u8, 114u8, 122u8, 121u8, 211u8, 20u8, 45u8, 62u8, 153u8,
                            179u8, 36u8, 105u8, 143u8, 209u8, 212u8, 30u8, 190u8, 1u8, 169u8,
                            235u8, 108u8, 177u8, 13u8, 170u8, 29u8, 132u8, 39u8, 189u8, 168u8,
                            185u8, 167u8,
                        ],
                    )
                }
                pub fn node_id_by_twin_id_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "NodeIdByTwinID",
                        Vec::new(),
                        [
                            239u8, 102u8, 114u8, 122u8, 121u8, 211u8, 20u8, 45u8, 62u8, 153u8,
                            179u8, 36u8, 105u8, 143u8, 209u8, 212u8, 30u8, 190u8, 1u8, 169u8,
                            235u8, 108u8, 177u8, 13u8, 170u8, 29u8, 132u8, 39u8, 189u8, 168u8,
                            185u8, 167u8,
                        ],
                    )
                }
                pub fn entities(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::Entity<
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_tfgrid::node::CityName,
                            runtime_types::pallet_tfgrid::node::CountryName,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Entities",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            175u8, 6u8, 58u8, 239u8, 50u8, 89u8, 51u8, 114u8, 159u8, 51u8, 191u8,
                            237u8, 28u8, 16u8, 100u8, 178u8, 104u8, 214u8, 192u8, 188u8, 115u8,
                            24u8, 38u8, 124u8, 103u8, 8u8, 204u8, 116u8, 204u8, 82u8, 205u8, 166u8,
                        ],
                    )
                }
                pub fn entities_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::Entity<
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_tfgrid::node::CityName,
                            runtime_types::pallet_tfgrid::node::CountryName,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Entities",
                        Vec::new(),
                        [
                            175u8, 6u8, 58u8, 239u8, 50u8, 89u8, 51u8, 114u8, 159u8, 51u8, 191u8,
                            237u8, 28u8, 16u8, 100u8, 178u8, 104u8, 214u8, 192u8, 188u8, 115u8,
                            24u8, 38u8, 124u8, 103u8, 8u8, 204u8, 116u8, 204u8, 82u8, 205u8, 166u8,
                        ],
                    )
                }
                pub fn entity_id_by_account_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "EntityIdByAccountID",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            17u8, 105u8, 203u8, 138u8, 121u8, 129u8, 207u8, 142u8, 115u8, 14u8,
                            220u8, 125u8, 229u8, 165u8, 194u8, 180u8, 41u8, 121u8, 1u8, 14u8, 75u8,
                            166u8, 36u8, 248u8, 167u8, 92u8, 8u8, 17u8, 171u8, 41u8, 81u8, 57u8,
                        ],
                    )
                }
                pub fn entity_id_by_account_id_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "EntityIdByAccountID",
                        Vec::new(),
                        [
                            17u8, 105u8, 203u8, 138u8, 121u8, 129u8, 207u8, 142u8, 115u8, 14u8,
                            220u8, 125u8, 229u8, 165u8, 194u8, 180u8, 41u8, 121u8, 1u8, 14u8, 75u8,
                            166u8, 36u8, 248u8, 167u8, 92u8, 8u8, 17u8, 171u8, 41u8, 81u8, 57u8,
                        ],
                    )
                }
                pub fn entity_id_by_name(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "EntityIdByName",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            252u8, 118u8, 24u8, 249u8, 84u8, 140u8, 121u8, 95u8, 165u8, 134u8,
                            131u8, 163u8, 249u8, 101u8, 133u8, 170u8, 96u8, 230u8, 130u8, 194u8,
                            210u8, 91u8, 92u8, 17u8, 90u8, 1u8, 42u8, 143u8, 141u8, 94u8, 194u8,
                            87u8,
                        ],
                    )
                }
                pub fn entity_id_by_name_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "EntityIdByName",
                        Vec::new(),
                        [
                            252u8, 118u8, 24u8, 249u8, 84u8, 140u8, 121u8, 95u8, 165u8, 134u8,
                            131u8, 163u8, 249u8, 101u8, 133u8, 170u8, 96u8, 230u8, 130u8, 194u8,
                            210u8, 91u8, 92u8, 17u8, 90u8, 1u8, 42u8, 143u8, 141u8, 94u8, 194u8,
                            87u8,
                        ],
                    )
                }
                pub fn twins(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::Twin<
                            runtime_types::pallet_tfgrid::twin::TwinIp,
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Twins",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            12u8, 20u8, 130u8, 33u8, 26u8, 198u8, 243u8, 86u8, 157u8, 155u8, 8u8,
                            12u8, 167u8, 25u8, 84u8, 113u8, 172u8, 123u8, 15u8, 231u8, 227u8, 80u8,
                            25u8, 168u8, 185u8, 58u8, 63u8, 92u8, 247u8, 100u8, 43u8, 197u8,
                        ],
                    )
                }
                pub fn twins_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::Twin<
                            runtime_types::pallet_tfgrid::twin::TwinIp,
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "Twins",
                        Vec::new(),
                        [
                            12u8, 20u8, 130u8, 33u8, 26u8, 198u8, 243u8, 86u8, 157u8, 155u8, 8u8,
                            12u8, 167u8, 25u8, 84u8, 113u8, 172u8, 123u8, 15u8, 231u8, 227u8, 80u8,
                            25u8, 168u8, 185u8, 58u8, 63u8, 92u8, 247u8, 100u8, 43u8, 197u8,
                        ],
                    )
                }
                pub fn twin_id_by_account_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "TwinIdByAccountID",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            146u8, 219u8, 190u8, 182u8, 38u8, 190u8, 193u8, 157u8, 215u8, 252u8,
                            190u8, 194u8, 206u8, 131u8, 117u8, 214u8, 31u8, 111u8, 81u8, 230u8,
                            244u8, 107u8, 45u8, 200u8, 42u8, 141u8, 206u8, 174u8, 54u8, 241u8,
                            48u8, 248u8,
                        ],
                    )
                }
                pub fn twin_id_by_account_id_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "TwinIdByAccountID",
                        Vec::new(),
                        [
                            146u8, 219u8, 190u8, 182u8, 38u8, 190u8, 193u8, 157u8, 215u8, 252u8,
                            190u8, 194u8, 206u8, 131u8, 117u8, 214u8, 31u8, 111u8, 81u8, 230u8,
                            244u8, 107u8, 45u8, 200u8, 42u8, 141u8, 206u8, 174u8, 54u8, 241u8,
                            48u8, 248u8,
                        ],
                    )
                }
                pub fn pricing_policies(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::PricingPolicy<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "PricingPolicies",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            211u8, 137u8, 245u8, 97u8, 245u8, 211u8, 109u8, 152u8, 205u8, 97u8,
                            165u8, 151u8, 85u8, 244u8, 196u8, 137u8, 116u8, 16u8, 2u8, 1u8, 210u8,
                            65u8, 205u8, 121u8, 43u8, 160u8, 238u8, 143u8, 25u8, 185u8, 29u8, 85u8,
                        ],
                    )
                }
                pub fn pricing_policies_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::PricingPolicy<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "PricingPolicies",
                        Vec::new(),
                        [
                            211u8, 137u8, 245u8, 97u8, 245u8, 211u8, 109u8, 152u8, 205u8, 97u8,
                            165u8, 151u8, 85u8, 244u8, 196u8, 137u8, 116u8, 16u8, 2u8, 1u8, 210u8,
                            65u8, 205u8, 121u8, 43u8, 160u8, 238u8, 143u8, 25u8, 185u8, 29u8, 85u8,
                        ],
                    )
                }
                pub fn pricing_policy_id_by_name(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "PricingPolicyIdByName",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            112u8, 85u8, 241u8, 157u8, 144u8, 168u8, 232u8, 49u8, 184u8, 217u8,
                            195u8, 196u8, 34u8, 5u8, 148u8, 145u8, 164u8, 47u8, 173u8, 133u8, 95u8,
                            3u8, 233u8, 48u8, 122u8, 32u8, 208u8, 97u8, 193u8, 193u8, 95u8, 184u8,
                        ],
                    )
                }
                pub fn pricing_policy_id_by_name_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "PricingPolicyIdByName",
                        Vec::new(),
                        [
                            112u8, 85u8, 241u8, 157u8, 144u8, 168u8, 232u8, 49u8, 184u8, 217u8,
                            195u8, 196u8, 34u8, 5u8, 148u8, 145u8, 164u8, 47u8, 173u8, 133u8, 95u8,
                            3u8, 233u8, 48u8, 122u8, 32u8, 208u8, 97u8, 193u8, 193u8, 95u8, 184u8,
                        ],
                    )
                }
                pub fn farming_policies_map(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::FarmingPolicy<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmingPoliciesMap",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            78u8, 199u8, 0u8, 128u8, 85u8, 115u8, 173u8, 228u8, 87u8, 3u8, 79u8,
                            255u8, 210u8, 182u8, 178u8, 187u8, 141u8, 109u8, 72u8, 230u8, 15u8,
                            141u8, 91u8, 243u8, 81u8, 167u8, 131u8, 199u8, 249u8, 161u8, 105u8,
                            68u8,
                        ],
                    )
                }
                pub fn farming_policies_map_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::FarmingPolicy<::core::primitive::u32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmingPoliciesMap",
                        Vec::new(),
                        [
                            78u8, 199u8, 0u8, 128u8, 85u8, 115u8, 173u8, 228u8, 87u8, 3u8, 79u8,
                            255u8, 210u8, 182u8, 178u8, 187u8, 141u8, 109u8, 72u8, 230u8, 15u8,
                            141u8, 91u8, 243u8, 81u8, 167u8, 131u8, 199u8, 249u8, 161u8, 105u8,
                            68u8,
                        ],
                    )
                }
                pub fn users_terms_and_conditions(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::pallet_tfgrid::terms_cond::TermsAndConditions,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "UsersTermsAndConditions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            69u8, 100u8, 149u8, 195u8, 174u8, 113u8, 128u8, 155u8, 253u8, 19u8,
                            189u8, 135u8, 193u8, 87u8, 99u8, 215u8, 99u8, 20u8, 96u8, 90u8, 245u8,
                            200u8, 185u8, 254u8, 137u8, 151u8, 210u8, 83u8, 227u8, 192u8, 173u8,
                            223u8,
                        ],
                    )
                }
                pub fn users_terms_and_conditions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::pallet_tfgrid::terms_cond::TermsAndConditions,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "UsersTermsAndConditions",
                        Vec::new(),
                        [
                            69u8, 100u8, 149u8, 195u8, 174u8, 113u8, 128u8, 155u8, 253u8, 19u8,
                            189u8, 135u8, 193u8, 87u8, 99u8, 215u8, 99u8, 20u8, 96u8, 90u8, 245u8,
                            200u8, 185u8, 254u8, 137u8, 151u8, 210u8, 83u8, 227u8, 192u8, 173u8,
                            223u8,
                        ],
                    )
                }
                pub fn allowed_node_certifiers(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "AllowedNodeCertifiers",
                        vec![],
                        [
                            51u8, 79u8, 135u8, 147u8, 42u8, 232u8, 196u8, 202u8, 77u8, 150u8,
                            246u8, 246u8, 15u8, 221u8, 81u8, 1u8, 128u8, 35u8, 94u8, 22u8, 194u8,
                            227u8, 186u8, 201u8, 4u8, 5u8, 31u8, 107u8, 200u8, 116u8, 113u8, 18u8,
                        ],
                    )
                }
                pub fn connection_price(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "ConnectionPrice",
                        vec![],
                        [
                            171u8, 158u8, 221u8, 186u8, 115u8, 132u8, 189u8, 141u8, 108u8, 135u8,
                            175u8, 15u8, 241u8, 64u8, 236u8, 245u8, 108u8, 123u8, 61u8, 64u8, 65u8,
                            134u8, 252u8, 82u8, 116u8, 136u8, 241u8, 151u8, 120u8, 133u8, 53u8,
                            149u8,
                        ],
                    )
                }
                pub fn farm_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmID",
                        vec![],
                        [
                            221u8, 61u8, 38u8, 227u8, 146u8, 82u8, 250u8, 31u8, 11u8, 182u8, 10u8,
                            117u8, 159u8, 55u8, 163u8, 86u8, 179u8, 78u8, 50u8, 246u8, 166u8, 97u8,
                            211u8, 226u8, 57u8, 53u8, 208u8, 215u8, 190u8, 30u8, 64u8, 164u8,
                        ],
                    )
                }
                pub fn node_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "NodeID",
                        vec![],
                        [
                            60u8, 27u8, 18u8, 72u8, 8u8, 101u8, 211u8, 14u8, 247u8, 63u8, 74u8,
                            243u8, 176u8, 113u8, 164u8, 223u8, 155u8, 222u8, 185u8, 169u8, 213u8,
                            8u8, 191u8, 16u8, 177u8, 104u8, 132u8, 102u8, 232u8, 57u8, 150u8, 90u8,
                        ],
                    )
                }
                pub fn entity_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "EntityID",
                        vec![],
                        [
                            175u8, 215u8, 4u8, 8u8, 123u8, 151u8, 164u8, 23u8, 41u8, 208u8, 93u8,
                            233u8, 131u8, 238u8, 145u8, 32u8, 168u8, 97u8, 176u8, 15u8, 200u8,
                            49u8, 69u8, 16u8, 76u8, 221u8, 26u8, 14u8, 155u8, 18u8, 62u8, 206u8,
                        ],
                    )
                }
                pub fn twin_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "TwinID",
                        vec![],
                        [
                            22u8, 97u8, 219u8, 124u8, 18u8, 161u8, 173u8, 20u8, 72u8, 207u8, 194u8,
                            9u8, 77u8, 75u8, 101u8, 125u8, 161u8, 62u8, 17u8, 40u8, 227u8, 133u8,
                            206u8, 49u8, 248u8, 82u8, 225u8, 197u8, 118u8, 97u8, 77u8, 216u8,
                        ],
                    )
                }
                pub fn pricing_policy_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "PricingPolicyID",
                        vec![],
                        [
                            125u8, 163u8, 187u8, 159u8, 124u8, 246u8, 19u8, 81u8, 249u8, 6u8,
                            148u8, 43u8, 223u8, 229u8, 156u8, 121u8, 88u8, 245u8, 213u8, 183u8,
                            212u8, 3u8, 41u8, 88u8, 45u8, 130u8, 6u8, 24u8, 26u8, 253u8, 33u8,
                            137u8,
                        ],
                    )
                }
                pub fn farming_policy_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "FarmingPolicyID",
                        vec![],
                        [
                            147u8, 105u8, 49u8, 209u8, 189u8, 172u8, 121u8, 187u8, 214u8, 5u8,
                            22u8, 109u8, 57u8, 169u8, 112u8, 173u8, 21u8, 122u8, 106u8, 8u8, 102u8,
                            76u8, 144u8, 172u8, 75u8, 189u8, 47u8, 138u8, 121u8, 182u8, 152u8,
                            144u8,
                        ],
                    )
                }
                pub fn pallet_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tfgrid::types::StorageVersion,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "PalletVersion",
                        vec![],
                        [
                            223u8, 246u8, 92u8, 184u8, 51u8, 17u8, 26u8, 201u8, 141u8, 194u8, 27u8,
                            17u8, 67u8, 141u8, 114u8, 153u8, 172u8, 129u8, 117u8, 255u8, 103u8,
                            203u8, 48u8, 52u8, 247u8, 50u8, 53u8, 92u8, 103u8, 155u8, 40u8, 22u8,
                        ],
                    )
                }
                pub fn zos_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TfgridModule",
                        "ZosVersion",
                        vec![],
                        [
                            240u8, 203u8, 177u8, 46u8, 214u8, 89u8, 19u8, 1u8, 13u8, 44u8, 127u8,
                            77u8, 195u8, 230u8, 218u8, 189u8, 172u8, 129u8, 25u8, 89u8, 189u8,
                            220u8, 61u8, 133u8, 231u8, 185u8, 247u8, 133u8, 238u8, 178u8, 126u8,
                            179u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_farm_name_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TfgridModule",
                        "MaxFarmNameLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_farm_public_ips(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TfgridModule",
                        "MaxFarmPublicIps",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_interfaces_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TfgridModule",
                        "MaxInterfacesLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_interface_ips_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "TfgridModule",
                        "MaxInterfaceIpsLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod smart_contract_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateNodeContract {
                pub node_id: ::core::primitive::u32,
                pub deployment_hash: [::core::primitive::u8; 32usize],
                pub deployment_data:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                pub public_ips: ::core::primitive::u32,
                pub solution_provider_id: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct UpdateNodeContract {
                pub contract_id: ::core::primitive::u64,
                pub deployment_hash: [::core::primitive::u8; 32usize],
                pub deployment_data:
                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct CancelContract {
                pub contract_id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddReports {
                pub reports:
                    ::std::vec::Vec<runtime_types::pallet_smart_contract::types::Consumption>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateNameContract {
                pub name: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddNruReports {
                pub reports:
                    ::std::vec::Vec<runtime_types::pallet_smart_contract::types::NruConsumption>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReportContractResources {
                pub contract_resources:
                    ::std::vec::Vec<runtime_types::pallet_smart_contract::types::ContractResources>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateRentContract {
                pub node_id: ::core::primitive::u32,
                pub solution_provider_id: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateSolutionProvider {
                pub description: ::std::vec::Vec<::core::primitive::u8>,
                pub link: ::std::vec::Vec<::core::primitive::u8>,
                pub providers: ::std::vec::Vec<
                    runtime_types::pallet_smart_contract::types::Provider<
                        ::subxt::utils::AccountId32,
                    >,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ApproveSolutionProvider {
                pub solution_provider_id: ::core::primitive::u64,
                pub approve: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct BillContractForBlock {
                pub contract_id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ServiceContractCreate {
                pub service_account: ::subxt::utils::AccountId32,
                pub consumer_account: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ServiceContractSetMetadata {
                pub service_contract_id: ::core::primitive::u64,
                pub metadata: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ServiceContractSetFees {
                pub service_contract_id: ::core::primitive::u64,
                pub base_fee: ::core::primitive::u64,
                pub variable_fee: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ServiceContractApprove {
                pub service_contract_id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ServiceContractReject {
                pub service_contract_id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ServiceContractCancel {
                pub service_contract_id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ServiceContractBill {
                pub service_contract_id: ::core::primitive::u64,
                pub variable_amount: ::core::primitive::u64,
                pub metadata: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct ChangeBillingFrequency {
                pub frequency: ::core::primitive::u64,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create_node_contract(
                    &self,
                    node_id: ::core::primitive::u32,
                    deployment_hash: [::core::primitive::u8; 32usize],
                    deployment_data: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    public_ips: ::core::primitive::u32,
                    solution_provider_id: ::core::option::Option<::core::primitive::u64>,
                ) -> ::subxt::tx::StaticTxPayload<CreateNodeContract> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "create_node_contract",
                        CreateNodeContract {
                            node_id,
                            deployment_hash,
                            deployment_data,
                            public_ips,
                            solution_provider_id,
                        },
                        [
                            126u8, 212u8, 193u8, 30u8, 3u8, 192u8, 239u8, 228u8, 6u8, 89u8, 8u8,
                            75u8, 53u8, 168u8, 97u8, 217u8, 172u8, 96u8, 62u8, 246u8, 50u8, 222u8,
                            50u8, 243u8, 150u8, 229u8, 189u8, 198u8, 77u8, 88u8, 103u8, 2u8,
                        ],
                    )
                }
                pub fn update_node_contract(
                    &self,
                    contract_id: ::core::primitive::u64,
                    deployment_hash: [::core::primitive::u8; 32usize],
                    deployment_data: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<UpdateNodeContract> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "update_node_contract",
                        UpdateNodeContract {
                            contract_id,
                            deployment_hash,
                            deployment_data,
                        },
                        [
                            198u8, 171u8, 204u8, 204u8, 216u8, 79u8, 114u8, 255u8, 11u8, 226u8,
                            95u8, 26u8, 25u8, 201u8, 82u8, 133u8, 43u8, 155u8, 102u8, 120u8, 151u8,
                            100u8, 23u8, 7u8, 61u8, 202u8, 117u8, 36u8, 107u8, 77u8, 118u8, 128u8,
                        ],
                    )
                }
                pub fn cancel_contract(
                    &self,
                    contract_id: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<CancelContract> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "cancel_contract",
                        CancelContract { contract_id },
                        [
                            23u8, 29u8, 23u8, 207u8, 51u8, 12u8, 72u8, 91u8, 253u8, 253u8, 14u8,
                            187u8, 157u8, 27u8, 9u8, 54u8, 101u8, 111u8, 230u8, 187u8, 72u8, 126u8,
                            26u8, 177u8, 109u8, 105u8, 92u8, 119u8, 91u8, 153u8, 155u8, 254u8,
                        ],
                    )
                }
                pub fn add_reports(
                    &self,
                    reports: ::std::vec::Vec<
                        runtime_types::pallet_smart_contract::types::Consumption,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<AddReports> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "add_reports",
                        AddReports { reports },
                        [
                            238u8, 219u8, 184u8, 91u8, 82u8, 96u8, 168u8, 156u8, 212u8, 44u8, 51u8,
                            27u8, 182u8, 159u8, 45u8, 215u8, 154u8, 201u8, 119u8, 102u8, 8u8,
                            155u8, 174u8, 32u8, 97u8, 139u8, 116u8, 201u8, 94u8, 161u8, 146u8,
                            20u8,
                        ],
                    )
                }
                pub fn create_name_contract(
                    &self,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<CreateNameContract> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "create_name_contract",
                        CreateNameContract { name },
                        [
                            86u8, 232u8, 19u8, 205u8, 36u8, 40u8, 121u8, 214u8, 179u8, 134u8,
                            110u8, 47u8, 214u8, 30u8, 125u8, 118u8, 134u8, 27u8, 138u8, 203u8,
                            92u8, 78u8, 209u8, 126u8, 165u8, 246u8, 201u8, 126u8, 227u8, 150u8,
                            119u8, 204u8,
                        ],
                    )
                }
                pub fn add_nru_reports(
                    &self,
                    reports: ::std::vec::Vec<
                        runtime_types::pallet_smart_contract::types::NruConsumption,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<AddNruReports> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "add_nru_reports",
                        AddNruReports { reports },
                        [
                            234u8, 91u8, 28u8, 176u8, 158u8, 140u8, 130u8, 251u8, 142u8, 217u8,
                            237u8, 131u8, 71u8, 84u8, 9u8, 169u8, 42u8, 102u8, 19u8, 108u8, 235u8,
                            13u8, 114u8, 158u8, 1u8, 86u8, 8u8, 9u8, 38u8, 205u8, 123u8, 235u8,
                        ],
                    )
                }
                pub fn report_contract_resources(
                    &self,
                    contract_resources: ::std::vec::Vec<
                        runtime_types::pallet_smart_contract::types::ContractResources,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ReportContractResources> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "report_contract_resources",
                        ReportContractResources { contract_resources },
                        [
                            89u8, 4u8, 140u8, 36u8, 147u8, 243u8, 5u8, 212u8, 198u8, 117u8, 222u8,
                            166u8, 93u8, 207u8, 168u8, 5u8, 173u8, 37u8, 101u8, 41u8, 88u8, 26u8,
                            241u8, 154u8, 162u8, 221u8, 35u8, 151u8, 157u8, 163u8, 22u8, 149u8,
                        ],
                    )
                }
                pub fn create_rent_contract(
                    &self,
                    node_id: ::core::primitive::u32,
                    solution_provider_id: ::core::option::Option<::core::primitive::u64>,
                ) -> ::subxt::tx::StaticTxPayload<CreateRentContract> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "create_rent_contract",
                        CreateRentContract {
                            node_id,
                            solution_provider_id,
                        },
                        [
                            65u8, 233u8, 148u8, 68u8, 20u8, 255u8, 41u8, 19u8, 192u8, 238u8, 249u8,
                            228u8, 223u8, 24u8, 1u8, 165u8, 180u8, 154u8, 2u8, 67u8, 162u8, 35u8,
                            214u8, 78u8, 103u8, 78u8, 205u8, 187u8, 209u8, 185u8, 112u8, 85u8,
                        ],
                    )
                }
                pub fn create_solution_provider(
                    &self,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                    link: ::std::vec::Vec<::core::primitive::u8>,
                    providers: ::std::vec::Vec<
                        runtime_types::pallet_smart_contract::types::Provider<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                ) -> ::subxt::tx::StaticTxPayload<CreateSolutionProvider> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "create_solution_provider",
                        CreateSolutionProvider {
                            description,
                            link,
                            providers,
                        },
                        [
                            113u8, 170u8, 220u8, 180u8, 190u8, 185u8, 63u8, 96u8, 130u8, 223u8,
                            138u8, 236u8, 177u8, 99u8, 90u8, 215u8, 64u8, 60u8, 0u8, 122u8, 234u8,
                            179u8, 68u8, 92u8, 248u8, 208u8, 104u8, 202u8, 178u8, 96u8, 91u8,
                            129u8,
                        ],
                    )
                }
                pub fn approve_solution_provider(
                    &self,
                    solution_provider_id: ::core::primitive::u64,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<ApproveSolutionProvider> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "approve_solution_provider",
                        ApproveSolutionProvider {
                            solution_provider_id,
                            approve,
                        },
                        [
                            55u8, 80u8, 110u8, 43u8, 214u8, 169u8, 43u8, 211u8, 224u8, 96u8, 244u8,
                            247u8, 30u8, 230u8, 122u8, 232u8, 134u8, 123u8, 119u8, 213u8, 244u8,
                            76u8, 19u8, 122u8, 143u8, 141u8, 51u8, 75u8, 201u8, 55u8, 16u8, 29u8,
                        ],
                    )
                }
                pub fn bill_contract_for_block(
                    &self,
                    contract_id: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<BillContractForBlock> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "bill_contract_for_block",
                        BillContractForBlock { contract_id },
                        [
                            83u8, 242u8, 199u8, 62u8, 211u8, 19u8, 65u8, 219u8, 202u8, 239u8,
                            107u8, 136u8, 28u8, 66u8, 157u8, 51u8, 74u8, 229u8, 186u8, 158u8,
                            160u8, 98u8, 200u8, 231u8, 158u8, 77u8, 64u8, 129u8, 29u8, 8u8, 241u8,
                            100u8,
                        ],
                    )
                }
                pub fn service_contract_create(
                    &self,
                    service_account: ::subxt::utils::AccountId32,
                    consumer_account: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractCreate> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_create",
                        ServiceContractCreate {
                            service_account,
                            consumer_account,
                        },
                        [
                            177u8, 47u8, 29u8, 18u8, 32u8, 57u8, 49u8, 170u8, 212u8, 1u8, 94u8,
                            38u8, 22u8, 104u8, 126u8, 126u8, 84u8, 57u8, 233u8, 73u8, 31u8, 182u8,
                            104u8, 200u8, 13u8, 184u8, 40u8, 154u8, 146u8, 183u8, 223u8, 247u8,
                        ],
                    )
                }
                pub fn service_contract_set_metadata(
                    &self,
                    service_contract_id: ::core::primitive::u64,
                    metadata: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractSetMetadata> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_set_metadata",
                        ServiceContractSetMetadata {
                            service_contract_id,
                            metadata,
                        },
                        [
                            221u8, 59u8, 41u8, 154u8, 166u8, 52u8, 223u8, 34u8, 233u8, 6u8, 59u8,
                            9u8, 223u8, 29u8, 136u8, 33u8, 212u8, 122u8, 217u8, 194u8, 112u8,
                            214u8, 4u8, 37u8, 201u8, 141u8, 205u8, 52u8, 226u8, 114u8, 19u8, 220u8,
                        ],
                    )
                }
                pub fn service_contract_set_fees(
                    &self,
                    service_contract_id: ::core::primitive::u64,
                    base_fee: ::core::primitive::u64,
                    variable_fee: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractSetFees> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_set_fees",
                        ServiceContractSetFees {
                            service_contract_id,
                            base_fee,
                            variable_fee,
                        },
                        [
                            132u8, 29u8, 46u8, 108u8, 44u8, 22u8, 84u8, 39u8, 62u8, 163u8, 88u8,
                            93u8, 49u8, 163u8, 201u8, 149u8, 217u8, 46u8, 24u8, 34u8, 223u8, 253u8,
                            59u8, 223u8, 68u8, 223u8, 125u8, 160u8, 160u8, 120u8, 77u8, 210u8,
                        ],
                    )
                }
                pub fn service_contract_approve(
                    &self,
                    service_contract_id: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractApprove> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_approve",
                        ServiceContractApprove {
                            service_contract_id,
                        },
                        [
                            106u8, 130u8, 16u8, 176u8, 148u8, 157u8, 134u8, 209u8, 198u8, 239u8,
                            56u8, 61u8, 209u8, 125u8, 174u8, 24u8, 242u8, 189u8, 22u8, 84u8, 191u8,
                            196u8, 245u8, 241u8, 97u8, 72u8, 185u8, 96u8, 36u8, 54u8, 121u8, 221u8,
                        ],
                    )
                }
                pub fn service_contract_reject(
                    &self,
                    service_contract_id: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractReject> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_reject",
                        ServiceContractReject {
                            service_contract_id,
                        },
                        [
                            4u8, 199u8, 107u8, 39u8, 222u8, 59u8, 65u8, 241u8, 195u8, 4u8, 147u8,
                            122u8, 252u8, 203u8, 151u8, 196u8, 13u8, 238u8, 73u8, 155u8, 254u8,
                            20u8, 40u8, 93u8, 33u8, 8u8, 197u8, 156u8, 172u8, 102u8, 113u8, 76u8,
                        ],
                    )
                }
                pub fn service_contract_cancel(
                    &self,
                    service_contract_id: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractCancel> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_cancel",
                        ServiceContractCancel {
                            service_contract_id,
                        },
                        [
                            146u8, 233u8, 152u8, 89u8, 49u8, 72u8, 64u8, 225u8, 93u8, 185u8, 175u8,
                            88u8, 78u8, 237u8, 96u8, 14u8, 86u8, 45u8, 241u8, 88u8, 45u8, 20u8,
                            240u8, 237u8, 210u8, 16u8, 71u8, 224u8, 26u8, 133u8, 2u8, 121u8,
                        ],
                    )
                }
                pub fn service_contract_bill(
                    &self,
                    service_contract_id: ::core::primitive::u64,
                    variable_amount: ::core::primitive::u64,
                    metadata: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<ServiceContractBill> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "service_contract_bill",
                        ServiceContractBill {
                            service_contract_id,
                            variable_amount,
                            metadata,
                        },
                        [
                            110u8, 248u8, 104u8, 158u8, 70u8, 196u8, 97u8, 194u8, 42u8, 215u8,
                            92u8, 94u8, 65u8, 109u8, 166u8, 148u8, 233u8, 108u8, 252u8, 165u8,
                            235u8, 84u8, 33u8, 19u8, 19u8, 71u8, 47u8, 210u8, 52u8, 120u8, 222u8,
                            216u8,
                        ],
                    )
                }
                pub fn change_billing_frequency(
                    &self,
                    frequency: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ChangeBillingFrequency> {
                    ::subxt::tx::StaticTxPayload::new(
                        "SmartContractModule",
                        "change_billing_frequency",
                        ChangeBillingFrequency { frequency },
                        [
                            133u8, 190u8, 190u8, 27u8, 119u8, 117u8, 70u8, 207u8, 20u8, 132u8,
                            142u8, 111u8, 64u8, 198u8, 28u8, 24u8, 150u8, 166u8, 229u8, 75u8,
                            242u8, 12u8, 4u8, 70u8, 73u8, 229u8, 100u8, 182u8, 38u8, 132u8, 129u8,
                            19u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_smart_contract::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A contract got created"]
            pub struct ContractCreated(pub runtime_types::pallet_smart_contract::types::Contract);
            impl ::subxt::events::StaticEvent for ContractCreated {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ContractCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A contract was updated"]
            pub struct ContractUpdated(pub runtime_types::pallet_smart_contract::types::Contract);
            impl ::subxt::events::StaticEvent for ContractUpdated {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ContractUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Node contract is canceled"]
            pub struct NodeContractCanceled {
                pub contract_id: ::core::primitive::u64,
                pub node_id: ::core::primitive::u32,
                pub twin_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for NodeContractCanceled {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "NodeContractCanceled";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "A Name contract is canceled"]
            pub struct NameContractCanceled {
                pub contract_id: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for NameContractCanceled {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "NameContractCanceled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "IP got reserved by a Node contract"]
            pub struct IPsReserved {
                pub contract_id: ::core::primitive::u64,
                pub public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                    runtime_types::tfchain_support::types::PublicIP,
                >,
            }
            impl ::subxt::events::StaticEvent for IPsReserved {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "IPsReserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "IP got freed by a Node contract"]
            pub struct IPsFreed {
                pub contract_id: ::core::primitive::u64,
                pub public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                    runtime_types::tfchain_support::types::PublicIP,
                >,
            }
            impl ::subxt::events::StaticEvent for IPsFreed {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "IPsFreed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Deprecated event"]
            pub struct ContractDeployed(
                pub ::core::primitive::u64,
                pub ::subxt::utils::AccountId32,
            );
            impl ::subxt::events::StaticEvent for ContractDeployed {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ContractDeployed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Deprecated event"]
            pub struct ConsumptionReportReceived(
                pub runtime_types::pallet_smart_contract::types::Consumption,
            );
            impl ::subxt::events::StaticEvent for ConsumptionReportReceived {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ConsumptionReportReceived";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ContractBilled(
                pub runtime_types::pallet_smart_contract::types::ContractBill,
            );
            impl ::subxt::events::StaticEvent for ContractBilled {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ContractBilled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A certain amount of tokens got burned by a contract"]
            pub struct TokensBurned {
                pub contract_id: ::core::primitive::u64,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TokensBurned {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "TokensBurned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Contract resources got updated"]
            pub struct UpdatedUsedResources(
                pub runtime_types::pallet_smart_contract::types::ContractResources,
            );
            impl ::subxt::events::StaticEvent for UpdatedUsedResources {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "UpdatedUsedResources";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Network resources report received for contract"]
            pub struct NruConsumptionReportReceived(
                pub runtime_types::pallet_smart_contract::types::NruConsumption,
            );
            impl ::subxt::events::StaticEvent for NruConsumptionReportReceived {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "NruConsumptionReportReceived";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            #[doc = "a Rent contract is canceled"]
            pub struct RentContractCanceled {
                pub contract_id: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for RentContractCanceled {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "RentContractCanceled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Contract grace period is triggered"]
            pub struct ContractGracePeriodStarted {
                pub contract_id: ::core::primitive::u64,
                pub node_id: ::core::primitive::u32,
                pub twin_id: ::core::primitive::u32,
                pub block_number: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for ContractGracePeriodStarted {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ContractGracePeriodStarted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Contract grace period was ended"]
            pub struct ContractGracePeriodEnded {
                pub contract_id: ::core::primitive::u64,
                pub node_id: ::core::primitive::u32,
                pub twin_id: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for ContractGracePeriodEnded {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ContractGracePeriodEnded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SolutionProviderCreated(
                pub  runtime_types::pallet_smart_contract::types::SolutionProvider<
                    ::subxt::utils::AccountId32,
                >,
            );
            impl ::subxt::events::StaticEvent for SolutionProviderCreated {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "SolutionProviderCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SolutionProviderApproved(
                pub ::core::primitive::u64,
                pub ::core::primitive::bool,
            );
            impl ::subxt::events::StaticEvent for SolutionProviderApproved {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "SolutionProviderApproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Service contract is created"]
            pub struct ServiceContractCreated(
                pub runtime_types::pallet_smart_contract::types::ServiceContract,
            );
            impl ::subxt::events::StaticEvent for ServiceContractCreated {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ServiceContractCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Service contract metadata is set"]
            pub struct ServiceContractMetadataSet(
                pub runtime_types::pallet_smart_contract::types::ServiceContract,
            );
            impl ::subxt::events::StaticEvent for ServiceContractMetadataSet {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ServiceContractMetadataSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Service contract fees are set"]
            pub struct ServiceContractFeesSet(
                pub runtime_types::pallet_smart_contract::types::ServiceContract,
            );
            impl ::subxt::events::StaticEvent for ServiceContractFeesSet {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ServiceContractFeesSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Service contract is approved"]
            pub struct ServiceContractApproved(
                pub runtime_types::pallet_smart_contract::types::ServiceContract,
            );
            impl ::subxt::events::StaticEvent for ServiceContractApproved {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ServiceContractApproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Service contract is canceled"]
            pub struct ServiceContractCanceled {
                pub service_contract_id: ::core::primitive::u64,
                pub cause: runtime_types::pallet_smart_contract::types::Cause,
            }
            impl ::subxt::events::StaticEvent for ServiceContractCanceled {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ServiceContractCanceled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A Service contract is billed"]
            pub struct ServiceContractBilled {
                pub service_contract: runtime_types::pallet_smart_contract::types::ServiceContract,
                pub bill: runtime_types::pallet_smart_contract::types::ServiceContractBill,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for ServiceContractBilled {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "ServiceContractBilled";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct BillingFrequencyChanged(pub ::core::primitive::u64);
            impl ::subxt::events::StaticEvent for BillingFrequencyChanged {
                const PALLET: &'static str = "SmartContractModule";
                const EVENT: &'static str = "BillingFrequencyChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn contracts(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::Contract,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "Contracts",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            124u8, 50u8, 198u8, 204u8, 107u8, 74u8, 148u8, 66u8, 32u8, 210u8,
                            125u8, 247u8, 79u8, 117u8, 66u8, 12u8, 237u8, 7u8, 102u8, 53u8, 125u8,
                            221u8, 199u8, 29u8, 101u8, 192u8, 135u8, 0u8, 132u8, 123u8, 25u8,
                            108u8,
                        ],
                    )
                }
                pub fn contracts_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::Contract,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "Contracts",
                        Vec::new(),
                        [
                            124u8, 50u8, 198u8, 204u8, 107u8, 74u8, 148u8, 66u8, 32u8, 210u8,
                            125u8, 247u8, 79u8, 117u8, 66u8, 12u8, 237u8, 7u8, 102u8, 53u8, 125u8,
                            221u8, 199u8, 29u8, 101u8, 192u8, 135u8, 0u8, 132u8, 123u8, 25u8,
                            108u8,
                        ],
                    )
                }
                pub fn contract_billing_information_by_id(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ContractBillingInformation,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractBillingInformationByID",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            224u8, 39u8, 81u8, 201u8, 115u8, 83u8, 195u8, 77u8, 48u8, 11u8, 76u8,
                            87u8, 140u8, 164u8, 155u8, 127u8, 222u8, 5u8, 23u8, 104u8, 180u8,
                            144u8, 140u8, 29u8, 6u8, 239u8, 208u8, 232u8, 139u8, 26u8, 149u8,
                            175u8,
                        ],
                    )
                }
                pub fn contract_billing_information_by_id_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ContractBillingInformation,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractBillingInformationByID",
                        Vec::new(),
                        [
                            224u8, 39u8, 81u8, 201u8, 115u8, 83u8, 195u8, 77u8, 48u8, 11u8, 76u8,
                            87u8, 140u8, 164u8, 155u8, 127u8, 222u8, 5u8, 23u8, 104u8, 180u8,
                            144u8, 140u8, 29u8, 6u8, 239u8, 208u8, 232u8, 139u8, 26u8, 149u8,
                            175u8,
                        ],
                    )
                }
                pub fn node_contract_resources(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ContractResources,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "NodeContractResources",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            173u8, 5u8, 80u8, 165u8, 13u8, 247u8, 191u8, 131u8, 80u8, 135u8, 21u8,
                            179u8, 255u8, 63u8, 224u8, 128u8, 242u8, 188u8, 252u8, 97u8, 143u8,
                            60u8, 10u8, 65u8, 222u8, 91u8, 199u8, 224u8, 164u8, 71u8, 167u8, 71u8,
                        ],
                    )
                }
                pub fn node_contract_resources_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ContractResources,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "NodeContractResources",
                        Vec::new(),
                        [
                            173u8, 5u8, 80u8, 165u8, 13u8, 247u8, 191u8, 131u8, 80u8, 135u8, 21u8,
                            179u8, 255u8, 63u8, 224u8, 128u8, 242u8, 188u8, 252u8, 97u8, 143u8,
                            60u8, 10u8, 65u8, 222u8, 91u8, 199u8, 224u8, 164u8, 71u8, 167u8, 71u8,
                        ],
                    )
                }
                pub fn contract_id_by_node_id_and_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractIDByNodeIDAndHash",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            35u8, 159u8, 208u8, 55u8, 173u8, 220u8, 70u8, 228u8, 208u8, 207u8,
                            81u8, 19u8, 161u8, 206u8, 202u8, 82u8, 25u8, 196u8, 254u8, 150u8,
                            151u8, 231u8, 37u8, 121u8, 206u8, 114u8, 153u8, 227u8, 165u8, 127u8,
                            154u8, 82u8,
                        ],
                    )
                }
                pub fn contract_id_by_node_id_and_hash_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractIDByNodeIDAndHash",
                        Vec::new(),
                        [
                            35u8, 159u8, 208u8, 55u8, 173u8, 220u8, 70u8, 228u8, 208u8, 207u8,
                            81u8, 19u8, 161u8, 206u8, 202u8, 82u8, 25u8, 196u8, 254u8, 150u8,
                            151u8, 231u8, 37u8, 121u8, 206u8, 114u8, 153u8, 227u8, 165u8, 127u8,
                            154u8, 82u8,
                        ],
                    )
                }
                pub fn active_node_contracts(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u64>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ActiveNodeContracts",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            106u8, 236u8, 6u8, 216u8, 52u8, 106u8, 27u8, 255u8, 56u8, 100u8, 167u8,
                            185u8, 102u8, 6u8, 26u8, 119u8, 77u8, 78u8, 45u8, 156u8, 23u8, 36u8,
                            45u8, 120u8, 106u8, 7u8, 238u8, 6u8, 241u8, 236u8, 237u8, 192u8,
                        ],
                    )
                }
                pub fn active_node_contracts_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u64>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ActiveNodeContracts",
                        Vec::new(),
                        [
                            106u8, 236u8, 6u8, 216u8, 52u8, 106u8, 27u8, 255u8, 56u8, 100u8, 167u8,
                            185u8, 102u8, 6u8, 26u8, 119u8, 77u8, 78u8, 45u8, 156u8, 23u8, 36u8,
                            45u8, 120u8, 106u8, 7u8, 238u8, 6u8, 241u8, 236u8, 237u8, 192u8,
                        ],
                    )
                }
                pub fn contracts_to_bill_at(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u64>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractsToBillAt",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            246u8, 9u8, 240u8, 238u8, 225u8, 84u8, 249u8, 16u8, 236u8, 230u8, 23u8,
                            195u8, 8u8, 126u8, 206u8, 63u8, 31u8, 217u8, 80u8, 217u8, 76u8, 160u8,
                            151u8, 148u8, 171u8, 232u8, 204u8, 246u8, 187u8, 76u8, 236u8, 173u8,
                        ],
                    )
                }
                pub fn contracts_to_bill_at_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u64>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractsToBillAt",
                        Vec::new(),
                        [
                            246u8, 9u8, 240u8, 238u8, 225u8, 84u8, 249u8, 16u8, 236u8, 230u8, 23u8,
                            195u8, 8u8, 126u8, 206u8, 63u8, 31u8, 217u8, 80u8, 217u8, 76u8, 160u8,
                            151u8, 148u8, 171u8, 232u8, 204u8, 246u8, 187u8, 76u8, 236u8, 173u8,
                        ],
                    )
                }
                pub fn contract_lock(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ContractLock<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractLock",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            195u8, 212u8, 107u8, 150u8, 30u8, 15u8, 70u8, 215u8, 49u8, 251u8,
                            102u8, 32u8, 86u8, 221u8, 13u8, 217u8, 79u8, 124u8, 64u8, 8u8, 1u8,
                            74u8, 198u8, 222u8, 114u8, 69u8, 48u8, 17u8, 134u8, 103u8, 76u8, 98u8,
                        ],
                    )
                }
                pub fn contract_lock_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ContractLock<
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractLock",
                        Vec::new(),
                        [
                            195u8, 212u8, 107u8, 150u8, 30u8, 15u8, 70u8, 215u8, 49u8, 251u8,
                            102u8, 32u8, 86u8, 221u8, 13u8, 217u8, 79u8, 124u8, 64u8, 8u8, 1u8,
                            74u8, 198u8, 222u8, 114u8, 69u8, 48u8, 17u8, 134u8, 103u8, 76u8, 98u8,
                        ],
                    )
                }
                pub fn contract_id_by_name_registration(
                    &self,
                    _0: impl ::std::borrow::Borrow<
                        runtime_types::pallet_smart_contract::name_contract::NameContractName,
                    >,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractIDByNameRegistration",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            80u8, 144u8, 131u8, 184u8, 255u8, 205u8, 52u8, 236u8, 76u8, 249u8,
                            102u8, 132u8, 143u8, 216u8, 195u8, 173u8, 167u8, 30u8, 174u8, 9u8,
                            163u8, 59u8, 72u8, 11u8, 198u8, 210u8, 180u8, 157u8, 79u8, 36u8, 187u8,
                            234u8,
                        ],
                    )
                }
                pub fn contract_id_by_name_registration_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractIDByNameRegistration",
                        Vec::new(),
                        [
                            80u8, 144u8, 131u8, 184u8, 255u8, 205u8, 52u8, 236u8, 76u8, 249u8,
                            102u8, 132u8, 143u8, 216u8, 195u8, 173u8, 167u8, 30u8, 174u8, 9u8,
                            163u8, 59u8, 72u8, 11u8, 198u8, 210u8, 180u8, 157u8, 79u8, 36u8, 187u8,
                            234u8,
                        ],
                    )
                }
                pub fn active_rent_contract_for_node(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ActiveRentContractForNode",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            13u8, 41u8, 41u8, 241u8, 33u8, 34u8, 95u8, 253u8, 236u8, 100u8, 85u8,
                            237u8, 53u8, 129u8, 240u8, 73u8, 193u8, 133u8, 225u8, 216u8, 9u8, 18u8,
                            230u8, 163u8, 116u8, 141u8, 134u8, 37u8, 173u8, 62u8, 111u8, 151u8,
                        ],
                    )
                }
                pub fn active_rent_contract_for_node_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ActiveRentContractForNode",
                        Vec::new(),
                        [
                            13u8, 41u8, 41u8, 241u8, 33u8, 34u8, 95u8, 253u8, 236u8, 100u8, 85u8,
                            237u8, 53u8, 129u8, 240u8, 73u8, 193u8, 133u8, 225u8, 216u8, 9u8, 18u8,
                            230u8, 163u8, 116u8, 141u8, 134u8, 37u8, 173u8, 62u8, 111u8, 151u8,
                        ],
                    )
                }
                pub fn contract_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ContractID",
                        vec![],
                        [
                            208u8, 233u8, 20u8, 113u8, 86u8, 178u8, 20u8, 58u8, 240u8, 244u8, 67u8,
                            25u8, 92u8, 139u8, 32u8, 242u8, 231u8, 169u8, 129u8, 115u8, 233u8,
                            67u8, 29u8, 209u8, 139u8, 101u8, 158u8, 208u8, 54u8, 163u8, 180u8, 0u8,
                        ],
                    )
                }
                pub fn solution_providers(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::SolutionProvider<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "SolutionProviders",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            168u8, 67u8, 73u8, 103u8, 217u8, 87u8, 36u8, 168u8, 7u8, 35u8, 170u8,
                            191u8, 190u8, 218u8, 145u8, 118u8, 39u8, 158u8, 172u8, 176u8, 54u8,
                            114u8, 195u8, 170u8, 177u8, 243u8, 69u8, 153u8, 187u8, 33u8, 89u8,
                            22u8,
                        ],
                    )
                }
                pub fn solution_providers_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::SolutionProvider<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "SolutionProviders",
                        Vec::new(),
                        [
                            168u8, 67u8, 73u8, 103u8, 217u8, 87u8, 36u8, 168u8, 7u8, 35u8, 170u8,
                            191u8, 190u8, 218u8, 145u8, 118u8, 39u8, 158u8, 172u8, 176u8, 54u8,
                            114u8, 195u8, 170u8, 177u8, 243u8, 69u8, 153u8, 187u8, 33u8, 89u8,
                            22u8,
                        ],
                    )
                }
                pub fn solution_provider_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "SolutionProviderID",
                        vec![],
                        [
                            7u8, 233u8, 99u8, 11u8, 180u8, 59u8, 43u8, 239u8, 35u8, 1u8, 253u8,
                            210u8, 174u8, 202u8, 218u8, 24u8, 14u8, 192u8, 121u8, 74u8, 93u8, 21u8,
                            252u8, 76u8, 207u8, 221u8, 117u8, 190u8, 92u8, 182u8, 89u8, 172u8,
                        ],
                    )
                }
                pub fn pallet_version(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::StorageVersion,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "PalletVersion",
                        vec![],
                        [
                            14u8, 137u8, 170u8, 8u8, 240u8, 72u8, 11u8, 100u8, 85u8, 96u8, 104u8,
                            122u8, 187u8, 150u8, 2u8, 137u8, 90u8, 138u8, 140u8, 173u8, 230u8,
                            188u8, 96u8, 51u8, 92u8, 50u8, 219u8, 196u8, 202u8, 8u8, 85u8, 215u8,
                        ],
                    )
                }
                pub fn billing_frequency(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "BillingFrequency",
                        vec![],
                        [
                            251u8, 143u8, 50u8, 202u8, 200u8, 247u8, 179u8, 157u8, 41u8, 16u8,
                            241u8, 79u8, 213u8, 239u8, 8u8, 214u8, 208u8, 75u8, 194u8, 110u8, 89u8,
                            56u8, 23u8, 213u8, 217u8, 41u8, 36u8, 50u8, 52u8, 89u8, 58u8, 124u8,
                        ],
                    )
                }
                pub fn service_contracts(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ServiceContract,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ServiceContracts",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            243u8, 79u8, 129u8, 217u8, 225u8, 122u8, 120u8, 230u8, 60u8, 80u8, 9u8,
                            143u8, 8u8, 159u8, 6u8, 239u8, 86u8, 158u8, 81u8, 81u8, 94u8, 4u8,
                            142u8, 197u8, 24u8, 193u8, 192u8, 212u8, 28u8, 121u8, 227u8, 226u8,
                        ],
                    )
                }
                pub fn service_contracts_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_smart_contract::types::ServiceContract,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ServiceContracts",
                        Vec::new(),
                        [
                            243u8, 79u8, 129u8, 217u8, 225u8, 122u8, 120u8, 230u8, 60u8, 80u8, 9u8,
                            143u8, 8u8, 159u8, 6u8, 239u8, 86u8, 158u8, 81u8, 81u8, 94u8, 4u8,
                            142u8, 197u8, 24u8, 193u8, 192u8, 212u8, 28u8, 121u8, 227u8, 226u8,
                        ],
                    )
                }
                pub fn service_contract_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "SmartContractModule",
                        "ServiceContractID",
                        vec![],
                        [
                            91u8, 250u8, 71u8, 222u8, 199u8, 108u8, 68u8, 141u8, 216u8, 160u8,
                            45u8, 97u8, 36u8, 225u8, 207u8, 173u8, 79u8, 162u8, 1u8, 186u8, 114u8,
                            43u8, 239u8, 44u8, 178u8, 97u8, 124u8, 183u8, 155u8, 158u8, 201u8,
                            167u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_name_contract_name_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "SmartContractModule",
                        "MaxNameContractNameLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_deployment_data_length(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "SmartContractModule",
                        "MaxDeploymentDataLength",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_node_contract_public_ips(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "SmartContractModule",
                        "MaxNodeContractPublicIps",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod tft_bridge_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddBridgeValidator {
                pub target: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveBridgeValidator {
                pub target: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetFeeAccount {
                pub target: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetWithdrawFee {
                pub amount: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetDepositFee {
                pub amount: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SwapToStellar {
                pub target_stellar_address: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ProposeOrVoteMintTransaction {
                pub transaction: ::std::vec::Vec<::core::primitive::u8>,
                pub target: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ProposeBurnTransactionOrAddSig {
                pub transaction_id: ::core::primitive::u64,
                pub target: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u64,
                pub signature: ::std::vec::Vec<::core::primitive::u8>,
                pub stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub sequence_number: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetBurnTransactionExecuted {
                pub transaction_id: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateRefundTransactionOrAddSig {
                pub tx_hash: ::std::vec::Vec<::core::primitive::u8>,
                pub target: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u64,
                pub signature: ::std::vec::Vec<::core::primitive::u8>,
                pub stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                pub sequence_number: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetRefundTransactionExecuted {
                pub tx_hash: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn add_bridge_validator(
                    &self,
                    target: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<AddBridgeValidator> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "add_bridge_validator",
                        AddBridgeValidator { target },
                        [
                            98u8, 0u8, 185u8, 221u8, 51u8, 177u8, 98u8, 165u8, 21u8, 255u8, 82u8,
                            168u8, 128u8, 197u8, 92u8, 29u8, 59u8, 248u8, 103u8, 112u8, 215u8,
                            245u8, 37u8, 203u8, 186u8, 35u8, 23u8, 129u8, 237u8, 102u8, 11u8, 80u8,
                        ],
                    )
                }
                pub fn remove_bridge_validator(
                    &self,
                    target: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<RemoveBridgeValidator> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "remove_bridge_validator",
                        RemoveBridgeValidator { target },
                        [
                            158u8, 159u8, 183u8, 164u8, 170u8, 203u8, 83u8, 81u8, 14u8, 85u8,
                            165u8, 238u8, 59u8, 145u8, 138u8, 33u8, 91u8, 224u8, 198u8, 245u8,
                            140u8, 77u8, 47u8, 91u8, 243u8, 84u8, 18u8, 188u8, 67u8, 67u8, 74u8,
                            173u8,
                        ],
                    )
                }
                pub fn set_fee_account(
                    &self,
                    target: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<SetFeeAccount> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "set_fee_account",
                        SetFeeAccount { target },
                        [
                            91u8, 201u8, 185u8, 107u8, 230u8, 9u8, 114u8, 83u8, 240u8, 182u8,
                            221u8, 204u8, 170u8, 182u8, 225u8, 50u8, 73u8, 55u8, 97u8, 156u8,
                            109u8, 1u8, 215u8, 178u8, 251u8, 69u8, 166u8, 3u8, 145u8, 29u8, 14u8,
                            212u8,
                        ],
                    )
                }
                pub fn set_withdraw_fee(
                    &self,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetWithdrawFee> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "set_withdraw_fee",
                        SetWithdrawFee { amount },
                        [
                            37u8, 189u8, 224u8, 244u8, 68u8, 147u8, 245u8, 150u8, 75u8, 250u8, 4u8,
                            105u8, 38u8, 49u8, 96u8, 101u8, 65u8, 220u8, 87u8, 73u8, 203u8, 128u8,
                            21u8, 212u8, 45u8, 199u8, 48u8, 140u8, 195u8, 20u8, 95u8, 109u8,
                        ],
                    )
                }
                pub fn set_deposit_fee(
                    &self,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetDepositFee> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "set_deposit_fee",
                        SetDepositFee { amount },
                        [
                            226u8, 133u8, 30u8, 160u8, 175u8, 128u8, 162u8, 16u8, 145u8, 60u8, 3u8,
                            46u8, 122u8, 23u8, 212u8, 9u8, 2u8, 124u8, 122u8, 14u8, 247u8, 25u8,
                            161u8, 104u8, 237u8, 127u8, 15u8, 85u8, 152u8, 223u8, 165u8, 13u8,
                        ],
                    )
                }
                pub fn swap_to_stellar(
                    &self,
                    target_stellar_address: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::StaticTxPayload<SwapToStellar> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "swap_to_stellar",
                        SwapToStellar {
                            target_stellar_address,
                            amount,
                        },
                        [
                            108u8, 127u8, 19u8, 206u8, 17u8, 142u8, 0u8, 4u8, 117u8, 50u8, 254u8,
                            117u8, 249u8, 62u8, 121u8, 33u8, 74u8, 49u8, 255u8, 146u8, 5u8, 129u8,
                            95u8, 119u8, 218u8, 11u8, 63u8, 216u8, 11u8, 57u8, 119u8, 204u8,
                        ],
                    )
                }
                pub fn propose_or_vote_mint_transaction(
                    &self,
                    transaction: ::std::vec::Vec<::core::primitive::u8>,
                    target: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ProposeOrVoteMintTransaction> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "propose_or_vote_mint_transaction",
                        ProposeOrVoteMintTransaction {
                            transaction,
                            target,
                            amount,
                        },
                        [
                            35u8, 15u8, 243u8, 104u8, 52u8, 185u8, 186u8, 93u8, 177u8, 62u8, 91u8,
                            112u8, 16u8, 252u8, 157u8, 64u8, 180u8, 161u8, 232u8, 187u8, 190u8,
                            231u8, 25u8, 214u8, 248u8, 188u8, 32u8, 243u8, 175u8, 111u8, 221u8,
                            92u8,
                        ],
                    )
                }
                pub fn propose_burn_transaction_or_add_sig(
                    &self,
                    transaction_id: ::core::primitive::u64,
                    target: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u64,
                    signature: ::std::vec::Vec<::core::primitive::u8>,
                    stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                    sequence_number: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<ProposeBurnTransactionOrAddSig> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "propose_burn_transaction_or_add_sig",
                        ProposeBurnTransactionOrAddSig {
                            transaction_id,
                            target,
                            amount,
                            signature,
                            stellar_pub_key,
                            sequence_number,
                        },
                        [
                            158u8, 35u8, 207u8, 242u8, 86u8, 227u8, 116u8, 216u8, 100u8, 151u8,
                            241u8, 25u8, 89u8, 244u8, 48u8, 0u8, 252u8, 192u8, 15u8, 6u8, 221u8,
                            79u8, 233u8, 147u8, 83u8, 202u8, 138u8, 190u8, 160u8, 171u8, 2u8,
                            121u8,
                        ],
                    )
                }
                pub fn set_burn_transaction_executed(
                    &self,
                    transaction_id: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<SetBurnTransactionExecuted> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "set_burn_transaction_executed",
                        SetBurnTransactionExecuted { transaction_id },
                        [
                            135u8, 164u8, 194u8, 148u8, 176u8, 5u8, 136u8, 73u8, 95u8, 239u8,
                            154u8, 194u8, 35u8, 209u8, 113u8, 12u8, 152u8, 201u8, 98u8, 66u8,
                            212u8, 191u8, 125u8, 199u8, 132u8, 146u8, 77u8, 255u8, 112u8, 64u8,
                            205u8, 34u8,
                        ],
                    )
                }
                pub fn create_refund_transaction_or_add_sig(
                    &self,
                    tx_hash: ::std::vec::Vec<::core::primitive::u8>,
                    target: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u64,
                    signature: ::std::vec::Vec<::core::primitive::u8>,
                    stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                    sequence_number: ::core::primitive::u64,
                ) -> ::subxt::tx::StaticTxPayload<CreateRefundTransactionOrAddSig> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "create_refund_transaction_or_add_sig",
                        CreateRefundTransactionOrAddSig {
                            tx_hash,
                            target,
                            amount,
                            signature,
                            stellar_pub_key,
                            sequence_number,
                        },
                        [
                            173u8, 100u8, 59u8, 109u8, 136u8, 28u8, 213u8, 177u8, 207u8, 205u8,
                            73u8, 117u8, 169u8, 206u8, 127u8, 253u8, 151u8, 176u8, 5u8, 87u8,
                            146u8, 51u8, 247u8, 10u8, 244u8, 92u8, 53u8, 230u8, 38u8, 198u8, 143u8,
                            134u8,
                        ],
                    )
                }
                pub fn set_refund_transaction_executed(
                    &self,
                    tx_hash: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetRefundTransactionExecuted> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTBridgeModule",
                        "set_refund_transaction_executed",
                        SetRefundTransactionExecuted { tx_hash },
                        [
                            93u8, 120u8, 88u8, 105u8, 70u8, 76u8, 239u8, 9u8, 239u8, 125u8, 221u8,
                            104u8, 133u8, 184u8, 103u8, 90u8, 102u8, 155u8, 97u8, 234u8, 147u8,
                            164u8, 89u8, 5u8, 19u8, 115u8, 166u8, 9u8, 232u8, 162u8, 142u8, 117u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_tft_bridge::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MintTransactionProposed(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::utils::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for MintTransactionProposed {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "MintTransactionProposed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MintTransactionVoted(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::events::StaticEvent for MintTransactionVoted {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "MintTransactionVoted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MintCompleted(
                pub  runtime_types::pallet_tft_bridge::MintTransaction<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u32,
                >,
            );
            impl ::subxt::events::StaticEvent for MintCompleted {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "MintCompleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MintTransactionExpired(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u64,
                pub ::subxt::utils::AccountId32,
            );
            impl ::subxt::events::StaticEvent for MintTransactionExpired {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "MintTransactionExpired";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransactionCreated(
                pub ::core::primitive::u64,
                pub ::subxt::utils::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for BurnTransactionCreated {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "BurnTransactionCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransactionProposed(
                pub ::core::primitive::u64,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for BurnTransactionProposed {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "BurnTransactionProposed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransactionSignatureAdded(
                pub ::core::primitive::u64,
                pub runtime_types::pallet_tft_bridge::StellarSignature,
            );
            impl ::subxt::events::StaticEvent for BurnTransactionSignatureAdded {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "BurnTransactionSignatureAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct BurnTransactionReady(pub ::core::primitive::u64);
            impl ::subxt::events::StaticEvent for BurnTransactionReady {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "BurnTransactionReady";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransactionProcessed(
                pub runtime_types::pallet_tft_bridge::BurnTransaction<::core::primitive::u32>,
            );
            impl ::subxt::events::StaticEvent for BurnTransactionProcessed {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "BurnTransactionProcessed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransactionExpired(
                pub ::core::primitive::u64,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for BurnTransactionExpired {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "BurnTransactionExpired";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RefundTransactionCreated(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for RefundTransactionCreated {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "RefundTransactionCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RefundTransactionsignatureAdded(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub runtime_types::pallet_tft_bridge::StellarSignature,
            );
            impl ::subxt::events::StaticEvent for RefundTransactionsignatureAdded {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "RefundTransactionsignatureAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RefundTransactionReady(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::events::StaticEvent for RefundTransactionReady {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "RefundTransactionReady";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RefundTransactionProcessed(
                pub runtime_types::pallet_tft_bridge::RefundTransaction<::core::primitive::u32>,
            );
            impl ::subxt::events::StaticEvent for RefundTransactionProcessed {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "RefundTransactionProcessed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RefundTransactionExpired(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u64,
            );
            impl ::subxt::events::StaticEvent for RefundTransactionExpired {
                const PALLET: &'static str = "TFTBridgeModule";
                const EVENT: &'static str = "RefundTransactionExpired";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "Validators",
                        vec![],
                        [
                            144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
                            242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
                            32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
                            68u8,
                        ],
                    )
                }
                pub fn fee_account(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "FeeAccount",
                        vec![],
                        [
                            29u8, 48u8, 115u8, 145u8, 233u8, 245u8, 153u8, 101u8, 115u8, 4u8, 96u8,
                            139u8, 244u8, 56u8, 104u8, 45u8, 4u8, 97u8, 82u8, 201u8, 209u8, 135u8,
                            4u8, 12u8, 122u8, 138u8, 33u8, 126u8, 92u8, 228u8, 86u8, 161u8,
                        ],
                    )
                }
                pub fn mint_transactions(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::MintTransaction<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "MintTransactions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            145u8, 61u8, 109u8, 253u8, 41u8, 100u8, 204u8, 178u8, 159u8, 88u8,
                            132u8, 48u8, 110u8, 28u8, 84u8, 116u8, 102u8, 149u8, 193u8, 242u8,
                            176u8, 36u8, 22u8, 152u8, 83u8, 206u8, 214u8, 88u8, 243u8, 118u8, 81u8,
                            1u8,
                        ],
                    )
                }
                pub fn mint_transactions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::MintTransaction<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "MintTransactions",
                        Vec::new(),
                        [
                            145u8, 61u8, 109u8, 253u8, 41u8, 100u8, 204u8, 178u8, 159u8, 88u8,
                            132u8, 48u8, 110u8, 28u8, 84u8, 116u8, 102u8, 149u8, 193u8, 242u8,
                            176u8, 36u8, 22u8, 152u8, 83u8, 206u8, 214u8, 88u8, 243u8, 118u8, 81u8,
                            1u8,
                        ],
                    )
                }
                pub fn executed_mint_transactions(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::MintTransaction<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "ExecutedMintTransactions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            35u8, 253u8, 158u8, 181u8, 77u8, 254u8, 253u8, 181u8, 29u8, 237u8,
                            12u8, 178u8, 128u8, 219u8, 30u8, 155u8, 1u8, 244u8, 95u8, 226u8, 202u8,
                            132u8, 30u8, 35u8, 35u8, 228u8, 178u8, 54u8, 214u8, 143u8, 23u8, 197u8,
                        ],
                    )
                }
                pub fn executed_mint_transactions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::MintTransaction<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "ExecutedMintTransactions",
                        Vec::new(),
                        [
                            35u8, 253u8, 158u8, 181u8, 77u8, 254u8, 253u8, 181u8, 29u8, 237u8,
                            12u8, 178u8, 128u8, 219u8, 30u8, 155u8, 1u8, 244u8, 95u8, 226u8, 202u8,
                            132u8, 30u8, 35u8, 35u8, 228u8, 178u8, 54u8, 214u8, 143u8, 23u8, 197u8,
                        ],
                    )
                }
                pub fn burn_transactions(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::BurnTransaction<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "BurnTransactions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            73u8, 200u8, 232u8, 89u8, 93u8, 163u8, 10u8, 123u8, 234u8, 164u8, 20u8,
                            65u8, 107u8, 197u8, 128u8, 95u8, 241u8, 181u8, 103u8, 36u8, 24u8,
                            191u8, 130u8, 15u8, 87u8, 117u8, 186u8, 67u8, 146u8, 196u8, 180u8,
                            111u8,
                        ],
                    )
                }
                pub fn burn_transactions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::BurnTransaction<::core::primitive::u32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "BurnTransactions",
                        Vec::new(),
                        [
                            73u8, 200u8, 232u8, 89u8, 93u8, 163u8, 10u8, 123u8, 234u8, 164u8, 20u8,
                            65u8, 107u8, 197u8, 128u8, 95u8, 241u8, 181u8, 103u8, 36u8, 24u8,
                            191u8, 130u8, 15u8, 87u8, 117u8, 186u8, 67u8, 146u8, 196u8, 180u8,
                            111u8,
                        ],
                    )
                }
                pub fn executed_burn_transactions(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::BurnTransaction<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "ExecutedBurnTransactions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            18u8, 175u8, 173u8, 52u8, 245u8, 255u8, 93u8, 208u8, 137u8, 40u8,
                            175u8, 96u8, 134u8, 72u8, 233u8, 78u8, 140u8, 121u8, 111u8, 10u8, 3u8,
                            18u8, 116u8, 37u8, 219u8, 143u8, 86u8, 92u8, 178u8, 84u8, 236u8, 126u8,
                        ],
                    )
                }
                pub fn executed_burn_transactions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::BurnTransaction<::core::primitive::u32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "ExecutedBurnTransactions",
                        Vec::new(),
                        [
                            18u8, 175u8, 173u8, 52u8, 245u8, 255u8, 93u8, 208u8, 137u8, 40u8,
                            175u8, 96u8, 134u8, 72u8, 233u8, 78u8, 140u8, 121u8, 111u8, 10u8, 3u8,
                            18u8, 116u8, 37u8, 219u8, 143u8, 86u8, 92u8, 178u8, 84u8, 236u8, 126u8,
                        ],
                    )
                }
                pub fn refund_transactions(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::RefundTransaction<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "RefundTransactions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            214u8, 110u8, 196u8, 161u8, 115u8, 193u8, 149u8, 120u8, 97u8, 59u8,
                            123u8, 34u8, 32u8, 16u8, 129u8, 196u8, 120u8, 47u8, 199u8, 52u8, 94u8,
                            15u8, 29u8, 89u8, 198u8, 212u8, 46u8, 141u8, 117u8, 141u8, 212u8,
                            207u8,
                        ],
                    )
                }
                pub fn refund_transactions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::RefundTransaction<::core::primitive::u32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "RefundTransactions",
                        Vec::new(),
                        [
                            214u8, 110u8, 196u8, 161u8, 115u8, 193u8, 149u8, 120u8, 97u8, 59u8,
                            123u8, 34u8, 32u8, 16u8, 129u8, 196u8, 120u8, 47u8, 199u8, 52u8, 94u8,
                            15u8, 29u8, 89u8, 198u8, 212u8, 46u8, 141u8, 117u8, 141u8, 212u8,
                            207u8,
                        ],
                    )
                }
                pub fn executed_refund_transactions(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::RefundTransaction<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "ExecutedRefundTransactions",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            241u8, 167u8, 177u8, 136u8, 184u8, 129u8, 101u8, 68u8, 79u8, 65u8,
                            91u8, 105u8, 120u8, 165u8, 124u8, 10u8, 27u8, 106u8, 117u8, 206u8,
                            219u8, 31u8, 124u8, 13u8, 59u8, 32u8, 233u8, 167u8, 218u8, 213u8,
                            147u8, 46u8,
                        ],
                    )
                }
                pub fn executed_refund_transactions_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_tft_bridge::RefundTransaction<::core::primitive::u32>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "ExecutedRefundTransactions",
                        Vec::new(),
                        [
                            241u8, 167u8, 177u8, 136u8, 184u8, 129u8, 101u8, 68u8, 79u8, 65u8,
                            91u8, 105u8, 120u8, 165u8, 124u8, 10u8, 27u8, 106u8, 117u8, 206u8,
                            219u8, 31u8, 124u8, 13u8, 59u8, 32u8, 233u8, 167u8, 218u8, 213u8,
                            147u8, 46u8,
                        ],
                    )
                }
                pub fn burn_transaction_id(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "BurnTransactionID",
                        vec![],
                        [
                            107u8, 118u8, 179u8, 190u8, 80u8, 47u8, 164u8, 1u8, 84u8, 34u8, 163u8,
                            48u8, 227u8, 198u8, 231u8, 23u8, 165u8, 179u8, 111u8, 247u8, 78u8,
                            236u8, 48u8, 109u8, 160u8, 207u8, 109u8, 221u8, 8u8, 237u8, 37u8,
                            112u8,
                        ],
                    )
                }
                pub fn withdraw_fee(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "WithdrawFee",
                        vec![],
                        [
                            165u8, 127u8, 174u8, 207u8, 73u8, 3u8, 204u8, 255u8, 107u8, 52u8, 75u8,
                            24u8, 17u8, 193u8, 232u8, 89u8, 43u8, 100u8, 52u8, 72u8, 60u8, 147u8,
                            122u8, 231u8, 236u8, 82u8, 186u8, 150u8, 25u8, 49u8, 178u8, 225u8,
                        ],
                    )
                }
                pub fn deposit_fee(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTBridgeModule",
                        "DepositFee",
                        vec![],
                        [
                            117u8, 41u8, 165u8, 137u8, 110u8, 69u8, 129u8, 134u8, 245u8, 103u8,
                            166u8, 234u8, 36u8, 244u8, 146u8, 223u8, 220u8, 186u8, 93u8, 164u8,
                            228u8, 124u8, 244u8, 1u8, 213u8, 146u8, 149u8, 133u8, 54u8, 51u8, 37u8,
                            11u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod tft_price_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetPrices {
                pub price: ::core::primitive::u32,
                pub block_number: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetMinTftPrice {
                pub price: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct SetMaxTftPrice {
                pub price: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set_prices(
                    &self,
                    price: ::core::primitive::u32,
                    block_number: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetPrices> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTPriceModule",
                        "set_prices",
                        SetPrices {
                            price,
                            block_number,
                        },
                        [
                            96u8, 173u8, 157u8, 128u8, 15u8, 13u8, 131u8, 19u8, 93u8, 255u8, 49u8,
                            107u8, 227u8, 218u8, 26u8, 26u8, 145u8, 116u8, 193u8, 52u8, 138u8,
                            142u8, 187u8, 27u8, 156u8, 133u8, 203u8, 223u8, 54u8, 132u8, 26u8,
                            165u8,
                        ],
                    )
                }
                pub fn set_min_tft_price(
                    &self,
                    price: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetMinTftPrice> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTPriceModule",
                        "set_min_tft_price",
                        SetMinTftPrice { price },
                        [
                            7u8, 8u8, 201u8, 124u8, 58u8, 186u8, 161u8, 56u8, 44u8, 7u8, 57u8,
                            219u8, 148u8, 5u8, 215u8, 63u8, 193u8, 8u8, 49u8, 147u8, 200u8, 25u8,
                            46u8, 59u8, 197u8, 157u8, 99u8, 145u8, 175u8, 63u8, 29u8, 224u8,
                        ],
                    )
                }
                pub fn set_max_tft_price(
                    &self,
                    price: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetMaxTftPrice> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFTPriceModule",
                        "set_max_tft_price",
                        SetMaxTftPrice { price },
                        [
                            85u8, 72u8, 178u8, 154u8, 171u8, 74u8, 102u8, 130u8, 205u8, 142u8,
                            216u8, 88u8, 211u8, 163u8, 150u8, 129u8, 35u8, 104u8, 57u8, 117u8,
                            241u8, 45u8, 198u8, 28u8, 24u8, 93u8, 239u8, 98u8, 125u8, 213u8, 241u8,
                            250u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_tft_price::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct PriceStored(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for PriceStored {
                const PALLET: &'static str = "TFTPriceModule";
                const EVENT: &'static str = "PriceStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct OffchainWorkerExecuted(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for OffchainWorkerExecuted {
                const PALLET: &'static str = "TFTPriceModule";
                const EVENT: &'static str = "OffchainWorkerExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct AveragePriceStored(pub ::core::primitive::u32);
            impl ::subxt::events::StaticEvent for AveragePriceStored {
                const PALLET: &'static str = "TFTPriceModule";
                const EVENT: &'static str = "AveragePriceStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AveragePriceIsAboveMaxPrice(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for AveragePriceIsAboveMaxPrice {
                const PALLET: &'static str = "TFTPriceModule";
                const EVENT: &'static str = "AveragePriceIsAboveMaxPrice";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AveragePriceIsBelowMinPrice(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::events::StaticEvent for AveragePriceIsBelowMinPrice {
                const PALLET: &'static str = "TFTPriceModule";
                const EVENT: &'static str = "AveragePriceIsBelowMinPrice";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn tft_price(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "TftPrice",
                        vec![],
                        [
                            157u8, 2u8, 212u8, 11u8, 139u8, 185u8, 199u8, 51u8, 9u8, 3u8, 234u8,
                            115u8, 121u8, 121u8, 85u8, 97u8, 113u8, 223u8, 183u8, 56u8, 0u8, 97u8,
                            251u8, 136u8, 241u8, 107u8, 25u8, 134u8, 209u8, 113u8, 95u8, 212u8,
                        ],
                    )
                }
                pub fn last_block_set(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "LastBlockSet",
                        vec![],
                        [
                            92u8, 252u8, 215u8, 136u8, 0u8, 113u8, 5u8, 44u8, 107u8, 220u8, 37u8,
                            144u8, 26u8, 95u8, 89u8, 250u8, 33u8, 5u8, 174u8, 93u8, 86u8, 173u8,
                            57u8, 169u8, 8u8, 247u8, 63u8, 25u8, 214u8, 113u8, 24u8, 87u8,
                        ],
                    )
                }
                pub fn average_tft_price(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "AverageTftPrice",
                        vec![],
                        [
                            56u8, 218u8, 204u8, 135u8, 191u8, 149u8, 19u8, 47u8, 191u8, 152u8,
                            186u8, 234u8, 83u8, 207u8, 20u8, 144u8, 194u8, 137u8, 213u8, 100u8,
                            89u8, 20u8, 136u8, 76u8, 69u8, 221u8, 91u8, 65u8, 54u8, 78u8, 228u8,
                            123u8,
                        ],
                    )
                }
                pub fn tft_price_history(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u16>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "TftPriceHistory",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                        )],
                        [
                            3u8, 178u8, 114u8, 6u8, 205u8, 220u8, 162u8, 53u8, 15u8, 193u8, 45u8,
                            194u8, 182u8, 22u8, 149u8, 168u8, 1u8, 66u8, 220u8, 120u8, 38u8, 246u8,
                            119u8, 74u8, 101u8, 208u8, 103u8, 53u8, 238u8, 185u8, 109u8, 190u8,
                        ],
                    )
                }
                pub fn tft_price_history_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "TftPriceHistory",
                        Vec::new(),
                        [
                            3u8, 178u8, 114u8, 6u8, 205u8, 220u8, 162u8, 53u8, 15u8, 193u8, 45u8,
                            194u8, 182u8, 22u8, 149u8, 168u8, 1u8, 66u8, 220u8, 120u8, 38u8, 246u8,
                            119u8, 74u8, 101u8, 208u8, 103u8, 53u8, 238u8, 185u8, 109u8, 190u8,
                        ],
                    )
                }
                pub fn buffer_range(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u16,
                        ::core::primitive::u16,
                    )>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "BufferRange",
                        vec![],
                        [
                            37u8, 151u8, 42u8, 23u8, 240u8, 62u8, 113u8, 72u8, 143u8, 39u8, 57u8,
                            55u8, 172u8, 86u8, 124u8, 12u8, 221u8, 55u8, 198u8, 197u8, 199u8, 54u8,
                            175u8, 195u8, 43u8, 166u8, 17u8, 160u8, 18u8, 58u8, 192u8, 4u8,
                        ],
                    )
                }
                pub fn min_tft_price(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "MinTftPrice",
                        vec![],
                        [
                            104u8, 18u8, 98u8, 235u8, 237u8, 7u8, 215u8, 241u8, 210u8, 50u8, 166u8,
                            84u8, 73u8, 234u8, 246u8, 197u8, 43u8, 117u8, 159u8, 85u8, 62u8, 76u8,
                            83u8, 80u8, 91u8, 38u8, 176u8, 50u8, 33u8, 85u8, 164u8, 65u8,
                        ],
                    )
                }
                pub fn max_tft_price(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFTPriceModule",
                        "MaxTftPrice",
                        vec![],
                        [
                            135u8, 169u8, 249u8, 160u8, 50u8, 96u8, 94u8, 196u8, 58u8, 123u8,
                            233u8, 182u8, 43u8, 81u8, 250u8, 110u8, 74u8, 9u8, 126u8, 215u8, 78u8,
                            93u8, 10u8, 251u8, 126u8, 32u8, 66u8, 113u8, 89u8, 67u8, 225u8, 64u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod scheduler {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduleNamed {
                pub id: [::core::primitive::u8; 32usize],
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CancelNamed {
                pub id: [::core::primitive::u8; 32usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ScheduleNamedAfter {
                pub id: [::core::primitive::u8; 32usize],
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Anonymously schedule a task."]
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<Schedule> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule",
                        Schedule {
                            when,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            66u8, 244u8, 203u8, 197u8, 214u8, 168u8, 79u8, 72u8, 225u8, 244u8,
                            137u8, 90u8, 213u8, 99u8, 49u8, 100u8, 88u8, 228u8, 171u8, 135u8,
                            109u8, 210u8, 170u8, 114u8, 194u8, 10u8, 72u8, 205u8, 88u8, 99u8,
                            125u8, 157u8,
                        ],
                    )
                }
                #[doc = "Cancel an anonymously scheduled task."]
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Cancel> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "cancel",
                        Cancel { when, index },
                        [
                            81u8, 251u8, 234u8, 17u8, 214u8, 75u8, 19u8, 59u8, 19u8, 30u8, 89u8,
                            74u8, 6u8, 216u8, 238u8, 165u8, 7u8, 19u8, 153u8, 253u8, 161u8, 103u8,
                            178u8, 227u8, 152u8, 180u8, 80u8, 156u8, 82u8, 126u8, 132u8, 120u8,
                        ],
                    )
                }
                #[doc = "Schedule a named task."]
                pub fn schedule_named(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<ScheduleNamed> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule_named",
                        ScheduleNamed {
                            id,
                            when,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            225u8, 130u8, 185u8, 168u8, 14u8, 17u8, 63u8, 230u8, 157u8, 195u8,
                            236u8, 106u8, 184u8, 187u8, 145u8, 136u8, 237u8, 154u8, 105u8, 14u8,
                            172u8, 5u8, 178u8, 151u8, 106u8, 40u8, 19u8, 205u8, 85u8, 124u8, 204u8,
                            197u8,
                        ],
                    )
                }
                #[doc = "Cancel a named scheduled task."]
                pub fn cancel_named(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                ) -> ::subxt::tx::StaticTxPayload<CancelNamed> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "cancel_named",
                        CancelNamed { id },
                        [
                            51u8, 3u8, 140u8, 50u8, 214u8, 211u8, 50u8, 4u8, 19u8, 43u8, 230u8,
                            114u8, 18u8, 108u8, 138u8, 67u8, 99u8, 24u8, 255u8, 11u8, 246u8, 37u8,
                            192u8, 207u8, 90u8, 157u8, 171u8, 93u8, 233u8, 189u8, 64u8, 180u8,
                        ],
                    )
                }
                #[doc = "Anonymously schedule a task after a delay."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`schedule`]."]
                #[doc = "# </weight>"]
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<ScheduleAfter> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule_after",
                        ScheduleAfter {
                            after,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            198u8, 140u8, 103u8, 124u8, 85u8, 226u8, 139u8, 177u8, 248u8, 158u8,
                            45u8, 82u8, 250u8, 112u8, 178u8, 169u8, 34u8, 251u8, 90u8, 39u8, 62u8,
                            161u8, 12u8, 14u8, 138u8, 56u8, 9u8, 82u8, 109u8, 22u8, 9u8, 242u8,
                        ],
                    )
                }
                #[doc = "Schedule a named task after a delay."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Same as [`schedule_named`](Self::schedule_named)."]
                #[doc = "# </weight>"]
                pub fn schedule_named_after(
                    &self,
                    id: [::core::primitive::u8; 32usize],
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<ScheduleNamedAfter> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Scheduler",
                        "schedule_named_after",
                        ScheduleNamedAfter {
                            id,
                            after,
                            maybe_periodic,
                            priority,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            209u8, 136u8, 96u8, 241u8, 114u8, 138u8, 18u8, 225u8, 70u8, 127u8,
                            89u8, 198u8, 165u8, 110u8, 88u8, 251u8, 99u8, 131u8, 11u8, 2u8, 188u8,
                            89u8, 93u8, 209u8, 116u8, 198u8, 120u8, 91u8, 134u8, 167u8, 212u8,
                            147u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Events type."]
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Scheduled some task."]
            pub struct Scheduled {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Canceled some task."]
            pub struct Canceled {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Dispatched some task."]
            pub struct Dispatched {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The call for the provided hash was not found so the task has been aborted."]
            pub struct CallUnavailable {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for CallUnavailable {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "CallUnavailable";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The given task was unable to be renewed since the agenda is full at that block."]
            pub struct PeriodicFailed {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for PeriodicFailed {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "PeriodicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The given task can never be executed since it is overweight."]
            pub struct PermanentlyOverweight {
                pub task: (::core::primitive::u32, ::core::primitive::u32),
                pub id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
            }
            impl ::subxt::events::StaticEvent for PermanentlyOverweight {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "PermanentlyOverweight";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn incomplete_since(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "IncompleteSince",
                        vec![],
                        [
                            149u8, 66u8, 239u8, 67u8, 235u8, 219u8, 101u8, 182u8, 145u8, 56u8,
                            252u8, 150u8, 253u8, 221u8, 125u8, 57u8, 38u8, 152u8, 153u8, 31u8,
                            92u8, 238u8, 66u8, 246u8, 104u8, 163u8, 94u8, 73u8, 222u8, 168u8,
                            193u8, 227u8,
                        ],
                    )
                }
                #[doc = " Items to be executed, indexed by the block number that they should be executed on."]
                pub fn agenda(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::option::Option<
                                runtime_types::pallet_scheduler::Scheduled<
                                    [::core::primitive::u8; 32usize],
                                    runtime_types::frame_support::traits::preimages::Bounded<
                                        runtime_types::tfchain_runtime::RuntimeCall,
                                    >,
                                    ::core::primitive::u32,
                                    runtime_types::tfchain_runtime::OriginCaller,
                                    ::subxt::utils::AccountId32,
                                >,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Agenda",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            77u8, 74u8, 230u8, 122u8, 183u8, 192u8, 24u8, 80u8, 140u8, 147u8, 26u8,
                            72u8, 134u8, 148u8, 90u8, 69u8, 72u8, 100u8, 81u8, 179u8, 236u8, 142u8,
                            37u8, 210u8, 106u8, 225u8, 12u8, 76u8, 127u8, 140u8, 178u8, 86u8,
                        ],
                    )
                }
                #[doc = " Items to be executed, indexed by the block number that they should be executed on."]
                pub fn agenda_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::option::Option<
                                runtime_types::pallet_scheduler::Scheduled<
                                    [::core::primitive::u8; 32usize],
                                    runtime_types::frame_support::traits::preimages::Bounded<
                                        runtime_types::tfchain_runtime::RuntimeCall,
                                    >,
                                    ::core::primitive::u32,
                                    runtime_types::tfchain_runtime::OriginCaller,
                                    ::subxt::utils::AccountId32,
                                >,
                            >,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Agenda",
                        Vec::new(),
                        [
                            77u8, 74u8, 230u8, 122u8, 183u8, 192u8, 24u8, 80u8, 140u8, 147u8, 26u8,
                            72u8, 134u8, 148u8, 90u8, 69u8, 72u8, 100u8, 81u8, 179u8, 236u8, 142u8,
                            37u8, 210u8, 106u8, 225u8, 12u8, 76u8, 127u8, 140u8, 178u8, 86u8,
                        ],
                    )
                }
                #[doc = " Lookup from a name to the block number and index of the task."]
                #[doc = ""]
                #[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
                #[doc = " identities."]
                pub fn lookup(
                    &self,
                    _0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Lookup",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
                            187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
                            121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
                            243u8,
                        ],
                    )
                }
                #[doc = " Lookup from a name to the block number and index of the task."]
                #[doc = ""]
                #[doc = " For v3 -> v4 the previously unbounded identities are Blake2-256 hashed to form the v4"]
                #[doc = " identities."]
                pub fn lookup_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Scheduler",
                        "Lookup",
                        Vec::new(),
                        [
                            82u8, 20u8, 178u8, 101u8, 108u8, 198u8, 71u8, 99u8, 16u8, 175u8, 15u8,
                            187u8, 229u8, 243u8, 140u8, 200u8, 99u8, 77u8, 248u8, 178u8, 45u8,
                            121u8, 193u8, 67u8, 165u8, 43u8, 234u8, 211u8, 158u8, 250u8, 103u8,
                            243u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The maximum weight that may be scheduled per block for any dispatchables."]
                pub fn maximum_weight(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Scheduler",
                        "MaximumWeight",
                        [
                            206u8, 61u8, 253u8, 247u8, 163u8, 40u8, 161u8, 52u8, 134u8, 140u8,
                            206u8, 83u8, 44u8, 166u8, 226u8, 115u8, 181u8, 14u8, 227u8, 130u8,
                            210u8, 32u8, 85u8, 29u8, 230u8, 97u8, 130u8, 165u8, 147u8, 134u8,
                            106u8, 76u8,
                        ],
                    )
                }
                #[doc = " The maximum number of scheduled calls in the queue for a single block."]
                pub fn max_scheduled_per_block(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Scheduler",
                        "MaxScheduledPerBlock",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod burning_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTft {
                pub amount: ::core::primitive::u128,
                pub message: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn burn_tft(
                    &self,
                    amount: ::core::primitive::u128,
                    message: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<BurnTft> {
                    ::subxt::tx::StaticTxPayload::new(
                        "BurningModule",
                        "burn_tft",
                        BurnTft { amount, message },
                        [
                            128u8, 21u8, 176u8, 159u8, 155u8, 223u8, 67u8, 123u8, 204u8, 180u8,
                            247u8, 20u8, 23u8, 147u8, 210u8, 170u8, 93u8, 25u8, 120u8, 199u8,
                            118u8, 118u8, 192u8, 96u8, 137u8, 238u8, 88u8, 144u8, 200u8, 99u8,
                            216u8, 243u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_burning::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransactionCreated(
                pub ::subxt::utils::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::events::StaticEvent for BurnTransactionCreated {
                const PALLET: &'static str = "BurningModule";
                const EVENT: &'static str = "BurnTransactionCreated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn burns(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<
                            runtime_types::pallet_burning::Burn<
                                ::subxt::utils::AccountId32,
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "BurningModule",
                        "Burns",
                        vec![],
                        [
                            13u8, 220u8, 110u8, 16u8, 241u8, 150u8, 168u8, 56u8, 121u8, 19u8,
                            121u8, 185u8, 86u8, 223u8, 147u8, 227u8, 247u8, 208u8, 167u8, 12u8,
                            1u8, 10u8, 89u8, 217u8, 141u8, 185u8, 217u8, 160u8, 215u8, 81u8, 211u8,
                            82u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod tfkv_store {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Set {
                pub key: ::std::vec::Vec<::core::primitive::u8>,
                pub value: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Delete {
                pub key: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the value stored at a particular key"]
                pub fn set(
                    &self,
                    key: ::std::vec::Vec<::core::primitive::u8>,
                    value: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<Set> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFKVStore",
                        "set",
                        Set { key, value },
                        [
                            130u8, 49u8, 106u8, 205u8, 45u8, 116u8, 19u8, 132u8, 120u8, 22u8, 26u8,
                            95u8, 67u8, 250u8, 174u8, 34u8, 124u8, 199u8, 83u8, 114u8, 100u8,
                            212u8, 125u8, 153u8, 37u8, 189u8, 101u8, 233u8, 50u8, 216u8, 36u8,
                            149u8,
                        ],
                    )
                }
                #[doc = "Read the value stored at a particular key, while removing it from the map."]
                #[doc = "Also emit the read value in an event"]
                pub fn delete(
                    &self,
                    key: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<Delete> {
                    ::subxt::tx::StaticTxPayload::new(
                        "TFKVStore",
                        "delete",
                        Delete { key },
                        [
                            19u8, 223u8, 19u8, 204u8, 49u8, 18u8, 125u8, 236u8, 64u8, 214u8, 39u8,
                            60u8, 156u8, 116u8, 23u8, 224u8, 20u8, 104u8, 119u8, 43u8, 18u8, 45u8,
                            203u8, 11u8, 97u8, 192u8, 104u8, 204u8, 153u8, 24u8, 46u8, 112u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_kvstore::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A user has set their entry"]
            pub struct EntrySet(
                pub ::subxt::utils::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::events::StaticEvent for EntrySet {
                const PALLET: &'static str = "TFKVStore";
                const EVENT: &'static str = "EntrySet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A user has read their entry, leaving it in storage"]
            pub struct EntryGot(
                pub ::subxt::utils::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::events::StaticEvent for EntryGot {
                const PALLET: &'static str = "TFKVStore";
                const EVENT: &'static str = "EntryGot";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A user has read their entry, removing it from storage"]
            pub struct EntryTaken(
                pub ::subxt::utils::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::events::StaticEvent for EntryTaken {
                const PALLET: &'static str = "TFKVStore";
                const EVENT: &'static str = "EntryTaken";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn tfkv_store(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFKVStore",
                        "TFKVStore",
                        vec![
                            ::subxt::storage::address::StorageMapKey::new(
                                _0.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                            ::subxt::storage::address::StorageMapKey::new(
                                _1.borrow(),
                                ::subxt::storage::address::StorageHasher::Blake2_128Concat,
                            ),
                        ],
                        [
                            129u8, 167u8, 246u8, 238u8, 119u8, 78u8, 77u8, 149u8, 191u8, 55u8,
                            53u8, 80u8, 132u8, 23u8, 165u8, 203u8, 87u8, 82u8, 43u8, 216u8, 111u8,
                            178u8, 94u8, 134u8, 3u8, 51u8, 210u8, 1u8, 226u8, 21u8, 173u8, 218u8,
                        ],
                    )
                }
                pub fn tfkv_store_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "TFKVStore",
                        "TFKVStore",
                        Vec::new(),
                        [
                            129u8, 167u8, 246u8, 238u8, 119u8, 78u8, 77u8, 149u8, 191u8, 55u8,
                            53u8, 80u8, 132u8, 23u8, 165u8, 203u8, 87u8, 82u8, 43u8, 216u8, 111u8,
                            178u8, 94u8, 134u8, 3u8, 51u8, 210u8, 1u8, 226u8, 21u8, 173u8, 218u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod council {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetMembers {
                pub new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                pub prime: ::core::option::Option<::subxt::utils::AccountId32>,
                pub old_count: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Execute {
                pub proposal: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub proposal: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Vote {
                pub proposal: ::subxt::utils::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub approve: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CloseOldWeight {
                pub proposal_hash: ::subxt::utils::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub proposal_weight_bound: runtime_types::sp_weights::OldWeight,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DisapproveProposal {
                pub proposal_hash: ::subxt::utils::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Close {
                pub proposal_hash: ::subxt::utils::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the collective's membership."]
                #[doc = ""]
                #[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
                #[doc = "- `prime`: The prime member whose vote sets the default."]
                #[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
                #[doc = "  weight estimation."]
                #[doc = ""]
                #[doc = "Requires root origin."]
                #[doc = ""]
                #[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
                #[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
                #[doc = ""]
                #[doc = "# WARNING:"]
                #[doc = ""]
                #[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
                #[doc = "implementation of the trait [`ChangeMembers`]."]
                #[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
                #[doc = "with other logic managing the member set."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(MP + N)` where:"]
                #[doc = "  - `M` old-members-count (code- and governance-bounded)"]
                #[doc = "  - `N` new-members-count (code- and governance-bounded)"]
                #[doc = "  - `P` proposals-count (code-bounded)"]
                #[doc = "- DB:"]
                #[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
                #[doc = "    members"]
                #[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
                #[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
                #[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
                #[doc = "# </weight>"]
                pub fn set_members(
                    &self,
                    new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    prime: ::core::option::Option<::subxt::utils::AccountId32>,
                    old_count: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<SetMembers> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "set_members",
                        SetMembers {
                            new_members,
                            prime,
                            old_count,
                        },
                        [
                            196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
                            34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
                            166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
                            234u8,
                        ],
                    )
                }
                #[doc = "Dispatch a proposal from a member using the `Member` origin."]
                #[doc = ""]
                #[doc = "Origin must be a member of the collective."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
                #[doc = "  `proposal`"]
                #[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
                #[doc = "- 1 event"]
                #[doc = "# </weight>"]
                pub fn execute(
                    &self,
                    proposal: runtime_types::tfchain_runtime::RuntimeCall,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Execute> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "execute",
                        Execute {
                            proposal: ::std::boxed::Box::new(proposal),
                            length_bound,
                        },
                        [
                            157u8, 199u8, 250u8, 178u8, 207u8, 131u8, 120u8, 96u8, 171u8, 113u8,
                            185u8, 17u8, 35u8, 168u8, 120u8, 39u8, 38u8, 245u8, 17u8, 15u8, 50u8,
                            170u8, 35u8, 80u8, 64u8, 176u8, 128u8, 146u8, 42u8, 210u8, 58u8, 153u8,
                        ],
                    )
                }
                #[doc = "Add a new proposal to either be voted on or executed directly."]
                #[doc = ""]
                #[doc = "Requires the sender to be member."]
                #[doc = ""]
                #[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
                #[doc = "or put up for voting."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
                #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                #[doc = "  - branching is influenced by `threshold` where:"]
                #[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
                #[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
                #[doc = "- DB:"]
                #[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
                #[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
                #[doc = "  - DB accesses influenced by `threshold`:"]
                #[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
                #[doc = "    - OR proposal insertion (`threshold <= 2`)"]
                #[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
                #[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
                #[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
                #[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
                #[doc = "  - 1 event"]
                #[doc = "# </weight>"]
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    proposal: runtime_types::tfchain_runtime::RuntimeCall,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Propose> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "propose",
                        Propose {
                            threshold,
                            proposal: ::std::boxed::Box::new(proposal),
                            length_bound,
                        },
                        [
                            250u8, 36u8, 149u8, 123u8, 64u8, 35u8, 89u8, 71u8, 151u8, 25u8, 46u8,
                            210u8, 194u8, 33u8, 53u8, 139u8, 155u8, 171u8, 233u8, 177u8, 245u8,
                            150u8, 212u8, 232u8, 131u8, 57u8, 46u8, 216u8, 210u8, 110u8, 9u8,
                            188u8,
                        ],
                    )
                }
                #[doc = "Add an aye or nay vote for the sender to the given proposal."]
                #[doc = ""]
                #[doc = "Requires the sender to be a member."]
                #[doc = ""]
                #[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
                #[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
                #[doc = "fee."]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
                #[doc = "- DB:"]
                #[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
                #[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
                #[doc = "- 1 event"]
                #[doc = "# </weight>"]
                pub fn vote(
                    &self,
                    proposal: ::subxt::utils::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<Vote> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "vote",
                        Vote {
                            proposal,
                            index,
                            approve,
                        },
                        [
                            108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
                            216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
                            42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
                        ],
                    )
                }
                #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                #[doc = ""]
                #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                #[doc = ""]
                #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                #[doc = "has enough votes to be approved or disapproved."]
                #[doc = ""]
                #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                #[doc = "unless there is a prime member set and the prime member cast an approval."]
                #[doc = ""]
                #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                #[doc = ""]
                #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                #[doc = "proposal."]
                #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(B + M + P1 + P2)` where:"]
                #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                #[doc = "  - `P2` is proposal-count (code-bounded)"]
                #[doc = "- DB:"]
                #[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
                #[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
                #[doc = "   `O(P2)`)"]
                #[doc = " - any mutations done while executing `proposal` (`P1`)"]
                #[doc = "- up to 3 events"]
                #[doc = "# </weight>"]
                pub fn close_old_weight(
                    &self,
                    proposal_hash: ::subxt::utils::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: runtime_types::sp_weights::OldWeight,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<CloseOldWeight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "close_old_weight",
                        CloseOldWeight {
                            proposal_hash,
                            index,
                            proposal_weight_bound,
                            length_bound,
                        },
                        [
                            133u8, 219u8, 90u8, 40u8, 102u8, 95u8, 4u8, 199u8, 45u8, 234u8, 109u8,
                            17u8, 162u8, 63u8, 102u8, 186u8, 95u8, 182u8, 13u8, 123u8, 227u8, 20u8,
                            186u8, 207u8, 12u8, 47u8, 87u8, 252u8, 244u8, 172u8, 60u8, 206u8,
                        ],
                    )
                }
                #[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
                #[doc = "state."]
                #[doc = ""]
                #[doc = "Must be called by the Root origin."]
                #[doc = ""]
                #[doc = "Parameters:"]
                #[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "Complexity: O(P) where P is the number of max proposals"]
                #[doc = "DB Weight:"]
                #[doc = "* Reads: Proposals"]
                #[doc = "* Writes: Voting, Proposals, ProposalOf"]
                #[doc = "# </weight>"]
                pub fn disapprove_proposal(
                    &self,
                    proposal_hash: ::subxt::utils::H256,
                ) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "disapprove_proposal",
                        DisapproveProposal { proposal_hash },
                        [
                            25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
                            72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
                            225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
                        ],
                    )
                }
                #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                #[doc = ""]
                #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                #[doc = ""]
                #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                #[doc = "has enough votes to be approved or disapproved."]
                #[doc = ""]
                #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                #[doc = "unless there is a prime member set and the prime member cast an approval."]
                #[doc = ""]
                #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                #[doc = ""]
                #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                #[doc = "proposal."]
                #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "## Weight"]
                #[doc = "- `O(B + M + P1 + P2)` where:"]
                #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                #[doc = "  - `P2` is proposal-count (code-bounded)"]
                #[doc = "- DB:"]
                #[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
                #[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
                #[doc = "   `O(P2)`)"]
                #[doc = " - any mutations done while executing `proposal` (`P1`)"]
                #[doc = "- up to 3 events"]
                #[doc = "# </weight>"]
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::utils::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Close> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Council",
                        "close",
                        Close {
                            proposal_hash,
                            index,
                            proposal_weight_bound,
                            length_bound,
                        },
                        [
                            191u8, 138u8, 89u8, 247u8, 97u8, 51u8, 45u8, 193u8, 76u8, 16u8, 80u8,
                            225u8, 197u8, 83u8, 204u8, 133u8, 169u8, 16u8, 86u8, 32u8, 125u8, 16u8,
                            116u8, 185u8, 45u8, 20u8, 76u8, 148u8, 206u8, 163u8, 154u8, 30u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_collective::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
            #[doc = "`MemberCount`)."]
            pub struct Proposed {
                pub account: ::subxt::utils::AccountId32,
                pub proposal_index: ::core::primitive::u32,
                pub proposal_hash: ::subxt::utils::H256,
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Proposed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion (given hash) has been voted on by given account, leaving"]
            #[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
            pub struct Voted {
                pub account: ::subxt::utils::AccountId32,
                pub proposal_hash: ::subxt::utils::H256,
                pub voted: ::core::primitive::bool,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Voted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Voted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was approved by the required threshold."]
            pub struct Approved {
                pub proposal_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Approved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Approved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was not approved by the required threshold."]
            pub struct Disapproved {
                pub proposal_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Disapproved {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
            pub struct Executed {
                pub proposal_hash: ::subxt::utils::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Executed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Executed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A single member did some action; result will be `Ok` if it returned without error."]
            pub struct MemberExecuted {
                pub proposal_hash: ::subxt::utils::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for MemberExecuted {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
            pub struct Closed {
                pub proposal_hash: ::subxt::utils::H256,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Closed {
                const PALLET: &'static str = "Council";
                const EVENT: &'static str = "Closed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The hashes of the active proposals."]
                pub fn proposals(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::subxt::utils::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "Proposals",
                        vec![],
                        [
                            10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
                            148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
                            60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
                            27u8,
                        ],
                    )
                }
                #[doc = " Actual proposal for a given hash, if it's current."]
                pub fn proposal_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_runtime::RuntimeCall,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "ProposalOf",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            94u8, 129u8, 207u8, 84u8, 243u8, 225u8, 239u8, 23u8, 126u8, 237u8,
                            103u8, 1u8, 1u8, 98u8, 103u8, 142u8, 213u8, 98u8, 165u8, 197u8, 74u8,
                            20u8, 136u8, 142u8, 56u8, 91u8, 170u8, 144u8, 99u8, 220u8, 13u8, 171u8,
                        ],
                    )
                }
                #[doc = " Actual proposal for a given hash, if it's current."]
                pub fn proposal_of_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_runtime::RuntimeCall,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "ProposalOf",
                        Vec::new(),
                        [
                            94u8, 129u8, 207u8, 84u8, 243u8, 225u8, 239u8, 23u8, 126u8, 237u8,
                            103u8, 1u8, 1u8, 98u8, 103u8, 142u8, 213u8, 98u8, 165u8, 197u8, 74u8,
                            20u8, 136u8, 142u8, 56u8, 91u8, 170u8, 144u8, 99u8, 220u8, 13u8, 171u8,
                        ],
                    )
                }
                #[doc = " Votes on a given proposal, if it is ongoing."]
                pub fn voting(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "Voting",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
                            168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
                            136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
                            221u8,
                        ],
                    )
                }
                #[doc = " Votes on a given proposal, if it is ongoing."]
                pub fn voting_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_collective::Votes<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "Voting",
                        Vec::new(),
                        [
                            89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
                            168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
                            136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
                            221u8,
                        ],
                    )
                }
                #[doc = " Proposals so far."]
                pub fn proposal_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "ProposalCount",
                        vec![],
                        [
                            132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
                            140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
                            24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
                            70u8,
                        ],
                    )
                }
                #[doc = " The current members of the collective. This is stored sorted (just by value)."]
                pub fn members(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        ::std::vec::Vec<::subxt::utils::AccountId32>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "Members",
                        vec![],
                        [
                            162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
                            206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
                            238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
                            222u8,
                        ],
                    )
                }
                #[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
                pub fn prime(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Council",
                        "Prime",
                        vec![],
                        [
                            108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
                            157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
                            209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
                            158u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod council_membership {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AddMember {
                pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveMember {
                pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SwapMember {
                pub remove: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                pub add: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ResetMembers {
                pub members: ::std::vec::Vec<::subxt::utils::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ChangeKey {
                pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetPrime {
                pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClearPrime;
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Add a member `who` to the set."]
                #[doc = ""]
                #[doc = "May only be called from `T::AddOrigin`."]
                pub fn add_member(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<AddMember> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "add_member",
                        AddMember { who },
                        [
                            165u8, 116u8, 123u8, 50u8, 236u8, 196u8, 108u8, 211u8, 112u8, 214u8,
                            121u8, 105u8, 7u8, 88u8, 125u8, 99u8, 24u8, 0u8, 168u8, 65u8, 158u8,
                            100u8, 42u8, 62u8, 101u8, 59u8, 30u8, 174u8, 170u8, 119u8, 141u8,
                            121u8,
                        ],
                    )
                }
                #[doc = "Remove a member `who` from the set."]
                #[doc = ""]
                #[doc = "May only be called from `T::RemoveOrigin`."]
                pub fn remove_member(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "remove_member",
                        RemoveMember { who },
                        [
                            177u8, 18u8, 217u8, 235u8, 254u8, 40u8, 137u8, 79u8, 146u8, 5u8, 55u8,
                            187u8, 129u8, 28u8, 54u8, 132u8, 115u8, 220u8, 132u8, 139u8, 91u8,
                            81u8, 0u8, 110u8, 188u8, 248u8, 1u8, 135u8, 93u8, 153u8, 95u8, 193u8,
                        ],
                    )
                }
                #[doc = "Swap out one member `remove` for another `add`."]
                #[doc = ""]
                #[doc = "May only be called from `T::SwapOrigin`."]
                #[doc = ""]
                #[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
                pub fn swap_member(
                    &self,
                    remove: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    add: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<SwapMember> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "swap_member",
                        SwapMember { remove, add },
                        [
                            96u8, 248u8, 50u8, 206u8, 192u8, 242u8, 162u8, 62u8, 28u8, 91u8, 11u8,
                            208u8, 15u8, 84u8, 188u8, 234u8, 219u8, 233u8, 200u8, 215u8, 157u8,
                            155u8, 40u8, 220u8, 132u8, 182u8, 57u8, 210u8, 94u8, 240u8, 95u8,
                            252u8,
                        ],
                    )
                }
                #[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
                #[doc = "pass `members` pre-sorted."]
                #[doc = ""]
                #[doc = "May only be called from `T::ResetOrigin`."]
                pub fn reset_members(
                    &self,
                    members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "reset_members",
                        ResetMembers { members },
                        [
                            9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
                            98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
                            209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
                        ],
                    )
                }
                #[doc = "Swap out the sending member for some other key `new`."]
                #[doc = ""]
                #[doc = "May only be called from `Signed` origin of a current member."]
                #[doc = ""]
                #[doc = "Prime membership is passed from the origin account to `new`, if extant."]
                pub fn change_key(
                    &self,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "change_key",
                        ChangeKey { new },
                        [
                            27u8, 236u8, 241u8, 168u8, 98u8, 39u8, 176u8, 220u8, 145u8, 48u8,
                            173u8, 25u8, 179u8, 103u8, 170u8, 13u8, 166u8, 181u8, 131u8, 160u8,
                            131u8, 219u8, 116u8, 34u8, 152u8, 152u8, 46u8, 100u8, 46u8, 5u8, 156u8,
                            195u8,
                        ],
                    )
                }
                #[doc = "Set the prime member. Must be a current member."]
                #[doc = ""]
                #[doc = "May only be called from `T::PrimeOrigin`."]
                pub fn set_prime(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<SetPrime> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "set_prime",
                        SetPrime { who },
                        [
                            0u8, 42u8, 111u8, 52u8, 151u8, 19u8, 239u8, 149u8, 183u8, 252u8, 87u8,
                            194u8, 145u8, 21u8, 245u8, 112u8, 221u8, 181u8, 87u8, 28u8, 48u8, 39u8,
                            210u8, 133u8, 241u8, 207u8, 255u8, 209u8, 139u8, 232u8, 119u8, 64u8,
                        ],
                    )
                }
                #[doc = "Remove the prime member if it exists."]
                #[doc = ""]
                #[doc = "May only be called from `T::PrimeOrigin`."]
                pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
                    ::subxt::tx::StaticTxPayload::new(
                        "CouncilMembership",
                        "clear_prime",
                        ClearPrime {},
                        [
                            186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
                            23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
                            155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_membership::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The given member was added; see the transaction for who."]
            pub struct MemberAdded;
            impl ::subxt::events::StaticEvent for MemberAdded {
                const PALLET: &'static str = "CouncilMembership";
                const EVENT: &'static str = "MemberAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The given member was removed; see the transaction for who."]
            pub struct MemberRemoved;
            impl ::subxt::events::StaticEvent for MemberRemoved {
                const PALLET: &'static str = "CouncilMembership";
                const EVENT: &'static str = "MemberRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Two members were swapped; see the transaction for who."]
            pub struct MembersSwapped;
            impl ::subxt::events::StaticEvent for MembersSwapped {
                const PALLET: &'static str = "CouncilMembership";
                const EVENT: &'static str = "MembersSwapped";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "The membership was reset; see the transaction for who the new set is."]
            pub struct MembersReset;
            impl ::subxt::events::StaticEvent for MembersReset {
                const PALLET: &'static str = "CouncilMembership";
                const EVENT: &'static str = "MembersReset";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "One of the members' keys changed."]
            pub struct KeyChanged;
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "CouncilMembership";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Phantom member, never used."]
            pub struct Dummy;
            impl ::subxt::events::StaticEvent for Dummy {
                const PALLET: &'static str = "CouncilMembership";
                const EVENT: &'static str = "Dummy";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current membership, stored as an ordered Vec."]
                pub fn members(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "CouncilMembership",
                        "Members",
                        vec![],
                        [
                            56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
                            46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
                            178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
                        ],
                    )
                }
                #[doc = " The current prime member, if one exists."]
                pub fn prime(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "CouncilMembership",
                        "Prime",
                        vec![],
                        [
                            108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
                            157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
                            209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
                            158u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_upgrade {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<SetCode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "RuntimeUpgrade",
                        "set_code",
                        SetCode { code },
                        [
                            27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
                            244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
                            4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod validator {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CreateValidatorRequest {
                pub validator_node_account: ::subxt::utils::AccountId32,
                pub stash_account: ::subxt::utils::AccountId32,
                pub description: ::std::vec::Vec<::core::primitive::u8>,
                pub tf_connect_id: ::std::vec::Vec<::core::primitive::u8>,
                pub info: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ActivateValidatorNode;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ChangeValidatorNodeAccount {
                pub new_node_validator_account: ::subxt::utils::AccountId32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Bond {
                pub validator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ApproveValidator {
                pub validator_account:
                    ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RemoveValidator {
                pub validator_account:
                    ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Create a request to become a validator"]
                #[doc = "Validator account (signer): the account of the validator (this account will be added to the council)"]
                #[doc = "Validator node account: the account that will validate on consensus layer"]
                #[doc = "Stash account: the \"bank\" account of the validator (where rewards should be sent to) the stash should be bonded to a validator"]
                #[doc = "Description: why someone wants to become a validator"]
                #[doc = "Tf Connect ID: the threefold connect ID of the person who wants to become a validator"]
                #[doc = "Info: some public info about the validator (website link, blog link, ..)"]
                #[doc = "A user can only have 1 validator request at a time"]
                pub fn create_validator_request(
                    &self,
                    validator_node_account: ::subxt::utils::AccountId32,
                    stash_account: ::subxt::utils::AccountId32,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                    tf_connect_id: ::std::vec::Vec<::core::primitive::u8>,
                    info: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::StaticTxPayload<CreateValidatorRequest> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Validator",
                        "create_validator_request",
                        CreateValidatorRequest {
                            validator_node_account,
                            stash_account,
                            description,
                            tf_connect_id,
                            info,
                        },
                        [
                            237u8, 39u8, 56u8, 134u8, 208u8, 79u8, 227u8, 126u8, 114u8, 243u8,
                            121u8, 79u8, 146u8, 97u8, 33u8, 164u8, 235u8, 147u8, 87u8, 223u8,
                            206u8, 39u8, 155u8, 176u8, 204u8, 25u8, 242u8, 237u8, 251u8, 53u8,
                            24u8, 160u8,
                        ],
                    )
                }
                #[doc = "Start participating in consensus"]
                #[doc = "Will activate the Validator node account on consensus level"]
                #[doc = "A user can only call this if his request to be a validator is approved by the council"]
                #[doc = "Should be called when his node is synced and ready to start validating"]
                pub fn activate_validator_node(
                    &self,
                ) -> ::subxt::tx::StaticTxPayload<ActivateValidatorNode> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Validator",
                        "activate_validator_node",
                        ActivateValidatorNode {},
                        [
                            1u8, 11u8, 148u8, 174u8, 1u8, 39u8, 91u8, 169u8, 15u8, 224u8, 40u8,
                            74u8, 212u8, 220u8, 40u8, 70u8, 74u8, 196u8, 202u8, 17u8, 219u8, 54u8,
                            76u8, 215u8, 181u8, 203u8, 174u8, 101u8, 42u8, 129u8, 43u8, 76u8,
                        ],
                    )
                }
                #[doc = "Change validator node account"]
                #[doc = "In case the Validator wishes to change his validator node account"]
                #[doc = "he can call this method with the new node validator account"]
                #[doc = "this new account will be added as a new consensus validator if he is validating already"]
                pub fn change_validator_node_account(
                    &self,
                    new_node_validator_account: ::subxt::utils::AccountId32,
                ) -> ::subxt::tx::StaticTxPayload<ChangeValidatorNodeAccount> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Validator",
                        "change_validator_node_account",
                        ChangeValidatorNodeAccount {
                            new_node_validator_account,
                        },
                        [
                            33u8, 84u8, 111u8, 189u8, 108u8, 151u8, 251u8, 4u8, 152u8, 131u8,
                            211u8, 44u8, 23u8, 188u8, 195u8, 183u8, 245u8, 176u8, 209u8, 96u8,
                            165u8, 34u8, 142u8, 31u8, 40u8, 65u8, 66u8, 37u8, 222u8, 241u8, 173u8,
                            127u8,
                        ],
                    )
                }
                #[doc = "Bond an account to a validator account"]
                #[doc = "Just proves that the stash account is indeed under control of the validator account"]
                pub fn bond(
                    &self,
                    validator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::StaticTxPayload<Bond> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Validator",
                        "bond",
                        Bond { validator },
                        [
                            197u8, 74u8, 194u8, 41u8, 138u8, 235u8, 92u8, 110u8, 76u8, 117u8, 48u8,
                            183u8, 181u8, 51u8, 21u8, 213u8, 52u8, 240u8, 236u8, 20u8, 173u8, 14u8,
                            41u8, 100u8, 174u8, 227u8, 27u8, 196u8, 172u8, 98u8, 17u8, 73u8,
                        ],
                    )
                }
                #[doc = "Approve validator (council)"]
                #[doc = "Approves a validator to be added as a council member and"]
                #[doc = "to participate in consensus"]
                pub fn approve_validator(
                    &self,
                    validator_account: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<ApproveValidator> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Validator",
                        "approve_validator",
                        ApproveValidator { validator_account },
                        [
                            125u8, 170u8, 37u8, 130u8, 137u8, 252u8, 214u8, 174u8, 27u8, 27u8,
                            127u8, 193u8, 194u8, 99u8, 213u8, 101u8, 74u8, 216u8, 250u8, 173u8,
                            170u8, 217u8, 163u8, 215u8, 148u8, 12u8, 241u8, 15u8, 252u8, 58u8,
                            146u8, 161u8,
                        ],
                    )
                }
                #[doc = "Remove validator"]
                #[doc = "Removes a validator from:"]
                #[doc = "1. Council"]
                #[doc = "2. Storage"]
                #[doc = "3. Consensus"]
                #[doc = "Can only be called by the user or the council"]
                pub fn remove_validator(
                    &self,
                    validator_account: ::subxt::utils::MultiAddress<
                        ::subxt::utils::AccountId32,
                        (),
                    >,
                ) -> ::subxt::tx::StaticTxPayload<RemoveValidator> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Validator",
                        "remove_validator",
                        RemoveValidator { validator_account },
                        [
                            138u8, 161u8, 130u8, 123u8, 218u8, 168u8, 76u8, 143u8, 134u8, 208u8,
                            145u8, 246u8, 145u8, 111u8, 231u8, 32u8, 124u8, 221u8, 206u8, 153u8,
                            52u8, 84u8, 17u8, 129u8, 29u8, 141u8, 107u8, 149u8, 241u8, 36u8, 225u8,
                            220u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_validator::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Bonded(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for Bonded {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ValidatorRequestCreated(
                pub ::subxt::utils::AccountId32,
                pub runtime_types::pallet_validator::types::Validator<::subxt::utils::AccountId32>,
            );
            impl ::subxt::events::StaticEvent for ValidatorRequestCreated {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "ValidatorRequestCreated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ValidatorRequestApproved(
                pub runtime_types::pallet_validator::types::Validator<::subxt::utils::AccountId32>,
            );
            impl ::subxt::events::StaticEvent for ValidatorRequestApproved {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "ValidatorRequestApproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ValidatorActivated(
                pub runtime_types::pallet_validator::types::Validator<::subxt::utils::AccountId32>,
            );
            impl ::subxt::events::StaticEvent for ValidatorActivated {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "ValidatorActivated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ValidatorRemoved(
                pub runtime_types::pallet_validator::types::Validator<::subxt::utils::AccountId32>,
            );
            impl ::subxt::events::StaticEvent for ValidatorRemoved {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "ValidatorRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeValidatorChanged(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for NodeValidatorChanged {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "NodeValidatorChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct NodeValidatorRemoved(pub ::subxt::utils::AccountId32);
            impl ::subxt::events::StaticEvent for NodeValidatorRemoved {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "NodeValidatorRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn validator(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_validator::types::Validator<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Validator",
                        "Validator",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            212u8, 254u8, 45u8, 202u8, 69u8, 167u8, 214u8, 106u8, 72u8, 212u8,
                            172u8, 217u8, 230u8, 31u8, 247u8, 180u8, 185u8, 243u8, 138u8, 237u8,
                            90u8, 241u8, 161u8, 153u8, 5u8, 42u8, 219u8, 35u8, 174u8, 198u8, 246u8,
                            227u8,
                        ],
                    )
                }
                pub fn validator_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_validator::types::Validator<
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Validator",
                        "Validator",
                        Vec::new(),
                        [
                            212u8, 254u8, 45u8, 202u8, 69u8, 167u8, 214u8, 106u8, 72u8, 212u8,
                            172u8, 217u8, 230u8, 31u8, 247u8, 180u8, 185u8, 243u8, 138u8, 237u8,
                            90u8, 241u8, 161u8, 153u8, 5u8, 42u8, 219u8, 35u8, 174u8, 198u8, 246u8,
                            227u8,
                        ],
                    )
                }
                pub fn bonded(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Validator",
                        "Bonded",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Twox64Concat,
                        )],
                        [
                            35u8, 197u8, 156u8, 60u8, 22u8, 59u8, 103u8, 83u8, 77u8, 15u8, 118u8,
                            193u8, 155u8, 97u8, 229u8, 36u8, 119u8, 128u8, 224u8, 162u8, 21u8,
                            46u8, 199u8, 221u8, 15u8, 74u8, 59u8, 70u8, 77u8, 218u8, 73u8, 165u8,
                        ],
                    )
                }
                pub fn bonded_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::subxt::utils::AccountId32>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Validator",
                        "Bonded",
                        Vec::new(),
                        [
                            35u8, 197u8, 156u8, 60u8, 22u8, 59u8, 103u8, 83u8, 77u8, 15u8, 118u8,
                            193u8, 155u8, 97u8, 229u8, 36u8, 119u8, 128u8, 224u8, 162u8, 21u8,
                            46u8, 199u8, 221u8, 15u8, 74u8, 59u8, 70u8, 77u8, 218u8, 73u8, 165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod dao {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub action: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                pub description: ::std::vec::Vec<::core::primitive::u8>,
                pub link: ::std::vec::Vec<::core::primitive::u8>,
                pub duration: ::core::option::Option<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Vote {
                pub farm_id: ::core::primitive::u32,
                pub proposal_hash: ::subxt::utils::H256,
                pub approve: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Veto {
                pub proposal_hash: ::subxt::utils::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Close {
                pub proposal_hash: ::subxt::utils::H256,
                #[codec(compact)]
                pub proposal_index: ::core::primitive::u32,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    action: runtime_types::tfchain_runtime::RuntimeCall,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                    link: ::std::vec::Vec<::core::primitive::u8>,
                    duration: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::tx::StaticTxPayload<Propose> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Dao",
                        "propose",
                        Propose {
                            threshold,
                            action: ::std::boxed::Box::new(action),
                            description,
                            link,
                            duration,
                        },
                        [
                            171u8, 172u8, 243u8, 246u8, 3u8, 156u8, 158u8, 245u8, 221u8, 252u8,
                            185u8, 226u8, 43u8, 1u8, 59u8, 200u8, 234u8, 97u8, 50u8, 206u8, 139u8,
                            103u8, 51u8, 74u8, 56u8, 15u8, 245u8, 161u8, 195u8, 140u8, 81u8, 216u8,
                        ],
                    )
                }
                pub fn vote(
                    &self,
                    farm_id: ::core::primitive::u32,
                    proposal_hash: ::subxt::utils::H256,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::tx::StaticTxPayload<Vote> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Dao",
                        "vote",
                        Vote {
                            farm_id,
                            proposal_hash,
                            approve,
                        },
                        [
                            230u8, 107u8, 246u8, 235u8, 38u8, 72u8, 134u8, 6u8, 111u8, 81u8, 249u8,
                            250u8, 128u8, 174u8, 104u8, 130u8, 112u8, 248u8, 126u8, 195u8, 135u8,
                            114u8, 73u8, 123u8, 133u8, 12u8, 209u8, 232u8, 25u8, 50u8, 127u8,
                            143u8,
                        ],
                    )
                }
                pub fn veto(
                    &self,
                    proposal_hash: ::subxt::utils::H256,
                ) -> ::subxt::tx::StaticTxPayload<Veto> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Dao",
                        "veto",
                        Veto { proposal_hash },
                        [
                            195u8, 158u8, 11u8, 249u8, 246u8, 193u8, 62u8, 22u8, 20u8, 189u8,
                            235u8, 209u8, 36u8, 129u8, 204u8, 240u8, 164u8, 58u8, 213u8, 231u8,
                            215u8, 213u8, 169u8, 244u8, 129u8, 95u8, 74u8, 19u8, 225u8, 43u8,
                            237u8, 7u8,
                        ],
                    )
                }
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::utils::H256,
                    proposal_index: ::core::primitive::u32,
                ) -> ::subxt::tx::StaticTxPayload<Close> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Dao",
                        "close",
                        Close {
                            proposal_hash,
                            proposal_index,
                        },
                        [
                            209u8, 5u8, 202u8, 138u8, 155u8, 45u8, 63u8, 201u8, 43u8, 194u8, 206u8,
                            73u8, 123u8, 8u8, 103u8, 86u8, 54u8, 150u8, 175u8, 17u8, 130u8, 109u8,
                            63u8, 73u8, 100u8, 95u8, 83u8, 238u8, 115u8, 194u8, 29u8, 234u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_dao::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Voted {
                pub account: ::subxt::utils::AccountId32,
                pub proposal_hash: ::subxt::utils::H256,
                pub voted: ::core::primitive::bool,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Voted {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "Voted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
            #[doc = "`MemberCount`)."]
            pub struct Proposed {
                pub account: ::subxt::utils::AccountId32,
                pub proposal_index: ::core::primitive::u32,
                pub proposal_hash: ::subxt::utils::H256,
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for Proposed {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was approved by the required threshold."]
            pub struct Approved {
                pub proposal_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Approved {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "Approved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was not approved by the required threshold."]
            pub struct Disapproved {
                pub proposal_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Disapproved {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
            pub struct Executed {
                pub proposal_hash: ::subxt::utils::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Executed {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "Executed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A proposal_hash was closed because its threshold was reached or after its duration was up."]
            pub struct Closed {
                pub proposal_hash: ::subxt::utils::H256,
                pub yes: ::core::primitive::u32,
                pub yes_weight: ::core::primitive::u64,
                pub no: ::core::primitive::u32,
                pub no_weight: ::core::primitive::u64,
            }
            impl ::subxt::events::StaticEvent for Closed {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "Closed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ClosedByCouncil {
                pub proposal_hash: ::subxt::utils::H256,
                pub vetos: ::std::vec::Vec<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for ClosedByCouncil {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "ClosedByCouncil";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct CouncilMemberVeto {
                pub proposal_hash: ::subxt::utils::H256,
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for CouncilMemberVeto {
                const PALLET: &'static str = "Dao";
                const EVENT: &'static str = "CouncilMemberVeto";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The hashes of the active proposals."]
                pub fn proposal_list(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::std::vec::Vec<::subxt::utils::H256>>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "ProposalList",
                        vec![],
                        [
                            33u8, 228u8, 22u8, 131u8, 144u8, 113u8, 74u8, 116u8, 132u8, 18u8,
                            130u8, 86u8, 85u8, 7u8, 158u8, 107u8, 92u8, 40u8, 150u8, 9u8, 92u8,
                            43u8, 13u8, 140u8, 224u8, 112u8, 60u8, 104u8, 1u8, 176u8, 240u8, 70u8,
                        ],
                    )
                }
                #[doc = " A map that indexes a hash to an active proposal object."]
                pub fn proposals(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_dao::proposal::DaoProposal<::core::primitive::u32>,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "Proposals",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            195u8, 4u8, 12u8, 179u8, 218u8, 51u8, 161u8, 90u8, 116u8, 76u8, 130u8,
                            13u8, 190u8, 157u8, 177u8, 246u8, 147u8, 216u8, 221u8, 18u8, 116u8,
                            102u8, 55u8, 27u8, 169u8, 4u8, 210u8, 238u8, 89u8, 93u8, 114u8, 82u8,
                        ],
                    )
                }
                #[doc = " A map that indexes a hash to an active proposal object."]
                pub fn proposals_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_dao::proposal::DaoProposal<::core::primitive::u32>,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "Proposals",
                        Vec::new(),
                        [
                            195u8, 4u8, 12u8, 179u8, 218u8, 51u8, 161u8, 90u8, 116u8, 76u8, 130u8,
                            13u8, 190u8, 157u8, 177u8, 246u8, 147u8, 216u8, 221u8, 18u8, 116u8,
                            102u8, 55u8, 27u8, 169u8, 4u8, 210u8, 238u8, 89u8, 93u8, 114u8, 82u8,
                        ],
                    )
                }
                pub fn proposal_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_runtime::RuntimeCall,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "ProposalOf",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            94u8, 129u8, 207u8, 84u8, 243u8, 225u8, 239u8, 23u8, 126u8, 237u8,
                            103u8, 1u8, 1u8, 98u8, 103u8, 142u8, 213u8, 98u8, 165u8, 197u8, 74u8,
                            20u8, 136u8, 142u8, 56u8, 91u8, 170u8, 144u8, 99u8, 220u8, 13u8, 171u8,
                        ],
                    )
                }
                pub fn proposal_of_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::tfchain_runtime::RuntimeCall,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "ProposalOf",
                        Vec::new(),
                        [
                            94u8, 129u8, 207u8, 84u8, 243u8, 225u8, 239u8, 23u8, 126u8, 237u8,
                            103u8, 1u8, 1u8, 98u8, 103u8, 142u8, 213u8, 98u8, 165u8, 197u8, 74u8,
                            20u8, 136u8, 142u8, 56u8, 91u8, 170u8, 144u8, 99u8, 220u8, 13u8, 171u8,
                        ],
                    )
                }
                #[doc = " Votes on a given proposal, if it is ongoing."]
                pub fn voting(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_dao::proposal::DaoVotes<
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "Voting",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            209u8, 161u8, 237u8, 184u8, 155u8, 150u8, 173u8, 105u8, 113u8, 185u8,
                            217u8, 199u8, 94u8, 240u8, 240u8, 8u8, 195u8, 38u8, 210u8, 125u8,
                            217u8, 129u8, 209u8, 156u8, 209u8, 162u8, 173u8, 205u8, 249u8, 181u8,
                            85u8, 60u8,
                        ],
                    )
                }
                #[doc = " Votes on a given proposal, if it is ongoing."]
                pub fn voting_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<
                        runtime_types::pallet_dao::proposal::DaoVotes<
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                            ::subxt::utils::AccountId32,
                        >,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "Voting",
                        Vec::new(),
                        [
                            209u8, 161u8, 237u8, 184u8, 155u8, 150u8, 173u8, 105u8, 113u8, 185u8,
                            217u8, 199u8, 94u8, 240u8, 240u8, 8u8, 195u8, 38u8, 210u8, 125u8,
                            217u8, 129u8, 209u8, 156u8, 209u8, 162u8, 173u8, 205u8, 249u8, 181u8,
                            85u8, 60u8,
                        ],
                    )
                }
                #[doc = " Proposals so far."]
                pub fn proposal_count(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "ProposalCount",
                        vec![],
                        [
                            132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
                            140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
                            24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
                            70u8,
                        ],
                    )
                }
                pub fn farm_weight(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "FarmWeight",
                        vec![::subxt::storage::address::StorageMapKey::new(
                            _0.borrow(),
                            ::subxt::storage::address::StorageHasher::Identity,
                        )],
                        [
                            166u8, 54u8, 21u8, 163u8, 105u8, 20u8, 198u8, 70u8, 104u8, 247u8,
                            250u8, 149u8, 227u8, 172u8, 22u8, 106u8, 184u8, 251u8, 193u8, 42u8,
                            110u8, 16u8, 178u8, 198u8, 157u8, 16u8, 156u8, 15u8, 249u8, 12u8,
                            108u8, 6u8,
                        ],
                    )
                }
                pub fn farm_weight_root(
                    &self,
                ) -> ::subxt::storage::address::StaticStorageAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::StaticStorageAddress::new(
                        "Dao",
                        "FarmWeight",
                        Vec::new(),
                        [
                            166u8, 54u8, 21u8, 163u8, 105u8, 20u8, 198u8, 70u8, 104u8, 247u8,
                            250u8, 149u8, 227u8, 172u8, 22u8, 106u8, 184u8, 251u8, 193u8, 42u8,
                            110u8, 16u8, 178u8, 198u8, 157u8, 16u8, 156u8, 15u8, 249u8, 12u8,
                            108u8, 6u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod utility {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Batch {
                pub calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AsDerivative {
                pub index: ::core::primitive::u16,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BatchAll {
                pub calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct DispatchAs {
                pub as_origin: ::std::boxed::Box<runtime_types::tfchain_runtime::OriginCaller>,
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ForceBatch {
                pub calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct WithWeight {
                pub call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Send a batch of dispatch calls."]
                #[doc = ""]
                #[doc = "May be called from any origin except `None`."]
                #[doc = ""]
                #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                #[doc = ""]
                #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                #[doc = "# </weight>"]
                #[doc = ""]
                #[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
                #[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
                #[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
                #[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
                #[doc = "event is deposited."]
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
                ) -> ::subxt::tx::StaticTxPayload<Batch> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "batch",
                        Batch { calls },
                        [
                            138u8, 132u8, 163u8, 251u8, 75u8, 230u8, 238u8, 172u8, 27u8, 195u8,
                            81u8, 216u8, 106u8, 204u8, 145u8, 118u8, 121u8, 38u8, 169u8, 178u8,
                            239u8, 33u8, 192u8, 202u8, 249u8, 98u8, 222u8, 7u8, 132u8, 27u8, 97u8,
                            235u8,
                        ],
                    )
                }
                #[doc = "Send a call through an indexed pseudonym of the sender."]
                #[doc = ""]
                #[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
                #[doc = "use the same filter as the origin of this call."]
                #[doc = ""]
                #[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
                #[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
                #[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
                #[doc = "in the Multisig pallet instead."]
                #[doc = ""]
                #[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<AsDerivative> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "as_derivative",
                        AsDerivative {
                            index,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            50u8, 20u8, 98u8, 94u8, 134u8, 248u8, 154u8, 170u8, 47u8, 228u8, 94u8,
                            28u8, 220u8, 148u8, 58u8, 98u8, 217u8, 67u8, 117u8, 36u8, 142u8, 210u8,
                            91u8, 28u8, 42u8, 204u8, 239u8, 52u8, 98u8, 210u8, 179u8, 23u8,
                        ],
                    )
                }
                #[doc = "Send a batch of dispatch calls and atomically execute them."]
                #[doc = "The whole transaction will rollback and fail if any of the calls failed."]
                #[doc = ""]
                #[doc = "May be called from any origin except `None`."]
                #[doc = ""]
                #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                #[doc = ""]
                #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                #[doc = "# </weight>"]
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
                ) -> ::subxt::tx::StaticTxPayload<BatchAll> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "batch_all",
                        BatchAll { calls },
                        [
                            122u8, 152u8, 51u8, 192u8, 50u8, 162u8, 238u8, 40u8, 197u8, 18u8,
                            167u8, 234u8, 175u8, 17u8, 234u8, 219u8, 137u8, 166u8, 139u8, 219u8,
                            10u8, 235u8, 61u8, 3u8, 135u8, 187u8, 51u8, 91u8, 152u8, 37u8, 156u8,
                            132u8,
                        ],
                    )
                }
                #[doc = "Dispatches a function call with a provided origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Root_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
                #[doc = "# </weight>"]
                pub fn dispatch_as(
                    &self,
                    as_origin: runtime_types::tfchain_runtime::OriginCaller,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                ) -> ::subxt::tx::StaticTxPayload<DispatchAs> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "dispatch_as",
                        DispatchAs {
                            as_origin: ::std::boxed::Box::new(as_origin),
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            238u8, 135u8, 185u8, 127u8, 78u8, 254u8, 142u8, 144u8, 25u8, 209u8,
                            184u8, 27u8, 193u8, 46u8, 240u8, 3u8, 100u8, 121u8, 101u8, 158u8, 83u8,
                            108u8, 99u8, 170u8, 45u8, 97u8, 72u8, 209u8, 138u8, 238u8, 179u8,
                            171u8,
                        ],
                    )
                }
                #[doc = "Send a batch of dispatch calls."]
                #[doc = "Unlike `batch`, it allows errors and won't interrupt."]
                #[doc = ""]
                #[doc = "May be called from any origin except `None`."]
                #[doc = ""]
                #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                #[doc = ""]
                #[doc = "If origin is root then the calls are dispatch without checking origin filter. (This"]
                #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                #[doc = "# </weight>"]
                pub fn force_batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
                ) -> ::subxt::tx::StaticTxPayload<ForceBatch> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "force_batch",
                        ForceBatch { calls },
                        [
                            244u8, 211u8, 144u8, 109u8, 5u8, 173u8, 235u8, 43u8, 137u8, 217u8,
                            12u8, 235u8, 59u8, 236u8, 202u8, 195u8, 75u8, 102u8, 63u8, 160u8, 43u8,
                            9u8, 94u8, 27u8, 138u8, 112u8, 64u8, 60u8, 85u8, 130u8, 135u8, 93u8,
                        ],
                    )
                }
                #[doc = "Dispatch a function call with a specified weight."]
                #[doc = ""]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Root origin to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Root_."]
                pub fn with_weight(
                    &self,
                    call: runtime_types::tfchain_runtime::RuntimeCall,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::StaticTxPayload<WithWeight> {
                    ::subxt::tx::StaticTxPayload::new(
                        "Utility",
                        "with_weight",
                        WithWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            92u8, 120u8, 208u8, 38u8, 36u8, 66u8, 218u8, 251u8, 178u8, 71u8, 253u8,
                            123u8, 176u8, 188u8, 252u8, 251u8, 247u8, 151u8, 32u8, 152u8, 153u8,
                            108u8, 53u8, 34u8, 136u8, 153u8, 195u8, 212u8, 93u8, 41u8, 133u8,
                            248u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
            #[doc = "well as the error."]
            pub struct BatchInterrupted {
                pub index: ::core::primitive::u32,
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::events::StaticEvent for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Batch of dispatches completed fully with no error."]
            pub struct BatchCompleted;
            impl ::subxt::events::StaticEvent for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "Batch of dispatches completed but has errors."]
            pub struct BatchCompletedWithErrors;
            impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompletedWithErrors";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A single item within a Batch of dispatches has completed with no error."]
            pub struct ItemCompleted;
            impl ::subxt::events::StaticEvent for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A single item within a Batch of dispatches has completed with error."]
            pub struct ItemFailed {
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::events::StaticEvent for ItemFailed {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            #[doc = "A call was dispatched."]
            pub struct DispatchedAs {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The limit on the number of batched calls."]
                pub fn batched_calls_limit(
                    &self,
                ) -> ::subxt::constants::StaticConstantAddress<
                    ::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
                > {
                    ::subxt::constants::StaticConstantAddress::new(
                        "Utility",
                        "batched_calls_limit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod preimages {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Bounded<_0> {
                        #[codec(index = 0)]
                        Legacy {
                            hash: ::subxt::utils::H256,
                        },
                        #[codec(index = 1)]
                        Inline(
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        ),
                        #[codec(index = 2)]
                        Lookup {
                            hash: ::subxt::utils::H256,
                            len: ::core::primitive::u32,
                        },
                        __Ignore(::core::marker::PhantomData<_0>),
                    }
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            Debug,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Provide a set of uncles."]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The uncle parent not in the chain."]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    #[doc = "Uncles already set in the block."]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    #[doc = "Too many uncles."]
                    TooManyUncles,
                    #[codec(index = 3)]
                    #[doc = "The uncle is genesis."]
                    GenesisUncle,
                    #[codec(index = 4)]
                    #[doc = "The uncle is too high in chain."]
                    TooHighUncle,
                    #[codec(index = 5)]
                    #[doc = "The uncle is already included."]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    #[doc = "The uncle isn't recent enough to be included."]
                    OldUncle,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    #[doc = "---------------------------------"]
                    #[doc = "- Origin account is already in memory, so no DB operations for them."]
                    #[doc = "# </weight>"]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "# <weight>"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    #[doc = "# </weight>"]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). # <weight>"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    #[doc = "  #</weight>"]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_burning {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    burn_tft {
                        amount: ::core::primitive::u128,
                        message: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    NotEnoughBalanceToBurn,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    BurnTransactionCreated(
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Burn<_0, _1, _2> {
                pub target: _0,
                pub amount: _1,
                pub block: _2,
                pub message: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the collective's membership."]
                    #[doc = ""]
                    #[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
                    #[doc = "- `prime`: The prime member whose vote sets the default."]
                    #[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
                    #[doc = "  weight estimation."]
                    #[doc = ""]
                    #[doc = "Requires root origin."]
                    #[doc = ""]
                    #[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
                    #[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
                    #[doc = ""]
                    #[doc = "# WARNING:"]
                    #[doc = ""]
                    #[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
                    #[doc = "implementation of the trait [`ChangeMembers`]."]
                    #[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
                    #[doc = "with other logic managing the member set."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(MP + N)` where:"]
                    #[doc = "  - `M` old-members-count (code- and governance-bounded)"]
                    #[doc = "  - `N` new-members-count (code- and governance-bounded)"]
                    #[doc = "  - `P` proposals-count (code-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
                    #[doc = "    members"]
                    #[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
                    #[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
                    #[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
                    #[doc = "# </weight>"]
                    set_members {
                        new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        prime: ::core::option::Option<::subxt::utils::AccountId32>,
                        old_count: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Dispatch a proposal from a member using the `Member` origin."]
                    #[doc = ""]
                    #[doc = "Origin must be a member of the collective."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
                    #[doc = "  `proposal`"]
                    #[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
                    #[doc = "- 1 event"]
                    #[doc = "# </weight>"]
                    execute {
                        proposal: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Add a new proposal to either be voted on or executed directly."]
                    #[doc = ""]
                    #[doc = "Requires the sender to be member."]
                    #[doc = ""]
                    #[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
                    #[doc = "or put up for voting."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - branching is influenced by `threshold` where:"]
                    #[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
                    #[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
                    #[doc = "- DB:"]
                    #[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
                    #[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
                    #[doc = "  - DB accesses influenced by `threshold`:"]
                    #[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
                    #[doc = "    - OR proposal insertion (`threshold <= 2`)"]
                    #[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
                    #[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
                    #[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
                    #[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
                    #[doc = "  - 1 event"]
                    #[doc = "# </weight>"]
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        proposal: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Add an aye or nay vote for the sender to the given proposal."]
                    #[doc = ""]
                    #[doc = "Requires the sender to be a member."]
                    #[doc = ""]
                    #[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
                    #[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
                    #[doc = "fee."]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
                    #[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
                    #[doc = "- 1 event"]
                    #[doc = "# </weight>"]
                    vote {
                        proposal: ::subxt::utils::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                    #[doc = ""]
                    #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                    #[doc = ""]
                    #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                    #[doc = "has enough votes to be approved or disapproved."]
                    #[doc = ""]
                    #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                    #[doc = "unless there is a prime member set and the prime member cast an approval."]
                    #[doc = ""]
                    #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                    #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                    #[doc = ""]
                    #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                    #[doc = "proposal."]
                    #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                    #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(B + M + P1 + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                    #[doc = "  - `P2` is proposal-count (code-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
                    #[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
                    #[doc = "   `O(P2)`)"]
                    #[doc = " - any mutations done while executing `proposal` (`P1`)"]
                    #[doc = "- up to 3 events"]
                    #[doc = "# </weight>"]
                    close_old_weight {
                        proposal_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        proposal_weight_bound: runtime_types::sp_weights::OldWeight,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
                    #[doc = "state."]
                    #[doc = ""]
                    #[doc = "Must be called by the Root origin."]
                    #[doc = ""]
                    #[doc = "Parameters:"]
                    #[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Complexity: O(P) where P is the number of max proposals"]
                    #[doc = "DB Weight:"]
                    #[doc = "* Reads: Proposals"]
                    #[doc = "* Writes: Voting, Proposals, ProposalOf"]
                    #[doc = "# </weight>"]
                    disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 6)]
                    #[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
                    #[doc = ""]
                    #[doc = "May be called by any signed account in order to finish voting and close the proposal."]
                    #[doc = ""]
                    #[doc = "If called before the end of the voting period it will only close the vote if it is"]
                    #[doc = "has enough votes to be approved or disapproved."]
                    #[doc = ""]
                    #[doc = "If called after the end of the voting period abstentions are counted as rejections"]
                    #[doc = "unless there is a prime member set and the prime member cast an approval."]
                    #[doc = ""]
                    #[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
                    #[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
                    #[doc = ""]
                    #[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
                    #[doc = "proposal."]
                    #[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
                    #[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "## Weight"]
                    #[doc = "- `O(B + M + P1 + P2)` where:"]
                    #[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
                    #[doc = "  - `M` is members-count (code- and governance-bounded)"]
                    #[doc = "  - `P1` is the complexity of `proposal` preimage."]
                    #[doc = "  - `P2` is proposal-count (code-bounded)"]
                    #[doc = "- DB:"]
                    #[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
                    #[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
                    #[doc = "   `O(P2)`)"]
                    #[doc = " - any mutations done while executing `proposal` (`P1`)"]
                    #[doc = "- up to 3 events"]
                    #[doc = "# </weight>"]
                    close {
                        proposal_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Account is not a member"]
                    NotMember,
                    #[codec(index = 1)]
                    #[doc = "Duplicate proposals not allowed"]
                    DuplicateProposal,
                    #[codec(index = 2)]
                    #[doc = "Proposal must exist"]
                    ProposalMissing,
                    #[codec(index = 3)]
                    #[doc = "Mismatched index"]
                    WrongIndex,
                    #[codec(index = 4)]
                    #[doc = "Duplicate vote ignored"]
                    DuplicateVote,
                    #[codec(index = 5)]
                    #[doc = "Members are already initialized!"]
                    AlreadyInitialized,
                    #[codec(index = 6)]
                    #[doc = "The close call was made too early, before the end of the voting."]
                    TooEarly,
                    #[codec(index = 7)]
                    #[doc = "There can only be a maximum of `MaxProposals` active proposals."]
                    TooManyProposals,
                    #[codec(index = 8)]
                    #[doc = "The given weight bound for the proposal was too low."]
                    WrongProposalWeight,
                    #[codec(index = 9)]
                    #[doc = "The given length bound for the proposal was too low."]
                    WrongProposalLength,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
                    #[doc = "`MemberCount`)."]
                    Proposed {
                        account: ::subxt::utils::AccountId32,
                        proposal_index: ::core::primitive::u32,
                        proposal_hash: ::subxt::utils::H256,
                        threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A motion (given hash) has been voted on by given account, leaving"]
                    #[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
                    Voted {
                        account: ::subxt::utils::AccountId32,
                        proposal_hash: ::subxt::utils::H256,
                        voted: ::core::primitive::bool,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "A motion was approved by the required threshold."]
                    Approved { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 3)]
                    #[doc = "A motion was not approved by the required threshold."]
                    Disapproved { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 4)]
                    #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
                    Executed {
                        proposal_hash: ::subxt::utils::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 5)]
                    #[doc = "A single member did some action; result will be `Ok` if it returned without error."]
                    MemberExecuted {
                        proposal_hash: ::subxt::utils::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 6)]
                    #[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
                    Closed {
                        proposal_hash: ::subxt::utils::H256,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Members(::core::primitive::u32, ::core::primitive::u32),
                #[codec(index = 1)]
                Member(_0),
                #[codec(index = 2)]
                _Phantom,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: ::std::vec::Vec<_0>,
                pub nays: ::std::vec::Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_dao {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        action: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                        description: ::std::vec::Vec<::core::primitive::u8>,
                        link: ::std::vec::Vec<::core::primitive::u8>,
                        duration: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 1)]
                    vote {
                        farm_id: ::core::primitive::u32,
                        proposal_hash: ::subxt::utils::H256,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    veto { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 3)]
                    close {
                        proposal_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        proposal_index: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    NoneValue,
                    #[codec(index = 1)]
                    StorageOverflow,
                    #[codec(index = 2)]
                    FarmNotExists,
                    #[codec(index = 3)]
                    NotCouncilMember,
                    #[codec(index = 4)]
                    WrongProposalLength,
                    #[codec(index = 5)]
                    DuplicateProposal,
                    #[codec(index = 6)]
                    NotAuthorizedToVote,
                    #[codec(index = 7)]
                    ProposalMissing,
                    #[codec(index = 8)]
                    WrongIndex,
                    #[codec(index = 9)]
                    DuplicateVote,
                    #[codec(index = 10)]
                    WrongProposalWeight,
                    #[codec(index = 11)]
                    TooEarly,
                    #[codec(index = 12)]
                    TimeLimitReached,
                    #[codec(index = 13)]
                    OngoingVoteAndTresholdStillNotMet,
                    #[codec(index = 14)]
                    FarmHasNoNodes,
                    #[codec(index = 15)]
                    InvalidProposalDuration,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    Voted {
                        account: ::subxt::utils::AccountId32,
                        proposal_hash: ::subxt::utils::H256,
                        voted: ::core::primitive::bool,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
                    #[doc = "`MemberCount`)."]
                    Proposed {
                        account: ::subxt::utils::AccountId32,
                        proposal_index: ::core::primitive::u32,
                        proposal_hash: ::subxt::utils::H256,
                        threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "A motion was approved by the required threshold."]
                    Approved { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 3)]
                    #[doc = "A motion was not approved by the required threshold."]
                    Disapproved { proposal_hash: ::subxt::utils::H256 },
                    #[codec(index = 4)]
                    #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
                    Executed {
                        proposal_hash: ::subxt::utils::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 5)]
                    #[doc = "A proposal_hash was closed because its threshold was reached or after its duration was up."]
                    Closed {
                        proposal_hash: ::subxt::utils::H256,
                        yes: ::core::primitive::u32,
                        yes_weight: ::core::primitive::u64,
                        no: ::core::primitive::u32,
                        no_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 6)]
                    ClosedByCouncil {
                        proposal_hash: ::subxt::utils::H256,
                        vetos: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    CouncilMemberVeto {
                        proposal_hash: ::subxt::utils::H256,
                        who: ::subxt::utils::AccountId32,
                    },
                }
            }
            pub mod proposal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DaoProposal<_0> {
                    pub index: _0,
                    pub description: ::std::vec::Vec<::core::primitive::u8>,
                    pub link: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct DaoVotes<_0, _1, _2> {
                    pub index: _0,
                    pub threshold: _0,
                    pub ayes: ::std::vec::Vec<runtime_types::pallet_dao::proposal::VoteWeight>,
                    pub nays: ::std::vec::Vec<runtime_types::pallet_dao::proposal::VoteWeight>,
                    pub end: _0,
                    pub vetos: ::std::vec::Vec<_2>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct VoteWeight {
                    pub farm_id: ::core::primitive::u32,
                    pub weight: ::core::primitive::u64,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::sp_core::bounded::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_kvstore {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the value stored at a particular key"]
                    set {
                        key: ::std::vec::Vec<::core::primitive::u8>,
                        value: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Read the value stored at a particular key, while removing it from the map."]
                    #[doc = "Also emit the read value in an event"]
                    delete {
                        key: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The requested user has not stored a value yet"]
                    NoValueStored,
                    #[codec(index = 1)]
                    KeyIsTooLarge,
                    #[codec(index = 2)]
                    ValueIsTooLarge,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A user has set their entry"]
                    EntrySet(
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 1)]
                    #[doc = "A user has read their entry, leaving it in storage"]
                    EntryGot(
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 2)]
                    #[doc = "A user has read their entry, removing it from storage"]
                    EntryTaken(
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
        }
        pub mod pallet_membership {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a member `who` to the set."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::AddOrigin`."]
                    add_member {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Remove a member `who` from the set."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::RemoveOrigin`."]
                    remove_member {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Swap out one member `remove` for another `add`."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::SwapOrigin`."]
                    #[doc = ""]
                    #[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
                    swap_member {
                        remove: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        add: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
                    #[doc = "pass `members` pre-sorted."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::ResetOrigin`."]
                    reset_members {
                        members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Swap out the sending member for some other key `new`."]
                    #[doc = ""]
                    #[doc = "May only be called from `Signed` origin of a current member."]
                    #[doc = ""]
                    #[doc = "Prime membership is passed from the origin account to `new`, if extant."]
                    change_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set the prime member. Must be a current member."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::PrimeOrigin`."]
                    set_prime {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Remove the prime member if it exists."]
                    #[doc = ""]
                    #[doc = "May only be called from `T::PrimeOrigin`."]
                    clear_prime,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Already a member."]
                    AlreadyMember,
                    #[codec(index = 1)]
                    #[doc = "Not a member."]
                    NotMember,
                    #[codec(index = 2)]
                    #[doc = "Too many members."]
                    TooManyMembers,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The given member was added; see the transaction for who."]
                    MemberAdded,
                    #[codec(index = 1)]
                    #[doc = "The given member was removed; see the transaction for who."]
                    MemberRemoved,
                    #[codec(index = 2)]
                    #[doc = "Two members were swapped; see the transaction for who."]
                    MembersSwapped,
                    #[codec(index = 3)]
                    #[doc = "The membership was reset; see the transaction for who the new set is."]
                    MembersReset,
                    #[codec(index = 4)]
                    #[doc = "One of the members' keys changed."]
                    KeyChanged,
                    #[codec(index = 5)]
                    #[doc = "Phantom member, never used."]
                    Dummy,
                }
            }
        }
        pub mod pallet_runtime_upgrade {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Anonymously schedule a task."]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Cancel an anonymously scheduled task."]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Schedule a named task."]
                    schedule_named {
                        id: [::core::primitive::u8; 32usize],
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a named scheduled task."]
                    cancel_named {
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 4)]
                    #[doc = "Anonymously schedule a task after a delay."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Same as [`schedule`]."]
                    #[doc = "# </weight>"]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Schedule a named task after a delay."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "Same as [`schedule_named`](Self::schedule_named)."]
                    #[doc = "# </weight>"]
                    schedule_named_after {
                        id: [::core::primitive::u8; 32usize],
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Failed to schedule a call"]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    #[doc = "Cannot find the scheduled call."]
                    NotFound,
                    #[codec(index = 2)]
                    #[doc = "Given target block number is in the past."]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    #[doc = "Reschedule failed because it does not change scheduled time."]
                    RescheduleNoChange,
                    #[codec(index = 4)]
                    #[doc = "Attempt to use a non-named function on a named task."]
                    Named,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Events type."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Scheduled some task."]
                    Scheduled {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Canceled some task."]
                    Canceled {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Dispatched some task."]
                    Dispatched {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "The call for the provided hash was not found so the task has been aborted."]
                    CallUnavailable {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                    #[codec(index = 4)]
                    #[doc = "The given task was unable to be renewed since the agenda is full at that block."]
                    PeriodicFailed {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                    #[codec(index = 5)]
                    #[doc = "The given task can never be executed since it is overweight."]
                    PermanentlyOverweight {
                        task: (::core::primitive::u32, ::core::primitive::u32),
                        id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Scheduled<_0, _1, _2, _3, _4> {
                pub maybe_id: ::core::option::Option<_0>,
                pub priority: ::core::primitive::u8,
                pub call: _1,
                pub maybe_periodic: ::core::option::Option<(_2, _2)>,
                pub origin: _3,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Sets the session key(s) of the function caller to `keys`."]
                    #[doc = "Allows an account to set its session key prior to becoming a validator."]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be signed."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
                    #[doc = "  `T::Keys::key_ids()` which is fixed."]
                    #[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
                    #[doc = "- DbWrites: `origin account`, `NextKeys`"]
                    #[doc = "- DbReads per key id: `KeyOwner`"]
                    #[doc = "- DbWrites per key id: `KeyOwner`"]
                    #[doc = "# </weight>"]
                    set_keys {
                        keys: runtime_types::tfchain_runtime::opaque::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Removes any session key(s) of the function caller."]
                    #[doc = ""]
                    #[doc = "This doesn't take effect until the next session."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
                    #[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
                    #[doc = "means being a controller account) or directly convertible into a validator ID (which"]
                    #[doc = "usually means being a stash account)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
                    #[doc = "  of `T::Keys::key_ids()` which is fixed."]
                    #[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
                    #[doc = "- DbWrites: `NextKeys`, `origin account`"]
                    #[doc = "- DbWrites per key id: `KeyOwner`"]
                    #[doc = "# </weight>"]
                    purge_keys,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the session pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid ownership proof."]
                    InvalidProof,
                    #[codec(index = 1)]
                    #[doc = "No associated validator ID for account."]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    #[doc = "Registered duplicate key."]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    #[doc = "No keys are associated with this account."]
                    NoKeys,
                    #[codec(index = 4)]
                    #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New session has happened. Note that the argument is the session index, not the"]
                    #[doc = "block number as the type might suggest."]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_smart_contract {
            use super::runtime_types;
            pub mod name_contract {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct NameContractName(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    create_node_contract {
                        node_id: ::core::primitive::u32,
                        deployment_hash: [::core::primitive::u8; 32usize],
                        deployment_data: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        public_ips: ::core::primitive::u32,
                        solution_provider_id: ::core::option::Option<::core::primitive::u64>,
                    },
                    #[codec(index = 1)]
                    update_node_contract {
                        contract_id: ::core::primitive::u64,
                        deployment_hash: [::core::primitive::u8; 32usize],
                        deployment_data: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 2)]
                    cancel_contract { contract_id: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    add_reports {
                        reports: ::std::vec::Vec<
                            runtime_types::pallet_smart_contract::types::Consumption,
                        >,
                    },
                    #[codec(index = 4)]
                    create_name_contract {
                        name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    add_nru_reports {
                        reports: ::std::vec::Vec<
                            runtime_types::pallet_smart_contract::types::NruConsumption,
                        >,
                    },
                    #[codec(index = 6)]
                    report_contract_resources {
                        contract_resources: ::std::vec::Vec<
                            runtime_types::pallet_smart_contract::types::ContractResources,
                        >,
                    },
                    #[codec(index = 7)]
                    create_rent_contract {
                        node_id: ::core::primitive::u32,
                        solution_provider_id: ::core::option::Option<::core::primitive::u64>,
                    },
                    #[codec(index = 8)]
                    create_solution_provider {
                        description: ::std::vec::Vec<::core::primitive::u8>,
                        link: ::std::vec::Vec<::core::primitive::u8>,
                        providers: ::std::vec::Vec<
                            runtime_types::pallet_smart_contract::types::Provider<
                                ::subxt::utils::AccountId32,
                            >,
                        >,
                    },
                    #[codec(index = 9)]
                    approve_solution_provider {
                        solution_provider_id: ::core::primitive::u64,
                        approve: ::core::primitive::bool,
                    },
                    #[codec(index = 10)]
                    bill_contract_for_block { contract_id: ::core::primitive::u64 },
                    #[codec(index = 11)]
                    service_contract_create {
                        service_account: ::subxt::utils::AccountId32,
                        consumer_account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 12)]
                    service_contract_set_metadata {
                        service_contract_id: ::core::primitive::u64,
                        metadata: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 13)]
                    service_contract_set_fees {
                        service_contract_id: ::core::primitive::u64,
                        base_fee: ::core::primitive::u64,
                        variable_fee: ::core::primitive::u64,
                    },
                    #[codec(index = 14)]
                    service_contract_approve {
                        service_contract_id: ::core::primitive::u64,
                    },
                    #[codec(index = 15)]
                    service_contract_reject {
                        service_contract_id: ::core::primitive::u64,
                    },
                    #[codec(index = 16)]
                    service_contract_cancel {
                        service_contract_id: ::core::primitive::u64,
                    },
                    #[codec(index = 17)]
                    service_contract_bill {
                        service_contract_id: ::core::primitive::u64,
                        variable_amount: ::core::primitive::u64,
                        metadata: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 18)]
                    change_billing_frequency { frequency: ::core::primitive::u64 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    TwinNotExists,
                    #[codec(index = 1)]
                    NodeNotExists,
                    #[codec(index = 2)]
                    FarmNotExists,
                    #[codec(index = 3)]
                    FarmHasNotEnoughPublicIPs,
                    #[codec(index = 4)]
                    FarmHasNotEnoughPublicIPsFree,
                    #[codec(index = 5)]
                    FailedToReserveIP,
                    #[codec(index = 6)]
                    FailedToFreeIPs,
                    #[codec(index = 7)]
                    ContractNotExists,
                    #[codec(index = 8)]
                    TwinNotAuthorizedToUpdateContract,
                    #[codec(index = 9)]
                    TwinNotAuthorizedToCancelContract,
                    #[codec(index = 10)]
                    NodeNotAuthorizedToDeployContract,
                    #[codec(index = 11)]
                    NodeNotAuthorizedToComputeReport,
                    #[codec(index = 12)]
                    PricingPolicyNotExists,
                    #[codec(index = 13)]
                    ContractIsNotUnique,
                    #[codec(index = 14)]
                    NameExists,
                    #[codec(index = 15)]
                    NameNotValid,
                    #[codec(index = 16)]
                    InvalidContractType,
                    #[codec(index = 17)]
                    TFTPriceValueError,
                    #[codec(index = 18)]
                    NotEnoughResourcesOnNode,
                    #[codec(index = 19)]
                    NodeNotAuthorizedToReportResources,
                    #[codec(index = 20)]
                    MethodIsDeprecated,
                    #[codec(index = 21)]
                    NodeHasActiveContracts,
                    #[codec(index = 22)]
                    NodeHasRentContract,
                    #[codec(index = 23)]
                    NodeIsNotDedicated,
                    #[codec(index = 24)]
                    NodeNotAvailableToDeploy,
                    #[codec(index = 25)]
                    CannotUpdateContractInGraceState,
                    #[codec(index = 26)]
                    NumOverflow,
                    #[codec(index = 27)]
                    OffchainSignedTxCannotSign,
                    #[codec(index = 28)]
                    OffchainSignedTxAlreadySent,
                    #[codec(index = 29)]
                    OffchainSignedTxNoLocalAccountAvailable,
                    #[codec(index = 30)]
                    NameContractNameTooShort,
                    #[codec(index = 31)]
                    NameContractNameTooLong,
                    #[codec(index = 32)]
                    InvalidProviderConfiguration,
                    #[codec(index = 33)]
                    NoSuchSolutionProvider,
                    #[codec(index = 34)]
                    SolutionProviderNotApproved,
                    #[codec(index = 35)]
                    TwinNotAuthorized,
                    #[codec(index = 36)]
                    ServiceContractNotExists,
                    #[codec(index = 37)]
                    ServiceContractCreationNotAllowed,
                    #[codec(index = 38)]
                    ServiceContractModificationNotAllowed,
                    #[codec(index = 39)]
                    ServiceContractApprovalNotAllowed,
                    #[codec(index = 40)]
                    ServiceContractRejectionNotAllowed,
                    #[codec(index = 41)]
                    ServiceContractBillingNotApprovedByBoth,
                    #[codec(index = 42)]
                    ServiceContractBillingVariableAmountTooHigh,
                    #[codec(index = 43)]
                    ServiceContractBillMetadataTooLong,
                    #[codec(index = 44)]
                    ServiceContractMetadataTooLong,
                    #[codec(index = 45)]
                    ServiceContractNotEnoughFundsToPayBill,
                    #[codec(index = 46)]
                    CanOnlyIncreaseFrequency,
                    #[codec(index = 47)]
                    IsNotAnAuthority,
                    #[codec(index = 48)]
                    WrongAuthority,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A contract got created"]
                    ContractCreated(runtime_types::pallet_smart_contract::types::Contract),
                    #[codec(index = 1)]
                    #[doc = "A contract was updated"]
                    ContractUpdated(runtime_types::pallet_smart_contract::types::Contract),
                    #[codec(index = 2)]
                    #[doc = "A Node contract is canceled"]
                    NodeContractCanceled {
                        contract_id: ::core::primitive::u64,
                        node_id: ::core::primitive::u32,
                        twin_id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "A Name contract is canceled"]
                    NameContractCanceled { contract_id: ::core::primitive::u64 },
                    #[codec(index = 4)]
                    #[doc = "IP got reserved by a Node contract"]
                    IPsReserved {
                        contract_id: ::core::primitive::u64,
                        public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::tfchain_support::types::PublicIP,
                        >,
                    },
                    #[codec(index = 5)]
                    #[doc = "IP got freed by a Node contract"]
                    IPsFreed {
                        contract_id: ::core::primitive::u64,
                        public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::tfchain_support::types::PublicIP,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Deprecated event"]
                    ContractDeployed(::core::primitive::u64, ::subxt::utils::AccountId32),
                    #[codec(index = 7)]
                    #[doc = "Deprecated event"]
                    ConsumptionReportReceived(
                        runtime_types::pallet_smart_contract::types::Consumption,
                    ),
                    #[codec(index = 8)]
                    ContractBilled(runtime_types::pallet_smart_contract::types::ContractBill),
                    #[codec(index = 9)]
                    #[doc = "A certain amount of tokens got burned by a contract"]
                    TokensBurned {
                        contract_id: ::core::primitive::u64,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Contract resources got updated"]
                    UpdatedUsedResources(
                        runtime_types::pallet_smart_contract::types::ContractResources,
                    ),
                    #[codec(index = 11)]
                    #[doc = "Network resources report received for contract"]
                    NruConsumptionReportReceived(
                        runtime_types::pallet_smart_contract::types::NruConsumption,
                    ),
                    #[codec(index = 12)]
                    #[doc = "a Rent contract is canceled"]
                    RentContractCanceled { contract_id: ::core::primitive::u64 },
                    #[codec(index = 13)]
                    #[doc = "A Contract grace period is triggered"]
                    ContractGracePeriodStarted {
                        contract_id: ::core::primitive::u64,
                        node_id: ::core::primitive::u32,
                        twin_id: ::core::primitive::u32,
                        block_number: ::core::primitive::u64,
                    },
                    #[codec(index = 14)]
                    #[doc = "A Contract grace period was ended"]
                    ContractGracePeriodEnded {
                        contract_id: ::core::primitive::u64,
                        node_id: ::core::primitive::u32,
                        twin_id: ::core::primitive::u32,
                    },
                    #[codec(index = 15)]
                    SolutionProviderCreated(
                        runtime_types::pallet_smart_contract::types::SolutionProvider<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 16)]
                    SolutionProviderApproved(::core::primitive::u64, ::core::primitive::bool),
                    #[codec(index = 17)]
                    #[doc = "A Service contract is created"]
                    ServiceContractCreated(
                        runtime_types::pallet_smart_contract::types::ServiceContract,
                    ),
                    #[codec(index = 18)]
                    #[doc = "A Service contract metadata is set"]
                    ServiceContractMetadataSet(
                        runtime_types::pallet_smart_contract::types::ServiceContract,
                    ),
                    #[codec(index = 19)]
                    #[doc = "A Service contract fees are set"]
                    ServiceContractFeesSet(
                        runtime_types::pallet_smart_contract::types::ServiceContract,
                    ),
                    #[codec(index = 20)]
                    #[doc = "A Service contract is approved"]
                    ServiceContractApproved(
                        runtime_types::pallet_smart_contract::types::ServiceContract,
                    ),
                    #[codec(index = 21)]
                    #[doc = "A Service contract is canceled"]
                    ServiceContractCanceled {
                        service_contract_id: ::core::primitive::u64,
                        cause: runtime_types::pallet_smart_contract::types::Cause,
                    },
                    #[codec(index = 22)]
                    #[doc = "A Service contract is billed"]
                    ServiceContractBilled {
                        service_contract:
                            runtime_types::pallet_smart_contract::types::ServiceContract,
                        bill: runtime_types::pallet_smart_contract::types::ServiceContractBill,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    BillingFrequencyChanged(::core::primitive::u64),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Cause {
                    #[codec(index = 0)]
                    CanceledByUser,
                    #[codec(index = 1)]
                    OutOfFunds,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Consumption {
                    pub contract_id: ::core::primitive::u64,
                    pub timestamp: ::core::primitive::u64,
                    pub cru: ::core::primitive::u64,
                    pub sru: ::core::primitive::u64,
                    pub hru: ::core::primitive::u64,
                    pub mru: ::core::primitive::u64,
                    pub nru: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Contract {
                    pub version: ::core::primitive::u32,
                    pub state: runtime_types::pallet_smart_contract::types::ContractState,
                    pub contract_id: ::core::primitive::u64,
                    pub twin_id: ::core::primitive::u32,
                    pub contract_type: runtime_types::pallet_smart_contract::types::ContractData,
                    pub solution_provider_id: ::core::option::Option<::core::primitive::u64>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ContractBill {
                    pub contract_id: ::core::primitive::u64,
                    pub timestamp: ::core::primitive::u64,
                    pub discount_level: runtime_types::pallet_smart_contract::types::DiscountLevel,
                    pub amount_billed: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ContractBillingInformation {
                    pub previous_nu_reported: ::core::primitive::u64,
                    pub last_updated: ::core::primitive::u64,
                    pub amount_unbilled: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum ContractData {
                    #[codec(index = 0)]
                    NodeContract(runtime_types::pallet_smart_contract::types::NodeContract),
                    #[codec(index = 1)]
                    NameContract(runtime_types::pallet_smart_contract::types::NameContract),
                    #[codec(index = 2)]
                    RentContract(runtime_types::pallet_smart_contract::types::RentContract),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ContractLock<_0> {
                    pub amount_locked: _0,
                    pub lock_updated: ::core::primitive::u64,
                    pub cycles: ::core::primitive::u16,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ContractResources {
                    pub contract_id: ::core::primitive::u64,
                    pub used: runtime_types::tfchain_support::resources::Resources,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum ContractState {
                    #[codec(index = 0)]
                    Created,
                    #[codec(index = 1)]
                    Deleted(runtime_types::pallet_smart_contract::types::Cause),
                    #[codec(index = 2)]
                    GracePeriod(::core::primitive::u64),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum DiscountLevel {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Default,
                    #[codec(index = 2)]
                    Bronze,
                    #[codec(index = 3)]
                    Silver,
                    #[codec(index = 4)]
                    Gold,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct NameContract {
                    pub name: runtime_types::pallet_smart_contract::name_contract::NameContractName,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct NodeContract {
                    pub node_id: ::core::primitive::u32,
                    pub deployment_hash: [::core::primitive::u8; 32usize],
                    pub deployment_data: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub public_ips: ::core::primitive::u32,
                    pub public_ips_list: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        runtime_types::tfchain_support::types::PublicIP,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct NruConsumption {
                    pub contract_id: ::core::primitive::u64,
                    pub timestamp: ::core::primitive::u64,
                    pub window: ::core::primitive::u64,
                    pub nru: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Provider<_0> {
                    pub who: _0,
                    pub take: ::core::primitive::u8,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct RentContract {
                    pub node_id: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ServiceContract {
                    pub service_contract_id: ::core::primitive::u64,
                    pub service_twin_id: ::core::primitive::u32,
                    pub consumer_twin_id: ::core::primitive::u32,
                    pub base_fee: ::core::primitive::u64,
                    pub variable_fee: ::core::primitive::u64,
                    pub metadata: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub accepted_by_service: ::core::primitive::bool,
                    pub accepted_by_consumer: ::core::primitive::bool,
                    pub last_bill: ::core::primitive::u64,
                    pub state: runtime_types::pallet_smart_contract::types::ServiceContractState,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct ServiceContractBill {
                    pub variable_amount: ::core::primitive::u64,
                    pub window: ::core::primitive::u64,
                    pub metadata: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum ServiceContractState {
                    #[codec(index = 0)]
                    Created,
                    #[codec(index = 1)]
                    AgreementReady,
                    #[codec(index = 2)]
                    ApprovedByBoth,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct SolutionProvider<_0> {
                    pub solution_provider_id: ::core::primitive::u64,
                    pub providers:
                        ::std::vec::Vec<runtime_types::pallet_smart_contract::types::Provider<_0>>,
                    pub description: ::std::vec::Vec<::core::primitive::u8>,
                    pub link: ::std::vec::Vec<::core::primitive::u8>,
                    pub approved: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum StorageVersion {
                    #[codec(index = 0)]
                    V1,
                    #[codec(index = 1)]
                    V2,
                    #[codec(index = 2)]
                    V3,
                    #[codec(index = 3)]
                    V4,
                    #[codec(index = 4)]
                    V5,
                    #[codec(index = 5)]
                    V6,
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- The weight of this call is defined by the caller."]
                    #[doc = "# </weight>"]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB change."]
                    #[doc = "# </weight>"]
                    set_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_tfgrid {
            use super::runtime_types;
            pub mod farm {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct FarmName(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
            }
            pub mod interface {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct InterfaceIp(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct InterfaceMac(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct InterfaceName(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
            }
            pub mod node {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct CityName(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct CountryName(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Location {
                    pub city: runtime_types::pallet_tfgrid::node::CityName,
                    pub country: runtime_types::pallet_tfgrid::node::CountryName,
                    pub latitude: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub longitude: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct SerialNumber(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    set_storage_version {
                        version: runtime_types::pallet_tfgrid::types::StorageVersion,
                    },
                    #[codec(index = 1)]
                    create_farm {
                        name: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::tfchain_support::types::IP4,
                        >,
                    },
                    #[codec(index = 2)]
                    update_farm {
                        id: ::core::primitive::u32,
                        name: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 3)]
                    add_stellar_payout_v2address {
                        farm_id: ::core::primitive::u32,
                        stellar_address: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_farm_certification {
                        farm_id: ::core::primitive::u32,
                        certification: runtime_types::tfchain_support::types::FarmCertification,
                    },
                    #[codec(index = 5)]
                    add_farm_ip {
                        id: ::core::primitive::u32,
                        ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        gw: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 6)]
                    remove_farm_ip {
                        id: ::core::primitive::u32,
                        ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 7)]
                    delete_farm { id: ::core::primitive::u32 },
                    #[codec(index = 8)]
                    create_node {
                        farm_id: ::core::primitive::u32,
                        resources: runtime_types::tfchain_support::resources::Resources,
                        location: runtime_types::pallet_tfgrid::types::LocationInput<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                        interfaces: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::tfchain_support::types::Interface<
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                        ::core::primitive::u8,
                                    >,
                                >,
                            >,
                        >,
                        secure_boot: ::core::primitive::bool,
                        virtualized: ::core::primitive::bool,
                        serial_number: ::core::option::Option<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    },
                    #[codec(index = 9)]
                    update_node {
                        node_id: ::core::primitive::u32,
                        farm_id: ::core::primitive::u32,
                        resources: runtime_types::tfchain_support::resources::Resources,
                        location: runtime_types::pallet_tfgrid::types::LocationInput<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                        interfaces: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            runtime_types::tfchain_support::types::Interface<
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    ::core::primitive::u8,
                                >,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                        ::core::primitive::u8,
                                    >,
                                >,
                            >,
                        >,
                        secure_boot: ::core::primitive::bool,
                        virtualized: ::core::primitive::bool,
                        serial_number: ::core::option::Option<
                            runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                ::core::primitive::u8,
                            >,
                        >,
                    },
                    #[codec(index = 10)]
                    set_node_certification {
                        node_id: ::core::primitive::u32,
                        node_certification:
                            runtime_types::tfchain_support::types::NodeCertification,
                    },
                    #[codec(index = 11)]
                    report_uptime { uptime: ::core::primitive::u64 },
                    #[codec(index = 12)]
                    add_node_public_config {
                        farm_id: ::core::primitive::u32,
                        node_id: ::core::primitive::u32,
                        public_config: ::core::option::Option<
                            runtime_types::tfchain_support::types::PublicConfig,
                        >,
                    },
                    #[codec(index = 13)]
                    delete_node { id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    create_entity {
                        target: ::subxt::utils::AccountId32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        country: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        city: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 15)]
                    update_entity {
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        country: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        city: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 16)]
                    delete_entity,
                    #[codec(index = 17)]
                    create_twin {
                        ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 18)]
                    update_twin {
                        ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 19)]
                    add_twin_entity {
                        twin_id: ::core::primitive::u32,
                        entity_id: ::core::primitive::u32,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 20)]
                    delete_twin_entity {
                        twin_id: ::core::primitive::u32,
                        entity_id: ::core::primitive::u32,
                    },
                    #[codec(index = 21)]
                    delete_twin { twin_id: ::core::primitive::u32 },
                    #[codec(index = 22)]
                    create_pricing_policy {
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        su: runtime_types::pallet_tfgrid::types::Policy,
                        cu: runtime_types::pallet_tfgrid::types::Policy,
                        nu: runtime_types::pallet_tfgrid::types::Policy,
                        ipu: runtime_types::pallet_tfgrid::types::Policy,
                        unique_name: runtime_types::pallet_tfgrid::types::Policy,
                        domain_name: runtime_types::pallet_tfgrid::types::Policy,
                        foundation_account: ::subxt::utils::AccountId32,
                        certified_sales_account: ::subxt::utils::AccountId32,
                        discount_for_dedication_nodes: ::core::primitive::u8,
                    },
                    #[codec(index = 23)]
                    update_pricing_policy {
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        su: runtime_types::pallet_tfgrid::types::Policy,
                        cu: runtime_types::pallet_tfgrid::types::Policy,
                        nu: runtime_types::pallet_tfgrid::types::Policy,
                        ipu: runtime_types::pallet_tfgrid::types::Policy,
                        unique_name: runtime_types::pallet_tfgrid::types::Policy,
                        domain_name: runtime_types::pallet_tfgrid::types::Policy,
                        foundation_account: ::subxt::utils::AccountId32,
                        certified_sales_account: ::subxt::utils::AccountId32,
                        discount_for_dedication_nodes: ::core::primitive::u8,
                    },
                    #[codec(index = 24)]
                    create_farming_policy {
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        su: ::core::primitive::u32,
                        cu: ::core::primitive::u32,
                        nu: ::core::primitive::u32,
                        ipv4: ::core::primitive::u32,
                        minimal_uptime: ::core::primitive::u16,
                        policy_end: ::core::primitive::u32,
                        immutable: ::core::primitive::bool,
                        default: ::core::primitive::bool,
                        node_certification:
                            runtime_types::tfchain_support::types::NodeCertification,
                        farm_certification:
                            runtime_types::tfchain_support::types::FarmCertification,
                    },
                    #[codec(index = 25)]
                    user_accept_tc {
                        document_link: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        document_hash: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 26)]
                    delete_node_farm { node_id: ::core::primitive::u32 },
                    #[codec(index = 27)]
                    set_farm_dedicated {
                        farm_id: ::core::primitive::u32,
                        dedicated: ::core::primitive::bool,
                    },
                    #[codec(index = 28)]
                    force_reset_farm_ip {
                        farm_id: ::core::primitive::u32,
                        ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 29)]
                    set_connection_price { price: ::core::primitive::u32 },
                    #[codec(index = 30)]
                    add_node_certifier { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 31)]
                    remove_node_certifier { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 32)]
                    update_farming_policy {
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        su: ::core::primitive::u32,
                        cu: ::core::primitive::u32,
                        nu: ::core::primitive::u32,
                        ipv4: ::core::primitive::u32,
                        minimal_uptime: ::core::primitive::u16,
                        policy_end: ::core::primitive::u32,
                        default: ::core::primitive::bool,
                        node_certification:
                            runtime_types::tfchain_support::types::NodeCertification,
                        farm_certification:
                            runtime_types::tfchain_support::types::FarmCertification,
                    },
                    #[codec(index = 33)]
                    attach_policy_to_farm {
                        farm_id: ::core::primitive::u32,
                        limits: ::core::option::Option<
                            runtime_types::tfchain_support::types::FarmingPolicyLimit,
                        >,
                    },
                    #[codec(index = 34)]
                    set_zos_version {
                        zos_version: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    NoneValue,
                    #[codec(index = 1)]
                    StorageOverflow,
                    #[codec(index = 2)]
                    CannotCreateNode,
                    #[codec(index = 3)]
                    NodeNotExists,
                    #[codec(index = 4)]
                    NodeWithTwinIdExists,
                    #[codec(index = 5)]
                    CannotDeleteNode,
                    #[codec(index = 6)]
                    NodeDeleteNotAuthorized,
                    #[codec(index = 7)]
                    NodeUpdateNotAuthorized,
                    #[codec(index = 8)]
                    FarmExists,
                    #[codec(index = 9)]
                    FarmNotExists,
                    #[codec(index = 10)]
                    CannotCreateFarmWrongTwin,
                    #[codec(index = 11)]
                    CannotUpdateFarmWrongTwin,
                    #[codec(index = 12)]
                    CannotDeleteFarm,
                    #[codec(index = 13)]
                    CannotDeleteFarmWithPublicIPs,
                    #[codec(index = 14)]
                    CannotDeleteFarmWithNodesAssigned,
                    #[codec(index = 15)]
                    CannotDeleteFarmWrongTwin,
                    #[codec(index = 16)]
                    IpExists,
                    #[codec(index = 17)]
                    IpNotExists,
                    #[codec(index = 18)]
                    EntityWithNameExists,
                    #[codec(index = 19)]
                    EntityWithPubkeyExists,
                    #[codec(index = 20)]
                    EntityNotExists,
                    #[codec(index = 21)]
                    EntitySignatureDoesNotMatch,
                    #[codec(index = 22)]
                    EntityWithSignatureAlreadyExists,
                    #[codec(index = 23)]
                    CannotUpdateEntity,
                    #[codec(index = 24)]
                    CannotDeleteEntity,
                    #[codec(index = 25)]
                    SignatureLengthIsIncorrect,
                    #[codec(index = 26)]
                    TwinExists,
                    #[codec(index = 27)]
                    TwinNotExists,
                    #[codec(index = 28)]
                    TwinWithPubkeyExists,
                    #[codec(index = 29)]
                    CannotCreateTwin,
                    #[codec(index = 30)]
                    UnauthorizedToUpdateTwin,
                    #[codec(index = 31)]
                    PricingPolicyExists,
                    #[codec(index = 32)]
                    PricingPolicyNotExists,
                    #[codec(index = 33)]
                    PricingPolicyWithDifferentIdExists,
                    #[codec(index = 34)]
                    CertificationCodeExists,
                    #[codec(index = 35)]
                    FarmingPolicyAlreadyExists,
                    #[codec(index = 36)]
                    FarmPayoutAdressAlreadyRegistered,
                    #[codec(index = 37)]
                    FarmerDoesNotHaveEnoughFunds,
                    #[codec(index = 38)]
                    UserDidNotSignTermsAndConditions,
                    #[codec(index = 39)]
                    FarmerDidNotSignTermsAndConditions,
                    #[codec(index = 40)]
                    FarmerNotAuthorized,
                    #[codec(index = 41)]
                    InvalidFarmName,
                    #[codec(index = 42)]
                    AlreadyCertifier,
                    #[codec(index = 43)]
                    NotCertifier,
                    #[codec(index = 44)]
                    NotAllowedToCertifyNode,
                    #[codec(index = 45)]
                    FarmingPolicyNotExists,
                    #[codec(index = 46)]
                    TwinIpTooShort,
                    #[codec(index = 47)]
                    TwinIpTooLong,
                    #[codec(index = 48)]
                    InvalidTwinIp,
                    #[codec(index = 49)]
                    FarmNameTooShort,
                    #[codec(index = 50)]
                    FarmNameTooLong,
                    #[codec(index = 51)]
                    InvalidPublicIP,
                    #[codec(index = 52)]
                    PublicIPTooShort,
                    #[codec(index = 53)]
                    PublicIPTooLong,
                    #[codec(index = 54)]
                    GatewayIPTooShort,
                    #[codec(index = 55)]
                    GatewayIPTooLong,
                    #[codec(index = 56)]
                    IP4TooShort,
                    #[codec(index = 57)]
                    IP4TooLong,
                    #[codec(index = 58)]
                    InvalidIP4,
                    #[codec(index = 59)]
                    GW4TooShort,
                    #[codec(index = 60)]
                    GW4TooLong,
                    #[codec(index = 61)]
                    InvalidGW4,
                    #[codec(index = 62)]
                    IP6TooShort,
                    #[codec(index = 63)]
                    IP6TooLong,
                    #[codec(index = 64)]
                    InvalidIP6,
                    #[codec(index = 65)]
                    GW6TooShort,
                    #[codec(index = 66)]
                    GW6TooLong,
                    #[codec(index = 67)]
                    InvalidGW6,
                    #[codec(index = 68)]
                    DomainTooShort,
                    #[codec(index = 69)]
                    DomainTooLong,
                    #[codec(index = 70)]
                    InvalidDomain,
                    #[codec(index = 71)]
                    MethodIsDeprecated,
                    #[codec(index = 72)]
                    InterfaceNameTooShort,
                    #[codec(index = 73)]
                    InterfaceNameTooLong,
                    #[codec(index = 74)]
                    InvalidInterfaceName,
                    #[codec(index = 75)]
                    InterfaceMacTooShort,
                    #[codec(index = 76)]
                    InterfaceMacTooLong,
                    #[codec(index = 77)]
                    InvalidMacAddress,
                    #[codec(index = 78)]
                    InterfaceIpTooShort,
                    #[codec(index = 79)]
                    InterfaceIpTooLong,
                    #[codec(index = 80)]
                    InvalidInterfaceIP,
                    #[codec(index = 81)]
                    InvalidZosVersion,
                    #[codec(index = 82)]
                    FarmingPolicyExpired,
                    #[codec(index = 83)]
                    InvalidHRUInput,
                    #[codec(index = 84)]
                    InvalidSRUInput,
                    #[codec(index = 85)]
                    InvalidCRUInput,
                    #[codec(index = 86)]
                    InvalidMRUInput,
                    #[codec(index = 87)]
                    LatitudeInputTooShort,
                    #[codec(index = 88)]
                    LatitudeInputTooLong,
                    #[codec(index = 89)]
                    InvalidLatitudeInput,
                    #[codec(index = 90)]
                    LongitudeInputTooShort,
                    #[codec(index = 91)]
                    LongitudeInputTooLong,
                    #[codec(index = 92)]
                    InvalidLongitudeInput,
                    #[codec(index = 93)]
                    CountryNameTooShort,
                    #[codec(index = 94)]
                    CountryNameTooLong,
                    #[codec(index = 95)]
                    InvalidCountryName,
                    #[codec(index = 96)]
                    CityNameTooShort,
                    #[codec(index = 97)]
                    CityNameTooLong,
                    #[codec(index = 98)]
                    InvalidCityName,
                    #[codec(index = 99)]
                    InvalidCountryCityPair,
                    #[codec(index = 100)]
                    SerialNumberTooShort,
                    #[codec(index = 101)]
                    SerialNumberTooLong,
                    #[codec(index = 102)]
                    InvalidSerialNumber,
                    #[codec(index = 103)]
                    DocumentLinkInputTooShort,
                    #[codec(index = 104)]
                    DocumentLinkInputTooLong,
                    #[codec(index = 105)]
                    InvalidDocumentLinkInput,
                    #[codec(index = 106)]
                    DocumentHashInputTooShort,
                    #[codec(index = 107)]
                    DocumentHashInputTooLong,
                    #[codec(index = 108)]
                    InvalidDocumentHashInput,
                    #[codec(index = 109)]
                    InvalidPublicConfig,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    FarmStored(
                        runtime_types::tfchain_support::types::Farm<
                            runtime_types::pallet_tfgrid::farm::FarmName,
                        >,
                    ),
                    #[codec(index = 1)]
                    FarmUpdated(
                        runtime_types::tfchain_support::types::Farm<
                            runtime_types::pallet_tfgrid::farm::FarmName,
                        >,
                    ),
                    #[codec(index = 2)]
                    FarmDeleted(::core::primitive::u32),
                    #[codec(index = 3)]
                    NodeStored(
                        runtime_types::tfchain_support::types::Node<
                            runtime_types::pallet_tfgrid::node::Location,
                            runtime_types::tfchain_support::types::Interface<
                                runtime_types::pallet_tfgrid::interface::InterfaceName,
                                runtime_types::pallet_tfgrid::interface::InterfaceMac,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    runtime_types::pallet_tfgrid::interface::InterfaceIp,
                                >,
                            >,
                            runtime_types::pallet_tfgrid::node::SerialNumber,
                        >,
                    ),
                    #[codec(index = 4)]
                    NodeUpdated(
                        runtime_types::tfchain_support::types::Node<
                            runtime_types::pallet_tfgrid::node::Location,
                            runtime_types::tfchain_support::types::Interface<
                                runtime_types::pallet_tfgrid::interface::InterfaceName,
                                runtime_types::pallet_tfgrid::interface::InterfaceMac,
                                runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                                    runtime_types::pallet_tfgrid::interface::InterfaceIp,
                                >,
                            >,
                            runtime_types::pallet_tfgrid::node::SerialNumber,
                        >,
                    ),
                    #[codec(index = 5)]
                    NodeDeleted(::core::primitive::u32),
                    #[codec(index = 6)]
                    NodeUptimeReported(
                        ::core::primitive::u32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 7)]
                    NodePublicConfigStored(
                        ::core::primitive::u32,
                        ::core::option::Option<runtime_types::tfchain_support::types::PublicConfig>,
                    ),
                    #[codec(index = 8)]
                    EntityStored(
                        runtime_types::pallet_tfgrid::types::Entity<
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_tfgrid::node::CityName,
                            runtime_types::pallet_tfgrid::node::CountryName,
                        >,
                    ),
                    #[codec(index = 9)]
                    EntityUpdated(
                        runtime_types::pallet_tfgrid::types::Entity<
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_tfgrid::node::CityName,
                            runtime_types::pallet_tfgrid::node::CountryName,
                        >,
                    ),
                    #[codec(index = 10)]
                    EntityDeleted(::core::primitive::u32),
                    #[codec(index = 11)]
                    TwinStored(
                        runtime_types::pallet_tfgrid::types::Twin<
                            runtime_types::pallet_tfgrid::twin::TwinIp,
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 12)]
                    TwinUpdated(
                        runtime_types::pallet_tfgrid::types::Twin<
                            runtime_types::pallet_tfgrid::twin::TwinIp,
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 13)]
                    TwinEntityStored(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 14)]
                    TwinEntityRemoved(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 15)]
                    TwinDeleted(::core::primitive::u32),
                    #[codec(index = 16)]
                    PricingPolicyStored(
                        runtime_types::pallet_tfgrid::types::PricingPolicy<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 17)]
                    FarmingPolicyStored(
                        runtime_types::pallet_tfgrid::types::FarmingPolicy<::core::primitive::u32>,
                    ),
                    #[codec(index = 18)]
                    FarmPayoutV2AddressRegistered(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 19)]
                    FarmMarkedAsDedicated(::core::primitive::u32),
                    #[codec(index = 20)]
                    ConnectionPriceSet(::core::primitive::u32),
                    #[codec(index = 21)]
                    NodeCertificationSet(
                        ::core::primitive::u32,
                        runtime_types::tfchain_support::types::NodeCertification,
                    ),
                    #[codec(index = 22)]
                    NodeCertifierAdded(::subxt::utils::AccountId32),
                    #[codec(index = 23)]
                    NodeCertifierRemoved(::subxt::utils::AccountId32),
                    #[codec(index = 24)]
                    FarmingPolicyUpdated(
                        runtime_types::pallet_tfgrid::types::FarmingPolicy<::core::primitive::u32>,
                    ),
                    #[codec(index = 25)]
                    FarmingPolicySet(
                        ::core::primitive::u32,
                        ::core::option::Option<
                            runtime_types::tfchain_support::types::FarmingPolicyLimit,
                        >,
                    ),
                    #[codec(index = 26)]
                    FarmCertificationSet(
                        ::core::primitive::u32,
                        runtime_types::tfchain_support::types::FarmCertification,
                    ),
                    #[codec(index = 27)]
                    ZosVersionUpdated(::std::vec::Vec<::core::primitive::u8>),
                }
            }
            pub mod terms_cond {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct TermsAndConditions {
                    pub account_id: ::subxt::utils::AccountId32,
                    pub timestamp: ::core::primitive::u64,
                    pub document_link: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub document_hash: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
            }
            pub mod twin {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct TwinIp(
                    pub  runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                );
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Entity<_0, _1, _2> {
                    pub version: ::core::primitive::u32,
                    pub id: ::core::primitive::u32,
                    pub name: ::std::vec::Vec<::core::primitive::u8>,
                    pub account_id: _0,
                    pub country: _2,
                    pub city: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct EntityProof {
                    pub entity_id: ::core::primitive::u32,
                    pub signature: ::std::vec::Vec<::core::primitive::u8>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct FarmingPolicy<_0> {
                    pub version: _0,
                    pub id: _0,
                    pub name: ::std::vec::Vec<::core::primitive::u8>,
                    pub cu: _0,
                    pub su: _0,
                    pub nu: _0,
                    pub ipv4: _0,
                    pub minimal_uptime: ::core::primitive::u16,
                    pub policy_created: _0,
                    pub policy_end: _0,
                    pub immutable: ::core::primitive::bool,
                    pub default: ::core::primitive::bool,
                    pub node_certification:
                        runtime_types::tfchain_support::types::NodeCertification,
                    pub farm_certification:
                        runtime_types::tfchain_support::types::FarmCertification,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct LocationInput<_0, _1, _2, _3> {
                    pub city: _0,
                    pub country: _1,
                    pub latitude: _2,
                    pub longitude: _2,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Policy {
                    pub value: ::core::primitive::u32,
                    pub unit: runtime_types::pallet_tfgrid::types::Unit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PricingPolicy<_0> {
                    pub version: ::core::primitive::u32,
                    pub id: ::core::primitive::u32,
                    pub name: ::std::vec::Vec<::core::primitive::u8>,
                    pub su: runtime_types::pallet_tfgrid::types::Policy,
                    pub cu: runtime_types::pallet_tfgrid::types::Policy,
                    pub nu: runtime_types::pallet_tfgrid::types::Policy,
                    pub ipu: runtime_types::pallet_tfgrid::types::Policy,
                    pub unique_name: runtime_types::pallet_tfgrid::types::Policy,
                    pub domain_name: runtime_types::pallet_tfgrid::types::Policy,
                    pub foundation_account: _0,
                    pub certified_sales_account: _0,
                    pub discount_for_dedication_nodes: ::core::primitive::u8,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum StorageVersion {
                    #[codec(index = 0)]
                    V1Struct,
                    #[codec(index = 1)]
                    V2Struct,
                    #[codec(index = 2)]
                    V3Struct,
                    #[codec(index = 3)]
                    V4Struct,
                    #[codec(index = 4)]
                    V5Struct,
                    #[codec(index = 5)]
                    V6Struct,
                    #[codec(index = 6)]
                    V7Struct,
                    #[codec(index = 7)]
                    V8Struct,
                    #[codec(index = 8)]
                    V9Struct,
                    #[codec(index = 9)]
                    V10Struct,
                    #[codec(index = 10)]
                    V11Struct,
                    #[codec(index = 11)]
                    V12Struct,
                    #[codec(index = 12)]
                    V13Struct,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Twin<_0, _1> {
                    pub version: ::core::primitive::u32,
                    pub id: ::core::primitive::u32,
                    pub account_id: _1,
                    pub ip: _0,
                    pub entities: ::std::vec::Vec<runtime_types::pallet_tfgrid::types::EntityProof>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum Unit {
                    #[codec(index = 0)]
                    Bytes,
                    #[codec(index = 1)]
                    Kilobytes,
                    #[codec(index = 2)]
                    Megabytes,
                    #[codec(index = 3)]
                    Gigabytes,
                    #[codec(index = 4)]
                    Terrabytes,
                }
            }
        }
        pub mod pallet_tft_bridge {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    add_bridge_validator { target: ::subxt::utils::AccountId32 },
                    #[codec(index = 1)]
                    remove_bridge_validator { target: ::subxt::utils::AccountId32 },
                    #[codec(index = 2)]
                    set_fee_account { target: ::subxt::utils::AccountId32 },
                    #[codec(index = 3)]
                    set_withdraw_fee { amount: ::core::primitive::u64 },
                    #[codec(index = 4)]
                    set_deposit_fee { amount: ::core::primitive::u64 },
                    #[codec(index = 5)]
                    swap_to_stellar {
                        target_stellar_address: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    propose_or_vote_mint_transaction {
                        transaction: ::std::vec::Vec<::core::primitive::u8>,
                        target: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 7)]
                    propose_burn_transaction_or_add_sig {
                        transaction_id: ::core::primitive::u64,
                        target: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u64,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                        stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                        sequence_number: ::core::primitive::u64,
                    },
                    #[codec(index = 8)]
                    set_burn_transaction_executed {
                        transaction_id: ::core::primitive::u64,
                    },
                    #[codec(index = 9)]
                    create_refund_transaction_or_add_sig {
                        tx_hash: ::std::vec::Vec<::core::primitive::u8>,
                        target: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u64,
                        signature: ::std::vec::Vec<::core::primitive::u8>,
                        stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
                        sequence_number: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    set_refund_transaction_executed {
                        tx_hash: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    ValidatorExists,
                    #[codec(index = 1)]
                    ValidatorNotExists,
                    #[codec(index = 2)]
                    TransactionValidatorExists,
                    #[codec(index = 3)]
                    TransactionValidatorNotExists,
                    #[codec(index = 4)]
                    MintTransactionExists,
                    #[codec(index = 5)]
                    MintTransactionAlreadyExecuted,
                    #[codec(index = 6)]
                    MintTransactionNotExists,
                    #[codec(index = 7)]
                    BurnTransactionExists,
                    #[codec(index = 8)]
                    BurnTransactionNotExists,
                    #[codec(index = 9)]
                    BurnSignatureExists,
                    #[codec(index = 10)]
                    EnoughBurnSignaturesPresent,
                    #[codec(index = 11)]
                    RefundSignatureExists,
                    #[codec(index = 12)]
                    BurnTransactionAlreadyExecuted,
                    #[codec(index = 13)]
                    RefundTransactionNotExists,
                    #[codec(index = 14)]
                    RefundTransactionAlreadyExecuted,
                    #[codec(index = 15)]
                    EnoughRefundSignaturesPresent,
                    #[codec(index = 16)]
                    NotEnoughBalanceToSwap,
                    #[codec(index = 17)]
                    AmountIsLessThanWithdrawFee,
                    #[codec(index = 18)]
                    AmountIsLessThanDepositFee,
                    #[codec(index = 19)]
                    WrongParametersProvided,
                    #[codec(index = 20)]
                    InvalidStellarPublicKey,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    MintTransactionProposed(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 1)]
                    MintTransactionVoted(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 2)]
                    MintCompleted(
                        runtime_types::pallet_tft_bridge::MintTransaction<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u32,
                        >,
                    ),
                    #[codec(index = 3)]
                    MintTransactionExpired(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                        ::subxt::utils::AccountId32,
                    ),
                    #[codec(index = 4)]
                    BurnTransactionCreated(
                        ::core::primitive::u64,
                        ::subxt::utils::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 5)]
                    BurnTransactionProposed(
                        ::core::primitive::u64,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    BurnTransactionSignatureAdded(
                        ::core::primitive::u64,
                        runtime_types::pallet_tft_bridge::StellarSignature,
                    ),
                    #[codec(index = 7)]
                    BurnTransactionReady(::core::primitive::u64),
                    #[codec(index = 8)]
                    BurnTransactionProcessed(
                        runtime_types::pallet_tft_bridge::BurnTransaction<::core::primitive::u32>,
                    ),
                    #[codec(index = 9)]
                    BurnTransactionExpired(
                        ::core::primitive::u64,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    RefundTransactionCreated(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 11)]
                    RefundTransactionsignatureAdded(
                        ::std::vec::Vec<::core::primitive::u8>,
                        runtime_types::pallet_tft_bridge::StellarSignature,
                    ),
                    #[codec(index = 12)]
                    RefundTransactionReady(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 13)]
                    RefundTransactionProcessed(
                        runtime_types::pallet_tft_bridge::RefundTransaction<::core::primitive::u32>,
                    ),
                    #[codec(index = 14)]
                    RefundTransactionExpired(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u64,
                    ),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct BurnTransaction<_0> {
                pub block: _0,
                pub amount: ::core::primitive::u64,
                pub target: ::std::vec::Vec<::core::primitive::u8>,
                pub signatures: ::std::vec::Vec<runtime_types::pallet_tft_bridge::StellarSignature>,
                pub sequence_number: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct MintTransaction<_0, _1> {
                pub amount: ::core::primitive::u64,
                pub target: _0,
                pub block: _1,
                pub votes: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RefundTransaction<_0> {
                pub block: _0,
                pub amount: ::core::primitive::u64,
                pub target: ::std::vec::Vec<::core::primitive::u8>,
                pub tx_hash: ::std::vec::Vec<::core::primitive::u8>,
                pub signatures: ::std::vec::Vec<runtime_types::pallet_tft_bridge::StellarSignature>,
                pub sequence_number: ::core::primitive::u64,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct StellarSignature {
                pub signature: ::std::vec::Vec<::core::primitive::u8>,
                pub stellar_pub_key: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_tft_price {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    set_prices {
                        price: ::core::primitive::u32,
                        block_number: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    set_min_tft_price { price: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    set_max_tft_price { price: ::core::primitive::u32 },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    ErrFetchingPrice,
                    #[codec(index = 1)]
                    OffchainSignedTxError,
                    #[codec(index = 2)]
                    NoLocalAcctForSigning,
                    #[codec(index = 3)]
                    AccountUnauthorizedToSetPrice,
                    #[codec(index = 4)]
                    MaxPriceBelowMinPriceError,
                    #[codec(index = 5)]
                    MinPriceAboveMaxPriceError,
                    #[codec(index = 6)]
                    IsNotAnAuthority,
                    #[codec(index = 7)]
                    WrongAuthority,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    PriceStored(::core::primitive::u32),
                    #[codec(index = 1)]
                    OffchainWorkerExecuted(::subxt::utils::AccountId32),
                    #[codec(index = 2)]
                    AveragePriceStored(::core::primitive::u32),
                    #[codec(index = 3)]
                    AveragePriceIsAboveMaxPrice(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 4)]
                    AveragePriceIsBelowMinPrice(::core::primitive::u32, ::core::primitive::u32),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                    #[doc = "# </weight>"]
                    #[doc = ""]
                    #[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
                    #[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
                    #[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
                    #[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
                    #[doc = "event is deposited."]
                    batch {
                        calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Send a call through an indexed pseudonym of the sender."]
                    #[doc = ""]
                    #[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
                    #[doc = "use the same filter as the origin of this call."]
                    #[doc = ""]
                    #[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
                    #[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
                    #[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
                    #[doc = "in the Multisig pallet instead."]
                    #[doc = ""]
                    #[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Send a batch of dispatch calls and atomically execute them."]
                    #[doc = "The whole transaction will rollback and fail if any of the calls failed."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatched without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                    #[doc = "# </weight>"]
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Dispatches a function call with a provided origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
                    #[doc = "# </weight>"]
                    dispatch_as {
                        as_origin: ::std::boxed::Box<runtime_types::tfchain_runtime::OriginCaller>,
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Send a batch of dispatch calls."]
                    #[doc = "Unlike `batch`, it allows errors and won't interrupt."]
                    #[doc = ""]
                    #[doc = "May be called from any origin except `None`."]
                    #[doc = ""]
                    #[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
                    #[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
                    #[doc = ""]
                    #[doc = "If origin is root then the calls are dispatch without checking origin filter. (This"]
                    #[doc = "includes bypassing `frame_system::Config::BaseCallFilter`)."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
                    #[doc = "# </weight>"]
                    force_batch {
                        calls: ::std::vec::Vec<runtime_types::tfchain_runtime::RuntimeCall>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Dispatch a function call with a specified weight."]
                    #[doc = ""]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Root origin to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Root_."]
                    with_weight {
                        call: ::std::boxed::Box<runtime_types::tfchain_runtime::RuntimeCall>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many calls batched."]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
                    #[doc = "well as the error."]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    #[doc = "Batch of dispatches completed fully with no error."]
                    BatchCompleted,
                    #[codec(index = 2)]
                    #[doc = "Batch of dispatches completed but has errors."]
                    BatchCompletedWithErrors,
                    #[codec(index = 3)]
                    #[doc = "A single item within a Batch of dispatches has completed with no error."]
                    ItemCompleted,
                    #[codec(index = 4)]
                    #[doc = "A single item within a Batch of dispatches has completed with error."]
                    ItemFailed {
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 5)]
                    #[doc = "A call was dispatched."]
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_validator {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Create a request to become a validator"]
                    #[doc = "Validator account (signer): the account of the validator (this account will be added to the council)"]
                    #[doc = "Validator node account: the account that will validate on consensus layer"]
                    #[doc = "Stash account: the \"bank\" account of the validator (where rewards should be sent to) the stash should be bonded to a validator"]
                    #[doc = "Description: why someone wants to become a validator"]
                    #[doc = "Tf Connect ID: the threefold connect ID of the person who wants to become a validator"]
                    #[doc = "Info: some public info about the validator (website link, blog link, ..)"]
                    #[doc = "A user can only have 1 validator request at a time"]
                    create_validator_request {
                        validator_node_account: ::subxt::utils::AccountId32,
                        stash_account: ::subxt::utils::AccountId32,
                        description: ::std::vec::Vec<::core::primitive::u8>,
                        tf_connect_id: ::std::vec::Vec<::core::primitive::u8>,
                        info: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Start participating in consensus"]
                    #[doc = "Will activate the Validator node account on consensus level"]
                    #[doc = "A user can only call this if his request to be a validator is approved by the council"]
                    #[doc = "Should be called when his node is synced and ready to start validating"]
                    activate_validator_node,
                    #[codec(index = 2)]
                    #[doc = "Change validator node account"]
                    #[doc = "In case the Validator wishes to change his validator node account"]
                    #[doc = "he can call this method with the new node validator account"]
                    #[doc = "this new account will be added as a new consensus validator if he is validating already"]
                    change_validator_node_account {
                        new_node_validator_account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    #[doc = "Bond an account to a validator account"]
                    #[doc = "Just proves that the stash account is indeed under control of the validator account"]
                    bond {
                        validator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Approve validator (council)"]
                    #[doc = "Approves a validator to be added as a council member and"]
                    #[doc = "to participate in consensus"]
                    approve_validator {
                        validator_account:
                            ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Remove validator"]
                    #[doc = "Removes a validator from:"]
                    #[doc = "1. Council"]
                    #[doc = "2. Storage"]
                    #[doc = "3. Consensus"]
                    #[doc = "Can only be called by the user or the council"]
                    remove_validator {
                        validator_account:
                            ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    BadOrigin,
                    #[codec(index = 1)]
                    NotCouncilMember,
                    #[codec(index = 2)]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    StashNotBonded,
                    #[codec(index = 4)]
                    StashBondedWithWrongValidator,
                    #[codec(index = 5)]
                    CannotBondWithSameAccount,
                    #[codec(index = 6)]
                    DuplicateValidator,
                    #[codec(index = 7)]
                    ValidatorNotFound,
                    #[codec(index = 8)]
                    ValidatorNotApproved,
                    #[codec(index = 9)]
                    UnauthorizedToActivateValidator,
                    #[codec(index = 10)]
                    ValidatorValidatingAlready,
                    #[codec(index = 11)]
                    ValidatorNotValidating,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    Bonded(::subxt::utils::AccountId32),
                    #[codec(index = 1)]
                    ValidatorRequestCreated(
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_validator::types::Validator<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 2)]
                    ValidatorRequestApproved(
                        runtime_types::pallet_validator::types::Validator<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 3)]
                    ValidatorActivated(
                        runtime_types::pallet_validator::types::Validator<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 4)]
                    ValidatorRemoved(
                        runtime_types::pallet_validator::types::Validator<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    #[codec(index = 5)]
                    NodeValidatorChanged(::subxt::utils::AccountId32),
                    #[codec(index = 6)]
                    NodeValidatorRemoved(::subxt::utils::AccountId32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Validator<_0> {
                    pub validator_node_account: _0,
                    pub stash_account: _0,
                    pub description: ::std::vec::Vec<::core::primitive::u8>,
                    pub tf_connect_id: ::std::vec::Vec<::core::primitive::u8>,
                    pub info: ::std::vec::Vec<::core::primitive::u8>,
                    pub state: runtime_types::pallet_validator::types::ValidatorRequestState,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum ValidatorRequestState {
                    #[codec(index = 0)]
                    Created,
                    #[codec(index = 1)]
                    Approved,
                    #[codec(index = 2)]
                    Validating,
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    Debug,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod bounded {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::utils::H256,
                        pub extrinsics_root: ::subxt::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        Debug,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct BlakeTwo256;
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                Debug,
            )]
            pub struct OldWeight(pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod substrate_validator_set {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a new validator."]
                    #[doc = ""]
                    #[doc = "New validator's session keys should be set in Session pallet before"]
                    #[doc = "calling this."]
                    #[doc = ""]
                    #[doc = "The origin can be configured using the `AddRemoveOrigin` type in the"]
                    #[doc = "host runtime. Can also be set to sudo/root."]
                    add_validator {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Remove a validator."]
                    #[doc = ""]
                    #[doc = "The origin can be configured using the `AddRemoveOrigin` type in the"]
                    #[doc = "host runtime. Can also be set to sudo/root."]
                    remove_validator {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Add an approved validator again when it comes back online."]
                    #[doc = ""]
                    #[doc = "For this call, the dispatch origin must be the validator itself."]
                    add_validator_again {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Target (post-removal) validator count is below the minimum."]
                    TooLowValidatorCount,
                    #[codec(index = 1)]
                    #[doc = "Validator is already in the validator set."]
                    Duplicate,
                    #[codec(index = 2)]
                    #[doc = "Validator is not approved for re-addition."]
                    ValidatorNotApproved,
                    #[codec(index = 3)]
                    #[doc = "Only the validator can add itself back after coming online."]
                    BadOrigin,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New validator addition initiated. Effective in ~2 sessions."]
                    ValidatorAdditionInitiated(::subxt::utils::AccountId32),
                    #[codec(index = 1)]
                    #[doc = "Validator removal initiated. Effective in ~2 sessions."]
                    ValidatorRemovalInitiated(::subxt::utils::AccountId32),
                }
            }
        }
        pub mod tfchain_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct SessionKeys {
                    pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<::subxt::utils::AccountId32>,
                ),
                #[codec(index = 18)]
                Council(runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>),
                #[codec(index = 2)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 4)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Call),
                #[codec(index = 5)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 7)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 9)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 10)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 11)]
                TfgridModule(runtime_types::pallet_tfgrid::pallet::Call),
                #[codec(index = 12)]
                SmartContractModule(runtime_types::pallet_smart_contract::pallet::Call),
                #[codec(index = 13)]
                TFTBridgeModule(runtime_types::pallet_tft_bridge::pallet::Call),
                #[codec(index = 14)]
                TFTPriceModule(runtime_types::pallet_tft_price::pallet::Call),
                #[codec(index = 15)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 16)]
                BurningModule(runtime_types::pallet_burning::pallet::Call),
                #[codec(index = 17)]
                TFKVStore(runtime_types::pallet_kvstore::pallet::Call),
                #[codec(index = 18)]
                Council(runtime_types::pallet_collective::pallet::Call),
                #[codec(index = 19)]
                CouncilMembership(runtime_types::pallet_membership::pallet::Call),
                #[codec(index = 20)]
                RuntimeUpgrade(runtime_types::pallet_runtime_upgrade::pallet::Call),
                #[codec(index = 21)]
                Validator(runtime_types::pallet_validator::pallet::Call),
                #[codec(index = 22)]
                Dao(runtime_types::pallet_dao::pallet::Call),
                #[codec(index = 23)]
                Utility(runtime_types::pallet_utility::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
            )]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 4)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Event),
                #[codec(index = 5)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 7)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 8)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 9)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 11)]
                TfgridModule(runtime_types::pallet_tfgrid::pallet::Event),
                #[codec(index = 12)]
                SmartContractModule(runtime_types::pallet_smart_contract::pallet::Event),
                #[codec(index = 13)]
                TFTBridgeModule(runtime_types::pallet_tft_bridge::pallet::Event),
                #[codec(index = 14)]
                TFTPriceModule(runtime_types::pallet_tft_price::pallet::Event),
                #[codec(index = 15)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                #[codec(index = 16)]
                BurningModule(runtime_types::pallet_burning::pallet::Event),
                #[codec(index = 17)]
                TFKVStore(runtime_types::pallet_kvstore::pallet::Event),
                #[codec(index = 18)]
                Council(runtime_types::pallet_collective::pallet::Event),
                #[codec(index = 19)]
                CouncilMembership(runtime_types::pallet_membership::pallet::Event),
                #[codec(index = 21)]
                Validator(runtime_types::pallet_validator::pallet::Event),
                #[codec(index = 22)]
                Dao(runtime_types::pallet_dao::pallet::Event),
                #[codec(index = 23)]
                Utility(runtime_types::pallet_utility::pallet::Event),
            }
        }
        pub mod tfchain_support {
            use super::runtime_types;
            pub mod resources {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Resources {
                    pub hru: ::core::primitive::u64,
                    pub sru: ::core::primitive::u64,
                    pub cru: ::core::primitive::u64,
                    pub mru: ::core::primitive::u64,
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Farm<_0> {
                    pub version: ::core::primitive::u32,
                    pub id: ::core::primitive::u32,
                    pub name: _0,
                    pub twin_id: ::core::primitive::u32,
                    pub pricing_policy_id: ::core::primitive::u32,
                    pub certification: runtime_types::tfchain_support::types::FarmCertification,
                    pub public_ips: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        runtime_types::tfchain_support::types::PublicIP,
                    >,
                    pub dedicated_farm: ::core::primitive::bool,
                    pub farming_policy_limits: ::core::option::Option<
                        runtime_types::tfchain_support::types::FarmingPolicyLimit,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum FarmCertification {
                    #[codec(index = 0)]
                    NotCertified,
                    #[codec(index = 1)]
                    Gold,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct FarmingPolicyLimit {
                    pub farming_policy_id: ::core::primitive::u32,
                    pub cu: ::core::option::Option<::core::primitive::u64>,
                    pub su: ::core::option::Option<::core::primitive::u64>,
                    pub end: ::core::option::Option<::core::primitive::u64>,
                    pub node_count: ::core::option::Option<::core::primitive::u32>,
                    pub node_certification: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct IP4 {
                    pub ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub gw: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct IP6 {
                    pub ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub gw: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Interface<_0, _1, _2> {
                    pub name: _0,
                    pub mac: _1,
                    pub ips: _2,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct Node<_0, _1, _2> {
                    pub version: ::core::primitive::u32,
                    pub id: ::core::primitive::u32,
                    pub farm_id: ::core::primitive::u32,
                    pub twin_id: ::core::primitive::u32,
                    pub resources: runtime_types::tfchain_support::resources::Resources,
                    pub location: _0,
                    pub public_config:
                        ::core::option::Option<runtime_types::tfchain_support::types::PublicConfig>,
                    pub created: ::core::primitive::u64,
                    pub farming_policy_id: ::core::primitive::u32,
                    pub interfaces: ::std::vec::Vec<_1>,
                    pub certification: runtime_types::tfchain_support::types::NodeCertification,
                    pub secure_boot: ::core::primitive::bool,
                    pub virtualized: ::core::primitive::bool,
                    pub serial_number: ::core::option::Option<_2>,
                    pub connection_price: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub enum NodeCertification {
                    #[codec(index = 0)]
                    Diy,
                    #[codec(index = 1)]
                    Certified,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PublicConfig {
                    pub ip4: runtime_types::tfchain_support::types::IP4,
                    pub ip6: ::core::option::Option<runtime_types::tfchain_support::types::IP6>,
                    pub domain: ::core::option::Option<
                        runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
                )]
                pub struct PublicIP {
                    pub ip: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub gateway: runtime_types::sp_core::bounded::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub contract_id: ::core::primitive::u64,
                }
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue,"]
    #[doc = r" exposed here for ease of use."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi {
            authorship::constants::ConstantsApi
        }
        pub fn tfgrid_module(&self) -> tfgrid_module::constants::ConstantsApi {
            tfgrid_module::constants::ConstantsApi
        }
        pub fn smart_contract_module(&self) -> smart_contract_module::constants::ConstantsApi {
            smart_contract_module::constants::ConstantsApi
        }
        pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
            scheduler::constants::ConstantsApi
        }
        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn validator_set(&self) -> validator_set::storage::StorageApi {
            validator_set::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi {
            authorship::storage::StorageApi
        }
        pub fn tfgrid_module(&self) -> tfgrid_module::storage::StorageApi {
            tfgrid_module::storage::StorageApi
        }
        pub fn smart_contract_module(&self) -> smart_contract_module::storage::StorageApi {
            smart_contract_module::storage::StorageApi
        }
        pub fn tft_bridge_module(&self) -> tft_bridge_module::storage::StorageApi {
            tft_bridge_module::storage::StorageApi
        }
        pub fn tft_price_module(&self) -> tft_price_module::storage::StorageApi {
            tft_price_module::storage::StorageApi
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi {
            scheduler::storage::StorageApi
        }
        pub fn burning_module(&self) -> burning_module::storage::StorageApi {
            burning_module::storage::StorageApi
        }
        pub fn tfkv_store(&self) -> tfkv_store::storage::StorageApi {
            tfkv_store::storage::StorageApi
        }
        pub fn council(&self) -> council::storage::StorageApi {
            council::storage::StorageApi
        }
        pub fn council_membership(&self) -> council_membership::storage::StorageApi {
            council_membership::storage::StorageApi
        }
        pub fn validator(&self) -> validator::storage::StorageApi {
            validator::storage::StorageApi
        }
        pub fn dao(&self) -> dao::storage::StorageApi {
            dao::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn validator_set(&self) -> validator_set::calls::TransactionApi {
            validator_set::calls::TransactionApi
        }
        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi {
            authorship::calls::TransactionApi
        }
        pub fn tfgrid_module(&self) -> tfgrid_module::calls::TransactionApi {
            tfgrid_module::calls::TransactionApi
        }
        pub fn smart_contract_module(&self) -> smart_contract_module::calls::TransactionApi {
            smart_contract_module::calls::TransactionApi
        }
        pub fn tft_bridge_module(&self) -> tft_bridge_module::calls::TransactionApi {
            tft_bridge_module::calls::TransactionApi
        }
        pub fn tft_price_module(&self) -> tft_price_module::calls::TransactionApi {
            tft_price_module::calls::TransactionApi
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi {
            scheduler::calls::TransactionApi
        }
        pub fn burning_module(&self) -> burning_module::calls::TransactionApi {
            burning_module::calls::TransactionApi
        }
        pub fn tfkv_store(&self) -> tfkv_store::calls::TransactionApi {
            tfkv_store::calls::TransactionApi
        }
        pub fn council(&self) -> council::calls::TransactionApi {
            council::calls::TransactionApi
        }
        pub fn council_membership(&self) -> council_membership::calls::TransactionApi {
            council_membership::calls::TransactionApi
        }
        pub fn runtime_upgrade(&self) -> runtime_upgrade::calls::TransactionApi {
            runtime_upgrade::calls::TransactionApi
        }
        pub fn validator(&self) -> validator::calls::TransactionApi {
            validator::calls::TransactionApi
        }
        pub fn dao(&self) -> dao::calls::TransactionApi {
            dao::calls::TransactionApi
        }
        pub fn utility(&self) -> utility::calls::TransactionApi {
            utility::calls::TransactionApi
        }
    }
    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
        client: &C,
    ) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
        if runtime_metadata_hash
            != [
                5u8, 77u8, 217u8, 211u8, 6u8, 129u8, 250u8, 91u8, 203u8, 199u8, 153u8, 227u8,
                254u8, 62u8, 2u8, 152u8, 161u8, 94u8, 104u8, 85u8, 37u8, 233u8, 224u8, 211u8,
                237u8, 228u8, 84u8, 226u8, 200u8, 130u8, 179u8, 208u8,
            ]
        {
            Err(::subxt::error::MetadataError::IncompatibleMetadata)
        } else {
            Ok(())
        }
    }
}
