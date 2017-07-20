
mod provider {
    // Autogenerated from JSON contract definition using Rust contract convertor.
    #![allow(unused_imports)]
    use std::string::String;
    use std::result::Result;
    use std::fmt;
    use {util, ethabi};
    use util::{FixedHash, Uint};

    pub struct Contract {
    	contract: ethabi::Contract,
    	pub address: util::Address,
    	do_call: Box<Fn(util::Address, Vec<u8>) -> Result<Vec<u8>, String> + Send + Sync + 'static>,
    }

    impl Contract {
    	pub fn new<F>(address: util::Address, do_call: F) -> Self
    		where F: Fn(util::Address, Vec<u8>) -> Result<Vec<u8>, String> + Send + Sync + 'static {
    		Contract {
    			contract: ethabi::Contract::new(ethabi::Interface::load(b"[{\"constant\":false,\"inputs\":[{\"name\":\"epochIndex\",\"type\":\"uint64\"},{\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"getCommitmentsAndShares\",\"outputs\":[{\"name\":\"\",\"type\":\"bytes\"},{\"name\":\"\",\"type\":\"bytes\"}],\"payable\":false,\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"epochIndex\",\"type\":\"uint64\"},{\"name\":\"commitments\",\"type\":\"bytes\"},{\"name\":\"shares\",\"type\":\"bytes\"}],\"name\":\"saveCommitmentsAndShares\",\"outputs\":[],\"payable\":false,\"type\":\"function\"}]").expect("JSON is autogenerated; qed")),
    			address: address,
    			do_call: Box::new(do_call),
    		}
    	}
    	fn as_string<T: fmt::Debug>(e: T) -> String { format!("{:?}", e) }

    	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"epochIndex","type":"uint64"},{"name":"sender","type":"address"}],"name":"getCommitmentsAndShares","outputs":[{"name":"","type":"bytes"},{"name":"","type":"bytes"}],"payable":false,"type":"function"}`
    	#[allow(dead_code)]
    	pub fn get_commitments_and_shares(&self, epoch_index: u64, sender: &util::Address) -> Result<(Vec<u8>, Vec<u8>), String>
    		 {
    		let call = self.contract.function("getCommitmentsAndShares".into()).map_err(Self::as_string)?;
    		let data = call.encode_call(
    			vec![ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U256::from(epoch_index as u64).to_big_endian(&mut r); r }), ethabi::Token::Address(sender.clone().0)]
    		).map_err(Self::as_string)?;
    		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
    		let mut result = output.into_iter().rev().collect::<Vec<_>>();
    		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = r.to_bytes().ok_or("Invalid type returned")?; r }, { let r = result.pop().ok_or("Invalid return arity")?; let r = r.to_bytes().ok_or("Invalid type returned")?; r }))
    	}

    	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"epochIndex","type":"uint64"},{"name":"commitments","type":"bytes"},{"name":"shares","type":"bytes"}],"name":"saveCommitmentsAndShares","outputs":[],"payable":false,"type":"function"}`
    	#[allow(dead_code)]
    	pub fn save_commitments_and_shares(&self, epoch_index: u64, commitments: &[u8], shares: &[u8]) -> Result<(), String>
    		 {
    		let call = self.contract.function("saveCommitmentsAndShares".into()).map_err(Self::as_string)?;
    		let data = call.encode_call(
    			vec![ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U256::from(epoch_index as u64).to_big_endian(&mut r); r }), ethabi::Token::Bytes(commitments.to_owned()), ethabi::Token::Bytes(shares.to_owned())]
    		).map_err(Self::as_string)?;
    		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;

    		Ok(())
    	}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fetches_commitments() {
        // Commitments is empty initially.
		let client = generate_dummy_client_with_spec_and_accounts(Spec::new_pvss_contract, None);
		let vc = Arc::new(PvssContract::new(Address::from_str("0000000000000000000000000000000000000005").unwrap()));
		vc.register_contract(Arc::downgrade(&client));

    }

    #[test]
    fn fetches_reveals() {
        // Reveals is empty initially.
    }

    #[test]
    fn write_commit_messages_and_read_from_chain() {

    }

    #[test]
    fn write_reveal_messages_and_read_from_chain() {

    }
}
