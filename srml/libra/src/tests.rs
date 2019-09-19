// Copyright 2017-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Tests for the module.

#![cfg(test)]

use runtime_io::with_externalities;
use crate::mock::{Aura, new_test_ext};
use types::transaction::SignedTransaction;

#[test]
fn initial_values() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {
        Aura::create_association_account();
	});
}

#[test]
fn txns_test() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {
		Aura::move_contratc_generate_signedtx();
	});
}

#[test]
fn full_test() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {

		let tx: Vec<u8> = Aura::return_a_tx();
		Aura::execute_libra_transaction(tx);
		let tx: Vec<u8> = Aura::return_a_tx();
		Aura::execute_libra_transaction(tx);
		let tx: Vec<u8> = Aura::return_a_tx();
		Aura::execute_libra_transaction(tx);
		println!("{:?}",Aura::libra_hash_map());
	});
}

#[test]
fn access_test() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {

		Aura::access_path_test();
		println!("end");
	});
}

#[test]
fn contract_test() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {
		println!("depoly a contratc start");
        let acc = Aura::init_state();
		let (txn,yl,yl2) = Aura::contract_test(acc);
		let txn_ser = Aura::serialize_a_txn(txn);
		Aura::execute_libra_transaction(txn_ser);
		println!("depoly a contratc");

		println!("a");
		let new_acc = Aura::create_a_new_account();
		Aura::set_account(new_acc.clone());
		println!("b");
		let call_txn = Aura::call_contract_test(new_acc,yl,yl2);
		println!("c");
		let call_txn_ser = Aura::serialize_a_txn(call_txn);
		println!("d");
		Aura::execute_libra_transaction(call_txn_ser);
		println!("end");
	});
}