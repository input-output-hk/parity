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

//! Spec seal.

use rlp::*;
use util::hash::{H64, H256, H520};
use ethjson;

/// Classic ethereum seal.
pub struct Ethereum {
	/// Seal nonce.
	pub nonce: H64,
	/// Seal mix hash.
	pub mix_hash: H256,
}

impl Into<Generic> for Ethereum {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.mix_hash).append(&self.nonce);
		Generic(s.out())
	}
}

/// AuthorityRound seal.
pub struct AuthorityRound {
	/// Seal step.
	pub step: usize,
	/// Seal signature.
	pub signature: H520,
}

impl Into<Generic> for AuthorityRound {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.step).append(&self.signature);
		Generic(s.out())
	}
}

/// Ouroboros seal.
pub struct Ouroboros {
	/// Seal step.
	pub step: usize,
    /// Step start time.
    pub step_start_time: usize,
	/// Seal signature.
	pub signature: H520,
}

impl Into<Generic> for Ouroboros {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.step).append(&self.step_start_time).append(&self.signature);
		Generic(s.out())
	}
}

/// Tendermint seal.
pub struct Tendermint {
	/// Seal round.
	pub round: usize,
	/// Proposal seal signature.
	pub proposal: H520,
	/// Precommit seal signatures.
	pub precommits: Vec<H520>,
}

impl Into<Generic> for Tendermint {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(3);
		s.append(&self.round).append(&self.proposal).append(&self.precommits);
		Generic(s.out())
	}
}

pub struct Generic(pub Vec<u8>);

/// Genesis seal type.
pub enum Seal {
	/// Classic ethereum seal.
	Ethereum(Ethereum),
	/// AuthorityRound seal.
	AuthorityRound(AuthorityRound),
    /// Ouroboros seal.
    Ouroboros(Ouroboros),
	/// Tendermint seal.
	Tendermint(Tendermint),
	/// Generic RLP seal.
	Generic(Generic),
}

impl From<ethjson::spec::Seal> for Seal {
	fn from(s: ethjson::spec::Seal) -> Self {
		match s {
			ethjson::spec::Seal::Ethereum(eth) => Seal::Ethereum(Ethereum {
				nonce: eth.nonce.into(),
				mix_hash: eth.mix_hash.into()
			}),
			ethjson::spec::Seal::AuthorityRound(ar) => Seal::AuthorityRound(AuthorityRound {
				step: ar.step.into(),
				signature: ar.signature.into()
			}),
			ethjson::spec::Seal::Ouroboros(o) => Seal::Ouroboros(Ouroboros {
				step: o.step.into(),
                step_start_time: o.step_start_time.into(),
				signature: o.signature.into()
			}),
			ethjson::spec::Seal::Tendermint(tender) => Seal::Tendermint(Tendermint {
				round: tender.round.into(),
				proposal: tender.proposal.into(),
				precommits: tender.precommits.into_iter().map(Into::into).collect()
			}),
			ethjson::spec::Seal::Generic(g) => Seal::Generic(Generic(g.into())),
		}
	}
}

impl Into<Generic> for Seal {
	fn into(self) -> Generic {
		match self {
			Seal::Generic(generic) => generic,
			Seal::Ethereum(eth) => eth.into(),
			Seal::AuthorityRound(ar) => ar.into(),
			Seal::Ouroboros(o) => o.into(),
			Seal::Tendermint(tender) => tender.into(),
		}
	}
}
