
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bdk::Wallet;
    use bdk::FeeRate;
    use bdk::database::MemoryDatabase;
    use bdk::electrum_client::{Client};
    use bdk::blockchain::{noop_progress, ElectrumBlockchain};
    
    use bitcoin::network::constants::Network;
    use bitcoin::blockdata::transaction::Transaction;
    use bitcoin::util::psbt::PartiallySignedTransaction;
    use bitcoin::util::address::Address;
    
    #[test]
    fn test_fee_absolute_and_rate_with_build_tx(){
      let xkey = "[db7d25b5/84'/1'/6']tpubDCCh4SuT3pSAQ1qAN86qKEzsLoBeiugoGGQeibmieRUKv8z6fCTTmEXsb9yeueBkUWjGVzJr91bCzeCNShorbBqjZV4WRGjz3CrJsCboXUe";
      let deposit_desc = format!("wpkh({}/0/*)", xkey);
      let change_desc = deposit_desc.replace("/0/*","/1/*");
      let to_address = Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt").unwrap();
      let amount = 5_000;
      let fee_rate = FeeRate::from_sat_per_vb(21.1);
      let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    
      let wallet = Wallet::new(
        &deposit_desc,
        Some(&change_desc),
        Network::Testnet,
        MemoryDatabase::default(),
        ElectrumBlockchain::from(client),
      ).unwrap();
      wallet.sync(noop_progress(), None).unwrap();
    
      let (psbt, details) = {
        let mut builder = wallet.build_tx();
        builder
        .enable_rbf()
        .add_recipient(to_address.script_pubkey(),amount)
        .fee_rate(fee_rate);
        builder.finish().unwrap()
      };
    
      let transaction:Transaction = psbt.extract_tx();
      let size = transaction.get_size();
      let fee_absolute = fee_rate.fee_vb(size);
      assert_eq!(fee_absolute,details.fee.unwrap());
    }
}
