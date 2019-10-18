/// Rust-Lightning daemon skeleton : Code your own lightning node in less than 2 hours (tm) !
/// Open a channel with your non-trusted friends, hand them some payment and try to double-spend the channel !
/// WARNING: The Rust-Lightning developers don't hold any responsibility in any form if you kill the whole LN testnet by mistake, misbehavior or plainly intentionally.

extern crate futures;
extern crate serde_json;
extern crate hyper;
extern crate lightning;
extern crate rand;
extern crate secp256k1;
extern crate bitcoin;
extern crate base64;
extern crate bitcoin_hashes;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_fs;
extern crate tokio_codec;

#[macro_use]
extern crate serde_derive;

mod rpc_client;
use rpc_client::*;

mod utils;
use utils::*;

use rand::{thread_rng, Rng};

use lightning::chain::keysinterface::{KeysInterface, KeysManager};
use lightning::util::logger::{Logger, Record};
use lightning::ln::router;

use bitcoin::network::constants;

use futures::future;
use futures::future::Future;
use futures::Stream;

use secp256k1::Secp256k1;
use secp256k1::key::PublicKey;

use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use std::io::Write;

struct LogPrinter {}
impl Logger for LogPrinter {
	fn log(&self, record: &Record) {
		if !record.args.to_string().contains("Received messages of type 258") && !record.args.to_string().contains("Rceived message of type 256") && !record.args.to_string().contains("Received message of type 257") {
			println!("{:<5} [{} : {}, {}] {}", record.level.to_string(), record.module_path, record.file, record.line, record.args);
		}
	}
}

fn main() {
	// Read arguments  rpcuser, rpcpassword, rpcserver, data_dir 
	if env::args().len() < 3 {  
		println!("USAGE: rust-lightning-jsonrpc user:pass@rpc_host:port storage_directory_path [port]");
		return;
	}

	let rpc_client = {
		let path = env::args().skip(1).next().unwrap();
		let path_parts: Vec<&str> = path.split('@').collect();
		if path_parts.len() != 2 {
			println!("Bad RPC URL provided");
			return;
		}
		println!("User {} Password {}", path_parts[0], path_parts[1]);
		Arc::new(RPCClient::new(path_parts[0], path_parts[1]))
	};

	// Connect to your full-node
	let mut network = constants::Network::Bitcoin;

	println!("Checking validity of RPC URL to bitcoind...");
	let mut thread_rt = tokio::runtime::current_thread::Runtime::new().unwrap();
	thread_rt.block_on(rpc_client.make_rpc_call("getblockchaininfo", &[], false).and_then(|v| {
		//assert!(v["verificationprogress"].as_f64().unwrap() > 0.99);
		//assert_eq!(v["bip9_softforks"]["segwit"]["status"].as_str().unwrap(), "active");

		match v["chain"].as_str().unwrap() {
			"test" => network = constants::Network::Testnet,
			"regtest" => network = constants::Network::Regtest,
			_ => panic!("Unknown network type"),
		}
		Ok(())
	})).unwrap();

	println!("Successfully started up on {}", match network { constants::Network::Bitcoin => "Mainet", constants::Network::Testnet => "Testnet", constants::Network::Regtest => "Regtest"});

	//XXX: Bonus: add persistence to your node by setting a data directory
	
	// Create a logger
	let logger = Arc::new(LogPrinter {});

	//Generate key material
	let secp_ctx = Secp256k1::new();
	let mut seed = [0; 32];
	thread_rng().fill_bytes(&mut seed);
	let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
	let keys = Arc::new(KeysManager::new(&seed, network, logger.clone(), now.as_secs(), now.subsec_nanos()));

	//XXX Init ChainInterface

	//XXX Init Router 
	
	//XXX Init Channel Manager

	//XXX Init PeerManager

	//XXX Init ManyChannelMonitor

	println!("Bound on port 9735! Our node_id: {}", hex_str(&PublicKey::from_secret_key(&secp_ctx, &keys.get_node_secret()).serialize()));
	println!("Started interactive shell! Commands:");
	println!("'c pubkey@host:port' Connect to given host+port, with given pubkey for auth");
	println!("'n pubkey value push_value' Create a channel with the given connected node (by pubkey), value in satoshis, and push the given msat value");
	println!("'k channel_id' Close a channel with the given id");
	println!("'f all' Force close all channels, closing to chain");
	println!("'l p' List the node_ids of all connected peers");
	println!("'l c' List details about all channels");
	println!("'s invoice [amt]' Send payment to an invoice, optionally with amount as whole msat if its not in the invoice");
	println!("'p' Gets a new invoice for receiving funds");
}
