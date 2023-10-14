use anyhow::Result;
mod jsonrpc;

fn main() -> Result<()> {
    let json_rpc_request: jsonrpc::JsonRpcRequest<'static, &str> = jsonrpc::JsonRpcRequest {
        jsonrpc: "2.0",
        method: jsonrpc::LspMethod::Initialize.to_string(),
        params: vec![],
        id: 1,
    };

    let json_request_body = serde_json::to_string(&json_rpc_request)?;

    println!("{:?}", json_request_body);

    Ok(())
}


