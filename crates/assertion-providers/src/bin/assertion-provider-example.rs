//! A standalone provider implementing a new namespace without extending Canon or its parser.
use assertion_providers::contract::Request;
use clap::Parser;
use serde_json::{Value, json};
use std::io::{self, Read};
#[derive(Parser)]
#[command(about = "Example explicitly registered provider: example.echo(value)")]
struct Cli {}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Cli::parse();
    let mut bytes = Vec::new();
    io::stdin().take(1024 * 1024 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > 1024 * 1024 {
        return Err("request exceeds byte budget".into());
    }
    let request: Request = serde_json::from_slice(&bytes)?;
    if request.format != "engineering-provider-request/1" || request.operation != "example.echo" {
        return Err("unsupported request".into());
    }
    let args: Value = serde_json::from_str(&request.arguments_json)?;
    let value = args.get("value").ok_or("missing value")?;
    println!(
        "{}",
        json!({"format":"engineering-provider-response/1","status":"known","value_json":serde_json::to_string(value)?})
    );
    Ok(())
}
