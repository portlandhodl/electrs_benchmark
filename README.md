# ⚡ Electrs Benchmark ⚡

## The Ultimate Electrum Server Performance Testing Tool

![Bitcoin](https://img.shields.io/badge/Bitcoin-000?style=for-the-badge&logo=bitcoin&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

Ever wondered if your Electrum server is as fast as lightning? Or perhaps slower than a turtle carrying a Bitcoin block? **Electrs Benchmark** is here to answer that burning question!

## 🤔 Why This Exists

Electrum servers are the backbone of many Bitcoin wallets and applications, but their performance can vary dramatically based on hardware, configuration, and implementation. This tool was created to:

- Benchmark different Electrum server implementations
- Test server performance under various loads
- Compare response times across different servers
- Help developers optimize their Electrum server deployments
- Provide objective metrics for server performance

Whether you're a Bitcoin node operator, a wallet developer, or just a curious Bitcoiner, this tool gives you the hard data you need to make informed decisions about which Electrum server to use or how to optimize your existing setup.

## 🚀 Features

- **Address UTXO Lookups**: Test how quickly a server can retrieve unspent transaction outputs for Bitcoin addresses
- **Transaction Fetches**: Measure the server's performance when retrieving full transaction data
- **Customizable Sample Sizes**: Adjust the number of tests to run based on your needs
- **Detailed Reports**: Get comprehensive benchmark results including total time and average time per operation
- **CSV Data Support**: Use your own datasets for testing

## 🛠️ Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/electrs_benchmark.git
cd electrs_benchmark

# Build the project
cargo build --release

# The binary will be available at target/release/electrs_benchmark
```

## 📊 How to Use

The benchmark tool is easy to use with sensible defaults, but also highly customizable:

```bash
# Run with default settings (connects to Blockstream's Electrum server)
./target/release/electrs_benchmark

# Specify a different Electrum server
./target/release/electrs_benchmark --server tcp://your-electrum-server.com:50001

# Customize the number of samples
./target/release/electrs_benchmark --address-samples 500 --tx-samples 300

# Get help with all available options
./target/release/electrs_benchmark --help
```

### Command-line Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--server` | `-s` | Electrum server URL | tcp://electrum.blockstream.info:50001 |
| `--address-samples` | `-a` | Number of address UTXO lookups to perform | 100 |
| `--tx-samples` | `-t` | Number of transaction fetches to perform | 100 |

## 📝 Understanding the Results

After running the benchmark, a `benchmark_results.txt` file will be generated with detailed performance metrics:

```
Electrum Benchmark Results - 2025-04-24 00:14:35
--------------------------------------
Address UTXO Lookup:
  Sample Size: 100 addresses
  Total Time: 5.23s
  Average Time per Address: 52.3ms

Transaction Fetch:
  Sample Size: 100 transactions
  Total Time: 3.45s
  Average Time per Transaction: 34.5ms
```

These metrics help you understand:
- How long it takes to process a batch of requests
- The average time per request type
- The relative performance between UTXO lookups and transaction fetches

## 🧪 Preparing Your Own Test Data

The benchmark uses CSV files located in the `benchmark_csv` directory:
- `bitcoin_addresses.csv`: Contains Bitcoin addresses to test UTXO lookups
- `bitcoin_txids.csv`: Contains transaction IDs to test transaction fetches

Each CSV file should have a simple format with a header row and one value per line:

```
value
bc1q...
bc1q...
```

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Add new benchmark types
- Improve the reporting
- Enhance the command-line interface
- Fix bugs or optimize performance

## ⚠️ Disclaimer

This tool is for benchmarking purposes only. Please be respectful of public Electrum servers and avoid running excessive tests that might overload them. Always prefer to benchmark against your own servers when running large tests.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

Happy benchmarking! May your servers be fast and your blocks full! ⚡
