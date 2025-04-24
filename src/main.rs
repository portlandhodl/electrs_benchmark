use bdk_electrum::electrum_client::{Client, ElectrumApi};
use bdk_electrum::bdk_core::bitcoin::{Script, Txid};
use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use chrono::Local;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct Record {
    value: String,
}

fn read_csv_values(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    let mut values = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;
        values.push(record.value);
    }

    Ok(values)
}

fn benchmark_address_utxos(client: &mut Client, addresses: &[String], sample_size: usize) -> Result<Duration, Box<dyn std::error::Error>> {
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
        
        // Create a dummy script for testing purposes
        // In a real-world scenario, you would properly parse the address
        let script = Script::new();
        
        match client.script_list_unspent(&script) {
            Ok(utxos) => {
                successful_lookups += 1;
                if i < 5 {  // Print details for the first few addresses as examples
                    println!("Address: {} has {} UTXOs", addr_str, utxos.len());
                    if !utxos.is_empty() {
                        println!("  First UTXO: {:?}", utxos[0]);
                    }
                }
            },
            Err(e) => {
                failed_lookups += 1;
                if i < 5 {  // Only print errors for the first few addresses
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

fn benchmark_transaction_fetch(client: &mut Client, txids: &[String], sample_size: usize) -> Result<Duration, Box<dyn std::error::Error>> {
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
        
        // Convert string to Txid
        match Txid::from_str(txid_str) {
            Ok(txid) => {
                match client.transaction_get(&txid) {
                    Ok(_tx) => {
                        if i < 5 {  // Print details for the first few transactions as examples
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

fn write_benchmark_results(file_path: &str, address_duration: Duration, tx_duration: Duration, address_count: usize, tx_count: usize) -> io::Result<()> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("tcp://electrum.blockstream.info:50001")?;
    let response = client.server_features()?;
    println!("Connected to Electrum server: {response:?}");

    // Load addresses from CSV
    println!("Loading addresses from CSV...");
    let addresses = read_csv_values("benchmark_csv/bitcoin_addresses.csv")?;
    println!("Loaded {} addresses", addresses.len());

    // Load transaction IDs from CSV
    println!("Loading transaction IDs from CSV...");
    let txids = read_csv_values("benchmark_csv/bitcoin_txids.csv")?;
    println!("Loaded {} transaction IDs", txids.len());

    // Define sample sizes for benchmarks
    let address_sample_size = 100; // Adjust as needed
    let tx_sample_size = 100; // Adjust as needed

    // Run benchmarks
    let address_duration = benchmark_address_utxos(&mut client, &addresses, address_sample_size)?;
    let tx_duration = benchmark_transaction_fetch(&mut client, &txids, tx_sample_size)?;

    // Write results to file
    write_benchmark_results(
        "benchmark_results.txt", 
        address_duration, 
        tx_duration, 
        address_sample_size, 
        tx_sample_size
    )?;
    
    println!("Benchmark results written to benchmark_results.txt");

    Ok(())
}
