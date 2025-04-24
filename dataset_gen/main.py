import csv
import random
import requests
import json
from requests.auth import HTTPBasicAuth

# Bitcoin node configuration
NODE_URL = "http://192.168.1.2:8332"
RPC_USER = "test"
RPC_PASSWORD = "test"

# Number of entries to generate
NUM_ENTRIES = 100000

def make_rpc_call(method, params=None, debug=False):
    """Make an RPC call to the Bitcoin node"""
    headers = {'content-type': 'application/json'}
    payload = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params if params else [],
        "id": 1
    }

    if debug:
        print(f"RPC call: {method} with params {params}")
    
    try:
        response = requests.post(
            NODE_URL,
            headers=headers,
            data=json.dumps(payload),
            auth=HTTPBasicAuth(RPC_USER, RPC_PASSWORD)
        )

        if response.status_code == 200:
            result = response.json()
            if 'result' in result:
                if debug and method != "getblockcount":  # Don't log large responses
                    print(f"RPC response: {method} succeeded")
                return result['result']
            else:
                print(f"RPC error: {result.get('error', 'Unknown error')}")
                return None
        else:
            print(f"HTTP Error: {response.status_code} - {response.text}")
            return None
    except Exception as e:
        print(f"Connection error: {e}")
        return None

def extract_address_from_tx(tx, debug=False):
    """Extract a valid address from a transaction"""
    if not tx:
        return None
        
    # Try to get address from vout
    if 'vout' in tx:
        for vout in tx['vout']:
            if 'scriptPubKey' not in vout:
                continue
                
            script_pub_key = vout['scriptPubKey']
            
            # Try different field names that might contain addresses
            address_fields = ['addresses', 'address']
            for field in address_fields:
                if field in script_pub_key and script_pub_key[field]:
                    if isinstance(script_pub_key[field], list) and script_pub_key[field]:
                        if debug:
                            print(f"Found address in {field} (list): {script_pub_key[field][0]}")
                        return script_pub_key[field][0]
                    elif isinstance(script_pub_key[field], str):
                        if debug:
                            print(f"Found address in {field} (string): {script_pub_key[field]}")
                        return script_pub_key[field]
            
            # Try to extract from asm field for P2PK outputs
            if 'asm' in script_pub_key and script_pub_key.get('type') == 'pubkey':
                if debug:
                    print(f"Found pubkey in asm: {script_pub_key['asm']}")
                # This would need conversion to address, but we'll just use the pubkey as a placeholder
                return f"PUBKEY:{script_pub_key['asm'].split(' ')[0]}"
                
    if debug:
        print(f"No address found in transaction {tx.get('txid', 'unknown')}")
    return None

def get_random_txids_and_addresses(num_entries, debug=False):
    """Get random txids and addresses from the blockchain"""
    txids = []
    addresses = []

    # Get the current block height
    block_height = make_rpc_call("getblockcount")
    if not block_height:
        print("Failed to get block height. Check your Bitcoin node connection.")
        return [], []

    print(f"Current block height: {block_height}")

    # Generate random block heights to sample from
    # Sample more blocks than needed in case some don't have usable transactions
    sample_size = min(num_entries * 3, block_height - 1)
    random_heights = random.sample(range(1, block_height), sample_size)

    print("Fetching data from random blocks...")
    progress_count = 0
    blocks_checked = 0

    for height in random_heights:
        blocks_checked += 1
        if blocks_checked % 10 == 0:
            print(f"Progress: {len(addresses)}/{num_entries} addresses found (checked {blocks_checked}/{sample_size} blocks)")
        
        if len(txids) >= num_entries and len(addresses) >= num_entries:
            break

        # Get block hash
        block_hash = make_rpc_call("getblockhash", [height])
        if not block_hash:
            if debug:
                print(f"Failed to get block hash for height {height}")
            continue

        # Get block data with transaction IDs first (more efficient)
        block_with_txids = make_rpc_call("getblock", [block_hash, 1])
        if not block_with_txids or 'tx' not in block_with_txids or not block_with_txids['tx']:
            if debug:
                print(f"Failed to get transactions for block {height}")
            continue
            
        # Get all transaction IDs from the block
        block_txids = block_with_txids['tx']
        
        # Skip coinbase transaction (first transaction in block) as it doesn't have normal inputs
        if len(block_txids) > 1:
            block_txids = block_txids[1:]
        else:
            if debug:
                print(f"Block {height} only has coinbase transaction, skipping")
            continue
        
        # If we have transactions, select one randomly
        if block_txids:
            # Select a random txid from the block
            selected_txid = random.choice(block_txids)
            
            # Try to get the transaction details
            try:
                # First try getrawtransaction
                tx = make_rpc_call("getrawtransaction", [selected_txid, True], debug)
                
                # If that fails, try gettransaction as fallback
                if not tx:
                    if debug:
                        print(f"getrawtransaction failed for {selected_txid}, trying gettransaction")
                    tx = make_rpc_call("gettransaction", [selected_txid], debug)
                
                if tx and 'txid' in tx:
                    txids.append(tx['txid'])
                    
                    # Try to extract address
                    address = extract_address_from_tx(tx, debug)
                    if address:
                        addresses.append(address)
                        if debug:
                            print(f"Added address: {address}")
                    elif debug:
                        print(f"No address found in transaction {selected_txid}")
                        
                elif debug:
                    print(f"Failed to get transaction details for {selected_txid}")
            except Exception as e:
                if debug:
                    print(f"Error processing transaction {selected_txid}: {e}")

    return txids, addresses

def save_to_csv(filename, data):
    """Save data to CSV file"""
    with open(filename, 'w', newline='') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(['value'])  # Header
        for item in data:
            writer.writerow([item])

def main():
    print("Starting to fetch random Bitcoin txids and addresses...")
    
    # Enable debug mode to see more detailed output
    debug_mode = True
    
    # Try to make a simple RPC call to check connection
    test_connection = make_rpc_call("getblockchaininfo", debug=debug_mode)
    if not test_connection:
        print("Failed to connect to Bitcoin node. Please check your connection settings.")
        print(f"NODE_URL: {NODE_URL}")
        print(f"RPC_USER: {RPC_USER}")
        print("RPC_PASSWORD: [hidden]")
        return

    txids, addresses = get_random_txids_and_addresses(NUM_ENTRIES, debug=debug_mode)
    
    print(f"\nCollection complete. Found {len(txids)} txids and {len(addresses)} addresses.")

    # If we don't have enough data, fill with duplicates (randomly sampled)
    if txids and len(txids) < NUM_ENTRIES:
        print(f"Only found {len(txids)} txids, duplicating to reach {NUM_ENTRIES}")
        while len(txids) < NUM_ENTRIES:
            txids.append(random.choice(txids))
    elif not txids:
        print("Warning: No transaction IDs were found. Check your Bitcoin node connection.")
        txids = ["no_txid_found"] * NUM_ENTRIES

    if addresses and len(addresses) < NUM_ENTRIES:
        print(f"Only found {len(addresses)} addresses, duplicating to reach {NUM_ENTRIES}")
        while len(addresses) < NUM_ENTRIES:
            addresses.append(random.choice(addresses))
    elif not addresses:
        print("Warning: No addresses were found. Check your Bitcoin node connection.")
        addresses = ["no_address_found"] * NUM_ENTRIES

    # Save to CSV files
    save_to_csv('bitcoin_txids.csv', txids[:NUM_ENTRIES])
    save_to_csv('bitcoin_addresses.csv', addresses[:NUM_ENTRIES])

    print(f"Successfully saved {NUM_ENTRIES} txids to 'bitcoin_txids.csv'")
    print(f"Successfully saved {NUM_ENTRIES} addresses to 'bitcoin_addresses.csv'")

if __name__ == "__main__":
    main()
