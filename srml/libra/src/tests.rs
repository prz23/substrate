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
		Aura::e2e_test();
	});
}

#[test]
fn full_test() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {

		let tx: Vec<u8> = Aura::return_a_tx();
		Aura::execute_libra_transaction(tx);
	});
}

#[test]
fn access_test() {
	with_externalities(&mut new_test_ext(vec![0, 1, 2, 3]), || {

		Arua::access_path_test();
		println!("end");
	});
}