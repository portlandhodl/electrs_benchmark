use bdk_electrum::electrum_client::{Client, ElectrumApi};
use clap::Parser;
use electrs_benchmark::{
    Args,
    benchmark_address_utxos,
    benchmark_transaction_fetch,
    write_benchmark_results,
    read_csv_values,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    
    println!("Connecting to Electrum server: {}", args.server);
    let mut client = Client::new(&args.server)?;
    
    println!("Loading addresses from CSV...");
    let addresses = read_csv_values("benchmark_csv/bitcoin_addresses.csv")?;
    println!("Loaded {} addresses", addresses.len());

    println!("Loading transaction IDs from CSV...");
    let txids = read_csv_values("benchmark_csv/bitcoin_txids.csv")?;
    println!("Loaded {} transaction IDs", txids.len());

    println!("Using sample sizes: {} addresses, {} transactions", args.address_samples, args.tx_samples);
    let address_duration = benchmark_address_utxos(&mut client, &addresses, args.address_samples)?;
    let tx_duration = benchmark_transaction_fetch(&mut client, &txids, args.tx_samples)?;

    write_benchmark_results(
        "benchmark_results.txt", 
        address_duration, 
        tx_duration, 
        args.address_samples, 
        args.tx_samples
    )?;
    
    println!("Benchmark results written to benchmark_results.txt");

    Ok(())
}
