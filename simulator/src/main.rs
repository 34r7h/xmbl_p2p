use xmbl_simulator::{SimulatorService, SimulationScenario};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 XMBL Network Simulator Starting...");
    println!("=====================================");
    
    // Create simulator service
    let mut simulator = SimulatorService::new("simulator_node_001".to_string());
    
    println!("📊 Simulator initialized with node ID: {}", simulator.node_id);
    println!("");
    
    // Create a test network
    println!("🌐 Creating test network...");
    simulator.create_network(10).await?;
    let (total_nodes, online_nodes) = simulator.get_network_stats();
    println!("✅ Network created with {} total nodes, {} online", total_nodes, online_nodes);
    println!("");
    
    // Run simulation scenarios
    println!("🎯 Running simulation scenarios...");
    
    // Scenario 1: Basic network operation
    println!("1️⃣ Basic Network Operation:");
    let mut params = HashMap::new();
    params.insert("node_count".to_string(), "10".to_string());
    params.insert("test_type".to_string(), "basic".to_string());
    
    let scenario_id = simulator.run_scenario(
        "Basic Network Test".to_string(),
        "Testing basic network connectivity".to_string(),
        params
    ).await?;
    println!("   ✅ Scenario completed with ID: {}", scenario_id);
    
    // Scenario 2: Storage test
    println!("2️⃣ Storage Test:");
    let mut storage_params = HashMap::new();
    storage_params.insert("storage_size_gb".to_string(), "1000".to_string());
    storage_params.insert("redundancy".to_string(), "3".to_string());
    
    let storage_scenario_id = simulator.run_scenario(
        "Storage Test".to_string(),
        "Testing storage operations".to_string(),
        storage_params
    ).await?;
    println!("   ✅ Scenario completed with ID: {}", storage_scenario_id);
    
    // Scenario 3: Compute test
    println!("3️⃣ Compute Test:");
    let mut compute_params = HashMap::new();
    compute_params.insert("compute_flops".to_string(), "1000000000".to_string());
    compute_params.insert("task_count".to_string(), "5".to_string());
    
    let compute_scenario_id = simulator.run_scenario(
        "Compute Test".to_string(),
        "Testing compute operations".to_string(),
        compute_params
    ).await?;
    println!("   ✅ Scenario completed with ID: {}", compute_scenario_id);
    
    println!("");
    
    // Get results
    println!("📈 Scenario Results:");
    if let Some(result) = simulator.get_scenario_results(&scenario_id) {
        println!("  • Basic Test: {}", if result.success { "✅ PASSED" } else { "❌ FAILED" });
        println!("  • Duration: {}ms", result.duration_ms);
        println!("  • Metrics: {:?}", result.metrics);
    }
    
    if let Some(result) = simulator.get_scenario_results(&storage_scenario_id) {
        println!("  • Storage Test: {}", if result.success { "✅ PASSED" } else { "❌ FAILED" });
        println!("  • Duration: {}ms", result.duration_ms);
    }
    
    if let Some(result) = simulator.get_scenario_results(&compute_scenario_id) {
        println!("  • Compute Test: {}", if result.success { "✅ PASSED" } else { "❌ FAILED" });
        println!("  • Duration: {}ms", result.duration_ms);
    }
    
    println!("");
    
    // Continuous monitoring mode
    println!("🔄 Entering continuous monitoring mode...");
    println!("   Press Ctrl+C to stop");
    println!("");
    
    let mut tick = 0;
    loop {
        tick += 1;
        
        // Get current network stats
        let (total, online) = simulator.get_network_stats();
        
        // Print status every 10 seconds
        if tick % 10 == 0 {
            println!("⏰ Tick {} - Network Status:", tick);
            println!("   • Total Nodes: {}", total);
            println!("   • Online Nodes: {}", online);
            println!("   • Health: {:.1}%", (online as f64 / total as f64) * 100.0);
            println!("");
        }
        
        // Simulate some activity by running a quick scenario
        if tick % 30 == 0 {
            let mut quick_params = HashMap::new();
            quick_params.insert("tick".to_string(), tick.to_string());
            
            let _ = simulator.run_scenario(
                "Health Check".to_string(),
                "Periodic health check".to_string(),
                quick_params
            ).await;
        }
        
        // Wait 1 second
        sleep(Duration::from_secs(1)).await;
    }
}
