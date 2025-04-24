use clap::Parser;

/// Command-line arguments for the Electrum server benchmark tool
#[derive(Parser, Debug)]
#[command(author, version, about = "Benchmark tool for Electrum servers")]
pub struct Args {
    /// Electrum server URL (e.g., tcp://electrum.blockstream.info:50001)
    #[arg(short, long, default_value = "tcp://electrum.blockstream.info:50001")]
    pub server: String,
    
    /// Number of address UTXO lookups to perform
    #[arg(short = 'a', long, default_value_t = 100)]
    pub address_samples: usize,
    
    /// Number of transaction fetches to perform
    #[arg(short = 't', long, default_value_t = 100)]
    pub tx_samples: usize,
}
