(function() {var implementors = {};
implementors["moonbeam_rpc_txpool"] = [{"text":"impl&lt;B, C, A&gt; <a class=\"trait\" href=\"moonbeam_rpc_txpool/trait.TxPoolServer.html\" title=\"trait moonbeam_rpc_txpool::TxPoolServer\">TxPoolServer</a> for <a class=\"struct\" href=\"moonbeam_rpc_txpool/struct.TxPool.html\" title=\"struct moonbeam_rpc_txpool::TxPool\">TxPool</a>&lt;B, C, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: ProvideRuntimeApi&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: HeaderMetadata&lt;B, Error = BlockChainError&gt; + HeaderBackend&lt;B&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: BlockT&lt;Hash = H256&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: ChainApi&lt;Block = B&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::Api: <a class=\"trait\" href=\"moonbeam_rpc_primitives_txpool/trait.TxPoolRuntimeApi.html\" title=\"trait moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi\">TxPoolRuntimeApi</a>&lt;B&gt;,&nbsp;</span>","synthetic":false,"types":["moonbeam_rpc_txpool::TxPool"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()