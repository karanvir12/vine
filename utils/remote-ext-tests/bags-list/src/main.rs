// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of vine.

// vine is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// vine is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with vine.  If not, see <http://www.gnu.org/licenses/>.

//! Remote tests for bags-list pallet.

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "PascalCase")]
enum Command {
	CheckMigration,
	SanityCheck,
	Snapshot,
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "PascalCase")]
enum Runtime {
	vine,
	
	
}

#[derive(Parser)]
struct Cli {
	#[arg(long, short, default_value = "wss://-rpc.vine.io:443")]
	uri: String,
	#[arg(long, short, ignore_case = true, value_enum, default_value_t = Runtime::)]
	runtime: Runtime,
	#[arg(long, short, ignore_case = true, value_enum, default_value_t = Command::SanityCheck)]
	command: Command,
	#[arg(long, short)]
	snapshot_limit: Option<usize>,
}

#[tokio::main]
async fn main() {
	let options = Cli::parse();
	sp_tracing::try_init_simple();

	log::info!(
		target: "remote-ext-tests",
		"using runtime {:?} / command: {:?}",
		options.runtime,
		options.command
	);

	use pallet_bags_list_remote_tests::*;
	match options.runtime {
		Runtime::vine => sp_core::crypto::set_default_ss58_version(
			<vine_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime:: => sp_core::crypto::set_default_ss58_version(
			<_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime:: => sp_core::crypto::set_default_ss58_version(
			<_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
	};

	match (options.runtime, options.command) {
		

		(Runtime::vine, Command::CheckMigration) => {
			use vine_runtime::{Block, Runtime};
			use vine_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "vine", options.uri.clone()).await;
		},
		(Runtime::vine, Command::SanityCheck) => {
			use vine_runtime::{Block, Runtime};
			use vine_runtime_constants::currency::UNITS;
			try_state::execute::<Runtime, Block>(UNITS as u64, "vine", options.uri.clone()).await;
		},
		(Runtime::vine, Command::Snapshot) => {
			use vine_runtime::{Block, Runtime};
			use vine_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},
	}
}
