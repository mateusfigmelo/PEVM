//! Chain specific utils

use std::{collections::HashMap, fmt::Debug};

use alloy_primitives::U256;
use alloy_rpc_types::{Header, Transaction};
use revm::primitives::{BlockEnv, SpecId, TxEnv};

use crate::{
    mv_memory::{LazyAddresses, MvMemory},
    BuildIdentityHasher,
};

/// Custom behaviours for different chains & networks
pub trait PevmChain: Debug {
    /// The error type for [Self::get_block_spec].
    type BlockSpecError: Debug + Clone + PartialEq;

    /// The error type for [Self::get_gas_price].
    type GasPriceError: Debug + Clone + PartialEq;

    /// Get chain id.
    fn id(&self) -> u64;

    /// Get block's [SpecId]
    fn get_block_spec(&self, header: &Header) -> Result<SpecId, Self::BlockSpecError>;

    /// Get tx gas price.
    fn get_gas_price(&self, tx: &Transaction) -> Result<U256, Self::GasPriceError>;

    /// Build [MvMemory]
    fn build_mv_memory(
        &self,
        _hasher: &ahash::RandomState,
        _block_env: &BlockEnv,
        txs: &[TxEnv],
    ) -> MvMemory {
        MvMemory::new(
            txs.len(),
            HashMap::with_hasher(BuildIdentityHasher::default()),
            LazyAddresses::default(),
        )
    }
}

mod ethereum;
pub use ethereum::PevmEthereum;