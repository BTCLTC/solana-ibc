pub mod impls;
use crate::SolanaIbcStoreHost;

#[derive(Debug)]
pub struct TransferModule;

impl SolanaIbcStoreHost for TransferModule {}
