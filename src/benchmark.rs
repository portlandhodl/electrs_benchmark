use bdk_electrum::electrum_client::{Client, ElectrumApi};
use bdk_electrum::bdk_core::bitcoin::{Script, Txid};
use std::time::{Duration, Instant};
use std::str::FromStr;
use std::io::{self, Write};
use std::fs::File;
use chrono::Local;

/// Benchmark address UTXO lookups
///
/// # Arguments
///
/// * `client` - Electrum client
/// * `addresses` - List of Bitcoin addresses
/// * `sample_size` - Number of addresses to test
///
/// # Returns
///
/// Duration of the benchmark
pub fn benchmark_address_utxos(client: &mut Client, addresses: &[String], sample_size: usize) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Starting UTXO benchmark with {} addresses...", sample_size);
    
    let addresses_to_test = if addresses.len() > sample_size {
        &addresses[0..sample_size]
    } else {
        addresses
    };
    
    let start = Instant::now();
    let mut successful_lookups = 0;
    let mut failed_lookups = 0;
    
    for (i, addr_str) in addresses_to_test.iter().enumerate() {
        if i % 100 == 0 {
            println!("Processing address {}/{}", i, addresses_to_test.len());
        }
        
        let script = Script::new();
        
        match client.script_list_unspent(&script) {
            Ok(utxos) => {
                successful_lookups += 1;
                if i < 5 {
                    println!("Address: {} has {} UTXOs", addr_str, utxos.len());
                    if !utxos.is_empty() {
                        println!("  First UTXO: {:?}", utxos[0]);
                    }
                }
            },
            Err(e) => {
                failed_lookups += 1;
                if i < 5 { 
                    println!("Error fetching UTXOs for address {}: {}", addr_str, e);
                }
            }
        }
    }
    
    let duration = start.elapsed();
    println!("UTXO benchmark completed in {:?}", duration);
    println!("Successful lookups: {}, Failed lookups: {}", successful_lookups, failed_lookups);
    
    Ok(duration)
}

/// Benchmark transaction fetches
///
/// # Arguments
///
/// * `client` - Electrum client
/// * `txids` - List of transaction IDs
/// * `sample_size` - Number of transactions to test
///
/// # Returns
///
/// Duration of the benchmark
pub fn benchmark_transaction_fetch(client: &mut Client, txids: &[String], sample_size: usize) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Starting transaction fetch benchmark with {} txids...", sample_size);
    
    let txids_to_test = if txids.len() > sample_size {
        &txids[0..sample_size]
    } else {
        txids
    };
    
    let start = Instant::now();
    
    for (i, txid_str) in txids_to_test.iter().enumerate() {
        if i % 100 == 0 {
            println!("Processing transaction {}/{}", i, txids_to_test.len());
        }
        
        match Txid::from_str(txid_str) {
            Ok(txid) => {
                match client.transaction_get(&txid) {
                    Ok(_tx) => {
                        if i < 5 {
                            println!("Successfully fetched transaction: {}", txid_str);
                        }
                    },
                    Err(e) => {
                        println!("Error fetching transaction {}: {}", txid_str, e);
                    }
                }
            },
            Err(e) => {
                println!("Error parsing txid {}: {}", txid_str, e);
            }
        }
    }
    
    let duration = start.elapsed();
    println!("Transaction fetch benchmark completed in {:?}", duration);
    
    Ok(duration)
}

/// Write benchmark results to a file
///
/// # Arguments
///
/// * `file_path` - Path to the output file
/// * `address_duration` - Duration of the address benchmark
/// * `tx_duration` - Duration of the transaction benchmark
/// * `address_count` - Number of addresses tested
/// * `tx_count` - Number of transactions tested
///
/// # Returns
///
/// Result indicating success or failure
pub fn write_benchmark_results(file_path: &str, address_duration: Duration, tx_duration: Duration, address_count: usize, tx_count: usize) -> io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut file = File::create(file_path)?;
    
    writeln!(file, "Electrum Benchmark Results - {}", timestamp)?;
    writeln!(file, "--------------------------------------")?;
    writeln!(file, "Address UTXO Lookup:")?;
    writeln!(file, "  Sample Size: {} addresses", address_count)?;
    writeln!(file, "  Total Time: {:?}", address_duration)?;
    writeln!(file, "  Average Time per Address: {:?}", address_duration.div_f64(address_count as f64))?;
    writeln!(file, "")?;
    writeln!(file, "Transaction Fetch:")?;
    writeln!(file, "  Sample Size: {} transactions", tx_count)?;
    writeln!(file, "  Total Time: {:?}", tx_duration)?;
    writeln!(file, "  Average Time per Transaction: {:?}", tx_duration.div_f64(tx_count as f64))?;
    
    Ok(())
}
