//learnt from https://github.com/kaspanet/rusty-kaspa/blob/master/rpc/grpc/server/src/tests/client_server.rs
//https://docs.rs/kaspa-wrpc-client/0.15.0/kaspa_wrpc_client/
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    KaspaRpcClient,
    WrpcEncoding,
};
use std::{time::Duration, error::Error};

//use Tokio asynchronous runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
    let encoding = WrpcEncoding::Borsh;//Binary object representation serializer for hashing, faster than json

    let url = Some("ws://127.0.0.1:16110");
    let resolver = None;
    let selected_network = None;
    let subscription_context = None;

    //create new rpc client configuration
    let client = KaspaRpcClient::new(encoding, url, resolver, selected_network, subscription_context)?;

    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_secs(5)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };
    client.connect(Some(options)).await?;
    
    //fetch and print node info via rpc
    let info = client.get_server_info().await?;
    println!("Connected to node:\n{:#?}", info);
    client.disconnect().await?;
    Ok(())
}
