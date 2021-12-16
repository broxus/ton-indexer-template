use std::sync::Arc;

use anyhow::Result;
use ton_indexer::utils::*;
use ton_indexer::*;

pub struct TonSubscriber {}

impl TonSubscriber {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

#[async_trait::async_trait]
impl ton_indexer::Subscriber for TonSubscriber {
    async fn process_block(
        &self,
        _meta: BriefBlockMeta,
        _block: &BlockStuff,
        _block_proof: Option<&BlockProofStuff>,
        _shard_state: &ShardStateStuff,
    ) -> Result<()> {
        Ok(())
    }
}
