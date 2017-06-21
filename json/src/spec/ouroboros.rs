// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Ouroboros params deserialization.

use uint::Uint;
use super::ValidatorSet;

/// Ouroboros params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct OuroborosParams {
	/// Time to wait before next block or authority switching, in seconds.
    /// Equivalent to slot duration in the Ouroboros paper.
	#[serde(rename="stepDuration")]
	pub step_duration: Uint,
	/// Validators. Equivalent to stakeholders/leaders in the Ouroboros paper.
	pub validators: ValidatorSet,
    /// Security parameter k. A transaction is declared stable if and only if
    /// it is in a block that is more than this many blocks deep in the
    /// ledger. Equivalent to blkSecurityParam in cardano.
    #[serde(rename="securityParameterK")]
    pub security_parameter_k: u64,
}

/// Ouroboros engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Ouroboros {
	/// Ethash params.
	pub params: OuroborosParams,
}

#[cfg(test)]
mod tests {
	use serde_json;
	use spec::ouroboros::Ouroboros;

	#[test]
	fn ouroboros_deserialization() {
		let s = r#"{
			"params": {
				"stepDuration": "0x02",
				"validators": {
					"list" : ["0xc6d9d2cd449a754c494264e1809c50e34d64562b"]
				},
                "securityParameterK": 60
			}
		}"#;

		let _deserialized: Ouroboros = serde_json::from_str(s).unwrap();
	}
}