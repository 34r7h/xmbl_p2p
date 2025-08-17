use xmbl_cli::{CliService, CliCommand};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let cli = CliService::new("demo_node_001".to_string());
    
    if args.len() < 2 {
        println!("XMBL CLI - Available commands:");
        println!("  node-info");
        println!("  node-start");
        println!("  node-stop");
        println!("  storage-store <file> <redundancy>");
        println!("  storage-get <cid>");
        println!("  storage-list");
        println!("  storage-stats");
        println!("  compute-submit <wasm_file> <input_file>");
        println!("  compute-status <task_id>");
        println!("  blockchain-balance <address>");
        println!("  blockchain-transfer <from> <to> <amount>");
        println!("  network-peers");
        println!("  network-ping <node_id>");
        println!("  network-status");
        return Ok(());
    }
    
    let command = match args[1].as_str() {
        "node-info" => CliCommand::NodeInfo,
        "node-start" => CliCommand::NodeStart,
        "node-stop" => CliCommand::NodeStop,
        "storage-store" => {
            if args.len() < 4 {
                println!("Usage: storage-store <file> <redundancy>");
                return Ok(());
            }
            CliCommand::StorageStore { 
                file: args[2].clone(), 
                redundancy: args[3].parse().unwrap_or(3) 
            }
        },
        "storage-get" => {
            if args.len() < 3 {
                println!("Usage: storage-get <cid>");
                return Ok(());
            }
            CliCommand::StorageGet { cid: args[2].clone() }
        },
        "storage-list" => CliCommand::StorageList,
        "storage-stats" => CliCommand::StorageStats,
        "compute-submit" => {
            if args.len() < 4 {
                println!("Usage: compute-submit <wasm_file> <input_file>");
                return Ok(());
            }
            CliCommand::ComputeSubmit { 
                wasm_file: args[2].clone(), 
                input_file: args[3].clone() 
            }
        },
        "compute-status" => {
            if args.len() < 3 {
                println!("Usage: compute-status <task_id>");
                return Ok(());
            }
            CliCommand::ComputeStatus { task_id: args[2].clone() }
        },
        "blockchain-balance" => {
            if args.len() < 3 {
                println!("Usage: blockchain-balance <address>");
                return Ok(());
            }
            CliCommand::BlockchainBalance { address: args[2].clone() }
        },
        "blockchain-transfer" => {
            if args.len() < 5 {
                println!("Usage: blockchain-transfer <from> <to> <amount>");
                return Ok(());
            }
            CliCommand::BlockchainTransfer { 
                from: args[2].clone(), 
                to: args[3].clone(), 
                amount: args[4].parse().unwrap_or(0) 
            }
        },
        "network-peers" => CliCommand::NetworkPeers,
        "network-ping" => {
            if args.len() < 3 {
                println!("Usage: network-ping <node_id>");
                return Ok(());
            }
            CliCommand::NetworkPing { node_id: args[2].clone() }
        },
        "network-status" => CliCommand::NetworkStatus,
        _ => {
            println!("Unknown command: {}", args[1]);
            return Ok(());
        }
    };
    
    match cli.run_command(command).await {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
