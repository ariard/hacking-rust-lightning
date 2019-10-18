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

#[macro_use]
extern crate serde_derive;

mod rpc_client;
use rpc_client::*;

mod chain_monitor;
use chain_monitor::*;

mod utils;
use utils::*;

use rand::{thread_rng, Rng};

use lightning::chain::keysinterface::{KeysInterface, KeysManager};
use lightning::util::logger::{Logger, Record};
use lightning::ln::router;

use bitcoin::network::constants;

use futures::future::Future;

use secp256k1::Secp256k1;
use secp256k1::key::PublicKey;

use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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

	// Init fee estimator
	let fee_estimator = Arc::new(FeeEstimator::new());
	
	//XXX Init chain monitor

	// Init router
	
	// Init Channel Manager

	// Init p2p stack - one thread
		



	//XXX Init channel monitoring

	//TODO Launch foreground shell
}
