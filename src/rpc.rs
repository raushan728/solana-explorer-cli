use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub fn get_client(rpc_url: &str) -> RpcClient {
    // Finalized commitment to give secure data
    RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::finalized())
}
