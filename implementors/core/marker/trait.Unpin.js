(function() {var implementors = {};
implementors["account"] = [{"text":"impl Unpin for EthereumSignature","synthetic":true,"types":[]},{"text":"impl Unpin for EthereumSigner","synthetic":true,"types":[]}];
implementors["moonbase_runtime"] = [{"text":"impl Unpin for SessionKeys","synthetic":true,"types":[]},{"text":"impl Unpin for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Unpin for Version","synthetic":true,"types":[]},{"text":"impl Unpin for BlockWeights","synthetic":true,"types":[]},{"text":"impl Unpin for BlockLength","synthetic":true,"types":[]},{"text":"impl Unpin for SS58Prefix","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MaxLocks","synthetic":true,"types":[]},{"text":"impl Unpin for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Unpin for DealWithFees&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Unpin for MoonbeamGasWeightMapping","synthetic":true,"types":[]},{"text":"impl Unpin for BlockGasLimit","synthetic":true,"types":[]},{"text":"impl Unpin for FixedGasPrice","synthetic":true,"types":[]},{"text":"impl Unpin for MaximumSchedulerWeight","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for LaunchPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for VotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for FastTrackVotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for EnactmentPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CooloffPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for MaxVotes","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for PreimageByteDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for InstantAllowed","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBond","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBondMinimum","synthetic":true,"types":[]},{"text":"impl Unpin for SpendPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CommunityTreasuryId","synthetic":true,"types":[]},{"text":"impl Unpin for ParachainBondPalletId","synthetic":true,"types":[]},{"text":"impl Unpin for MaxApprovals","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionConverter","synthetic":true,"types":[]},{"text":"impl Unpin for ReservedXcmpWeight","synthetic":true,"types":[]},{"text":"impl Unpin for MinBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for BondDuration","synthetic":true,"types":[]},{"text":"impl Unpin for MinSelectedCandidates","synthetic":true,"types":[]},{"text":"impl Unpin for MaxNominatorsPerCollator","synthetic":true,"types":[]},{"text":"impl Unpin for MaxCollatorsPerNominator","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultCollatorCommission","synthetic":true,"types":[]},{"text":"impl Unpin for MinCollatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for MinNominatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for VestingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumReward","synthetic":true,"types":[]},{"text":"impl Unpin for Initialized","synthetic":true,"types":[]},{"text":"impl Unpin for InitializationPayment","synthetic":true,"types":[]},{"text":"impl Unpin for DepositAmount","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProxies","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxPending","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyType","synthetic":true,"types":[]},{"text":"impl Unpin for Runtime","synthetic":true,"types":[]},{"text":"impl Unpin for Event","synthetic":true,"types":[]},{"text":"impl Unpin for Origin","synthetic":true,"types":[]},{"text":"impl Unpin for OriginCaller","synthetic":true,"types":[]},{"text":"impl Unpin for PalletInfo","synthetic":true,"types":[]},{"text":"impl Unpin for Call","synthetic":true,"types":[]},{"text":"impl Unpin for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Unpin for RuntimeApi","synthetic":true,"types":[]},{"text":"impl&lt;Block, C&gt; Unpin for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Block as Block&gt;::Hash: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing: Hasher + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Number: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;C as CallApiAt&lt;Block&gt;&gt;::StateBackend as Backend&lt;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing&gt;&gt;::Transaction: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_cli"] = [{"text":"impl Unpin for Subcommand","synthetic":true,"types":[]},{"text":"impl Unpin for BuildSpecCommand","synthetic":true,"types":[]},{"text":"impl Unpin for ExportGenesisStateCommand","synthetic":true,"types":[]},{"text":"impl Unpin for ExportGenesisWasmCommand","synthetic":true,"types":[]},{"text":"impl Unpin for RunCmd","synthetic":true,"types":[]},{"text":"impl Unpin for Cli","synthetic":true,"types":[]},{"text":"impl Unpin for RelayChainCli","synthetic":true,"types":[]}];
implementors["moonbeam_cli_opt"] = [{"text":"impl Unpin for Sealing","synthetic":true,"types":[]},{"text":"impl Unpin for EthApi","synthetic":true,"types":[]},{"text":"impl Unpin for RpcParams","synthetic":true,"types":[]}];
implementors["moonbeam_evm_tracer"] = [{"text":"impl Unpin for CallListTracer","synthetic":true,"types":[]},{"text":"impl Unpin for RawTracer","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_core_debug"] = [{"text":"impl Unpin for TraceParams","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_core_trace"] = [{"text":"impl Unpin for FilterRequest","synthetic":true,"types":[]},{"text":"impl Unpin for RequestBlockId","synthetic":true,"types":[]},{"text":"impl Unpin for RequestBlockTag","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_core_txpool"] = [{"text":"impl Unpin for Transaction","synthetic":true,"types":[]},{"text":"impl Unpin for Summary","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for TxPoolResult&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_debug"] = [{"text":"impl Unpin for Debug","synthetic":true,"types":[]},{"text":"impl&lt;B, C, BE&gt; Unpin for DebugHandler&lt;B, C, BE&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;BE: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_primitives_debug"] = [{"text":"impl Unpin for TransactionTrace","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionTraceAction","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionTraceOutput","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionTraceResult","synthetic":true,"types":[]},{"text":"impl Unpin for TraceType","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionTrace","synthetic":true,"types":[]},{"text":"impl Unpin for RawStepLog","synthetic":true,"types":[]},{"text":"impl Unpin for CallInner","synthetic":true,"types":[]},{"text":"impl Unpin for Call","synthetic":true,"types":[]},{"text":"impl Unpin for CallResult","synthetic":true,"types":[]},{"text":"impl Unpin for CreateResult","synthetic":true,"types":[]},{"text":"impl Unpin for CallType","synthetic":true,"types":[]},{"text":"impl Unpin for CreateType","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_primitives_txpool"] = [{"text":"impl Unpin for TxPoolResponse","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_trace"] = [{"text":"impl&lt;B, C&gt; Unpin for Trace&lt;B, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for CacheBatchId","synthetic":true,"types":[]},{"text":"impl Unpin for CacheRequester","synthetic":true,"types":[]},{"text":"impl&lt;B, C, BE&gt; Unpin for CacheTask&lt;B, C, BE&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_rpc_txpool"] = [{"text":"impl&lt;B, C, A&gt; Unpin for TxPool&lt;B, C, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_runtime"] = [{"text":"impl Unpin for SessionKeys","synthetic":true,"types":[]},{"text":"impl Unpin for BaseFilter","synthetic":true,"types":[]},{"text":"impl Unpin for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Unpin for Version","synthetic":true,"types":[]},{"text":"impl Unpin for BlockWeights","synthetic":true,"types":[]},{"text":"impl Unpin for BlockLength","synthetic":true,"types":[]},{"text":"impl Unpin for SS58Prefix","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MaxLocks","synthetic":true,"types":[]},{"text":"impl Unpin for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Unpin for DealWithFees&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Unpin for MoonbeamGasWeightMapping","synthetic":true,"types":[]},{"text":"impl Unpin for BlockGasLimit","synthetic":true,"types":[]},{"text":"impl Unpin for FixedGasPrice","synthetic":true,"types":[]},{"text":"impl Unpin for MaximumSchedulerWeight","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for LaunchPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for VotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for FastTrackVotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for EnactmentPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CooloffPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for MaxVotes","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for PreimageByteDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for InstantAllowed","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBond","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBondMinimum","synthetic":true,"types":[]},{"text":"impl Unpin for SpendPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CommunityTreasuryId","synthetic":true,"types":[]},{"text":"impl Unpin for ParachainBondPalletId","synthetic":true,"types":[]},{"text":"impl Unpin for MaxApprovals","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionConverter","synthetic":true,"types":[]},{"text":"impl Unpin for ReservedXcmpWeight","synthetic":true,"types":[]},{"text":"impl Unpin for MinBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for BondDuration","synthetic":true,"types":[]},{"text":"impl Unpin for MinSelectedCandidates","synthetic":true,"types":[]},{"text":"impl Unpin for MaxNominatorsPerCollator","synthetic":true,"types":[]},{"text":"impl Unpin for MaxCollatorsPerNominator","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultCollatorCommission","synthetic":true,"types":[]},{"text":"impl Unpin for MinCollatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for MinNominatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for VestingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumReward","synthetic":true,"types":[]},{"text":"impl Unpin for Initialized","synthetic":true,"types":[]},{"text":"impl Unpin for InitializationPayment","synthetic":true,"types":[]},{"text":"impl Unpin for DepositAmount","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProxies","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxPending","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyType","synthetic":true,"types":[]},{"text":"impl Unpin for Runtime","synthetic":true,"types":[]},{"text":"impl Unpin for Event","synthetic":true,"types":[]},{"text":"impl Unpin for Origin","synthetic":true,"types":[]},{"text":"impl Unpin for OriginCaller","synthetic":true,"types":[]},{"text":"impl Unpin for PalletInfo","synthetic":true,"types":[]},{"text":"impl Unpin for Call","synthetic":true,"types":[]},{"text":"impl Unpin for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Unpin for RuntimeApi","synthetic":true,"types":[]},{"text":"impl&lt;Block, C&gt; Unpin for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Block as Block&gt;::Hash: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing: Hasher + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Number: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;C as CallApiAt&lt;Block&gt;&gt;::StateBackend as Backend&lt;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing&gt;&gt;::Transaction: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonbeam_service"] = [{"text":"impl Unpin for Extensions","synthetic":true,"types":[]},{"text":"impl Unpin for ExtensionsFork","synthetic":true,"types":[]},{"text":"impl Unpin for Client","synthetic":true,"types":[]},{"text":"impl Unpin for MoonbeamExecutor","synthetic":true,"types":[]},{"text":"impl Unpin for MoonriverExecutor","synthetic":true,"types":[]},{"text":"impl Unpin for MoonshadowExecutor","synthetic":true,"types":[]},{"text":"impl Unpin for MoonbaseExecutor","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionConverters","synthetic":true,"types":[]}];
implementors["moonkey"] = [{"text":"impl Unpin for Opts","synthetic":true,"types":[]}];
implementors["moonriver_runtime"] = [{"text":"impl Unpin for SessionKeys","synthetic":true,"types":[]},{"text":"impl Unpin for BaseFilter","synthetic":true,"types":[]},{"text":"impl Unpin for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Unpin for Version","synthetic":true,"types":[]},{"text":"impl Unpin for BlockWeights","synthetic":true,"types":[]},{"text":"impl Unpin for BlockLength","synthetic":true,"types":[]},{"text":"impl Unpin for SS58Prefix","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MaxLocks","synthetic":true,"types":[]},{"text":"impl Unpin for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Unpin for DealWithFees&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Unpin for MoonbeamGasWeightMapping","synthetic":true,"types":[]},{"text":"impl Unpin for BlockGasLimit","synthetic":true,"types":[]},{"text":"impl Unpin for FixedGasPrice","synthetic":true,"types":[]},{"text":"impl Unpin for MaximumSchedulerWeight","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for LaunchPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for VotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for FastTrackVotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for EnactmentPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CooloffPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for MaxVotes","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for PreimageByteDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for InstantAllowed","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBond","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBondMinimum","synthetic":true,"types":[]},{"text":"impl Unpin for SpendPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CommunityTreasuryId","synthetic":true,"types":[]},{"text":"impl Unpin for ParachainBondPalletId","synthetic":true,"types":[]},{"text":"impl Unpin for MaxApprovals","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionConverter","synthetic":true,"types":[]},{"text":"impl Unpin for ReservedXcmpWeight","synthetic":true,"types":[]},{"text":"impl Unpin for MinBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for BondDuration","synthetic":true,"types":[]},{"text":"impl Unpin for MinSelectedCandidates","synthetic":true,"types":[]},{"text":"impl Unpin for MaxNominatorsPerCollator","synthetic":true,"types":[]},{"text":"impl Unpin for MaxCollatorsPerNominator","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultCollatorCommission","synthetic":true,"types":[]},{"text":"impl Unpin for MinCollatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for MinNominatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for VestingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumReward","synthetic":true,"types":[]},{"text":"impl Unpin for Initialized","synthetic":true,"types":[]},{"text":"impl Unpin for InitializationPayment","synthetic":true,"types":[]},{"text":"impl Unpin for DepositAmount","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProxies","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxPending","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyType","synthetic":true,"types":[]},{"text":"impl Unpin for Runtime","synthetic":true,"types":[]},{"text":"impl Unpin for Event","synthetic":true,"types":[]},{"text":"impl Unpin for Origin","synthetic":true,"types":[]},{"text":"impl Unpin for OriginCaller","synthetic":true,"types":[]},{"text":"impl Unpin for PalletInfo","synthetic":true,"types":[]},{"text":"impl Unpin for Call","synthetic":true,"types":[]},{"text":"impl Unpin for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Unpin for RuntimeApi","synthetic":true,"types":[]},{"text":"impl&lt;Block, C&gt; Unpin for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Block as Block&gt;::Hash: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing: Hasher + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Number: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;C as CallApiAt&lt;Block&gt;&gt;::StateBackend as Backend&lt;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing&gt;&gt;::Transaction: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["moonshadow_runtime"] = [{"text":"impl Unpin for SessionKeys","synthetic":true,"types":[]},{"text":"impl Unpin for BaseFilter","synthetic":true,"types":[]},{"text":"impl Unpin for BlockHashCount","synthetic":true,"types":[]},{"text":"impl Unpin for Version","synthetic":true,"types":[]},{"text":"impl Unpin for BlockWeights","synthetic":true,"types":[]},{"text":"impl Unpin for BlockLength","synthetic":true,"types":[]},{"text":"impl Unpin for SS58Prefix","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MaxLocks","synthetic":true,"types":[]},{"text":"impl Unpin for ExistentialDeposit","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Unpin for DealWithFees&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionByteFee","synthetic":true,"types":[]},{"text":"impl Unpin for MoonbeamGasWeightMapping","synthetic":true,"types":[]},{"text":"impl Unpin for BlockGasLimit","synthetic":true,"types":[]},{"text":"impl Unpin for FixedGasPrice","synthetic":true,"types":[]},{"text":"impl Unpin for MaximumSchedulerWeight","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for CouncilMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMotionDuration","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for TechComitteeMaxMembers","synthetic":true,"types":[]},{"text":"impl Unpin for LaunchPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for VotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for FastTrackVotingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for EnactmentPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CooloffPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for MaxVotes","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProposals","synthetic":true,"types":[]},{"text":"impl Unpin for PreimageByteDeposit","synthetic":true,"types":[]},{"text":"impl Unpin for InstantAllowed","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBond","synthetic":true,"types":[]},{"text":"impl Unpin for ProposalBondMinimum","synthetic":true,"types":[]},{"text":"impl Unpin for SpendPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for CommunityTreasuryId","synthetic":true,"types":[]},{"text":"impl Unpin for ParachainBondPalletId","synthetic":true,"types":[]},{"text":"impl Unpin for MaxApprovals","synthetic":true,"types":[]},{"text":"impl Unpin for TransactionConverter","synthetic":true,"types":[]},{"text":"impl Unpin for ReservedXcmpWeight","synthetic":true,"types":[]},{"text":"impl Unpin for MinBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultBlocksPerRound","synthetic":true,"types":[]},{"text":"impl Unpin for BondDuration","synthetic":true,"types":[]},{"text":"impl Unpin for MinSelectedCandidates","synthetic":true,"types":[]},{"text":"impl Unpin for MaxNominatorsPerCollator","synthetic":true,"types":[]},{"text":"impl Unpin for MaxCollatorsPerNominator","synthetic":true,"types":[]},{"text":"impl Unpin for DefaultCollatorCommission","synthetic":true,"types":[]},{"text":"impl Unpin for MinCollatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for MinNominatorStk","synthetic":true,"types":[]},{"text":"impl Unpin for VestingPeriod","synthetic":true,"types":[]},{"text":"impl Unpin for MinimumReward","synthetic":true,"types":[]},{"text":"impl Unpin for Initialized","synthetic":true,"types":[]},{"text":"impl Unpin for InitializationPayment","synthetic":true,"types":[]},{"text":"impl Unpin for DepositAmount","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxProxies","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositBase","synthetic":true,"types":[]},{"text":"impl Unpin for AnnouncementDepositFactor","synthetic":true,"types":[]},{"text":"impl Unpin for MaxPending","synthetic":true,"types":[]},{"text":"impl Unpin for ProxyType","synthetic":true,"types":[]},{"text":"impl Unpin for Runtime","synthetic":true,"types":[]},{"text":"impl Unpin for Event","synthetic":true,"types":[]},{"text":"impl Unpin for Origin","synthetic":true,"types":[]},{"text":"impl Unpin for OriginCaller","synthetic":true,"types":[]},{"text":"impl Unpin for PalletInfo","synthetic":true,"types":[]},{"text":"impl Unpin for Call","synthetic":true,"types":[]},{"text":"impl Unpin for GenesisConfig","synthetic":true,"types":[]},{"text":"impl Unpin for RuntimeApi","synthetic":true,"types":[]},{"text":"impl&lt;Block, C&gt; Unpin for RuntimeApiImpl&lt;Block, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Block as Block&gt;::Hash: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing: Hasher + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Number: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;C as CallApiAt&lt;Block&gt;&gt;::StateBackend as Backend&lt;&lt;&lt;Block as Block&gt;::Header as Header&gt;::Hashing&gt;&gt;::Transaction: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["pallet_author_mapping"] = [{"text":"impl&lt;AccountId, Balance&gt; Unpin for RegistrationInfo&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Pallet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Event&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AuthorId: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for GenesisConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AuthorId: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AuthorId: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["pallet_ethereum_chain_id"] = [{"text":"impl&lt;T&gt; Unpin for Pallet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for GenesisConfig","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for _GeneratedPrefixForStorageChainId&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["parachain_staking"] = [{"text":"impl&lt;T&gt; Unpin for Range&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Balance&gt; Unpin for InflationInfo&lt;Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for SubstrateWeight&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Pallet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Unpin for Bond&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for CollatorStatus","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Unpin for CollatorSnapshot&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Unpin for Collator&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;AccountId, Balance&gt; Unpin for Nominator&lt;AccountId, Balance&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;BlockNumber&gt; Unpin for RoundInfo&lt;BlockNumber&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;BlockNumber: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Error&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Event&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::BlockNumber: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for GenesisConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for Call&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Config&gt;::AccountId: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for _GeneratedPrefixForStorageAtStake&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for _GeneratedPrefixForStorageStaked&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for _GeneratedPrefixForStorageInflationConfig&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for _GeneratedPrefixForStoragePoints&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for _GeneratedPrefixForStorageAwardedPts&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["precompiles"] = [{"text":"impl&lt;R&gt; Unpin for MoonbeamPrecompiles&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Unpin,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()